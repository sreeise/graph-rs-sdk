use crate::client::Ident;
use crate::http::{
    AsyncDownload, BlockingDownload, DownloadClient, GraphResponse, UploadSessionClient,
};
use crate::url::GraphUrl;
use crate::GRAPH_URL;
use graph_error::{GraphFailure, GraphResult};
use handlebars::Handlebars;
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName, CONTENT_TYPE};
use reqwest::{redirect::Policy, Method};
use serde_json::Value;
use std::cell::RefCell;
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use url::Url;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

pub enum RequestAttribute<Body, Form> {
    Token(String),
    Ident(Ident),
    Url(GraphUrl),
    Method(reqwest::Method),
    Body(Body),
    BodyFile(PathBuf),
    Headers(HeaderMap),
    ClearHeaders,
    Download(PathBuf),
    Upload(PathBuf),
    Form(Form),
    RequestType(GraphRequestType),
}

pub trait RequestClient {
    type Body: From<String> + From<Vec<u8>> + From<&'static [u8]> + From<&'static str>;
    type Form;

    fn token(&self) -> String;
    fn set_token(&self, token: &str);
    fn ident(&self) -> Ident;
    fn set_ident(&self, ident: Ident);
    fn url(&self) -> GraphUrl;
    fn to_url(&self) -> Url;
    fn set_url(&self, url: GraphUrl);
    fn method(&self) -> Method;
    fn set_method(&self, method: Method);
    fn set_body<T: Into<Self::Body>>(&self, body: T);
    fn set_body_with_file(&self, path: PathBuf) -> GraphResult<()>;
    fn header<T: IntoHeaderName>(&self, name: T, value: HeaderValue);
    fn set_header_map(&self, header_map: HeaderMap);
    fn clear_headers(&self);
    fn set_download_dir(&self, dir: PathBuf);
    fn set_upload_session(&self, file: PathBuf);
    fn set_form(&self, form: Self::Form);
    fn set_request_type(&self, req_type: GraphRequestType);
    fn request_type(&self) -> GraphRequestType;
    fn url_ref<F>(&self, f: F)
    where
        F: Fn(&GraphUrl) + Sync;
    fn url_mut<F>(&self, f: F)
    where
        F: Fn(&mut GraphUrl) + Sync;
    fn registry<F>(&self, f: F)
    where
        F: Fn(&mut Handlebars) + Sync;
    fn render_template(&self, template: &str, json: &serde_json::Value) -> String;
    fn extend_path(&self, path: &[&str]);
    fn set_request(
        &self,
        req_attr: Vec<RequestAttribute<Self::Body, Self::Form>>,
    ) -> GraphResult<()>;
}

pub struct GraphRequest<Client, Body, Form> {
    token: String,
    ident: Ident,
    client: Client,
    pub url: GraphUrl,
    pub method: Method,
    pub body: Option<Body>,
    pub headers: HeaderMap<HeaderValue>,
    pub upload_session_file: Option<PathBuf>,
    pub download_dir: Option<PathBuf>,
    pub form: Option<Form>,
    pub req_type: GraphRequestType,
    pub registry: Handlebars,
}

impl<Client, Body, Form> Debug for GraphRequest<Client, Body, Form> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphRequest")
            .field("ident", &self.ident)
            .field("url", &self.url)
            .field("method", &self.method)
            .field("headers", &self.headers)
            .field("upload_session_file", &self.upload_session_file)
            .field("download_dir", &self.download_dir)
            .field("req_type", &self.req_type)
            .finish()
    }
}

impl<Client, Body, Form> AsRef<GraphUrl> for GraphRequest<Client, Body, Form> {
    fn as_ref(&self) -> &GraphUrl {
        &self.url
    }
}

impl<Client, Body, Form> AsMut<GraphUrl> for GraphRequest<Client, Body, Form> {
    fn as_mut(&mut self) -> &mut GraphUrl {
        &mut self.url
    }
}

pub type BlockingClient = GraphRequest<
    reqwest::blocking::Client,
    reqwest::blocking::Body,
    reqwest::blocking::multipart::Form,
>;

pub(crate) type AsyncClient =
    GraphRequest<reqwest::Client, reqwest::Body, reqwest::multipart::Form>;

