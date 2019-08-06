use crate::drive::driveurl::{DriveUrl, MutateUrl};
use crate::drive::event::DriveEvent;
use crate::drive::pipeline::{Body, DataPipeline, Pipeline};
use crate::drive::{ItemResult, Request};
use graph_error::GraphFailure;
use serde_json::Value;

pub struct ReqBuilder {
    pipeline: DataPipeline,
}

impl ReqBuilder {
    pub fn new(pipeline: DataPipeline) -> ReqBuilder {
        ReqBuilder { pipeline }
    }

    pub fn body(&mut self, body: Body) -> &mut Self {
        self.pipeline.body = Some(body);
        self
    }

    pub fn content_type(&mut self, s: &str) -> &mut Self {
        self.pipeline.content_type = s.to_string();
        self
    }

    pub fn send(&mut self) -> ItemResult<Value> {
        let req = self.pipeline.request_builder()?;
        let mut response = req.send()?;

        if let Some(err) = GraphFailure::err_from(&mut response) {
            return Err(err);
        }
        let value: Value = response.json()?;
        Ok(value)
    }
}

impl AsRef<DriveUrl> for ReqBuilder {
    fn as_ref(&self) -> &DriveUrl {
        &self.pipeline.url
    }
}

impl AsMut<DriveUrl> for ReqBuilder {
    fn as_mut(&mut self) -> &mut DriveUrl {
        &mut self.pipeline.url
    }
}

impl<T> From<&ReqBuilder> for Request<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    fn from(dr: &ReqBuilder) -> Self {
        Request::from(Pipeline::new(dr.pipeline.clone(), DriveEvent::GetItem))
    }
}

impl MutateUrl for ReqBuilder {}
