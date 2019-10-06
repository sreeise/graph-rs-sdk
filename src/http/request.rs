use crate::client::Ident;
use crate::http::{FetchClient, GraphResponse, UploadSessionClient};
use crate::url::GraphUrl;
use graph_error::{GraphFailure, GraphResult};
use graph_rs_types::complextypes::UploadSession;
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName, CONTENT_TYPE};
use reqwest::{Method, RedirectPolicy, RequestBuilder};
use std::path::{Path, PathBuf};
use url::Url;
use crate::GRAPH_URL;

#[derive(Clone, Eq, PartialEq)]
pub enum GraphRequestType {
    Basic,
    Redirect,
}

impl Default for GraphRequestType {
    fn default() -> Self {
        GraphRequestType::Basic
    }
}

pub struct GraphRequestBuilder {
    pub url: GraphUrl,
    pub method: Method,
    pub body: Option<reqwest::Body>,
    pub headers: HeaderMap<HeaderValue>,
    pub upload_session_file: Option<PathBuf>,
    pub download_dir: Option<PathBuf>,
    pub req_type: GraphRequestType,
}

impl GraphRequestBuilder {
    pub fn new(url: GraphUrl) -> GraphRequestBuilder {
        let mut headers = HeaderMap::default();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        GraphRequestBuilder {
            url,
            method: Default::default(),
            body: None,
            headers,
            upload_session_file: None,
            download_dir: None,
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

    pub fn body(&self) -> Option<&reqwest::Body> {
        self.body.as_ref()
    }

    pub fn set_body<B: Into<reqwest::Body>>(&mut self, body: B) -> &mut Self {
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

    pub fn set_request_type(&mut self, req_type: GraphRequestType) -> &mut Self {
        self.req_type = req_type;
        self
    }
}

impl AsRef<GraphUrl> for GraphRequestBuilder {
    fn as_ref(&self) -> &GraphUrl {
        &self.url
    }
}

impl AsMut<GraphUrl> for GraphRequestBuilder {
    fn as_mut(&mut self) -> &mut GraphUrl {
        &mut self.url
    }
}

impl From<Url> for GraphRequestBuilder {
    fn from(url: Url) -> Self {
        GraphRequestBuilder::new(GraphUrl::from(url))
    }
}

impl Default for GraphRequestBuilder {
    fn default() -> Self {
        GraphRequestBuilder::new(GraphUrl::parse(GRAPH_URL).unwrap())
    }
}

pub struct GraphRequest {
    token: String,
    ident: Ident,
    client: reqwest::Client,
    redirect_client: reqwest::Client,
}

impl GraphRequest {
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

    pub fn download(&mut self, request: GraphRequestBuilder) -> FetchClient {
        FetchClient::new(self.token.as_str(), request)
    }

    pub fn upload_session(
        &mut self,
        request: GraphRequestBuilder,
    ) -> GraphResult<UploadSessionClient> {
        let file = request
            .upload_session_file
            .clone()
            .ok_or_else(|| GraphFailure::none_err("file for upload session"))?;
        let mut response = self.response(request)?;
        if let Some(err) = GraphFailure::from_response(&mut response) {
            return Err(err);
        }

        let upload_session: UploadSession = response.json()?;
        let mut session = UploadSessionClient::new(upload_session)?;
        session.set_file(file)?;
        Ok(session)
    }

    pub fn build(&mut self, request: GraphRequestBuilder) -> RequestBuilder {
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
        }
    }

    pub fn response(&mut self, request: GraphRequestBuilder) -> GraphResult<reqwest::Response> {
        let builder = self.build(request);
        let mut response = builder.send()?;
        if let Some(err) = GraphFailure::from_response(&mut response) {
            return Err(err);
        }
        Ok(response)
    }

    pub fn execute<T>(&mut self, request: GraphRequestBuilder) -> GraphResult<GraphResponse<T>>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let mut response = self.response(request)?;
        let value: T = response.json()?;
        Ok(GraphResponse::new(response, value))
    }
}

impl Default for GraphRequest {
    fn default() -> Self {
        let redirect_client = reqwest::Client::builder()
            .redirect(RedirectPolicy::custom(|attempt| {
                // There should be only 1 redirect to download a drive item.
                if attempt.previous().len() > 1 {
                    return attempt.too_many_redirects();
                }
                attempt.stop()
            }))
            .build()
            .map_err(GraphFailure::from)
            .unwrap();

        GraphRequest {
            token: Default::default(),
            ident: Default::default(),
            client: reqwest::Client::new(),
            redirect_client,
        }
    }
}