impl BlockingClient {
    pub fn new_blocking(url: GraphUrl) -> BlockingClient {
        let mut headers = HeaderMap::default();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        BlockingClient {
            token: Default::default(),
            ident: Default::default(),
            client: reqwest::blocking::Client::builder()
                .redirect(Policy::limited(2))
                .build()
                .map_err(GraphFailure::from)
                .unwrap(),
            url,
            method: Default::default(),
            body: None,
            headers,
            upload_session_file: None,
            download_dir: None,
            form: None,
            req_type: Default::default(),
            registry: Handlebars::new(),
        }
    }

    pub fn inner_client(&mut self) -> &mut reqwest::blocking::Client {
        &mut self.client
    }

    pub fn download(&mut self) -> BlockingDownload {
        let request = self.clone();
        DownloadClient::new(request)
    }

    pub fn upload_session(&mut self) -> GraphResult<UploadSessionClient<BlockingHttpClient>> {
        let file = self
            .upload_session_file
            .take()
            .ok_or_else(|| GraphFailure::invalid("file for upload session"))?;

        let response = self.response()?;
        if let Some(err) = GraphFailure::from_response(&response) {
            return Err(err);
        }

        let upload_session: serde_json::Value = response.json()?;
        let mut session = UploadSessionClient::new(upload_session)?;
        session.set_file(file)?;
        Ok(session)
    }

    pub fn build(&mut self) -> reqwest::blocking::RequestBuilder {
        let headers = self.headers.clone();
        self.headers.clear();
        self.headers
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let builder = self
            .client
            .request(self.method.clone(), self.url.as_str())
            .headers(headers)
            .bearer_auth(self.token.as_str());

        match self.req_type {
            GraphRequestType::Basic | GraphRequestType::Redirect => {
                if self.body.is_some() {
                    builder.body(self.body.take().unwrap())
                } else {
                    builder
                }
            },
            GraphRequestType::Multipart => builder.multipart(self.form.take().unwrap()),
        }
    }

    pub fn response(&mut self) -> GraphResult<reqwest::blocking::Response> {
        let builder = self.build();
        let response = builder.send()?;
        if let Some(err) = GraphFailure::from_response(&response) {
            return Err(err);
        }
        Ok(response)
    }

    pub fn execute<T>(&mut self) -> GraphResult<GraphResponse<T>>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        GraphResponse::try_from(self.response()?)
    }

    pub fn clone(&mut self) -> Self {
        GraphRequest {
            token: self.token.to_string(),
            ident: self.ident,
            client: reqwest::blocking::Client::builder()
                .redirect(Policy::limited(2))
                .build()
                .map_err(GraphFailure::from)
                .unwrap(),
            url: self.url.clone(),
            method: self.method.clone(),
            body: self.body.take(),
            headers: self.headers.clone(),
            upload_session_file: self.upload_session_file.take(),
            download_dir: self.download_dir.take(),
            form: self.form.take(),
            req_type: self.req_type,
            registry: Handlebars::new(),
        }
    }
}

impl From<Url> for BlockingClient {
    fn from(url: Url) -> Self {
        BlockingClient::new_blocking(GraphUrl::from(url))
    }
}

impl Default for BlockingClient {
    fn default() -> Self {
        BlockingClient::new_blocking(GraphUrl::parse(GRAPH_URL).unwrap())
    }
}

impl AsyncClient {
    pub fn new_async(url: GraphUrl) -> AsyncClient {
        let mut headers = HeaderMap::default();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        AsyncClient {
            token: Default::default(),
            ident: Default::default(),
            client: reqwest::Client::builder()
                .redirect(Policy::limited(2))
                .build()
                .map_err(GraphFailure::from)
                .unwrap(),
            url,
            method: Default::default(),
            body: None,
            headers,
            upload_session_file: None,
            download_dir: None,
            form: None,
            req_type: Default::default(),
            registry: Handlebars::new(),
        }
    }

    pub fn inner_client(&mut self) -> &mut reqwest::Client {
        &mut self.client
    }

    pub fn download(&mut self) -> AsyncDownload {
        let request = self.clone();
        DownloadClient::new_async(request)
    }

    pub async fn upload_session(&mut self) -> GraphResult<UploadSessionClient<AsyncHttpClient>> {
        let file = self
            .upload_session_file
            .take()
            .ok_or_else(|| GraphFailure::invalid("file for upload session"))?;

        let response = self.response().await?;
        if let Some(err) = GraphFailure::from_async_response(&response) {
            if let Ok(text) = response.text().await {
                err.try_set_graph_error_message(text.as_str());
            }
            return Err(err);
        }

        let upload_session: serde_json::Value = response.json().await?;
        let mut session = UploadSessionClient::new_async(upload_session)?;
        session.set_file_async(file).await?;
        Ok(session)
    }

