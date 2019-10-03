use crate::client::Ident;
use crate::http::{Download, FetchClient, GraphResponse};
use crate::url::GraphUrl;
use graph_error::{GraphFailure, GraphResult};
use handlebars::Handlebars;
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName, CONTENT_TYPE};
use reqwest::{Method, RedirectPolicy, RequestBuilder};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use url::Url;

#[derive(Default, Clone)]
pub struct DownloadRequest {
    pub directory: PathBuf,
    pub is_direct_download: bool,
    pub file_name: Option<OsString>,
    pub extension: Option<String>,
}

impl DownloadRequest {
    pub fn new(directory: PathBuf, is_direct_download: bool) -> DownloadRequest {
        DownloadRequest {
            directory,
            is_direct_download,
            file_name: Default::default(),
            extension: Default::default(),
        }
    }

    pub fn set_directory(&mut self, directory: PathBuf) {
        self.directory = directory;
    }

    pub fn set_direct_download(&mut self, is_direct_download: bool) {
        self.is_direct_download = is_direct_download;
    }

    pub fn set_file_name(&mut self, file_name: Option<OsString>) {
        self.file_name = file_name.map(|s| s.to_os_string());
    }

    pub fn set_extension(&mut self, extension: Option<&str>) {
        self.extension = extension.map(|s| s.to_string());
    }
}

pub struct GraphRequest {
    pub url: GraphUrl,
    pub method: Method,
    pub body: Option<reqwest::Body>,
    pub upload_session_file: Option<PathBuf>,
    pub download_request: DownloadRequest,
    headers: HeaderMap<HeaderValue>,
    token: String,
    ident: Ident,
    registry: Handlebars,
    client: reqwest::Client,
}

impl GraphRequest {
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

    pub fn body(&self) -> Option<&reqwest::Body> {
        self.body.as_ref()
    }

    pub fn body_mut(&mut self) -> &mut Option<reqwest::Body> {
        &mut self.body
    }

    pub fn set_body<B: Into<reqwest::Body>>(&mut self, body: B) -> &mut Self {
        self.body = Some(body.into());
        self
    }

    pub fn headers(&self) -> &HeaderMap<HeaderValue> {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap<HeaderValue> {
        &mut self.headers
    }

    pub fn header(&mut self, name: impl IntoHeaderName, value: HeaderValue) -> &mut Self {
        self.headers.insert(name, value);
        self
    }

    pub fn set_token(&mut self, token: &str) -> &mut Self {
        self.token = token.to_string();
        self
    }

    pub fn set_upload_session<P: AsRef<Path>>(&mut self, file: P) -> &mut Self {
        self.upload_session_file = Some(file.as_ref().to_path_buf());
        self
    }

    pub fn set_ident(&mut self, ident: Ident) -> &mut Self {
        self.ident = ident;
        self
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn set_direct_download(&mut self, value: bool, url: &str) -> &mut Self {
        self.download_request.set_direct_download(value);
        self.set_url(GraphUrl::parse(url).unwrap());
        self
    }

    pub fn rename_download(&mut self, name: OsString) {
        self.download_request.file_name = Some(name);
    }

    pub fn registry(&mut self) -> &mut Handlebars {
        &mut self.registry
    }

    pub fn redirect(&mut self) -> GraphResult<RequestBuilder> {
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

        let method = self.method().clone();
        let url = self.url().to_string();
        let mut headers = self.headers().clone();

        if !headers.contains_key(CONTENT_TYPE) {
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }

        if let Some(body) = self.body.take() {
            Ok(client
                .request(method, &url)
                .headers(headers)
                .bearer_auth(self.token.as_str())
                .body(body))
        } else {
            Ok(client
                .request(method, &url)
                .headers(headers)
                .bearer_auth(self.token.as_str()))
        }
    }

    pub fn builder(&mut self) -> RequestBuilder {
        let method = self.method().clone();
        let url = self.url().to_string();
        let mut headers = self.headers().clone();

        if !headers.contains_key(CONTENT_TYPE) {
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }

        if let Some(body) = self.body.take() {
            self.client
                .request(method, &url)
                .headers(headers)
                .bearer_auth(self.token.as_str())
                .body(body)
        } else {
            self.client
                .request(method, &url)
                .headers(headers)
                .bearer_auth(self.token.as_str())
        }
    }

    pub fn response(&mut self) -> GraphResult<reqwest::Response> {
        let builder: RequestBuilder = self.builder();
        let mut response = builder.send()?;
        if let Some(err) = GraphFailure::from_response(&mut response) {
            error!("Error executing request: {:#?}", &response);
            return Err(err);
        }
        Ok(response)
    }

    pub fn graph_response<T>(&mut self) -> GraphResult<GraphResponse<T>>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let builder = self.builder();
        let mut response = builder.send()?;
        if let Some(err) = GraphFailure::from_response(&mut response) {
            error!("Error executing request: {:#?}", &response);
            Err(err)
        } else {
            let value: T = response.json()?;
            Ok(GraphResponse::new(response, value))
        }
    }

    pub fn json<T>(&mut self) -> GraphResult<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let builder = self.builder();
        let mut response = builder.send()?;
        if let Some(err) = GraphFailure::from_response(&mut response) {
            error!("Error executing request: {:#?}", &response);
            Err(err)
        } else {
            Ok(response.json()?)
        }
    }
}

impl AsRef<GraphUrl> for GraphRequest {
    fn as_ref(&self) -> &GraphUrl {
        &self.url
    }
}

impl AsMut<GraphUrl> for GraphRequest {
    fn as_mut(&mut self) -> &mut GraphUrl {
        &mut self.url
    }
}

impl From<GraphUrl> for GraphRequest {
    fn from(url: GraphUrl) -> Self {
        GraphRequest {
            url,
            method: Default::default(),
            body: Default::default(),
            headers: Default::default(),
            upload_session_file: Default::default(),
            token: Default::default(),
            download_request: Default::default(),
            ident: Default::default(),
            registry: Handlebars::new(),
            client: reqwest::Client::new(),
        }
    }
}

impl From<Url> for GraphRequest {
    fn from(url: Url) -> Self {
        GraphRequest {
            url: GraphUrl::from(url),
            method: Default::default(),
            body: Default::default(),
            headers: Default::default(),
            upload_session_file: Default::default(),
            token: Default::default(),
            download_request: Default::default(),
            ident: Default::default(),
            registry: Handlebars::new(),
            client: reqwest::Client::new(),
        }
    }
}

impl Download for GraphRequest {
    fn download(&mut self) -> GraphResult<FetchClient> {
        let mut fetch_client = FetchClient::new(
            self.url.as_str(),
            self.download_request.directory.to_path_buf(),
            self.token.as_str(),
        );

        if !self.download_request.is_direct_download {
            info!("Using redirect on download");
            let builder: RequestBuilder = self.redirect()?;
            fetch_client.set_redirect(builder);
        }

        if let Some(file_name) = self.download_request.file_name.as_ref() {
            fetch_client.rename(file_name.to_os_string());
        }

        if let Some(ext) = self.download_request.extension.as_ref() {
            fetch_client.set_extension(ext);
        }

        Ok(fetch_client)
    }
}
