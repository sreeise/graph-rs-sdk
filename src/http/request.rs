use crate::client::Ident;
use crate::http::{
    AsyncDownload, BlockingDownload, DownloadClient, GraphResponse, UploadSessionClient,
};
use crate::url::GraphUrl;
use crate::GRAPH_URL;
use graph_error::{GraphFailure, GraphResult};
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName, CONTENT_TYPE};
use reqwest::{redirect::Policy, Method};
use serde::export::Formatter;
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

    fn token(&mut self) -> &String;
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

    fn token(&mut self) -> &String {
        &self.token
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
            url: GraphUrl::parse(GRAPH_URL).unwrap(),
            method: Default::default(),
            body: None,
            headers,
            upload_session_file: None,
            download_dir: None,
            form: None,
            req_type: Default::default(),
        }
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
        self.headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
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
            url: GraphUrl::parse(GRAPH_URL).unwrap(),
            method: Default::default(),
            body: None,
            headers,
            upload_session_file: None,
            download_dir: None,
            form: None,
            req_type: Default::default(),
        }
    }
}

impl From<Url> for AsyncClient {
    fn from(url: Url) -> Self {
        AsyncClient::new_async(GraphUrl::from(url))
    }
}
