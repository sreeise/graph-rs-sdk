use crate::drive::event::{DownloadFormat, DriveEvent};
use crate::drive::intoitem::{IntoFetch, IntoItem, MutateDownload};
use crate::drive::{ItemResponse, ItemResult};
use crate::io::fetch::FetchBuilder;
use crate::prelude::{DriveUrl, MutateUrl};
use graph_error::GraphFailure;
use reqwest::{header, Client, RedirectPolicy, RequestBuilder};
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn client() -> ItemResult<Client> {
    reqwest::Client::builder()
        .build()
        .map_err(GraphFailure::from)
}

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

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RequestType {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

#[derive(Clone, PartialEq)]
pub enum Body {
    String(String),
    File(OsString),
}

#[derive(Clone, PartialEq)]
pub struct DataPipeline {
    token: String,
    pub url: DriveUrl,
    pub request_type: RequestType,
    pub content_type: String,
    pub body: Option<Body>,
}

impl DataPipeline {
    pub fn new(token: &str, url: DriveUrl) -> DataPipeline {
        DataPipeline {
            token: token.into(),
            url,
            request_type: RequestType::Get,
            content_type: "application/json".into(),
            body: None,
        }
    }

    fn merge_body(&self, builder: RequestBuilder) -> ItemResult<RequestBuilder> {
        if let Some(body) = self.body.as_ref() {
            match body {
                Body::String(s) => Ok(builder.body(s.to_string())),
                Body::File(f) => {
                    let file = File::open(f)?;
                    Ok(builder.body(file))
                },
            }
        } else {
            Ok(builder)
        }
    }

    pub fn as_get(&mut self) {
        self.request_type = RequestType::Get;
    }

    pub fn as_post(&mut self) {
        self.request_type = RequestType::Post;
    }

    pub fn as_put(&mut self) {
        self.request_type = RequestType::Put;
    }

    pub fn as_patch(&mut self) {
        self.request_type = RequestType::Patch;
    }

    pub fn as_delete(&mut self) {
        self.request_type = RequestType::Delete;
    }

    pub fn request_builder(&self) -> ItemResult<RequestBuilder> {
        match self.request_type {
            RequestType::Get => {
                let builder = client()?
                    .get(self.url.as_str())
                    .bearer_auth(self.token.as_str())
                    .header(header::CONTENT_TYPE, self.content_type.as_str());
                self.merge_body(builder)
            },
            RequestType::Post => {
                let builder = client()?
                    .post(self.url.as_str())
                    .bearer_auth(self.token.as_str())
                    .header(header::CONTENT_TYPE, self.content_type.as_str());
                self.merge_body(builder)
            },
            RequestType::Put => {
                let builder = client()?
                    .put(self.url.as_str())
                    .bearer_auth(self.token.as_str())
                    .header(header::CONTENT_TYPE, self.content_type.as_str());
                self.merge_body(builder)
            },
            RequestType::Patch => {
                let builder = client()?
                    .patch(self.url.as_str())
                    .bearer_auth(self.token.as_str())
                    .header(header::CONTENT_TYPE, self.content_type.as_str());
                self.merge_body(builder)
            },
            RequestType::Delete => {
                let builder = client()?
                    .delete(self.url.as_str())
                    .bearer_auth(self.token.as_str())
                    .header(header::CONTENT_TYPE, self.content_type.as_str());
                self.merge_body(builder)
            },
        }
    }
}

impl AsMut<DriveUrl> for DataPipeline {
    fn as_mut(&mut self) -> &mut DriveUrl {
        &mut self.url
    }
}

impl AsRef<DriveUrl> for DataPipeline {
    fn as_ref(&self) -> &DriveUrl {
        &self.url
    }
}

impl MutateUrl for DataPipeline {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromToFile)]
pub struct FetchPipeline {
    token: String,
    pub url: DriveUrl,
    pub download_url: String,
    pub is_download: bool,
    pub directory: PathBuf,
    pub file_name: Option<OsString>,
    pub format: Option<DownloadFormat>,
}

impl FetchPipeline {
    pub fn new(directory: PathBuf, url: DriveUrl, token: &str) -> FetchPipeline {
        FetchPipeline {
            token: token.into(),
            url,
            download_url: String::new(),
            is_download: false,
            directory,
            file_name: None,
            format: None,
        }
    }
}

impl From<DataPipeline> for FetchPipeline {
    fn from(data: DataPipeline) -> Self {
        FetchPipeline::new(PathBuf::new(), data.url, data.token.as_str())
    }
}

