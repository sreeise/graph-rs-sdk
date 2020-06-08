use crate::client::Ident;
use crate::http::{DownloadClient, GraphResponse, UploadSessionClient, BlockingDownload};
use crate::url::GraphUrl;
use crate::GRAPH_URL;
use graph_error::{GraphFailure, GraphResult};
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName, CONTENT_TYPE};
use reqwest::{Method, redirect::Policy};
use std::path::{Path, PathBuf};
use url::Url;
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq)]
pub enum GraphRequestType {
    Basic,
    Redirect,
    Multipart,
}

impl Default for GraphRequestType {
    fn default() -> Self {
        GraphRequestType::Basic
    }
}

pub struct GraphRequestBuilder<B, F> {
    pub url: GraphUrl,
    pub method: Method,
    pub body: Option<B>,
    pub headers: HeaderMap<HeaderValue>,
    pub upload_session_file: Option<PathBuf>,
    pub download_dir: Option<PathBuf>,
    pub form: Option<F>,
    pub req_type: GraphRequestType,
}

pub type BlockingBuilder = GraphRequestBuilder<reqwest::blocking::Body, reqwest::blocking::multipart::Form>;
pub type AsyncBuilder = GraphRequestBuilder<reqwest::Body, reqwest::multipart::Form>;

impl<B, F> GraphRequestBuilder<B, F> {
    pub fn new(url: GraphUrl) -> GraphRequestBuilder<B, F> {
        let mut headers = HeaderMap::default();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        GraphRequestBuilder {
            url,
            method: Default::default(),
            body: None,
            headers,
            upload_session_file: None,
            download_dir: None,
            form: None,
            req_type: Default::default(),
        }
    }

    pub fn url(&self) -> &GraphUrl {
        &self.url
    }

    pub fn to_url(&self) -> Url {
        self.url.to_url()
    }

    pub fn set_url(&mut self, url: GraphUrl) -> &mut Self {
        self.url = url;
        self
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn set_method(&mut self, method: Method) -> &mut Self {
        self.method = method;
        self
    }

    pub fn body(&self) -> Option<&B> {
        self.body.as_ref()
    }

    pub fn set_body(&mut self, body: impl Into<B>) -> &mut Self {
        self.body = Some(body.into());
        self
    }

    pub fn headers(&self) -> &HeaderMap<HeaderValue> {
        &self.headers
    }

    pub fn header(&mut self, name: impl IntoHeaderName, value: HeaderValue) -> &mut Self {
        self.headers.insert(name, value);
        self
    }

    pub fn set_download_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self {
        self.download_dir = Some(dir.as_ref().to_path_buf());
        self
    }

    pub fn set_upload_session<P: AsRef<Path>>(&mut self, file: P) -> &mut Self {
        self.upload_session_file = Some(file.as_ref().to_path_buf());
        self
    }

    pub fn set_form(&mut self, form: F) -> &mut Self {
        self.form = Some(form);
        self.req_type = GraphRequestType::Multipart;
        self
    }

    pub fn set_request_type(&mut self, req_type: GraphRequestType) -> &mut Self {
        self.req_type = req_type;
        self
    }
}

impl<B, F> AsRef<GraphUrl> for GraphRequestBuilder<B, F> {
    fn as_ref(&self) -> &GraphUrl {
        &self.url
    }
}

impl<B, F> AsMut<GraphUrl> for GraphRequestBuilder<B, F> {
    fn as_mut(&mut self) -> &mut GraphUrl {
        &mut self.url
    }
}

impl<B, F> From<Url> for GraphRequestBuilder<B, F> {
    fn from(url: Url) -> Self {
        GraphRequestBuilder::new(GraphUrl::from(url))
    }
}

impl Default for BlockingBuilder {
    fn default() -> Self {
        GraphRequestBuilder::new(GraphUrl::parse(GRAPH_URL).unwrap())
    }
}

pub struct GraphRequest<C> {
    token: String,
    ident: Ident,
    client: C,
    redirect_client: C,
}

impl<C> GraphRequest<C> {
    pub fn set_token(&mut self, token: &str) -> &mut Self {
        self.token = token.to_string();
        self
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn set_ident(&mut self, ident: Ident) -> &mut Self {
        self.ident = ident;
        self
    }

    pub(crate) fn token(&mut self) -> &String {
        &self.token
    }
}

pub type BlockingClient = GraphRequest<reqwest::blocking::Client>;
pub type AsyncClient = GraphRequest<reqwest::Client>;

impl GraphRequest<reqwest::blocking::Client> {
    pub fn new_blocking() -> BlockingClient {
        let redirect_client = reqwest::blocking::Client::builder()
            .redirect(Policy::limited(2))
            .build()
            .map_err(GraphFailure::from)
            .unwrap();

        GraphRequest {
            token: Default::default(),
            ident: Default::default(),
            client: reqwest::blocking::Client::new(),
            redirect_client,
        }
    }

