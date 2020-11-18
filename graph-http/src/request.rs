use crate::url::GraphUrl;
use graph_core::resource::ResourceIdentity;
use handlebars::Handlebars;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Method;
use std::fmt::Debug;
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RequestType {
    Basic,
    Redirect,
    Multipart,
}

impl Default for RequestType {
    fn default() -> Self {
        RequestType::Basic
    }
}

pub enum RequestAttribute<Body, Form> {
    Token(String),
    Ident(ResourceIdentity),
    Url(GraphUrl),
    Method(reqwest::Method),
    Body(Body),
    BodyFile(PathBuf),
    Headers(HeaderMap),
    ClearHeaders,
    Download(PathBuf),
    Upload(PathBuf),
    Form(Form),
    RequestType(RequestType),
}

pub struct GraphRequest<Client, Body, Form> {
    pub(crate) token: String,
    pub(crate) ident: ResourceIdentity,
    pub(crate) client: Client,
    pub(crate) registry: Handlebars,
    pub url: GraphUrl,
    pub method: Method,
    pub body: Option<Body>,
    pub headers: HeaderMap<HeaderValue>,
    pub upload_session_file: Option<PathBuf>,
    pub download_dir: Option<PathBuf>,
    pub form: Option<Form>,
    pub req_type: RequestType,
}

impl<Client, Body, Form> Debug for GraphRequest<Client, Body, Form> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphRequest")
            .field("token", &"[REDACTED]")
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