    pub fn build(&mut self) -> reqwest::RequestBuilder {
        let headers = self.headers.clone();
        self.headers.clear();
        self.headers
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let builder = self
            .client
            .request(self.method.clone(), self.url.as_str())
            .headers(headers)
            .bearer_auth(self.token.as_str());

        match self.req_type {
            GraphRequestType::Basic | GraphRequestType::Redirect => {
                if self.body.is_some() {
                    builder.body(self.body.take().unwrap())
                } else {
                    builder
                }
            },
            GraphRequestType::Multipart => builder.multipart(self.form.take().unwrap()),
        }
    }

    pub async fn response(&mut self) -> GraphResult<reqwest::Response> {
        let builder = self.build();
        let response = builder.send().await?;
        if let Some(err) = GraphFailure::from_async_response(&response) {
            return Err(err);
        }
        Ok(response)
    }

    pub async fn execute<T>(&mut self) -> GraphResult<GraphResponse<T>>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let response = self.response().await?;
        GraphResponse::try_from_async(response).await
    }

    pub fn clone(&mut self) -> Self {
        GraphRequest {
            token: self.token.to_string(),
            ident: self.ident,
            client: reqwest::Client::builder()
                .redirect(Policy::limited(2))
                .build()
                .map_err(GraphFailure::from)
                .unwrap(),
            url: self.url.clone(),
            method: self.method.clone(),
            body: self.body.take(),
            headers: self.headers.clone(),
            upload_session_file: self.upload_session_file.take(),
            download_dir: self.download_dir.take(),
            form: self.form.take(),
            req_type: self.req_type,
            registry: Handlebars::new(),
        }
    }
}

impl Default for AsyncClient {
    fn default() -> Self {
        AsyncClient::new_async(GraphUrl::parse(GRAPH_URL).unwrap())
    }
}

impl From<Url> for AsyncClient {
    fn from(url: Url) -> Self {
        AsyncClient::new_async(GraphUrl::from(url))
    }
}

pub struct HttpClient<Client> {
    client: Client,
}

pub type AsyncHttpClient = HttpClient<std::sync::Arc<tokio::sync::Mutex<AsyncClient>>>;
pub type BlockingHttpClient = HttpClient<RefCell<BlockingClient>>;

impl HttpClient<RefCell<BlockingClient>> {
    pub fn new(url: GraphUrl) -> HttpClient<RefCell<BlockingClient>> {
        HttpClient {
            client: RefCell::new(BlockingClient::new_blocking(url)),
        }
    }

    pub fn download(&self) -> BlockingDownload {
        self.client.borrow_mut().download()
    }

    pub fn upload_session(&self) -> GraphResult<UploadSessionClient<BlockingHttpClient>> {
        self.client.borrow_mut().upload_session()
    }

    pub fn build(&self) -> reqwest::blocking::RequestBuilder {
        self.client.borrow_mut().build()
    }

    pub fn response(&self) -> GraphResult<reqwest::blocking::Response> {
        self.client.borrow_mut().response()
    }

    pub fn execute<T>(&self) -> GraphResult<GraphResponse<T>>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        self.client.borrow_mut().execute()
    }

    pub(crate) fn inner_url_ref<F>(&self, f: F)
    where
        F: Fn(&GraphUrl),
    {
        f(&self.client.borrow().url)
    }
}

impl RequestClient for HttpClient<RefCell<BlockingClient>> {
    type Body = reqwest::blocking::Body;
    type Form = reqwest::blocking::multipart::Form;

    fn token(&self) -> String {
        self.client.borrow().token.clone()
    }

    fn set_token(&self, token: &str) {
        self.client.borrow_mut().token = token.to_string();
    }

    fn ident(&self) -> Ident {
        self.client.borrow().ident
    }

    fn set_ident(&self, ident: Ident) {
        self.client.borrow_mut().ident = ident;
    }

    fn url(&self) -> GraphUrl {
        self.client.borrow().url.clone()
    }

    fn to_url(&self) -> Url {
        self.client.borrow_mut().url.to_url()
    }

    fn set_url(&self, url: GraphUrl) {
        self.client.borrow_mut().url = url;
    }

    fn method(&self) -> Method {
        self.client.borrow().method.clone()
    }

    fn set_method(&self, method: Method) {
        self.client.borrow_mut().method = method;
    }

