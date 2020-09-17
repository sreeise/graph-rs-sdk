use crate::traits::*;
use crate::types::*;
use async_trait::async_trait;
use graph_error::{ErrorMessage, GraphError, GraphFailure, GraphResult};
use reqwest::Response;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct GraphResponse<T> {
    body: T,
    status: u16,
    headers: reqwest::header::HeaderMap,
}

impl<T> GraphResponse<T> {
    pub fn new(body: T, status: u16, headers: reqwest::header::HeaderMap) -> GraphResponse<T> {
        GraphResponse {
            body,
            status,
            headers,
        }
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

impl<T> TryFrom<reqwest::blocking::Response> for GraphResponse<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    type Error = GraphFailure;

    fn try_from(response: reqwest::blocking::Response) -> GraphResult<GraphResponse<T>> {
        if let Ok(mut error) = GraphError::try_from(&response) {
            let error_message: GraphResult<ErrorMessage> =
                response.json().map_err(GraphFailure::from);
            if let Ok(message) = error_message {
                error.set_error_message(message);
            }
            Err(GraphFailure::from(error))
        } else {
            let status = response.status().as_u16();
            let headers = response.headers().to_owned();
            let body: T = response.json()?;
            Ok(GraphResponse::new(body, status, headers))
        }
    }
}

impl TryFrom<reqwest::blocking::Response> for GraphResponse<Content> {
    type Error = GraphFailure;

    fn try_from(response: reqwest::blocking::Response) -> Result<Self, Self::Error> {
        if let Ok(mut error) = GraphError::try_from(&response) {
            let error_message: GraphResult<ErrorMessage> =
                response.json().map_err(GraphFailure::from);
            if let Ok(message) = error_message {
                error.set_error_message(message);
            }
            return Err(GraphFailure::from(error));
        }

        let headers = response.headers().to_owned();
        let status = response.status().as_u16();
        if let Ok(content) = response.text() {
            Ok(GraphResponse::new(Content::from(content), status, headers))
        } else {
            Ok(GraphResponse::new(
                Content::from(String::new()),
                status,
                headers,
            ))
        }
    }
}

impl<T> TryFrom<GraphResult<reqwest::blocking::Response>> for GraphResponse<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    type Error = GraphFailure;

    fn try_from(result: GraphResult<reqwest::blocking::Response>) -> Result<Self, Self::Error> {
        std::convert::TryFrom::try_from(result?)
    }
}

impl TryFrom<GraphResult<reqwest::blocking::Response>> for GraphResponse<Content> {
    type Error = GraphFailure;

    fn try_from(result: GraphResult<reqwest::blocking::Response>) -> Result<Self, Self::Error> {
        std::convert::TryFrom::try_from(result?)
    }
}

#[async_trait]
impl<T> AsyncTryFrom<reqwest::Response> for GraphResponse<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    type Error = GraphFailure;

    async fn async_try_from(response: reqwest::Response) -> Result<Self, Self::Error> {
        if let Ok(mut error) = GraphError::try_from(&response) {
            let error_message: GraphResult<ErrorMessage> =
                response.json().await.map_err(GraphFailure::from);
            if let Ok(message) = error_message {
                error.set_error_message(message);
            }
            Err(GraphFailure::from(error))
        } else {
            let status = response.status().as_u16();
            let headers = response.headers().to_owned();
            let body: T = response.json().await?;
            Ok(GraphResponse::new(body, status, headers))
        }
    }
}

#[async_trait]
impl AsyncTryFrom<reqwest::Response> for GraphResponse<Content> {
    type Error = GraphFailure;

    async fn async_try_from(response: Response) -> Result<Self, Self::Error> {
        if let Ok(mut error) = GraphError::try_from(&response) {
            let error_message: GraphResult<ErrorMessage> =
                response.json().await.map_err(GraphFailure::from);
            if let Ok(message) = error_message {
                error.set_error_message(message);
            }
            return Err(GraphFailure::from(error));
        }

        let status = response.status().as_u16();
        let headers = response.headers().clone();
        if let Ok(content) = response.text().await {
            Ok(GraphResponse::new(Content::from(content), status, headers))
        } else {
            Ok(GraphResponse::new(
                Content::from(String::new()),
                status,
                headers,
            ))
        }
    }
}

#[async_trait]
impl<T> AsyncTryFrom<GraphResult<reqwest::Response>> for GraphResponse<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    type Error = GraphFailure;

    async fn async_try_from(result: GraphResult<reqwest::Response>) -> Result<Self, Self::Error> {
        AsyncTryFrom::<reqwest::Response>::async_try_from(result?).await
    }
}

#[async_trait]
impl AsyncTryFrom<GraphResult<reqwest::Response>> for GraphResponse<Content> {
    type Error = GraphFailure;

    async fn async_try_from(result: GraphResult<reqwest::Response>) -> Result<Self, Self::Error> {
        AsyncTryFrom::<reqwest::Response>::async_try_from(result?).await
    }
}
