use crate::http::HttpByteRange;
use from_as::*;
use graph_error::{GraphFailure, GraphResult};
use graph_rs_types::complextypes::FileSystemInfo;
use graph_rs_types::complextypes::UploadSession;
use reqwest::header::{CONTENT_LENGTH, CONTENT_RANGE, CONTENT_TYPE};
use reqwest::{RequestBuilder, Response};
use std::convert::TryFrom;
use std::path::{Path, PathBuf};

#[derive(Default, Debug, Clone, Serialize, Deserialize, AsFile, FromFile)]
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
    Done((Box<serde_json::Value>, Response)),
}

pub struct UploadSessionClient {
    upload_session_url: String,
    byte_ranges: HttpByteRange,
    client: reqwest::Client,
}

impl UploadSessionClient {
    pub fn new(upload_session: UploadSession) -> GraphResult<UploadSessionClient> {
        let url = upload_session
            .upload_url
            .as_ref()
            .ok_or_else(|| GraphFailure::invalid("upload url from response"))?;
        Ok(UploadSessionClient {
            upload_session_url: url.to_string(),
            byte_ranges: Default::default(),
            client: reqwest::Client::new(),
        })
    }

    pub fn from_range<P: AsRef<Path>>(&mut self, start: u64, end: u64, file: P) -> GraphResult<()> {
        self.byte_ranges = HttpByteRange::from_range(start, end, file)?;
        Ok(())
    }

    pub fn has_next(&self) -> bool {
        !self.byte_ranges.is_empty()
    }

    pub fn set_file(&mut self, file: PathBuf) -> GraphResult<()> {
        self.byte_ranges = HttpByteRange::try_from(file)?;
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
        let (body, content_length, content_range) = self.byte_ranges.pop_front()?;

        // The Authorization header and bearer token should only be sent
        // when issuing the POST during the first step.
        let response = self
            .client
            .put(self.upload_session_url.as_str())
            .header(CONTENT_TYPE, "application/json")
            .header(CONTENT_LENGTH, content_length)
            .header(CONTENT_RANGE, content_range)
            .body(body)
            .send()
            .map_err(GraphFailure::from);

        if let Ok(mut response) = response {
            if let Some(e) = GraphFailure::from_response(&mut response) {
                return Some(Err(e));
            }

            let status = response.status().as_u16();
            if status.eq(&200) || status.eq(&201) {
                let result: GraphResult<serde_json::Value> =
                    response.json().map_err(GraphFailure::from);
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
