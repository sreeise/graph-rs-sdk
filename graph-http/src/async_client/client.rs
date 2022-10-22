use crate::download::AsyncDownload;
use crate::download::DownloadClient;
use crate::traits::*;
use crate::uploadsession::UploadSessionClient;
use crate::url::GraphUrl;
use crate::{
    GraphRequest, GraphResponse, HttpClient, Registry, RequestAttribute, RequestClient, RequestType,
};
use graph_core::resource::ResourceIdentity;
use graph_error::WithGraphErrorAsync;
use graph_error::{GraphFailure, GraphResult};
use handlebars::Handlebars;
use parking_lot::Mutex;
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName, CONTENT_TYPE};
use reqwest::redirect::Policy;
use reqwest::{Method, RequestBuilder};
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

pub(crate) type AsyncClient =
    GraphRequest<reqwest::Client, reqwest::Body, reqwest::multipart::Form>;

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
            timeout: Duration::from_secs(30),
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

        let response = self.response().await?.with_graph_error().await?;
        let upload_session: serde_json::Value = response.json().await?;
        let mut session = UploadSessionClient::new_async(upload_session)?;
        session.set_file(file).await?;
        Ok(session)
    }

    pub fn build_upload_session(&mut self) -> (Option<PathBuf>, RequestBuilder) {
        let file = self.upload_session_file.take();
        let builder = self.build();
        (file, builder)
    }

    pub fn build(&mut self) -> RequestBuilder {
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
    pub async fn response(&mut self) -> GraphResult<reqwest::Response> {
        let builder = self.build();
        let response = builder.send().await?;
        Ok(response)
    }

    /// Builds the requests and sends it, converting to a GraphResponse and deserializing
    /// the body.
    pub async fn execute<T>(&mut self) -> GraphResult<GraphResponse<T>>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let builder = self.build();
        let response = builder.send().await?;
        AsyncTryFrom::<reqwest::Response>::async_try_from(response).await
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
            timeout: Duration::from_secs(30),
        }
    }

    fn register_ident_helper(&mut self, resource_identity: ResourceIdentity) {
        Registry::register_internal_helper(resource_identity, &mut self.registry);
    }
}

impl Default for AsyncClient {
    fn default() -> Self {
        AsyncClient::new_async(GraphUrl::parse("https://graph.microsoft.com/v1.0").unwrap())
    }
}

impl From<Url> for AsyncClient {
    fn from(url: Url) -> Self {
        AsyncClient::new_async(GraphUrl::from(url))
    }
}

pub type AsyncHttpClient = HttpClient<Arc<Mutex<AsyncClient>>>;

impl AsyncHttpClient {
    pub fn new(url: GraphUrl) -> AsyncHttpClient {
        AsyncHttpClient {
            client: Arc::new(Mutex::new(AsyncClient::new_async(url))),
        }
    }

    pub fn clone_inner(&self) -> Arc<Mutex<AsyncClient>> {
        Arc::clone(&self.client)
    }

    pub async fn download(&self) -> AsyncDownload {
        self.client.lock().download()
    }

    fn get_upload_session_values(&self) -> GraphResult<(PathBuf, RequestBuilder)> {
        let mut client = self.client.lock();
        let upload_session_file = client
            .upload_session_file
            .take()
            .ok_or_else(|| GraphFailure::invalid("file for upload session"))?;
        let request_builder = client.build();
        Ok((upload_session_file, request_builder))
    }

    pub async fn upload_session(&self) -> GraphResult<UploadSessionClient<AsyncHttpClient>> {
        let (file, request) = self.get_upload_session_values()?;
        let response = request.send().await?.with_graph_error().await?;
        let upload_session: serde_json::Value = response.json().await?;
        let mut session = UploadSessionClient::new_async(upload_session)?;
        session.set_file(file).await?;
        Ok(session)
    }

    pub fn build_upload_session(&self) -> (Option<PathBuf>, RequestBuilder) {
        self.client.lock().build_upload_session()
    }

    pub fn build(&self) -> RequestBuilder {
        self.client.lock().build()
    }

