use crate::download::AsyncDownload;
use crate::download::DownloadClient;
use crate::traits::*;
use crate::uploadsession::UploadSessionClient;
use crate::url::GraphUrl;
use crate::{
    GraphRequest, GraphResponse, HttpClient, RequestAttribute, RequestClient, RequestType,
};
use graph_error::{ErrorMessage, GraphError, GraphFailure, GraphResult};
use handlebars::Handlebars;
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName, CONTENT_TYPE};
use reqwest::redirect::Policy;
use reqwest::Method;
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
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
        if let Ok(mut error) = GraphError::try_from(&response) {
            let error_message: GraphResult<ErrorMessage> =
                response.json().await.map_err(GraphFailure::from);
            if let Ok(message) = error_message {
                error.set_error_message(message);
            }
            return Err(GraphFailure::GraphError(error));
        }

        let upload_session: serde_json::Value = response.json().await?;
        let mut session = UploadSessionClient::new_async(upload_session)?;
        session.set_file(file).await?;
        Ok(session)
    }

    pub fn build_upload_session(&mut self) -> (Option<PathBuf>, reqwest::RequestBuilder) {
        let file = self.upload_session_file.take();
        let builder = self.build();
        (file, builder)
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
            RequestType::Basic | RequestType::Redirect => {
                if self.body.is_some() {
                    builder.body(self.body.take().unwrap())
                } else {
                    builder
                }
            },
            RequestType::Multipart => builder.multipart(self.form.take().unwrap()),
        }
    }

    pub async fn response(&mut self) -> GraphResult<reqwest::Response> {
        let builder = self.build();
        let response = builder.send().await?;
        Ok(response)
    }

    pub async fn execute<T>(&mut self) -> GraphResult<GraphResponse<T>>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let response = self.response().await?;
        AsyncTryFrom::<reqwest::Response>::async_try_from(response).await
    }

    pub fn clone(&mut self) -> Self {
        GraphRequest {
            token: self.token.to_string(),
            ident: self.ident.to_string(),
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
        AsyncClient::new_async(GraphUrl::parse("https://graph.microsoft.com/v1.0").unwrap())
    }
}

impl From<Url> for AsyncClient {
    fn from(url: Url) -> Self {
        AsyncClient::new_async(GraphUrl::from(url))
    }
}

pub type AsyncHttpClient = HttpClient<std::sync::Arc<tokio::sync::Mutex<AsyncClient>>>;

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

    async fn inner_ident(&self) -> String {
        self.client.lock().await.ident.to_string()
    }

    async fn inner_set_ident(&self, ident: String) {
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
        inner.req_type = RequestType::Multipart;
    }

    async fn inner_set_request_type(&self, req_type: RequestType) {
        self.client.lock().await.req_type = req_type;
    }

    async fn inner_request_type(&self) -> RequestType {
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

    async fn inner_render_template(&self, template: &str, json: &serde_json::Value) -> String {
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

    pub async fn build_upload_session(&self) -> (Option<PathBuf>, reqwest::RequestBuilder) {
        self.client.lock().await.build_upload_session()
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

    fn ident(&self) -> String {
        futures::executor::block_on(self.inner_ident())
    }

    fn set_ident(&self, ident: String) {
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

    fn set_request_type(&self, req_type: RequestType) {
        futures::executor::block_on(self.inner_set_request_type(req_type));
    }

    fn request_type(&self) -> RequestType {
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
