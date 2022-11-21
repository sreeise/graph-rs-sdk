use crate::async_client::{AsyncClient, AsyncHttpClient};
use crate::blocking_client::{BlockingClient, BlockingHttpClient};
use crate::byte_range::ByteRangeIterator;
use crate::traits::*;
use crate::url::GraphUrl;
use crate::{GraphResponse, RequestAttribute, RequestClient};
use async_trait::async_trait;
use graph_error::{GraphFailure, GraphResult, WithGraphError, WithGraphErrorAsync};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_LENGTH, CONTENT_RANGE, CONTENT_TYPE};
use reqwest::StatusCode;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::path::Path;

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

impl NextSession {
    fn from_response(
        res: (StatusCode, GraphResult<GraphResponse<serde_json::Value>>),
    ) -> Option<GraphResult<NextSession>> {
        if let Ok(value) = res.1 {
            return if res.0.eq(&200) || res.0.eq(&201) {
                Some(Ok(NextSession::Done(value)))
            } else {
                Some(Ok(NextSession::Next(value)))
            };
        } else if let Err(e) = res.1 {
            return Some(Err(e));
        }
        None
    }
}

pub struct UploadSessionClient<C> {
    upload_session_url: String,
    byte_ranges: ByteRangeIterator,
    client: C,
}

impl<C> UploadSessionClient<C> {
    pub fn from_range<P: AsRef<Path>>(&mut self, start: u64, end: u64, file: P) -> GraphResult<()> {
        self.byte_ranges = ByteRangeIterator::from_range(start, end, file)?;
        Ok(())
    }

    pub fn has_next(&self) -> bool {
        !self.byte_ranges.is_empty()
    }
}

impl<C> UploadSessionClient<C>
where
    C: RequestClient,
{
    // The Authorization header and bearer token should only be sent
    // when issuing the POST during the first step.
    fn build_next_request(&self, body: Vec<u8>, content_length: u64, content_range: String) {
        let mut header_map = HeaderMap::new();
        header_map.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        header_map.insert(
            CONTENT_LENGTH,
            HeaderValue::from_str(&content_length.to_string()).unwrap(),
        );
        header_map.insert(
            CONTENT_RANGE,
            HeaderValue::from_str(content_range.as_str()).unwrap(),
        );

        self.client
            .set_request(vec![
                RequestAttribute::Headers(header_map),
                RequestAttribute::Method(reqwest::Method::PUT),
                RequestAttribute::Body(body.into()),
            ])
            .unwrap();
    }
}

impl<C> Debug for UploadSessionClient<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UploadSessionClient")
            .field("upload_session_url", &self.upload_session_url)
            .field("byte_ranges", &self.byte_ranges)
            .finish()
    }
}

impl UploadSessionClient<BlockingHttpClient> {
    pub fn new(
        upload_session: serde_json::Value,
    ) -> GraphResult<UploadSessionClient<BlockingHttpClient>> {
        let url = upload_session["uploadUrl"]
            .as_str()
            .ok_or_else(|| GraphFailure::not_found("no \"uploadUrl\""))?;
        Ok(UploadSessionClient {
            upload_session_url: url.to_string(),
            byte_ranges: Default::default(),
            client: BlockingHttpClient::from(BlockingClient::new_blocking(GraphUrl::parse(url)?)),
        })
    }

    pub fn set_file<P: AsRef<Path>>(&mut self, file: P) -> GraphResult<()> {
        self.byte_ranges = ByteRangeIterator::try_from(file.as_ref().to_path_buf())?;
        Ok(())
    }

    pub fn cancel(&mut self) -> reqwest::blocking::RequestBuilder {
        self.client.set_method(reqwest::Method::DELETE);
        self.client.build()
    }

    pub fn status(&mut self) -> GraphResult<reqwest::blocking::Response> {
        self.client.response()
    }
}

impl Iterator for UploadSessionClient<BlockingHttpClient> {
    type Item = GraphResult<NextSession>;

    fn next(&mut self) -> Option<Self::Item> {
        let (body, content_length, content_range) = self.byte_ranges.pop_front()?;
        self.build_next_request(body, content_length, content_range);
        match self.client.response() {
            Ok(response) => match response.with_graph_error() {
                Ok(response) => {
                    let status = response.status();
                    let result = std::convert::TryFrom::try_from(response);
                    NextSession::from_response((status, result))
                }
                Err(e) => Some(Err(e.into())),
            },
            Err(e) => Some(Err(e)),
        }
    }
}

impl UploadSessionClient<AsyncHttpClient> {
    pub fn new_async(
        upload_session: serde_json::Value,
    ) -> GraphResult<UploadSessionClient<AsyncHttpClient>> {
        let url = upload_session["uploadUrl"]
            .as_str()
            .ok_or_else(|| GraphFailure::not_found("no \"uploadUrl\""))?;
        Ok(UploadSessionClient {
            upload_session_url: url.to_string(),
            byte_ranges: Default::default(),
            client: AsyncHttpClient::from(AsyncClient::new_async(GraphUrl::parse(url)?)),
        })
    }

    pub async fn set_file<P: AsRef<Path>>(&mut self, file: P) -> GraphResult<()> {
        self.byte_ranges = ByteRangeIterator::async_try_from(file.as_ref().to_path_buf()).await?;
        Ok(())
    }

    pub async fn cancel(&mut self) -> reqwest::RequestBuilder {
        self.client.set_method(reqwest::Method::DELETE);
        self.client.build()
    }

    pub async fn status(&mut self) -> GraphResult<reqwest::Response> {
        self.client.response().await
    }
}

#[async_trait]
impl AsyncIterator for UploadSessionClient<AsyncHttpClient> {
    type Item = GraphResult<NextSession>;

    async fn next(&mut self) -> Option<Self::Item> {
        let (body, content_length, content_range) = self.byte_ranges.pop_front()?;
        self.build_next_request(body, content_length, content_range);
        let request_builder = self.client.build();
        let result = request_builder.send().await.map_err(GraphFailure::from);
        match result {
            Ok(response) => match response.with_graph_error().await {
                Ok(response) => {
                    let status = response.status();
                    let result = AsyncTryFrom::<reqwest::Response>::async_try_from(response).await;
                    NextSession::from_response((status, result))
                }
                Err(e) => Some(Err(e.into())),
            },
            Err(e) => Some(Err(e)),
        }
    }
}
