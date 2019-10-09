use crate::types::asyncjobstatus::AsyncJobStatus;
use crate::types::content::Content;
use from_as::TryFrom;
use graph_error::{GraphError, GraphFailure, GraphResult};

#[derive(Debug)]
pub struct GraphResponse<T> {
    value: T,
    response: reqwest::Response,
}

impl<T> GraphResponse<T> {
    pub fn new(response: reqwest::Response, value: T) -> GraphResponse<T> {
        GraphResponse { value, response }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn response(&self) -> &reqwest::Response {
        &self.response
    }

    pub fn response_mut(&mut self) -> &mut reqwest::Response {
        &mut self.response
    }

    pub fn status(&self) -> reqwest::StatusCode {
        self.response.status()
    }

    pub fn success(&mut self) -> bool {
        let status = self.status().as_u16();
        status == 200 || status == 202 || status == 204
    }

    pub fn error(&self) -> Option<GraphError> {
        let status = self.status().as_u16();
        if GraphError::is_error(status) {
            return Some(GraphError::try_from(status).unwrap_or_default());
        }
        None
    }

    pub fn async_job_status(&mut self) -> Option<GraphResult<AsyncJobStatus>> {
        let headers = self.response.headers();
        // The location header contains the URL for monitoring progress.
        let location: &reqwest::header::HeaderValue = headers.get(reqwest::header::LOCATION)?;
        let location_str = location.to_str().ok()?;
        let response = reqwest::Client::new()
            .get(location_str)
            .send()
            .map_err(GraphFailure::from);
        if let Ok(mut response) = response {
            if let Some(err) = GraphFailure::from_response(&mut response) {
                return Some(Err(err));
            }
            Some(response.json().map_err(GraphFailure::from))
        } else if let Err(e) = response {
            Some(Err(e))
        } else {
            None
        }
    }
}

impl<T> AsRef<T> for GraphResponse<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> AsMut<T> for GraphResponse<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> TryFrom<reqwest::Response> for GraphResponse<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    type Error = GraphFailure;

    fn try_from(mut response: reqwest::Response) -> GraphResult<GraphResponse<T>> {
        let value: T = response.json()?;
        Ok(GraphResponse { value, response })
    }
}

impl TryFrom<reqwest::Response> for GraphResponse<Content> {
    type Error = GraphFailure;

    fn try_from(mut value: reqwest::Response) -> Result<Self, Self::Error> {
        if let Ok(content) = value.text() {
            Ok(GraphResponse::new(value, Content::from(content)))
        } else {
            Ok(GraphResponse::new(value, Content::from(String::new())))
        }
    }
}
