use crate::request::{RequestAttribute, RequestType};
use crate::url::GraphUrl;
use graph_core::resource::ResourceIdentity;
use graph_error::{GraphFailure, GraphResult};
use handlebars::Handlebars;
use reqwest::header::{HeaderMap, HeaderValue, IntoHeaderName};
use reqwest::Method;
use std::path::PathBuf;
use url::Url;

pub trait RequestClient {
    type Body: From<String> + From<Vec<u8>> + From<&'static [u8]> + From<&'static str>;
    type Form;

    fn token(&self) -> String;
    fn set_token(&self, token: &str);
    fn ident(&self) -> ResourceIdentity;
    fn set_ident(&self, ident: ResourceIdentity);
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
    fn register_ident_helper(&self, resource_identity: ResourceIdentity);
    fn extend_path(&self, path: &[&str]);
    fn set_request(
        &self,
        req_attr: Vec<RequestAttribute<Self::Body, Self::Form>>,
    ) -> GraphResult<()>;

    fn set_body_with_serialize<B: serde::Serialize>(&self, body: &B) -> GraphResult<()> {
        let body_result = serde_json::to_string(body).map_err(GraphFailure::from);
        if let Ok(body) = body_result {
            self.set_body(body);
        } else if let Err(err) = body_result {
            return Err(err);
        }
        Ok(())
    }
}

pub struct HttpClient<Client> {
    pub(crate) client: Client,
}
