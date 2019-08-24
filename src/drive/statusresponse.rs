use crate::drive::drive_item::asyncjobstatus::AsyncJobStatus;
use crate::drive::event::DriveEvent;
use crate::drive::pipelines::pipeline::Pipeline;
use crate::drive::pipelines::request::Request;
use crate::drive::ItemResult;
use crate::graph_error::{GraphError, GraphFailure};
use reqwest::{header, Response};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct StatusResponse {
    event: DriveEvent,
    response: Response,
}

impl StatusResponse {
    pub fn new(event: DriveEvent, response: Response) -> StatusResponse {
        StatusResponse { event, response }
    }

    pub fn drive_event(&self) -> DriveEvent {
        self.event
    }

    pub fn status(&self) -> u16 {
        self.response.status().as_u16()
    }

    pub fn async_job_status(&mut self) -> ItemResult<Option<AsyncJobStatus>> {
        let headers = self.response.headers();
        // The location header contains the URL for monitoring progress.
        let option_location: Option<&reqwest::header::HeaderValue> = headers.get(header::LOCATION);
        if let Some(location) = option_location {
            let location_str = location.to_str().map_err(GraphFailure::from)?;
            let client = reqwest::Client::builder().build()?;
            let mut response = client.get(location_str).send()?;

            let status = response.status().as_u16();
            if GraphError::is_error(status) {
                return Err(GraphFailure::from(
                    GraphError::try_from(status).unwrap_or_default(),
                ));
            }

            let progress: AsyncJobStatus = response.json()?;
            Ok(Some(progress))
        } else {
            Ok(None)
        }
    }

    pub fn success(&mut self) -> bool {
        match self.event {
            DriveEvent::Copy => self.status() == 202,
            _ => self.status() == 204,
        }
    }

    pub fn response(&self) -> &Response {
        &self.response
    }

    pub fn error(&self) -> Option<GraphError> {
        if GraphError::is_error(self.status()) {
            return Some(GraphError::try_from(self.status()).unwrap_or_default());
        }
        None
    }
}

impl From<Pipeline> for Request<StatusResponse> {
    fn from(pipeline: Pipeline) -> Self {
        Request::new(Box::new(pipeline))
    }
}
