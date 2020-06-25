use crate::http::{AsyncClient, AsyncIterator, BlockingClient, GraphResponse, HttpByteRange};
use crate::url::GraphUrl;
use async_trait::async_trait;
use graph_error::{GraphFailure, GraphResult};
use reqwest::header::{CONTENT_LENGTH, CONTENT_RANGE, CONTENT_TYPE};
use std::convert::TryFrom;
use std::path::{Path, PathBuf};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    #[serde(rename = "@microsoft.graph.conflictBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_graph_conflict_behavior: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "fileSystemInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

pub trait StartUploadSession<C> {
    fn start_upload_session(&mut self) -> GraphResult<UploadSessionClient<C>>;
}

pub enum NextSession {
    Next(GraphResponse<serde_json::Value>),
    Done(GraphResponse<serde_json::Value>),
}

pub struct UploadSessionClient<C> {
    upload_session_url: String,
    byte_ranges: HttpByteRange,
    client: C,
}

impl<C> UploadSessionClient<C> {
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

    pub async fn set_file_async(&mut self, file: PathBuf) -> GraphResult<()> {
        self.byte_ranges = HttpByteRange::try_from_async(file).await?;
        Ok(())
    }
}

impl UploadSessionClient<BlockingClient> {
    pub fn new(
        upload_session: serde_json::Value,
    ) -> GraphResult<UploadSessionClient<BlockingClient>> {
        let url = upload_session["uploadUrl"].as_str()?;
        Ok(UploadSessionClient {
            upload_session_url: url.to_string(),
            byte_ranges: Default::default(),
            client: BlockingClient::new_blocking(GraphUrl::parse(url)?),
        })
    }

    pub fn cancel(&mut self) -> reqwest::blocking::RequestBuilder {
        self.client
            .inner_client()
            .delete(self.upload_session_url.as_str())
            .header(CONTENT_TYPE, "application/json")
    }

    pub fn status(&mut self) -> GraphResult<reqwest::blocking::Response> {
        self.client
            .inner_client()
            .get(self.upload_session_url.as_str())
            .header(CONTENT_TYPE, "application/json")
            .send()
            .map_err(GraphFailure::from)
    }
}

impl UploadSessionClient<AsyncClient> {
    pub fn new_async(
        upload_session: serde_json::Value,
    ) -> GraphResult<UploadSessionClient<AsyncClient>> {
        let url = upload_session["uploadUrl"].as_str()?;
        Ok(UploadSessionClient {
            upload_session_url: url.to_string(),
            byte_ranges: Default::default(),
            client: AsyncClient::new_async(GraphUrl::parse(url)?),
        })
    }

    pub fn cancel(&mut self) -> reqwest::RequestBuilder {
        self.client
            .inner_client()
            .delete(self.upload_session_url.as_str())
            .header(CONTENT_TYPE, "application/json")
    }

    pub async fn status(&mut self) -> GraphResult<reqwest::Response> {
        self.client
            .inner_client()
            .get(self.upload_session_url.as_str())
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
            .map_err(GraphFailure::from)
    }
}

impl Iterator for UploadSessionClient<BlockingClient> {
    type Item = GraphResult<NextSession>;

    fn next(&mut self) -> Option<Self::Item> {
        let (body, content_length, content_range) = self.byte_ranges.pop_front()?;

        // The Authorization header and bearer token should only be sent
        // when issuing the POST during the first step.
        let response = self
            .client
            .inner_client()
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
                let result = GraphResponse::try_from(response).map_err(GraphFailure::from);
                match result {
                    Ok(value) => return Some(Ok(NextSession::Done(value))),
                    Err(e) => return Some(Err(e)),
                }
            } else {
                let result = GraphResponse::try_from(response).map_err(GraphFailure::from);
                match result {
                    Ok(next) => return Some(Ok(NextSession::Next(next))),
                    Err(e) => return Some(Err(e)),
                }
            }
        } else if let Err(e) = response {
            return Some(Err(e));
        }
        None
    }
}

#[async_trait]
impl AsyncIterator for UploadSessionClient<AsyncClient> {
    type Item = GraphResult<NextSession>;

    async fn next(&mut self) -> Option<Self::Item> {
        let (body, content_length, content_range) = self.byte_ranges.pop_front()?;

        // The Authorization header and bearer token should only be sent
        // when issuing the POST during the first step.
        let response = self
            .client
            .inner_client()
            .put(self.upload_session_url.as_str())
            .header(CONTENT_TYPE, "application/json")
            .header(CONTENT_LENGTH, content_length)
            .header(CONTENT_RANGE, content_range)
            .body(body)
            .send()
            .await
            .map_err(GraphFailure::from);

        if let Ok(response) = response {
            if let Some(e) = GraphFailure::from_async_response(&response) {
                if let Ok(text) = response.text().await {
                    if let Some(e) = e.try_set_graph_error_message(text.as_str()) {
                        return Some(Err(e));
                    }
                }

                return Some(Err(e));
            }

            let status = response.status().as_u16();
            if status.eq(&200) || status.eq(&201) {
                let result = GraphResponse::try_from_async(response)
                    .await
                    .map_err(GraphFailure::from);
                match result {
                    Ok(value) => return Some(Ok(NextSession::Done(value))),
                    Err(e) => return Some(Err(e)),
                }
            } else {
                let result = GraphResponse::try_from_async(response)
                    .await
                    .map_err(GraphFailure::from);
                match result {
                    Ok(next) => return Some(Ok(NextSession::Next(next))),
                    Err(e) => return Some(Err(e)),
                }
            }
        }
        None
    }
}
