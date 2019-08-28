use crate::drive::drive_item::driveitem::DriveItem;
use crate::drive::drive_item::uploadsession::UploadSession;
use crate::drive::event::DriveEvent;
use crate::drive::pipelines::datapipeline::DataPipeline;
use crate::drive::pipelines::pipeline::Pipeline;
use crate::drive::pipelines::request::PipelineRequest;
use crate::drive::pipelines::request::Request;
use crate::drive::statusresponse::StatusResponse;
use crate::drive::ItemResult;
use crate::io::fetch::ByteRange;
use crate::prelude::DriveUrl;
use graph_error::{GraphError, GraphFailure};
use reqwest::{header, Client};
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::ffi::OsString;
use std::io::ErrorKind;
use url::Url;

pub enum SessionResult {
    Next(UploadSession),
    Done(Box<DriveItem>),
}

#[derive(Clone)]
pub struct UploadSessionPipeline {
    token: String,
    upload_session_url: String,
    file_size: u64,
    file: OsString,
    byte_ranges: VecDeque<(u64, u64, Vec<u8>)>,
    client: Client,
}

impl UploadSessionPipeline {
    pub fn new(
        token: &str,
        upload_session_url: &str,
        file: OsString,
    ) -> ItemResult<UploadSessionPipeline> {
        let mut session = UploadSessionPipeline {
            token: token.into(),
            upload_session_url: upload_session_url.into(),
            file_size: 0,
            file: file.clone(),
            byte_ranges: VecDeque::new(),
            client: Request::<()>::client()?,
        };

        let byte_reader = ByteRange::new(file);
        session.file_size = byte_reader.file_size()?;
        session.byte_ranges = byte_reader.read_to_vec()?;
        Ok(session)
    }

    pub fn set_file(&mut self, file: OsString) -> ItemResult<()> {
        self.file = file.clone();
        let byte_reader = ByteRange::new(file);
        self.file_size = byte_reader.file_size()?;
        self.byte_ranges = byte_reader.read_to_vec()?;
        Ok(())
    }

    pub fn cancel_upload_session(&mut self) -> Request<StatusResponse> {
        let drive_url =
            DriveUrl::try_from(Url::parse(self.upload_session_url.as_str()).unwrap()).unwrap();
        let mut data = DataPipeline::new(self.token.as_str(), drive_url);
        data.as_delete();
        Request::from(Pipeline::new(data, DriveEvent::CancelUploadSession))
    }

    pub fn status(&mut self) -> ItemResult<UploadSession> {
        let mut response = self
            .client
            .get(self.upload_session_url.as_str())
            .header(header::CONTENT_TYPE, "application/json")
            .send()?;

        let status = response.status().as_u16();
        if GraphError::is_error(status) {
            return Err(GraphFailure::from(
                GraphError::try_from(&mut response).unwrap_or_default(),
            ));
        }

        let session: UploadSession = response.json()?;
        Ok(session)
    }

    /// Resume the upload session based on the next byte ranges
    /// returned from the status response.
    pub fn resume(&mut self) -> ItemResult<SessionResult> {
        let session = self.status()?;
        let expected = session
            .next_expected_ranges()
            .clone()
            .ok_or_else(|| GraphFailure::none_err("next_expected_ranges"))?;

        let s = expected
            .get(0)
            .ok_or_else(|| GraphFailure::none_err("next_expected_ranges"))?;
        let mut v: Vec<&str> = s.split("-").collect();
        v.retain(|a| !a.is_empty());
        let s = v
            .get(0)
            .ok_or_else(|| GraphFailure::none_err("next_expected_ranges"))?;

        let start = s.parse::<u64>()?;
        let byte_reader = ByteRange::new(self.file.clone());
        self.file_size = byte_reader.file_size()?;
        self.byte_ranges = byte_reader.read_from(start)?;
        self.next()
    }

    pub fn is_empty(&self) -> bool {
        self.byte_ranges.is_empty()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> ItemResult<SessionResult> {
        if let Some(next) = self.byte_ranges.pop_front() {
            let mut content_length = next.1 - next.0;
            content_length += 1;
            let start = next.0.to_string();
            let end = next.1.to_string();
            let file_size = self.file_size.to_string();
            let content_range = format!(
                "bytes {}-{}/{}",
                start.as_str(),
                end.as_str(),
                file_size.as_str()
            );

            // The Authorization header and bearer token should only be sent
            // when issuing the POST during the first step.
            let mut response = self
                .client
                .put(self.upload_session_url.as_str())
                .header(header::CONTENT_LENGTH, content_length)
                .header(header::CONTENT_RANGE, content_range)
                .header(header::CONTENT_TYPE, "application/json")
                .body(next.2)
                .send()?;

            let status = response.status().as_u16();
            if GraphError::is_error(status) {
                return Err(GraphFailure::from(
                    GraphError::try_from(&mut response).unwrap_or_default(),
                ));
            }

            if status.eq(&200) || status.eq(&201) {
                let drive_item: DriveItem = response.json()?;
                return Ok(SessionResult::Done(Box::new(drive_item)));
            } else {
                let upload_session: UploadSession = response.json()?;
                return Ok(SessionResult::Next(upload_session));
            }
        }

        Err(GraphFailure::error_kind(
            ErrorKind::InvalidData,
            "No available byte ranges for upload session. A file may need to be set.",
        ))
    }
}

pub fn upload_session_pipeline() -> impl PipelineRequest<UploadSessionPipeline, DataPipeline> {
    move |data: DataPipeline| {
        let builder = data.request_builder()?;
        let mut response = builder.send()?;

        if let Some(err) = GraphFailure::err_from(&mut response) {
            Err(err)
        } else {
            let upload_session: UploadSession = response.json()?;
            let url = upload_session.upload_url().clone().unwrap();
            Ok(UploadSessionPipeline::new(
                data.token.as_str(),
                url.as_str(),
                data.upload_session_file,
            )?)
        }
    }
}
