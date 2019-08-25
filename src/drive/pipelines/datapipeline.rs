use crate::drive::driveurl::{DriveUrl, MutateUrl};
use crate::drive::ItemResult;
use graph_error::GraphFailure;
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName};
use reqwest::{header, Client, RequestBuilder};
use std::ffi::OsString;
use std::fs::File;

fn client() -> ItemResult<Client> {
    reqwest::Client::builder()
        .build()
        .map_err(GraphFailure::from)
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
    pub(crate) token: String,
    pub url: DriveUrl,
    pub request_type: RequestType,
    pub content_type: String,
    pub body: Option<Body>,
    pub headers: HeaderMap,
    pub upload_session_file: OsString,
}

impl DataPipeline {
    pub fn new(token: &str, url: DriveUrl) -> DataPipeline {
        DataPipeline {
            token: token.into(),
            url,
            request_type: RequestType::Get,
            content_type: "application/json".into(),
            body: None,
            headers: HeaderMap::new(),
            upload_session_file: Default::default(),
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

    pub fn set_upload_session(&mut self, file: OsString) {
        self.upload_session_file = file;
    }

    pub fn header<K>(&mut self, key: K, val: HeaderValue)
    where
        K: IntoHeaderName,
    {
        self.headers.insert(key, val);
    }

    pub fn request_builder(&self) -> ItemResult<RequestBuilder> {
        let mut headers = self.headers.clone();
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_str(self.content_type.as_str()).unwrap(),
        );

        match self.request_type {
            RequestType::Get => {
                let builder = client()?
                    .get(self.url.as_str())
                    .bearer_auth(self.token.as_str())
                    .headers(headers);
                self.merge_body(builder)
            },
            RequestType::Post => {
                let builder = client()?
                    .post(self.url.as_str())
                    .bearer_auth(self.token.as_str())
                    .headers(headers);
                self.merge_body(builder)
            },
            RequestType::Put => {
                let builder = client()?
                    .put(self.url.as_str())
                    .bearer_auth(self.token.as_str())
                    .headers(headers);
                self.merge_body(builder)
            },
            RequestType::Patch => {
                let builder = client()?
                    .patch(self.url.as_str())
                    .bearer_auth(self.token.as_str())
                    .headers(headers);
                self.merge_body(builder)
            },
            RequestType::Delete => {
                let builder = client()?
                    .delete(self.url.as_str())
                    .bearer_auth(self.token.as_str())
                    .headers(headers);
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
