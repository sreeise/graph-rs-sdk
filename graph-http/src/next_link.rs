use crate::api_impl::GraphUrl;
use crate::traits::ODataNextLink;
use async_trait::async_trait;
use bytes::{BufMut, BytesMut};
use graph_error::{GraphFailure, GraphResult, WithGraphErrorAsync};
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use std::fmt::{Debug, Formatter};
use tokio::sync::mpsc::Receiver;

pub struct NextLink;

#[derive(Deserialize)]
pub struct NextLinkValues<V> {
    pub(crate) value: Vec<V>,
    #[serde(rename = "@odata.nextLink")]
    pub(crate) next_link: Option<String>,
}

pub struct NextLinkResponse<Value> {
    pub url: reqwest::Url,
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Value,
    pub error: Option<GraphFailure>,
}

impl<Value> NextLinkResponse<Value> {
    pub fn new(
        url: reqwest::Url,
        status: StatusCode,
        headers: HeaderMap,
        body: Value,
        error: Option<GraphFailure>,
    ) -> NextLinkResponse<Value> {
        NextLinkResponse {
            url,
            status,
            headers,
            body,
            error,
        }
    }

    pub fn body(&self) -> &Value {
        &self.body
    }

    pub fn into_body(self) -> Value {
        self.body
    }

    pub fn is_err(&self) -> bool {
        self.error.is_some()
    }

    pub fn err(&self) -> Option<&GraphFailure> {
        self.error.as_ref()
    }

    pub fn is_success(&self) -> bool {
        self.status.is_success()
    }
}

impl<V> From<GraphFailure> for NextLinkResponse<Option<V>> {
    fn from(err: GraphFailure) -> Self {
        NextLinkResponse::new(
            GraphUrl::default().to_reqwest_url(),
            StatusCode::BAD_REQUEST,
            HeaderMap::default(),
            None,
            Some(err),
        )
    }
}

impl<V> From<GraphFailure> for NextLinkResponse<Vec<V>> {
    fn from(err: GraphFailure) -> Self {
        NextLinkResponse::new(
            GraphUrl::default().to_reqwest_url(),
            StatusCode::BAD_REQUEST,
            HeaderMap::default(),
            vec![],
            Some(err),
        )
    }
}

impl<V> From<(reqwest::Url, GraphFailure)> for NextLinkResponse<Vec<V>> {
    fn from(value: (reqwest::Url, GraphFailure)) -> Self {
        NextLinkResponse::new(
            value.0,
            StatusCode::BAD_REQUEST,
            HeaderMap::default(),
            vec![],
            Some(value.1),
        )
    }
}

impl<V> From<(String, GraphFailure)> for NextLinkResponse<Vec<V>> {
    fn from(value: (String, GraphFailure)) -> Self {
        NextLinkResponse::new(
            reqwest::Url::parse(value.0.as_str()).unwrap_or(GraphUrl::default().to_reqwest_url()),
            StatusCode::BAD_REQUEST,
            HeaderMap::default(),
            vec![],
            Some(value.1),
        )
    }
}

impl<V> From<(String, GraphFailure)> for NextLinkResponse<Option<V>> {
    fn from(value: (String, GraphFailure)) -> Self {
        NextLinkResponse::new(
            reqwest::Url::parse(value.0.as_str()).unwrap_or(GraphUrl::default().to_reqwest_url()),
            StatusCode::BAD_REQUEST,
            HeaderMap::default(),
            None,
            Some(value.1),
        )
    }
}

impl<Value> Debug for NextLinkResponse<Value>
where
    Value: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NextLinkResponse<Value>")
            .field("url", &self.url)
            .field("status", &self.status)
            .field("headers", &self.headers)
            .field("body", &self.body)
            .field("error", &self.error)
            .finish()
    }
}