    fn set_body<T: Into<Self::Body>>(&self, body: T) {
        self.client.borrow_mut().body = Some(body.into());
    }

    fn set_body_with_file(&self, path: PathBuf) -> GraphResult<()> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        self.set_body(buffer);
        Ok(())
    }

    fn header<T: IntoHeaderName>(&self, name: T, value: HeaderValue) {
        self.client.borrow_mut().headers.insert(name, value);
    }

    fn set_header_map(&self, header_map: HeaderMap) {
        self.client.borrow_mut().headers = header_map;
    }

    fn clear_headers(&self) {
        self.client.borrow_mut().headers.clear();
    }

    fn set_download_dir(&self, dir: PathBuf) {
        self.client.borrow_mut().download_dir = Some(dir);
    }

    fn set_upload_session(&self, file: PathBuf) {
        self.client.borrow_mut().upload_session_file = Some(file);
    }

    fn set_form(&self, form: Self::Form) {
        let mut inner = self.client.borrow_mut();
        inner.form = Some(form);
        inner.req_type = GraphRequestType::Multipart;
    }

    fn set_request_type(&self, req_type: GraphRequestType) {
        self.client.borrow_mut().req_type = req_type;
    }

    fn request_type(&self) -> GraphRequestType {
        self.client.borrow().req_type
    }

    fn url_ref<F>(&self, f: F)
    where
        F: Fn(&GraphUrl) + Sync,
    {
        f(&self.client.borrow().url)
    }

    fn url_mut<F>(&self, f: F)
    where
        F: Fn(&mut GraphUrl) + Sync,
    {
        f(&mut self.client.borrow_mut().url)
    }

    fn registry<F>(&self, f: F)
    where
        F: Fn(&mut Handlebars) + Sync,
    {
        f(&mut self.client.borrow_mut().registry)
    }

    fn render_template(&self, template: &str, json: &serde_json::Value) -> String {
        self.client
            .borrow_mut()
            .registry
            .render_template(template, json)
            .unwrap()
    }

    fn extend_path(&self, path: &[&str]) {
        self.client.borrow_mut().url.extend_path(path);
    }

    #[allow(clippy::identity_conversion)]
    fn set_request(
        &self,
        req_att: Vec<RequestAttribute<reqwest::blocking::Body, reqwest::blocking::multipart::Form>>,
    ) -> GraphResult<()> {
        let mut client = self.client.borrow_mut();
        for att in req_att {
            match att {
                RequestAttribute::Token(token) => client.token = token,
                RequestAttribute::Ident(ident) => client.ident = ident,
                RequestAttribute::Url(url) => client.url = url,
                RequestAttribute::Method(method) => client.method = method,
                RequestAttribute::Body(body) => client.body = Some(body.into()),
                RequestAttribute::BodyFile(path) => {
                    let mut file = File::open(path)?;
                    let mut buffer = String::new();
                    file.read_to_string(&mut buffer)?;
                    client.body = Some(buffer.into());
                },
                RequestAttribute::Headers(headers) => client.headers = headers,
                RequestAttribute::ClearHeaders => client.headers.clear(),
                RequestAttribute::Download(path) => client.download_dir = Some(path),
                RequestAttribute::Upload(path) => client.upload_session_file = Some(path),
                RequestAttribute::Form(form) => client.form = Some(form),
                RequestAttribute::RequestType(req_type) => client.req_type = req_type,
            }
        }
        Ok(())
    }
}

impl Debug for HttpClient<RefCell<BlockingClient>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.client.fmt(f)
    }
}

impl From<BlockingClient> for HttpClient<RefCell<BlockingClient>> {
    fn from(client: BlockingClient) -> Self {
        HttpClient {
            client: RefCell::new(client),
        }
    }
}

impl HttpClient<std::sync::Arc<tokio::sync::Mutex<AsyncClient>>> {
    pub fn new(url: GraphUrl) -> HttpClient<std::sync::Arc<tokio::sync::Mutex<AsyncClient>>> {
        HttpClient {
            client: std::sync::Arc::new(tokio::sync::Mutex::new(AsyncClient::new_async(url))),
        }
    }

    async fn inner_token(&self) -> String {
        self.client.lock().await.token.clone()
    }

    async fn inner_set_token(&self, token: &str) {
        self.client.lock().await.token = token.to_string();
    }

    async fn inner_ident(&self) -> Ident {
        self.client.lock().await.ident
    }

