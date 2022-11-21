use crate::download::{BlockingDownload, DownloadClient};
use crate::upload_session::UploadSessionClient;
use crate::url::GraphUrl;
use crate::{
    GraphRequestWrapper, GraphResponse, HttpClient, Registry, RequestAttribute, RequestClient,
    RequestType,
};
use graph_core::resource::ResourceIdentity;
use graph_error::{GraphFailure, GraphResult, WithGraphError};
use handlebars::Handlebars;
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName, CONTENT_TYPE};
use reqwest::redirect::Policy;
use reqwest::Method;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::Duration;
use url::Url;

pub type BlockingClient = GraphRequestWrapper<
    reqwest::blocking::Client,
    reqwest::blocking::Body,
    reqwest::blocking::multipart::Form,
>;

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
            timeout: Duration::from_secs(30),
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

        let response = self.response()?.with_graph_error()?;
        let upload_session: serde_json::Value = response.json()?;
        let mut session = UploadSessionClient::new(upload_session)?;
        session.set_file(file)?;
        Ok(session)
    }

    pub fn build_upload_session(&mut self) -> (Option<PathBuf>, reqwest::blocking::RequestBuilder) {
        let file = self.upload_session_file.take();
        let builder = self.build();
        (file, builder)
    }

    pub fn build(&mut self) -> reqwest::blocking::RequestBuilder {
        let headers = self.headers.clone();
        self.headers.clear();
        self.headers
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let builder = self
            .client
            .request(self.method.clone(), self.url.as_str())
            .timeout(self.timeout)
            .headers(headers)
            .bearer_auth(self.token.as_str());

        match self.req_type {
            RequestType::Basic | RequestType::Redirect => {
                if self.body.is_some() {
                    builder.body(self.body.take().unwrap())
                } else {
                    builder
                }
            }
            RequestType::Multipart => builder.multipart(self.form.take().unwrap()),
        }
    }

    /// Builds the request and sends it.
    ///
    /// Requests that require a redirect are automatic so we don't need
    /// to do anything special for these requests.
    pub fn response(&mut self) -> GraphResult<reqwest::blocking::Response> {
        let builder = self.build();
        let response = builder.send()?;
        Ok(response)
    }

    /// Builds the requests and sends it, converting to a GraphResponse and deserializing
    /// the body.
    pub fn execute<T>(&mut self) -> GraphResult<GraphResponse<T>>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        std::convert::TryFrom::try_from(self.response()?)
    }

    pub fn clone(&mut self) -> Self {
        GraphRequestWrapper {
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
            timeout: Duration::from_secs(30),
        }
    }

    fn register_ident_helper(&mut self, resource_identity: ResourceIdentity) {
        Registry::register_internal_helper(resource_identity, &mut self.registry);
    }
}

impl From<Url> for BlockingClient {
    fn from(url: Url) -> Self {
        BlockingClient::new_blocking(GraphUrl::from(url))
    }
}

impl Default for BlockingClient {
    fn default() -> Self {
        BlockingClient::new_blocking(GraphUrl::parse("https://graph.microsoft.com/v1.0").unwrap())
    }
}

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

    pub fn build_upload_session(&self) -> (Option<PathBuf>, reqwest::blocking::RequestBuilder) {
        self.client.borrow_mut().build_upload_session()
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

    pub fn inner_url_ref<F>(&self, f: F)
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

    fn ident(&self) -> ResourceIdentity {
        self.client.borrow().ident
    }

    fn set_ident(&self, ident: ResourceIdentity) {
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
        inner.req_type = RequestType::Multipart;
    }

    fn set_request_type(&self, req_type: RequestType) {
        self.client.borrow_mut().req_type = req_type;
    }

    fn request_type(&self) -> RequestType {
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

    fn register_ident_helper(&self, resource_identity: ResourceIdentity) {
        self.client
            .borrow_mut()
            .register_ident_helper(resource_identity);
    }

    fn extend_path(&self, path: &[&str]) {
        self.client.borrow_mut().url.extend_path(path);
    }

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
                RequestAttribute::Body(body) => client.body = Some(body),
                RequestAttribute::BodyFile(path) => {
                    let mut file = File::open(path)?;
                    let mut buffer = String::new();
                    file.read_to_string(&mut buffer)?;
                    client.body = Some(buffer.into());
                }
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

    fn set_timeout(&self, duration: Duration) {
        self.client.borrow_mut().timeout = duration;
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