    pub fn download(&mut self, request: BlockingBuilder) -> BlockingDownload {
        DownloadClient::new(self.token.as_str(), request)
    }

    pub fn upload_session(
        &mut self,
        request: BlockingBuilder,
    ) -> GraphResult<UploadSessionClient> {
        let file = request
            .upload_session_file
            .clone()
            .ok_or_else(|| GraphFailure::invalid("file for upload session"))?;
        let mut response = self.response(request)?;
        if let Some(err) = GraphFailure::from_response(&mut response) {
            return Err(err);
        }

        let upload_session: serde_json::Value = response.json()?;
        let mut session = UploadSessionClient::new(upload_session)?;
        session.set_file(file)?;
        Ok(session)
    }

    pub fn build(&mut self, request: BlockingBuilder) -> reqwest::blocking::RequestBuilder {
        match request.req_type {
            GraphRequestType::Basic => {
                if let Some(body) = request.body {
                    self.client
                        .request(request.method, request.url.as_str())
                        .headers(request.headers)
                        .bearer_auth(self.token.as_str())
                        .body(body)
                } else {
                    self.client
                        .request(request.method, request.url.as_str())
                        .headers(request.headers)
                        .bearer_auth(self.token.as_str())
                }
            },
            GraphRequestType::Redirect => {
                if let Some(body) = request.body {
                    self.redirect_client
                        .request(request.method, request.url.as_str())
                        .headers(request.headers)
                        .bearer_auth(self.token.as_str())
                        .body(body)
                } else {
                    self.redirect_client
                        .request(request.method, request.url.as_str())
                        .headers(request.headers)
                        .bearer_auth(self.token.as_str())
                }
            },
            GraphRequestType::Multipart => self
                .client
                .request(request.method, request.url.as_str())
                .headers(request.headers)
                .multipart(request.form.unwrap())
                .bearer_auth(self.token.as_str()),
        }
    }

    pub fn response(&mut self, request: BlockingBuilder) -> GraphResult<reqwest::blocking::Response> {
        let builder = self.build(request);
        let mut response = builder.send()?;
        if let Some(err) = GraphFailure::from_response(&mut response) {
            return Err(err);
        }
        Ok(response)
    }

    pub fn execute<T>(&mut self, request: BlockingBuilder) -> GraphResult<GraphResponse<T>>
        where
                for<'de> T: serde::Deserialize<'de>,
    {
        let response = self.response(request)?;
        GraphResponse::try_from(response)
    }
}

impl GraphRequest<reqwest::Client> {
    pub fn build(&mut self, request: AsyncBuilder) -> reqwest::RequestBuilder {
        match request.req_type {
            GraphRequestType::Basic => {
                if let Some(body) = request.body {
                    self.client
                        .request(request.method, request.url.as_str())
                        .headers(request.headers)
                        .bearer_auth(self.token.as_str())
                        .body(body)
                } else {
                    self.client
                        .request(request.method, request.url.as_str())
                        .headers(request.headers)
                        .bearer_auth(self.token.as_str())
                }
            },
            GraphRequestType::Redirect => {
                if let Some(body) = request.body {
                    self.redirect_client
                        .request(request.method, request.url.as_str())
                        .headers(request.headers)
                        .bearer_auth(self.token.as_str())
                        .body(body)
                } else {
                    self.redirect_client
                        .request(request.method, request.url.as_str())
                        .headers(request.headers)
                        .bearer_auth(self.token.as_str())
                }
            },
            GraphRequestType::Multipart => self
                .client
                .request(request.method, request.url.as_str())
                .headers(request.headers)
                .multipart(request.form.unwrap())
                .bearer_auth(self.token.as_str()),
        }
    }

    pub async fn response(&mut self, request: AsyncBuilder) -> GraphResult<reqwest::Response> {
        let builder = self.build(request);
        let response = builder.send().await?;
        if let Some(err) = GraphFailure::from_async_response(&response) {
            return Err(err);
        }
        Ok(response)
    }

    pub async fn execute<T>(&mut self, request: AsyncBuilder) -> GraphResult<GraphResponse<T>>
        where
                for<'de> T: serde::Deserialize<'de>,
    {
        let response = self.response(request).await?;
        GraphResponse::try_from_async(response).await
    }
}

impl Default for GraphRequest<reqwest::blocking::Client> {
    fn default() -> Self {
        let redirect_client = reqwest::blocking::Client::builder()
            .redirect(Policy::limited(2))
            .build()
            .map_err(GraphFailure::from)
            .unwrap();

        GraphRequest {
            token: Default::default(),
            ident: Default::default(),
            client: reqwest::blocking::Client::new(),
            redirect_client,
        }
    }
}