    async fn inner_set_ident(&self, ident: Ident) {
        self.client.lock().await.ident = ident;
    }

    async fn inner_url(&self) -> GraphUrl {
        self.client.lock().await.url.clone()
    }

    async fn inner_to_url(&self) -> Url {
        self.client.lock().await.url.to_url()
    }

    async fn inner_set_url(&self, url: GraphUrl) {
        self.client.lock().await.url = url;
    }

    async fn inner_method(&self) -> Method {
        self.client.lock().await.method.clone()
    }

    async fn inner_set_method(&self, method: Method) {
        self.client.lock().await.method = method;
    }

    async fn inner_set_body<T: Into<reqwest::Body>>(&self, body: T) {
        self.client.lock().await.body = Some(body.into());
    }

    async fn inner_set_body_with_file(&self, path: PathBuf) -> GraphResult<()> {
        let buffer = tokio::fs::read_to_string(path).await?;
        self.inner_set_body(buffer).await;
        Ok(())
    }

    async fn inner_header<T: IntoHeaderName>(&self, name: T, value: HeaderValue) {
        self.client.lock().await.headers.insert(name, value);
    }

    async fn inner_set_header_map(&self, header_map: HeaderMap) {
        self.client.lock().await.headers = header_map;
    }

    async fn inner_clear_headers(&self) {
        self.client.lock().await.headers.clear();
    }

    async fn inner_set_download_dir(&self, dir: PathBuf) {
        self.client.lock().await.download_dir = Some(dir);
    }

    async fn inner_set_upload_session(&self, file: PathBuf) {
        self.client.lock().await.upload_session_file = Some(file);
    }

    async fn inner_set_form(&self, form: reqwest::multipart::Form) {
        let mut inner = self.client.lock().await;
        inner.form = Some(form);
        inner.req_type = GraphRequestType::Multipart;
    }

    async fn inner_set_request_type(&self, req_type: GraphRequestType) {
        self.client.lock().await.req_type = req_type;
    }

    async fn inner_request_type(&self) -> GraphRequestType {
        self.client.lock().await.req_type
    }

    async fn inner_url_ref<F>(&self, f: F)
    where
        F: Fn(&GraphUrl) + Sync,
    {
        f(&self.client.lock().await.url)
    }

    async fn inner_url_mut<F>(&self, f: F)
    where
        F: Fn(&mut GraphUrl) + Sync,
    {
        f(&mut self.client.lock().await.url)
    }

    async fn inner_debug(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.client.lock().await.fmt(f)
    }

    async fn inner_registry<F>(&self, f: F)
    where
        F: Fn(&mut Handlebars) + Sync,
    {
        f(&mut self.client.lock().await.registry)
    }

    async fn inner_render_template(&self, template: &str, json: &Value) -> String {
        self.client
            .lock()
            .await
            .registry
            .render_template(template, json)
            .unwrap()
    }

    async fn inner_extend_path(&self, path: &[&str]) {
        self.client.lock().await.url.extend_path(path);
    }

    #[allow(clippy::identity_conversion)]
    async fn inner_set_request(
        &self,
        req_att: Vec<RequestAttribute<reqwest::Body, reqwest::multipart::Form>>,
    ) -> GraphResult<()> {
        let mut client = self.client.lock().await;
        for att in req_att {
            match att {
                RequestAttribute::Token(token) => client.token = token,
                RequestAttribute::Ident(ident) => client.ident = ident,
                RequestAttribute::Url(url) => client.url = url,
                RequestAttribute::Method(method) => client.method = method,
                RequestAttribute::Body(body) => client.body = Some(body.into()),
                RequestAttribute::BodyFile(path) => {
                    let buffer = futures::executor::block_on(tokio::fs::read_to_string(path))?;
                    client.body = Some(buffer.into());
                },
                RequestAttribute::Headers(headers) => client.headers = headers,
                RequestAttribute::ClearHeaders => client.headers.clear(),
                RequestAttribute::Download(path) => client.download_dir = Some(path),
                RequestAttribute::Upload(path) => client.upload_session_file = Some(path),
                RequestAttribute::Form(form) => client.form = Some(form),
                RequestAttribute::RequestType(req_type) => client.req_type = req_type,
            }
        }
        Ok(())
    }

    pub async fn download(&self) -> AsyncDownload {
        self.client.lock().await.download()
    }

    pub async fn upload_session(&self) -> GraphResult<UploadSessionClient<AsyncHttpClient>> {
        self.client.lock().await.upload_session().await
    }

