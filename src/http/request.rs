use crate::http::{Download, FetchClient};
use crate::types::statusresponse::StatusResponse;
use crate::url::GraphUrl;
use from_as::TryFrom;
use graph_error::{GraphError, GraphFailure, GraphResult};
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName, CONTENT_TYPE};
use reqwest::{Method, RedirectPolicy, RequestBuilder};
use std::ffi::{OsStr, OsString};
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

    pub fn set_file_name(&mut self, file_name: Option<&OsStr>) {
        self.file_name = file_name.map(|s| s.to_os_string());
    }

    pub fn set_extension(&mut self, extension: Option<&str>) {
        self.extension = extension.map(|s| s.to_string());
    }
}

pub struct Request {
    pub url: GraphUrl,
    pub method: Method,
    pub body: Option<reqwest::Body>,
    pub upload_session_file: Option<PathBuf>,
    pub download_request: DownloadRequest,
    headers: HeaderMap<HeaderValue>,
    token: String,
    client: reqwest::Client,
}

impl Request {
    pub fn url(&self) -> &GraphUrl {
        &self.url
    }

    pub fn to_url(&self) -> Url {
        self.url.to_url()
    }

    pub fn set_url(&mut self, url: GraphUrl) {
        self.url = url;
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn set_method(&mut self, method: Method) {
        self.method = method;
    }

    pub fn body(&self) -> Option<&reqwest::Body> {
        self.body.as_ref()
    }

    pub fn body_mut(&mut self) -> &mut Option<reqwest::Body> {
        &mut self.body
    }

    pub fn set_body(&mut self, body: Option<reqwest::Body>) {
        self.body = body;
    }

    pub fn headers(&self) -> &HeaderMap<HeaderValue> {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap<HeaderValue> {
        &mut self.headers
    }

    pub fn header(&mut self, name: impl IntoHeaderName, value: HeaderValue) {
        self.headers.insert(name, value);
    }

    pub fn set_token(&mut self, token: &str) {
        self.token = token.to_string();
    }

    pub fn set_upload_session_file<P: AsRef<Path>>(&mut self, file: P) {
        self.upload_session_file = Some(file.as_ref().to_path_buf());
    }

    pub(crate) fn redirect(&mut self) -> GraphResult<RequestBuilder> {
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
        } else  {
            Ok(client
                .request(method, &url)
                .headers(headers)
                .bearer_auth(self.token.as_str()))
        }
    }

    pub(crate) fn builder(&mut self) -> RequestBuilder {
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

    pub(crate) fn status_response(&mut self) -> GraphResult<StatusResponse> {
        let builder: RequestBuilder = self.builder();
        let mut response = builder.send()?;
        if let Some(err) = GraphFailure::from_response(&mut response) {
            return Err(err);
        }
        Ok(StatusResponse::new(response))
    }

    pub fn json<T>(&mut self) -> GraphResult<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let builder = self.builder();
        let mut response = builder.send()?;
        let status = response.status().as_u16();
        if GraphError::is_error(status) {
            Err(GraphFailure::try_from(&mut response).unwrap_or_default())
        } else {
            Ok(response.json()?)
        }
    }
}

impl AsRef<GraphUrl> for Request {
    fn as_ref(&self) -> &GraphUrl {
        &self.url
    }
}

impl AsMut<GraphUrl> for Request {
    fn as_mut(&mut self) -> &mut GraphUrl {
        &mut self.url
    }
}

impl From<GraphUrl> for Request {
    fn from(url: GraphUrl) -> Self {
        Request {
            url,
            method: Default::default(),
            body: Default::default(),
            headers: Default::default(),
            upload_session_file: Default::default(),
            token: Default::default(),
            download_request: Default::default(),
            client: reqwest::Client::new(),
        }
    }
}

impl From<Url> for Request {
    fn from(url: Url) -> Self {
        Request {
            url: GraphUrl::from(url),
            method: Default::default(),
            body: Default::default(),
            headers: Default::default(),
            upload_session_file: Default::default(),
            token: Default::default(),
            download_request: Default::default(),
            client: reqwest::Client::new(),
        }
    }
}

impl Download for Request {
    fn download(&mut self) -> GraphResult<FetchClient> {
        let url = {
            if self.download_request.is_direct_download {
                let builder: RequestBuilder = self.redirect()?;
                let mut response = builder.send()?;
                let status = response.status().as_u16();
                if GraphError::is_error(status) {
                    return Err(GraphFailure::try_from(&mut response).unwrap_or_default());
                }
                response.url().clone()
            } else {
                self.to_url()
            }
        };

        let mut fetch_client =
            FetchClient::new(url.as_ref(), self.download_directory(), self.token.as_str());

        if let Some(file_name) = self.file_name().as_ref() {
            fetch_client.rename(file_name.to_os_string());
        }

        if let Some(ext) = self.extension().as_ref() {
            fetch_client.set_extension(ext);
        }

        Ok(fetch_client)
    }

    fn download_directory(&self) -> PathBuf {
        self.download_request.directory.clone()
    }

    fn file_name(&self) -> Option<OsString> {
        self.download_request.file_name.clone()
    }

    fn extension(&self) -> Option<String> {
        self.download_request.extension.clone()
    }
}
