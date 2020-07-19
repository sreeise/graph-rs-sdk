use crate::types::asyncjobstatus::AsyncJobStatus;
use crate::types::content::Content;
use crate::types::delta::{DeltaLink, MetadataLink, NextLink};
use from_as::TryFrom;
use graph_error::{GraphError, GraphFailure, GraphResult};

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

    pub fn body_to_owned(self) -> T {
        self.body
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn success(&self) -> bool {
        self.status == 200 || self.status == 202 || self.status == 204
    }

    pub fn error(&self) -> Option<GraphError> {
        if GraphError::is_error(self.status) {
            return Some(GraphError::try_from(self.status).unwrap_or_default());
        }
        None
    }

    async fn inner_async_job_status(&mut self) -> Option<GraphResult<AsyncJobStatus>> {
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

    pub fn async_job_status(&mut self) -> Option<GraphResult<AsyncJobStatus>> {
        futures::executor::block_on(self.inner_async_job_status())
    }

    pub(crate) async fn try_from_async(response: reqwest::Response) -> GraphResult<GraphResponse<T>>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let status = response.status().as_u16();
        if GraphError::is_error(status) {
            let possible_error = GraphFailure::from_async_response(&response).unwrap_or_default();
            return Err(GraphFailure::from_async_res_with_err_message(response)
                .await
                .ok_or(possible_error)?);
        }

        let headers = response.headers().clone();
        let body: T = response.json().await?;
        Ok(GraphResponse::new(body, status, headers))
    }
}

impl GraphResponse<Content> {
    pub(crate) async fn try_from_async_content(
        response: reqwest::Response,
    ) -> GraphResult<GraphResponse<Content>> {
        let status = response.status().as_u16();
        if GraphError::is_error(status) {
            let possible_error = GraphFailure::from_async_response(&response).unwrap_or_default();
            return Err(GraphFailure::from_async_res_with_err_message(response)
                .await
                .ok_or(possible_error)?);
        }

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
        let headers = response.headers().clone();
        let status = response.status().as_u16();
        let body: T = response.json()?;
        Ok(GraphResponse::new(body, status, headers))
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
            Ok(GraphResponse::new(
                Content::from(String::new()),
                status,
                headers,
            ))
        }
    }
}

impl<T> NextLink for GraphResponse<T>
where
    T: NextLink,
{
    fn next_link(&self) -> Option<String> {
        self.body.next_link()
    }
}

impl<T> DeltaLink for GraphResponse<T>
where
    T: DeltaLink,
{
    fn delta_link(&self) -> Option<String> {
        self.body.delta_link()
    }
}

impl<T> MetadataLink for GraphResponse<T>
where
    T: MetadataLink,
{
    fn metadata_link(&self) -> Option<String> {
        self.body.metadata_link()
    }
}
