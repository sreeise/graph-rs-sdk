use crate::traits::*;
use crate::url::GraphUrl;
use async_trait::async_trait;
use graph_error::{AsyncResponseErrorExt, ResponseErrorExt};
use graph_error::{GraphFailure, GraphResult};
use serde::de::DeserializeOwned;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct GraphResponse<T> {
    url: GraphUrl,
    body: T,
    status: u16,
    headers: reqwest::header::HeaderMap,
}

impl<T> GraphResponse<T> {
    pub fn new(
        url: GraphUrl,
        body: T,
        status: u16,
        headers: reqwest::header::HeaderMap,
    ) -> GraphResponse<T> {
        GraphResponse {
            url,
            body,
            status,
            headers,
        }
    }

    pub fn url(&self) -> &GraphUrl {
        &self.url
    }

    pub fn body(&self) -> &T {
        &self.body
    }

    pub fn into_body(self) -> T {
        self.body
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn headers(&self) -> &reqwest::header::HeaderMap {
        &self.headers
    }

    async fn inner_async_job_status(&mut self) -> Option<GraphResult<serde_json::Value>> {
        // The location header contains the URL for monitoring progress.
        let location: &reqwest::header::HeaderValue =
            self.headers.get(reqwest::header::LOCATION)?;
        let location_str = location.to_str().ok()?;
        let response = reqwest::Client::new()
            .get(location_str)
            .send()
            .await
            .map_err(GraphFailure::from);
        if let Ok(response) = response {
            if let Some(err) = GraphFailure::from_async_response(&response) {
                return Some(Err(err));
            }
            Some(response.json().await.map_err(GraphFailure::from))
        } else if let Err(e) = response {
            Some(Err(e))
        } else {
            None
        }
    }

    pub fn async_job_status(&mut self) -> Option<GraphResult<serde_json::Value>> {
        futures::executor::block_on(self.inner_async_job_status())
    }

    pub(crate) fn from_no_content(
        response: reqwest::blocking::Response,
    ) -> GraphResult<GraphResponse<serde_json::Value>> {
        let (url, status, headers, response) = response.map_err_msg()?;
        response
            .text()
            .map(|s| serde_json::from_str(s.as_str()).unwrap_or(serde_json::Value::String(s)))
            .or(Ok(serde_json::Value::String(String::new())))
            .map(|body| GraphResponse::new(GraphUrl::from(url), body, status, headers))
    }

    pub(crate) async fn async_from_no_content(
        response: reqwest::Response,
    ) -> GraphResult<GraphResponse<serde_json::Value>> {
        let (url, status, headers, response) = response.async_map_err_msg().await?;
        response
            .text()
            .await
            .map(|s| serde_json::from_str(s.as_str()).unwrap_or(serde_json::Value::String(s)))
            .or(Ok(serde_json::Value::String(String::new())))
            .map(|body| GraphResponse::new(GraphUrl::from(url), body, status, headers))
    }
}

impl<T> AsRef<T> for GraphResponse<T> {
    fn as_ref(&self) -> &T {
        &self.body
    }
}

impl<T> AsMut<T> for GraphResponse<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.body
    }
}

impl<T: DeserializeOwned> TryFrom<reqwest::blocking::Response> for GraphResponse<T> {
    type Error = GraphFailure;

    fn try_from(response: reqwest::blocking::Response) -> GraphResult<GraphResponse<T>> {
        let (url, status, headers, response) = response.map_err_msg()?;
        response
            .json()
            .map(|body| GraphResponse::new(GraphUrl::from(url), body, status, headers))
            .map_err(GraphFailure::from)
    }
}

impl<T: DeserializeOwned> TryFrom<GraphResult<reqwest::blocking::Response>> for GraphResponse<T> {
    type Error = GraphFailure;

    fn try_from(result: GraphResult<reqwest::blocking::Response>) -> Result<Self, Self::Error> {
        std::convert::TryFrom::try_from(result?)
    }
}

#[async_trait]
impl<T: DeserializeOwned> AsyncTryFrom<reqwest::Response> for GraphResponse<T> {
    type Error = GraphFailure;

    async fn async_try_from(response: reqwest::Response) -> Result<Self, Self::Error> {
        let (url, status, headers, response) = response.async_map_err_msg().await?;
        response
            .json()
            .await
            .map(|body| GraphResponse::new(GraphUrl::from(url), body, status, headers))
            .map_err(GraphFailure::from)
    }
}

#[async_trait]
impl<T: DeserializeOwned> AsyncTryFrom<GraphResult<reqwest::Response>> for GraphResponse<T> {
    type Error = GraphFailure;

    async fn async_try_from(result: GraphResult<reqwest::Response>) -> Result<Self, Self::Error> {
        AsyncTryFrom::<reqwest::Response>::async_try_from(result?).await
    }
}
