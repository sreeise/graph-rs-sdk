use crate::client::Ident;
use crate::http::{
    AsyncDownload, BlockingDownload, DownloadClient, GraphResponse, UploadSessionClient,
};
use crate::url::GraphUrl;
use crate::GRAPH_URL;
use async_trait::async_trait;
use graph_error::{GraphFailure, GraphResult};
use handlebars::Handlebars;
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName, CONTENT_TYPE};
use reqwest::{redirect::Policy, Method};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
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

pub trait RequestClient: Debug + AsRef<GraphUrl> + AsMut<GraphUrl> {
    type Body: From<String> + From<Vec<u8>> + From<&'static [u8]> + From<&'static str>;
    type Form;

    fn token(&mut self) -> String;
    fn set_token(&mut self, token: &str) -> &mut Self;
    fn ident(&self) -> Ident;
    fn set_ident(&mut self, ident: Ident) -> &mut Self;
    fn url(&self) -> &GraphUrl;
    fn to_url(&self) -> Url;
    fn set_url(&mut self, url: GraphUrl) -> &mut Self;
    fn method(&self) -> &Method;
    fn set_method(&mut self, method: Method) -> &mut Self;
    fn body(&self) -> Option<&Self::Body>;
    fn set_body(&mut self, body: impl Into<Self::Body>) -> &mut Self;
    fn set_body_with_file<P: AsRef<Path>>(&mut self, body: P) -> GraphResult<()>;
    fn headers(&self) -> &HeaderMap<HeaderValue>;
    fn header(&mut self, name: impl IntoHeaderName, value: HeaderValue) -> &mut Self;
    fn set_header_map(&mut self, header_map: HeaderMap) -> &mut Self;
    fn clear_headers(&mut self) -> &mut Self;
    fn set_download_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self;
    fn set_upload_session<P: AsRef<Path>>(&mut self, file: P) -> &mut Self;
    fn set_form(&mut self, form: Self::Form) -> &mut Self;
    fn set_request_type(&mut self, req_type: GraphRequestType) -> &mut Self;
    fn request_type(&self) -> GraphRequestType;
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

impl<Client, Body, Form> RequestClient for GraphRequest<Client, Body, Form>
where
    Body: From<String> + From<Vec<u8>> + From<&'static [u8]> + From<&'static str>,
{
    type Body = Body;
    type Form = Form;

    fn token(&mut self) -> String {
        self.token.clone()
    }

    fn set_token(&mut self, token: &str) -> &mut Self {
        self.token = token.to_string();
        self
    }

    fn ident(&self) -> Ident {
        self.ident
    }

    fn set_ident(&mut self, ident: Ident) -> &mut Self {
        self.ident = ident;
        self
    }

    fn url(&self) -> &GraphUrl {
        &self.url
    }

    fn to_url(&self) -> Url {
        self.url.to_url()
    }

    fn set_url(&mut self, url: GraphUrl) -> &mut Self {
        self.url = url;
        self
    }

    fn method(&self) -> &Method {
        &self.method
    }

    fn set_method(&mut self, method: Method) -> &mut Self {
        self.method = method;
        self
    }

    fn body(&self) -> Option<&Self::Body> {
        self.body.as_ref()
    }

    fn set_body(&mut self, body: impl Into<Self::Body>) -> &mut Self {
        self.body = Some(body.into());
        self
    }

    fn set_body_with_file<P: AsRef<Path>>(&mut self, path: P) -> GraphResult<()> {
        let mut file = File::open(path.as_ref())?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        self.set_body(buffer);
        Ok(())
    }

    fn headers(&self) -> &HeaderMap<HeaderValue> {
        &self.headers
    }

    fn header(&mut self, name: impl IntoHeaderName, value: HeaderValue) -> &mut Self {
        self.headers.insert(name, value);
        self
    }

    fn set_header_map(&mut self, header_map: HeaderMap) -> &mut Self {
        self.headers = header_map;
        self
    }

    fn clear_headers(&mut self) -> &mut Self {
        self.headers.clear();
        self
    }

    fn set_download_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self {
        self.download_dir = Some(dir.as_ref().to_path_buf());
        self
    }

    fn set_upload_session<P: AsRef<Path>>(&mut self, file: P) -> &mut Self {
        self.upload_session_file = Some(file.as_ref().to_path_buf());
        self
    }

    fn set_form(&mut self, form: Form) -> &mut Self {
        self.form = Some(form);
        self.req_type = GraphRequestType::Multipart;
        self
    }

    fn set_request_type(&mut self, req_type: GraphRequestType) -> &mut Self {
        self.req_type = req_type;
        self
    }

    fn request_type(&self) -> GraphRequestType {
        self.req_type
    }
}

pub type BlockingClient = GraphRequest<
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
        }
    }

    pub fn inner_client(&mut self) -> &mut reqwest::blocking::Client {
        &mut self.client
    }

    pub fn download(&mut self) -> BlockingDownload {
        let request = self.clone();
        DownloadClient::new(request)
    }

    pub fn upload_session(&mut self) -> GraphResult<UploadSessionClient<BlockingClient>> {
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
        match self.req_type {
            GraphRequestType::Basic | GraphRequestType::Redirect => {
                if self.body.is_some() {
                    self.client
                        .request(self.method.clone(), self.url.as_str())
                        .headers(self.headers.clone())
                        .bearer_auth(self.token.as_str())
                        .body(self.body.take().unwrap())
                } else {
                    self.client
                        .request(self.method.clone(), self.url.as_str())
                        .headers(self.headers.clone())
                        .bearer_auth(self.token.as_str())
                }
            },
            GraphRequestType::Multipart => self
                .client
                .request(self.method.clone(), self.url.as_str())
                .headers(self.headers.clone())
                .bearer_auth(self.token.as_str())
                .multipart(self.form.take().unwrap()),
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

pub type AsyncClient = GraphRequest<reqwest::Client, reqwest::Body, reqwest::multipart::Form>;

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
        }
    }

    pub fn inner_client(&mut self) -> &mut reqwest::Client {
        &mut self.client
    }

    pub fn download(&mut self) -> AsyncDownload {
        let request = self.clone();
        DownloadClient::new_async(request)
    }

    pub async fn upload_session(&mut self) -> GraphResult<UploadSessionClient<AsyncClient>> {
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
        match self.req_type {
            GraphRequestType::Basic | GraphRequestType::Redirect => {
                if self.body.is_some() {
                    self.client
                        .request(self.method.clone(), self.url.as_str())
                        .headers(headers)
                        .bearer_auth(self.token.as_str())
                        .body(self.body.take().unwrap())
                } else {
                    self.client
                        .request(self.method.clone(), self.url.as_str())
                        .headers(headers)
                        .bearer_auth(self.token.as_str())
                }
            },
            GraphRequestType::Multipart => self
                .client
                .request(self.method.clone(), self.url.as_str())
                .headers(self.headers.clone())
                .bearer_auth(self.token.as_str())
                .multipart(self.form.take().unwrap()),
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

pub struct HttpClient<Client, Registry> {
    client: Client,
    registry: Registry,
}

impl HttpClient<RefCell<BlockingClient>, RefCell<Handlebars>> {
    pub fn new(url: GraphUrl) -> HttpClient<RefCell<BlockingClient>, RefCell<Handlebars>> {
        HttpClient {
            client: RefCell::new(BlockingClient::new_blocking(url)),
            registry: RefCell::new(Handlebars::new()),
        }
    }

    /*
    pub(crate) fn request(&self) -> RefMut<BlockingClient> {
        self.client.borrow_mut()
    }

    pub(crate) fn registry(&self) -> RefMut<Handlebars>  {
        self.registry.borrow_mut()
    }
     */

    /*
       fn token(&'a self) -> String {
           self.client.borrow().token.clone()
       }

       fn set_token(&'a self, token: &str) -> &'a Self {
           self.client.borrow_mut().token = token.to_string();
           self
       }

       fn ident(&'a self) -> Ident {
           self.client.borrow().ident
       }

       fn set_ident(&'a self, ident: Ident) -> &'a Self {
           self.client.borrow_mut().ident = ident;
           self
       }

       fn url(&'a self) -> &'a GraphUrl {
           &self.client.borrow().url
       }

       fn to_url(&'a self) -> Url {
           self.client.borrow_mut().url.to_url()
       }

       fn set_url(&'a self, url: GraphUrl) -> &'a Self {
           self.client.borrow_mut().url = url;
           self
       }

       fn method(&'a self) -> &'a Method {
           &self.client.borrow_mut().method
       }

       fn set_method(&'a self, method: Method) -> &'a Self {
           self.client.borrow_mut().method = method;
           self
       }

       fn body(&'a self) -> Option<&'a reqwest::blocking::Body> {
           self.client.borrow_mut().body.as_ref()
       }

       fn set_body(&'a self, body: impl Into<reqwest::blocking::Body>) -> &'a Self {
           self.client.borrow_mut().body = Some(body.into());
           self
       }

       fn set_body_with_file<P: AsRef<Path>>(&'a self, path: P) -> GraphResult<()> {
           let mut file = File::open(path.as_ref())?;
           let mut buffer = String::new();
           file.read_to_string(&mut buffer)?;
           self.set_body(buffer);
           Ok(())
       }

       fn headers(&'a self) -> &'a HeaderMap<HeaderValue> {
           &self.client.borrow().headers
       }

       fn header(&'a self, name: impl IntoHeaderName, value: HeaderValue) -> &'a Self {
           self.client.borrow_mut().headers.insert(name, value);
           self
       }

       fn set_header_map(&'a self, header_map: HeaderMap) -> &'a Self {
           self.client.borrow_mut().headers = header_map;
           self
       }

       fn clear_headers(&'a self) -> &'a Self {
           self.client.borrow_mut().headers.clear();
           self
       }


       fn set_download_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self {
           self.download_dir = Some(dir.as_ref().to_path_buf());
           self
       }

       fn set_upload_session<P: AsRef<Path>>(&mut self, file: P) -> &mut Self {
           self.upload_session_file = Some(file.as_ref().to_path_buf());
           self
       }

       fn set_form(&mut self, form: reqwest::multipart::Form) -> &mut Self {
           self.form = Some(form);
           self.req_type = GraphRequestType::Multipart;
           self
       }

       fn set_request_type(&mut self, req_type: GraphRequestType) -> &mut Self {
           self.req_type = req_type;
           self
       }

       fn request_type(&self) -> GraphRequestType {
           self.req_type
       }
    */
}

impl
    HttpClient<
        std::sync::Arc<tokio::sync::Mutex<AsyncClient>>,
        std::sync::Arc<tokio::sync::Mutex<Handlebars>>,
    >
{
    pub fn new(
        url: GraphUrl,
    ) -> HttpClient<
        std::sync::Arc<tokio::sync::Mutex<AsyncClient>>,
        std::sync::Arc<tokio::sync::Mutex<Handlebars>>,
    > {
        HttpClient {
            client: std::sync::Arc::new(tokio::sync::Mutex::new(AsyncClient::new_async(url))),
            registry: std::sync::Arc::new(tokio::sync::Mutex::new(Handlebars::new())),
        }
    }

    /*
       async fn token(&'a self) -> String {
           self.client.lock().await.token.clone()
       }

       async fn set_token(&'a self, token: &str) -> &'a Self {
           self.client.lock().await.token = token.to_string();
           self
       }

       async fn ident(&'a self) -> Ident {
           self.client.lock().await.ident
       }

       async fn set_ident(&'a self, ident: Ident) -> &'a Self {
           self.client.lock().await.ident = ident;
           self
       }

       async fn url(&'a self) -> GraphUrl {
           self.client.lock().await.url.clone()
       }

       async fn to_url(&'a self) -> Url {
           self.client.lock().await.url.to_url()
       }

       async fn set_url(&'a self, url: GraphUrl) -> &'a Self {
           self.client.lock().await.url = url;
           self
       }

       async fn method(&'a self) -> Method {
           self.client.lock().await.method.clone()
       }

       async fn set_method(&'a self, method: Method) -> &'a Self {
           self.client.lock().await.method = method;
           self
       }

       async fn set_body<T: Into<reqwest::Body> + Sync + Send>(&'a self, body: T) -> &'a Self {
           self.client.lock().await.body = Some(body.into());
           self
       }

       async fn set_body_with_file(&'a self, path: &str) -> GraphResult<()> {
           let buffer = tokio::fs::read_to_string(path).await?;
           self.set_body(buffer).await;
           Ok(())
       }

       async fn header<T: IntoHeaderName + Sync + Send>(
           &'a self,
           name: T,
           value: HeaderValue,
       ) -> &'a Self {
           self.client.lock().await.headers.insert(name, value);
           self
       }

       async fn set_header_map(&'a self, header_map: HeaderMap) -> &'a Self {
           self.client.lock().await.headers = header_map;
           self
       }

       async fn clear_headers(&'a self) -> &'a Self {
           self.client.lock().await.headers.clear();
           self
       }

       async fn set_download_dir<P: AsRef<Path> + Sync + Send>(&'a self, dir: P) -> &'a Self {
           self.client.lock().await.download_dir = Some(dir.as_ref().to_path_buf());
           self
       }

       async fn set_upload_session<P: AsRef<Path> + Sync + Send>(&'a self, file: P) -> &'a Self {
           self.client.lock().await.upload_session_file = Some(file.as_ref().to_path_buf());
           self
       }

       async fn set_form(&'a self, form: reqwest::multipart::Form) -> &'a Self {
           let mut inner = self.client.lock().await;
           inner.form = Some(form);
           inner.req_type = GraphRequestType::Multipart;
           self
       }

       async fn set_request_type(&'a self, req_type: GraphRequestType) -> &'a Self {
           self.client.lock().await.req_type = req_type;
           self
       }

       async fn request_type(&'a self) -> GraphRequestType {
           self.client.lock().await.req_type
       }
    */
}

pub trait SyncRequestClient {
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
    fn set_body_with_file<P: AsRef<Path>>(&self, path: P) -> GraphResult<()>;
    fn header<T: IntoHeaderName>(&self, name: T, value: HeaderValue);
    fn set_header_map(&self, header_map: HeaderMap);
    fn clear_headers(&self);
    fn set_download_dir<P: AsRef<Path>>(&self, dir: P);
    fn set_upload_session<P: AsRef<Path>>(&self, file: P);
    fn set_form(&self, form: Self::Form);
    fn set_request_type(&self, req_type: GraphRequestType);
    fn request_type(&self) -> GraphRequestType;
    fn url_ref<F>(&self, f: F) where F: Fn(&GraphUrl);
    fn url_mut<F>(&self, f: F) where F: Fn(&mut GraphUrl);
}

#[async_trait]
pub trait AsyncRequestClient {
    type Body: From<String> + From<Vec<u8>> + From<&'static [u8]> + From<&'static str>;
    type Form;

    async fn token(&self) -> String;
    async fn set_token(&self, token: &str);
    async fn ident(&self) -> Ident;
    async fn set_ident(&self, ident: Ident);
    async fn url(&self) -> GraphUrl;
    async fn to_url(&self) -> Url;
    async fn set_url(&self, url: GraphUrl);
    async fn method(&self) -> Method;
    async fn set_method(&self, method: Method);
    async fn set_body<T: Into<Self::Body> + Sync + Send>(&self, body: T);
    async fn set_body_with_file(&self, path: &str) -> GraphResult<()>;
    async fn header<T: IntoHeaderName + Send + Sync>(&self, name: T, value: HeaderValue);
    async fn set_header_map(&self, header_map: HeaderMap);
    async fn clear_headers(&self);
    async fn set_download_dir<P: AsRef<Path> + Sync + Send>(&self, dir: P);
    async fn set_upload_session<P: AsRef<Path> + Sync + Send>(&self, file: P);
    async fn set_form(&self, form: Self::Form);
    async fn set_request_type(&self, req_type: GraphRequestType);
    async fn request_type(&self) -> GraphRequestType;
    async fn url_ref<F>(&self, f: F) where F: Fn(&GraphUrl) + Send + Sync;
    async fn url_mut<F>(&self, f: F) where F: Fn(&mut GraphUrl) + Send + Sync;
}

#[async_trait]
impl AsyncRequestClient
    for HttpClient<
        std::sync::Arc<tokio::sync::Mutex<AsyncClient>>,
        std::sync::Arc<tokio::sync::Mutex<Handlebars>>,
    >
{
    type Body = reqwest::Body;
    type Form = reqwest::multipart::Form;

    async fn token(&self) -> String {
        self.client.lock().await.token.clone()
    }

    async fn set_token(&self, token: &str) {
        self.client.lock().await.token = token.to_string();
    }

    async fn ident(&self) -> Ident {
        self.client.lock().await.ident
    }

    async fn set_ident(&self, ident: Ident) {
        self.client.lock().await.ident = ident;
    }

    async fn url(&self) -> GraphUrl {
        self.client.lock().await.url.clone()
    }

    async fn to_url(&self) -> Url {
        self.client.lock().await.url.to_url()
    }

    async fn set_url(&self, url: GraphUrl) {
        self.client.lock().await.url = url;
    }

    async fn method(&self) -> Method {
        self.client.lock().await.method.clone()
    }

    async fn set_method(&self, method: Method) {
        self.client.lock().await.method = method;
    }

    async fn set_body<T: Into<Self::Body> + Sync + Send>(&self, body: T) {
        self.client.lock().await.body = Some(body.into());
    }

    async fn set_body_with_file(&self, path: &str) -> GraphResult<()> {
        let buffer = tokio::fs::read_to_string(path).await?;
        self.set_body(buffer).await;
        Ok(())
    }

    async fn header<T: IntoHeaderName + Sync + Send>(&self, name: T, value: HeaderValue) {
        self.client.lock().await.headers.insert(name, value);
    }

    async fn set_header_map(&self, header_map: HeaderMap) {
        self.client.lock().await.headers = header_map;
    }

    async fn clear_headers(&self) {
        self.client.lock().await.headers.clear();
    }

    async fn set_download_dir<P: AsRef<Path> + Sync + Send>(&self, dir: P) {
        self.client.lock().await.download_dir = Some(dir.as_ref().to_path_buf());
    }

    async fn set_upload_session<P: AsRef<Path> + Sync + Send>(&self, file: P) {
        self.client.lock().await.upload_session_file = Some(file.as_ref().to_path_buf());
    }

    async fn set_form(&self, form: Self::Form) {
        let mut inner = self.client.lock().await;
        inner.form = Some(form);
        inner.req_type = GraphRequestType::Multipart;
    }

    async fn set_request_type(&self, req_type: GraphRequestType) {
        self.client.lock().await.req_type = req_type;
    }

    async fn request_type(&self) -> GraphRequestType {
        self.client.lock().await.req_type
    }

    async fn url_ref<F>(&self, f: F) where F: Fn(&GraphUrl) + Send + Sync {
        f(&self.client.lock().await.url)
    }

    async fn url_mut<F>(&self, f: F) where F: Fn(&mut GraphUrl) + Send + Sync {
        f(&mut self.client.lock().await.url)
    }
}

impl SyncRequestClient for HttpClient<RefCell<BlockingClient>, RefCell<Handlebars>> {
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

    fn set_body_with_file<P: AsRef<Path>>(&self, path: P) -> GraphResult<()> {
        let mut file = File::open(path.as_ref())?;
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

    fn set_download_dir<P: AsRef<Path>>(&self, dir: P) {
        self.client.borrow_mut().download_dir = Some(dir.as_ref().to_path_buf());
    }

    fn set_upload_session<P: AsRef<Path>>(&self, file: P) {
        self.client.borrow_mut().upload_session_file = Some(file.as_ref().to_path_buf());
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

    fn url_ref<F>(&self, f: F) where F: Fn(&GraphUrl) {
        f(&self.client.borrow().url)
    }

    fn url_mut<F>(&self, f: F) where F: Fn(&mut GraphUrl) {
        f(&mut self.client.borrow_mut().url)
    }
}
