use crate::traits::*;
use crate::url::GraphUrl;
use async_trait::async_trait;
use graph_error::{ErrorMessage, GraphError, GraphFailure, GraphResult};
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
        if let Ok(mut error) = GraphError::try_from(&response) {
            let error_message: GraphResult<ErrorMessage> =
                response.json().map_err(GraphFailure::from);
            if let Ok(message) = error_message {
                error.set_error_message(message);
            }
            Err(GraphFailure::from(error))
        } else {
            let url = GraphUrl::from(response.url());
            let status = response.status().as_u16();
            let headers = response.headers().to_owned();
            let result: GraphResult<String> = response.text().map_err(GraphFailure::from);

            let body = match result {
                Ok(s) => {
                    let value: GraphResult<serde_json::Value> =
                        serde_json::from_str(s.as_str()).map_err(GraphFailure::from);
                    if let Ok(value) = value {
                        value
                    } else {
                        serde_json::Value::String(s)
                    }
                },
                Err(_) => serde_json::Value::String(String::new()),
            };

            Ok(GraphResponse::new(url, body, status, headers))
        }
    }

    pub(crate) async fn async_from_no_content(
        response: reqwest::Response,
    ) -> GraphResult<GraphResponse<serde_json::Value>> {
        if let Ok(mut error) = GraphError::try_from(&response) {
            let error_message: GraphResult<ErrorMessage> =
                response.json().await.map_err(GraphFailure::from);
            if let Ok(message) = error_message {
                error.set_error_message(message);
            }
            Err(GraphFailure::from(error))
        } else {
            let url = GraphUrl::from(response.url());
            let status = response.status().as_u16();
            let headers = response.headers().to_owned();
            let result: GraphResult<String> = response.text().await.map_err(GraphFailure::from);

            let body = match result {
                Ok(s) => {
                    let value: GraphResult<serde_json::Value> =
                        serde_json::from_str(s.as_str()).map_err(GraphFailure::from);
                    if let Ok(value) = value {
                        value
                    } else {
                        serde_json::Value::String(s)
                    }
                },
                Err(_) => serde_json::Value::String(String::new()),
            };

            Ok(GraphResponse::new(url, body, status, headers))
        }
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
            let url = GraphUrl::from(response.url());
            let status = response.status().as_u16();
            let headers = response.headers().to_owned();
            let body: T = response.json()?;
            Ok(GraphResponse::new(url, body, status, headers))
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
            let url = GraphUrl::from(response.url());
            let status = response.status().as_u16();
            let headers = response.headers().to_owned();
            let body: T = response.json().await?;
            Ok(GraphResponse::new(url, body, status, headers))
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
