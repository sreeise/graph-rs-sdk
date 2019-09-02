use crate::http::ByteRange;
use graph_error::{GraphError, GraphFailure, GraphResult};
use graph_rs_types::complextypes::FileSystemInfo;
use graph_rs_types::complextypes::UploadSession;
use graph_rs_types::entitytypes::DriveItem;
use reqwest::header::{CONTENT_LENGTH, CONTENT_RANGE, CONTENT_TYPE};
use reqwest::{RequestBuilder, Response};
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::path::PathBuf;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    #[serde(rename = "@microsoft.graph.conflictBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_graph_conflict_behavior: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "fileSystemInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_info: Option<FileSystemInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

pub trait StartUploadSession {
    fn start_upload_session(&mut self) -> GraphResult<UploadSessionClient>;
}

pub enum NextSession {
    Next((UploadSession, Response)),
    Done((Box<DriveItem>, Response)),
}

pub struct UploadSessionClient {
    upload_session_url: String,
    file_size: u64,
    file: PathBuf,
    pub byte_ranges: VecDeque<(u64, u64, Vec<u8>)>,
    client: reqwest::Client,
}

impl UploadSessionClient {
    pub fn new(upload_session: UploadSession) -> GraphResult<UploadSessionClient> {
        let url = upload_session
            .upload_url
            .as_ref()
            .ok_or_else(|| GraphFailure::none_err("upload url from response"))?;
        Ok(UploadSessionClient {
            upload_session_url: url.to_string(),
            file_size: 0,
            file: Default::default(),
            byte_ranges: Default::default(),
            client: reqwest::Client::new(),
        })
    }

    pub fn has_next(&self) -> bool {
        !self.byte_ranges.is_empty()
    }

    pub fn set_file(&mut self, file: PathBuf) -> GraphResult<()> {
        self.file = file;
        let byte_reader = ByteRange::new(self.file.clone());
        self.file_size = byte_reader.file_size()?;
        self.byte_ranges = byte_reader.read_to_vec()?;
        Ok(())
    }

    pub fn cancel(&mut self) -> RequestBuilder {
        self.client
            .delete(self.upload_session_url.as_str())
            .header(CONTENT_TYPE, "application/json")
    }

    pub fn status(&mut self) -> GraphResult<Response> {
        self.client
            .get(self.upload_session_url.as_str())
            .header(CONTENT_TYPE, "application/json")
            .send()
            .map_err(GraphFailure::from)
    }
}

impl Iterator for UploadSessionClient {
    type Item = GraphResult<NextSession>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.byte_ranges.pop_front()?;
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
        let response = self
            .client
            .put(self.upload_session_url.as_str())
            .header(CONTENT_TYPE, "application/json")
            .header(CONTENT_LENGTH, content_length)
            .header(CONTENT_RANGE, content_range)
            .body(next.2)
            .send()
            .map_err(GraphFailure::from);

        if let Ok(mut response) = response {
            let status = response.status().as_u16();
            if GraphError::is_error(status) {
                return Some(Err(GraphFailure::from(
                    GraphError::try_from(&mut response).unwrap_or_default(),
                )));
            }

            if status.eq(&200) || status.eq(&201) {
                let result: GraphResult<DriveItem> = response.json().map_err(GraphFailure::from);
                match result {
                    Ok(drive_item) => {
                        return Some(Ok(NextSession::Done((Box::new(drive_item), response))))
                    },
                    Err(e) => return Some(Err(e)),
                }
            } else {
                let result: GraphResult<UploadSession> =
                    response.json().map_err(GraphFailure::from);
                match result {
                    Ok(next) => return Some(Ok(NextSession::Next((next, response)))),
                    Err(e) => return Some(Err(e)),
                }
            }
        } else if let Err(e) = response {
            return Some(Err(e));
        }
        None
    }
}
