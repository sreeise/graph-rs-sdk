use crate::url::GraphUrl;
use url::Url;
use reqwest::Method;
use std::path::PathBuf;
use graph_error::GraphResult;
use reqwest::header::{HeaderValue, IntoHeaderName, HeaderMap};
use crate::request::{RequestType, RequestAttribute};
use handlebars::Handlebars;

pub trait RequestClient {
    type Body: From<String> + From<Vec<u8>> + From<&'static [u8]> + From<&'static str>;
    type Form;

    fn token(&self) -> String;
    fn set_token(&self, token: &str);
    fn ident(&self) -> String;
    fn set_ident(&self, ident: String);
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
    fn set_request_type(&self, req_type: RequestType);
    fn request_type(&self) -> RequestType;
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

pub struct HttpClient<Client> {
    pub(crate) client: Client,
}
