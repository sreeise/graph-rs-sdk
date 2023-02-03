use crate::odata_query::ODataQuery;
use crate::url::GraphUrl;
use graph_core::resource::ResourceIdentity;
use graph_error::{GraphFailure, GraphResult};
use reqwest::RequestBuilder;

#[derive(Clone, Builder, Debug)]
#[builder(setter(into))]
pub struct ResourceConfig {
    pub resource_identity: ResourceIdentity,
    pub url: GraphUrl,
    pub resource_identity_id: Option<String>,
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct ResponseHandler {
    pub(crate) request_builder: RequestBuilder,
    pub(crate) error: Option<GraphFailure>,
}

impl ResponseHandler {
    pub fn new(request_builder: RequestBuilder, error: Option<GraphFailure>) -> ResponseHandler {
        ResponseHandler {
            request_builder,
            error,
        }
    }

    pub async fn send(self) -> GraphResult<reqwest::Response> {
        if let Some(err) = self.error {
            return Err(err);
        }

        self.request_builder
            .send()
            .await
            .map_err(GraphFailure::from)
    }

    // Just for testing.
    pub fn url(self) -> reqwest::Url {
        let request = self.request_builder.build().unwrap();

        request.url().clone()
    }

    fn query(mut self, key: &str, value: &str) -> Self {
        self.request_builder = self.request_builder.query(&[(key, value)]);
        self
    }
}

impl From<RequestBuilder> for ResponseHandler {
    fn from(request_builder: RequestBuilder) -> Self {
        ResponseHandler::new(request_builder, None)
    }
}

impl ODataQuery for ResponseHandler {
    fn append_query_pair<KV: AsRef<str>>(self, key: KV, value: KV) -> Self {
        self.query(key.as_ref(), value.as_ref())
    }
}

pub trait RequestComposer {
    type ResponseType;

    fn send(self) -> Self::ResponseType;
}