    pub async fn response(&self) -> GraphResult<reqwest::Response> {
        let builder = self.build();
        builder.send().await.map_err(GraphFailure::from)
    }

    pub async fn execute<T>(&self) -> GraphResult<GraphResponse<T>>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let request = self.build();
        let response = request.send().await?;
        AsyncTryFrom::<reqwest::Response>::async_try_from(response).await
    }
}

impl RequestClient for AsyncHttpClient {
    type Body = reqwest::Body;
    type Form = reqwest::multipart::Form;

    fn token(&self) -> String {
        self.client.lock().token.clone()
    }

    fn set_token(&self, token: &str) {
        self.client.lock().token = token.to_string();
    }

    fn ident(&self) -> ResourceIdentity {
        self.client.lock().ident
    }

    fn set_ident(&self, ident: ResourceIdentity) {
        self.client.lock().ident = ident;
    }

    fn url(&self) -> GraphUrl {
        self.client.lock().url.clone()
    }

    fn to_url(&self) -> Url {
        self.client.lock().url.to_url()
    }

    fn set_url(&self, url: GraphUrl) {
        self.client.lock().url = url;
    }

    fn method(&self) -> Method {
        self.client.lock().method.clone()
    }

    fn set_method(&self, method: Method) {
        self.client.lock().method = method;
    }

    fn set_body<T: Into<reqwest::Body>>(&self, body: T) {
        self.client.lock().body = Some(body.into());
    }

    fn set_body_with_file(&self, path: PathBuf) -> GraphResult<()> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        self.client.lock().body = Some(buffer.into());
        Ok(())
    }

    fn header<T: IntoHeaderName>(&self, name: T, value: HeaderValue) {
        self.client.lock().headers.insert(name, value);
    }

    fn set_header_map(&self, header_map: HeaderMap) {
        self.client.lock().headers = header_map;
    }

    fn clear_headers(&self) {
        self.client.lock().headers.clear();
    }

    fn set_download_dir(&self, dir: PathBuf) {
        self.client.lock().download_dir = Some(dir);
    }

    fn set_upload_session(&self, file: PathBuf) {
        self.client.lock().upload_session_file = Some(file);
    }

    fn set_form(&self, form: reqwest::multipart::Form) {
        let mut client = self.client.lock();
        client.form = Some(form);
        client.req_type = RequestType::Multipart;
    }

    fn set_request_type(&self, req_type: RequestType) {
        self.client.lock().req_type = req_type;
    }

    fn request_type(&self) -> RequestType {
        self.client.lock().req_type
    }

    fn url_ref<F>(&self, f: F)
    where
        F: Fn(&GraphUrl) + Sync,
    {
        f(&self.client.lock().url)
    }

    fn url_mut<F>(&self, f: F)
    where
        F: Fn(&mut GraphUrl) + Sync,
    {
        f(&mut self.client.lock().url)
    }

    fn registry<F>(&self, f: F)
    where
        F: Fn(&mut Handlebars) + Sync,
    {
        f(&mut self.client.lock().registry)
    }

    fn render_template(&self, template: &str, json: &serde_json::Value) -> String {
        self.client
            .lock()
            .registry
            .render_template(template, json)
            .unwrap()
    }

    fn register_ident_helper(&self, resource_identity: ResourceIdentity) {
        self.client.lock().register_ident_helper(resource_identity);
    }

    fn extend_path(&self, path: &[&str]) {
        self.client.lock().url.extend_path(path);
    }

    fn set_request(
        &self,
        req_att: Vec<RequestAttribute<reqwest::Body, reqwest::multipart::Form>>,
    ) -> GraphResult<()> {
        for att in req_att {
            let mut client = self.client.lock();
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
        self.client.lock().timeout = duration;
    }
}

impl Debug for AsyncHttpClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.client.lock().fmt(f)
    }
}

impl From<AsyncClient> for AsyncHttpClient {
    fn from(client: AsyncClient) -> Self {
        AsyncHttpClient {
            client: Arc::new(Mutex::new(client)),
        }
    }
}
