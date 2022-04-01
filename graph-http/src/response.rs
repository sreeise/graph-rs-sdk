use crate::traits::*;
use crate::url::GraphUrl;
use async_trait::async_trait;
use graph_error::WithGraphError;
use graph_error::WithGraphErrorAsync;
use graph_error::{GraphFailure, GraphResult};
use reqwest::{header::HeaderMap, StatusCode};
use serde::de::DeserializeOwned;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct GraphResponse<T> {
    url: GraphUrl,
    body: T,
    status: StatusCode,
    headers: HeaderMap,
}

impl<T> GraphResponse<T> {
    pub fn new(url: GraphUrl, body: T, status: StatusCode, headers: HeaderMap) -> GraphResponse<T> {
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

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn async_job_status(&mut self) -> Option<GraphResult<serde_json::Value>> {
        // The location header contains the URL for monitoring progress.
        self.headers
            .get(reqwest::header::LOCATION)?
            .to_str()
            .ok()
            .map(|location| {
                reqwest::blocking::Client::new()
                    .get(location)
                    .send()?
                    .json()
                    .map_err(GraphFailure::from)
            })
    }

    pub(crate) fn from_no_content(
        response: reqwest::blocking::Response,
    ) -> GraphResult<GraphResponse<serde_json::Value>> {
        let response = response.with_graph_error()?;
        let url = GraphUrl::from(response.url());
        let status = response.status();
        let headers = response.headers().to_owned();
        response
            .text()
            .map(|s| serde_json::from_str(s.as_str()).unwrap_or(serde_json::Value::String(s)))
            .or_else(|_| Ok(serde_json::Value::String(String::new())))
            .map(|body| GraphResponse::new(url, body, status, headers))
    }

    pub(crate) async fn async_from_no_content(
        response: reqwest::Response,
    ) -> GraphResult<GraphResponse<serde_json::Value>> {
        let response = response.with_graph_error().await?;
        let url = GraphUrl::from(response.url());
        let status = response.status();
        let headers = response.headers().to_owned();
        response
            .text()
            .await
            .map(|s| serde_json::from_str(s.as_str()).unwrap_or(serde_json::Value::String(s)))
            .or_else(|_| Ok(serde_json::Value::String(String::new())))
            .map(|body| GraphResponse::new(url, body, status, headers))
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
        let response = response.with_graph_error()?;
        let url = GraphUrl::from(response.url());
        let status = response.status();
        let headers = response.headers().to_owned();
        Ok(GraphResponse::new(url, response.json()?, status, headers))
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
        let response = response.with_graph_error().await?;
        let url = GraphUrl::from(response.url());
        let status = response.status();
        let headers = response.headers().to_owned();

        Ok(GraphResponse::new(
            url,
            response.json().await?,
            status,
            headers,
        ))
    }
}

#[async_trait]
impl<T: DeserializeOwned> AsyncTryFrom<GraphResult<reqwest::Response>> for GraphResponse<T> {
    type Error = GraphFailure;

    async fn async_try_from(result: GraphResult<reqwest::Response>) -> Result<Self, Self::Error> {
        AsyncTryFrom::<reqwest::Response>::async_try_from(result?).await
    }
}