    pub async fn build(&self) -> reqwest::RequestBuilder {
        self.client.lock().await.build()
    }

    pub async fn response(&self) -> GraphResult<reqwest::Response> {
        self.client.lock().await.response().await
    }

    pub async fn execute<T>(&self) -> GraphResult<GraphResponse<T>>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        self.client.lock().await.execute().await
    }
}

impl RequestClient for HttpClient<std::sync::Arc<tokio::sync::Mutex<AsyncClient>>> {
    type Body = reqwest::Body;
    type Form = reqwest::multipart::Form;

    fn token(&self) -> String {
        futures::executor::block_on(self.inner_token())
    }

    fn set_token(&self, token: &str) {
        futures::executor::block_on(self.inner_set_token(token));
    }

    fn ident(&self) -> Ident {
        futures::executor::block_on(self.inner_ident())
    }

    fn set_ident(&self, ident: Ident) {
        futures::executor::block_on(self.inner_set_ident(ident))
    }

    fn url(&self) -> GraphUrl {
        futures::executor::block_on(self.inner_url())
    }

    fn to_url(&self) -> Url {
        futures::executor::block_on(self.inner_to_url())
    }

    fn set_url(&self, url: GraphUrl) {
        futures::executor::block_on(self.inner_set_url(url));
    }

    fn method(&self) -> Method {
        futures::executor::block_on(self.inner_method())
    }

    fn set_method(&self, method: Method) {
        futures::executor::block_on(self.inner_set_method(method));
    }

    fn set_body<T: Into<Self::Body>>(&self, body: T) {
        futures::executor::block_on(self.inner_set_body(body));
    }

    fn set_body_with_file(&self, path: PathBuf) -> GraphResult<()> {
        futures::executor::block_on(self.inner_set_body_with_file(path))
    }

    fn header<T: IntoHeaderName>(&self, name: T, value: HeaderValue) {
        futures::executor::block_on(self.inner_header(name, value));
    }

    fn set_header_map(&self, header_map: HeaderMap<HeaderValue>) {
        futures::executor::block_on(self.inner_set_header_map(header_map));
    }

    fn clear_headers(&self) {
        futures::executor::block_on(self.inner_clear_headers());
    }

    fn set_download_dir(&self, dir: PathBuf) {
        futures::executor::block_on(self.inner_set_download_dir(dir));
    }

    fn set_upload_session(&self, file: PathBuf) {
        futures::executor::block_on(self.inner_set_upload_session(file));
    }

    fn set_form(&self, form: Self::Form) {
        futures::executor::block_on(self.inner_set_form(form));
    }

    fn set_request_type(&self, req_type: GraphRequestType) {
        futures::executor::block_on(self.inner_set_request_type(req_type));
    }

    fn request_type(&self) -> GraphRequestType {
        futures::executor::block_on(self.inner_request_type())
    }

    fn url_ref<F>(&self, f: F)
    where
        F: Fn(&GraphUrl) + Sync,
    {
        futures::executor::block_on(self.inner_url_ref(f));
    }

    fn url_mut<F>(&self, f: F)
    where
        F: Fn(&mut GraphUrl) + Sync,
    {
        futures::executor::block_on(self.inner_url_mut(f));
    }

    fn registry<F>(&self, f: F)
    where
        F: Fn(&mut Handlebars) + Sync,
    {
        futures::executor::block_on(self.inner_registry(f));
    }

    fn render_template(&self, template: &str, json: &serde_json::Value) -> String {
        futures::executor::block_on(self.inner_render_template(template, json))
    }

    fn extend_path(&self, path: &[&str]) {
        futures::executor::block_on(self.inner_extend_path(path));
    }

    fn set_request(
        &self,
        req_att: Vec<RequestAttribute<reqwest::Body, reqwest::multipart::Form>>,
    ) -> GraphResult<()> {
        futures::executor::block_on(self.inner_set_request(req_att))
    }
}

impl Debug for HttpClient<std::sync::Arc<tokio::sync::Mutex<AsyncClient>>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        futures::executor::block_on(self.inner_debug(f))
    }
}

impl From<AsyncClient> for HttpClient<std::sync::Arc<tokio::sync::Mutex<AsyncClient>>> {
    fn from(client: AsyncClient) -> Self {
        AsyncHttpClient {
            client: std::sync::Arc::new(tokio::sync::Mutex::new(client)),
        }
    }
}
