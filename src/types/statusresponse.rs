use crate::graph_error::{GraphError, GraphFailure, GraphResult};
use from_as::*;
use reqwest::{header, Response};

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/asyncjobstatus?view=odsp-graph-online
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct AsyncJobStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    operation: Option<String>,
    #[serde(rename = "percentageComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage_complete: Option<f64>,
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(rename = "statusDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    status_description: Option<String>,
}

#[derive(Debug)]
pub struct StatusResponse {
    response: Response,
}

impl StatusResponse {
    pub fn new(response: Response) -> StatusResponse {
        StatusResponse { response }
    }

    pub fn status(&self) -> u16 {
        self.response.status().as_u16()
    }

    pub fn async_job_status(&mut self) -> GraphResult<Option<AsyncJobStatus>> {
        let headers = self.response.headers();
        // The location header contains the URL for monitoring progress.
        let option_location: Option<&reqwest::header::HeaderValue> = headers.get(header::LOCATION);
        if let Some(location) = option_location {
            let location_str = location.to_str().map_err(GraphFailure::from)?;
            let mut response = reqwest::Client::new().get(location_str).send()?;

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
        self.status() == 202 || self.status() == 204
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
