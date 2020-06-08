use crate::types::asyncjobstatus::AsyncJobStatus;
use crate::types::content::Content;
use crate::types::delta::{DeltaLink, MetadataLink, NextLink};
use from_as::TryFrom;
use graph_error::{GraphError, GraphFailure, GraphResult};

#[derive(Debug)]
pub struct GraphResponse<T> {
    value: T,
    status: u16,
    headers: reqwest::header::HeaderMap,
}

impl<T> GraphResponse<T> {
    pub fn new(value: T, status: u16, headers: reqwest::header::HeaderMap) -> GraphResponse<T> {
        GraphResponse { value, status, headers }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn into_value(self) -> T {
        self.value
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn success(&mut self) -> bool {
        self.status == 200 || self.status == 202 || self.status == 204
    }

    pub fn error(&self) -> Option<GraphError> {
        if GraphError::is_error(self.status) {
            return Some(GraphError::try_from(self.status).unwrap_or_default());
        }
        None
    }

    pub fn async_job_status(&mut self) -> Option<GraphResult<AsyncJobStatus>> {
        // The location header contains the URL for monitoring progress.
        let location: &reqwest::header::HeaderValue = self.headers.get(reqwest::header::LOCATION)?;
        let location_str = location.to_str().ok()?;
        let response = reqwest::blocking::Client::new()
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

    pub(crate) async fn try_from_async(response: reqwest::Response) -> GraphResult<GraphResponse<T>>
        where
                for<'de> T: serde::Deserialize<'de>,
    {
        let headers = response.headers().clone();
        let status = response.status().as_u16();
        let value: T = response.json().await?;
        Ok(GraphResponse::new(value, status, headers))
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

impl<T> TryFrom<reqwest::blocking::Response> for GraphResponse<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    type Error = GraphFailure;

    fn try_from(response: reqwest::blocking::Response) -> GraphResult<GraphResponse<T>> {
        let headers = response.headers().clone();
        let status = response.status().as_u16();
        let value: T = response.json()?;
        Ok(GraphResponse::new(value, status, headers))
    }
}

impl TryFrom<reqwest::blocking::Response> for GraphResponse<Content> {
    type Error = GraphFailure;

    fn try_from(response: reqwest::blocking::Response) -> Result<Self, Self::Error> {
        let headers = response.headers().clone();
        let status = response.status().as_u16();
        if let Ok(content) = response.text() {
            Ok(GraphResponse::new(Content::from(content), status, headers))
        } else {
            Ok(GraphResponse::new(Content::from(String::new()), status, headers))
        }
    }
}

impl<T> NextLink for GraphResponse<T>
where
    T: NextLink,
{
    fn next_link(&self) -> Option<String> {
        self.value.next_link()
    }
}

impl<T> DeltaLink for GraphResponse<T>
where
    T: DeltaLink,
{
    fn delta_link(&self) -> Option<String> {
        self.value.delta_link()
    }
}

impl<T> MetadataLink for GraphResponse<T>
where
    T: MetadataLink,
{
    fn metadata_link(&self) -> Option<String> {
        self.value.metadata_link()
    }
}