pub fn pipeline_request<T>() -> impl PipelineRequest<T, DataPipeline>
where
    for<'de> T: serde::Deserialize<'de>,
{
    move |data: DataPipeline| {
        let builder = data.request_builder()?;
        let mut response = builder.send()?;

        if let Some(err) = GraphFailure::err_from(&mut response) {
            Err(err)
        } else {
            let value: T = response.json()?;
            Ok(value)
        }
    }
}

pub fn fetch_redirect() -> impl PipelineRequest<PathBuf, FetchPipeline> {
    move |data: FetchPipeline| {
        let client = reqwest::Client::builder()
            .redirect(RedirectPolicy::custom(|attempt| {
                // There should be only 1 redirect to download a drive item.
                if attempt.previous().len() > 1 {
                    return attempt.too_many_redirects();
                }
                attempt.stop()
            }))
            .build()
            .map_err(GraphFailure::from)?;

        let mut res = client
            .get(data.url.as_str())
            .bearer_auth(data.token.as_str())
            .send()?;

        if let Some(err) = GraphFailure::err_from(&mut res) {
            return Err(err);
        }

        let mut fetch_builder =
            FetchBuilder::new(res.url().as_str(), data.directory, data.token.as_str());

        if let Some(file_name) = data.file_name {
            fetch_builder.rename(file_name);
        }

        if let Some(ext) = data.format {
            fetch_builder.format(ext);
        }

        Ok(fetch_builder.fetch()?)
    }
}

pub fn fetch_download() -> impl PipelineRequest<PathBuf, FetchPipeline> {
    move |data: FetchPipeline| {
        let mut fetch_builder = FetchBuilder::new(
            data.download_url.as_str(),
            data.directory,
            data.token.as_str(),
        );

        if let Some(file_name) = data.file_name {
            fetch_builder.rename(file_name);
        }

        if let Some(ext) = data.format {
            fetch_builder.format(ext);
        }

        Ok(fetch_builder.fetch()?)
    }
}

#[derive(Clone, PartialEq)]
pub struct Pipeline {
    pub pipeline: DataPipeline,
    pub event: DriveEvent,
}

impl Pipeline {
    pub fn new(pipeline: DataPipeline, event: DriveEvent) -> Pipeline {
        Pipeline { pipeline, event }
    }
}

impl AsMut<DataPipeline> for Pipeline {
    fn as_mut(&mut self) -> &mut DataPipeline {
        &mut self.pipeline
    }
}

impl AsRef<DriveEvent> for Pipeline {
    fn as_ref(&self) -> &DriveEvent {
        &self.event
    }
}

impl AsRef<DriveUrl> for Pipeline {
    fn as_ref(&self) -> &DriveUrl {
        &self.pipeline.as_ref()
    }
}

impl AsMut<DriveUrl> for Pipeline {
    fn as_mut(&mut self) -> &mut DriveUrl {
        self.pipeline.as_mut()
    }
}

impl MutateUrl for Pipeline {}

impl<T> IntoItem<T> for Pipeline
where
    for<'de> T: serde::Deserialize<'de>,
{
    fn send(&mut self) -> ItemResult<T> {
        pipeline_request().send(self.pipeline.clone())
    }
}

impl IntoItem<ItemResponse> for Pipeline {
    fn send(&mut self) -> ItemResult<ItemResponse> {
        let builder = self.pipeline.request_builder()?;
        let mut response = builder.send()?;

        if let Some(err) = GraphFailure::err_from(&mut response) {
            return Err(err);
        }

        Ok(ItemResponse::new(self.event, response))
    }
}

#[derive(Clone, PartialEq)]
pub struct DownloadPipeline {
    pub pipeline: FetchPipeline,
    pub is_direct_download: bool,
}

impl MutateDownload for DownloadPipeline {
    fn format(&mut self, format: DownloadFormat) {
        self.pipeline
            .url
            .append_query_pair("format", format.as_ref());
        self.pipeline.format = Some(format);
    }

    fn rename(&mut self, file_name: OsString) {
        self.pipeline.file_name = Some(file_name);
    }
}

impl IntoFetch for DownloadPipeline {
    fn send(&mut self) -> ItemResult<PathBuf> {
        if self.is_direct_download {
            fetch_download().send(self.pipeline.clone())
        } else {
            fetch_redirect().send(self.pipeline.clone())
        }
    }
}
