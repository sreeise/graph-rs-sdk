use crate::drive::event::DriveEvent;
use crate::drive::item::IntoItem;
use crate::drive::pipelines::datapipeline::{Body, DataPipeline};
use crate::drive::pipelines::pipeline::Pipeline;
use crate::drive::ItemResult;
use crate::prelude::*;
use graph_error::GraphFailure;
use reqwest::{Client, RequestBuilder};
use serde_json::Value;

pub trait PipelineRequest<T, U> {
    fn send(&self, pipeline: U) -> ItemResult<T>;
}

impl<T, U, F> PipelineRequest<T, U> for F
where
    F: Fn(U) -> ItemResult<T>,
{
    fn send(&self, pipeline: U) -> ItemResult<T> {
        self(pipeline)
    }
}

pub fn pipeline_request<T>() -> impl PipelineRequest<T, DataPipeline>
where
    for<'de> T: serde::Deserialize<'de>,
{
    move |data: DataPipeline| {
        let builder: RequestBuilder = data.request_builder()?;
        let mut response = builder.send()?;

        if let Some(err) = GraphFailure::err_from(&mut response) {
            Err(err)
        } else {
            let value: T = response.json()?;
            Ok(value)
        }
    }
}

pub struct Request<T> {
    item: Box<dyn IntoItem<T>>,
}

impl<T> Request<T> {
    pub fn new(item: Box<dyn IntoItem<T>>) -> Request<T> {
        Request { item }
    }

    pub fn send(&mut self) -> ItemResult<T> {
        self.item.send()
    }

    pub fn client() -> ItemResult<Client> {
        reqwest::Client::builder()
            .build()
            .map_err(GraphFailure::from)
    }
}

impl<T> MutateUrl for Request<T> {}

impl<T> AsRef<DriveUrl> for Request<T> {
    fn as_ref(&self) -> &DriveUrl {
        self.item.as_ref().as_ref()
    }
}

impl<T> AsMut<DriveUrl> for Request<T> {
    fn as_mut(&mut self) -> &mut DriveUrl {
        self.item.as_mut().as_mut()
    }
}

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
        Request::from(Pipeline::new(dr.pipeline.clone(), DriveEvent::Custom))
    }
}

impl MutateUrl for ReqBuilder {}
