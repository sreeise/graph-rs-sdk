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

    /// Get the URL.
    pub fn url(&self) -> &GraphUrl {
        &self.url
    }

    /// Get a reference to the body.
    pub fn body(&self) -> &T {
        &self.body
    }

    /// Returns the body and ownership of it and dropping the
    /// GraphResponse in the process.
    pub fn into_body(self) -> T {
        self.body
    }

    /// Get the HTTP status code of the response.
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Get the headers returned of the response.
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// This makes a request to the location header returned in this response
    /// for monitoring progress of long running actions.
    ///
    /// This is a blocking request and not asycc. Use the method `async_job_status`
    /// if you are using the async graph client.
    pub fn job_status(&self) -> Option<GraphResult<GraphResponse<serde_json::Value>>> {
        let url = self.headers.get(reqwest::header::LOCATION)?.to_str().ok()?;

        let result = reqwest::blocking::Client::new()
            .get(url)
            .send()
            .map_err(GraphFailure::from);

        if let Err(e) = result {
            return Some(Err(e));
        } else if let Ok(response) = result {
            return Some(GraphResponse::try_from(response));
        }

        None
    }

    /// This makes a request to the location header returned in this response
    /// for monitoring progress of long running actions.
    ///
    /// This is a async request and not blocking. Use the method `job_status` if you
    /// are using the blocking graph client.
    pub async fn async_job_status(&self) -> Option<GraphResult<GraphResponse<serde_json::Value>>> {
        let url = self.headers.get(reqwest::header::LOCATION)?.to_str().ok()?;

        let result = reqwest::Client::new()
            .get(url)
            .send()
            .await
            .map_err(GraphFailure::from);

        if let Err(e) = result {
            return Some(Err(e));
        } else if let Ok(response) = result {
            return Some(GraphResponse::async_try_from(response).await);
        }

        None
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
