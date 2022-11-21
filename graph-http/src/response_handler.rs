use crate::client::Client;
use crate::url::GraphUrl;
use graph_core::resource::ResourceIdentity;
use graph_error::{GraphFailure, GraphResult};
use reqwest::redirect::Policy;
use reqwest::RequestBuilder;
use std::io::Write;

#[derive(Clone, Builder, Debug)]
#[builder(setter(into))]
pub struct ResourceConfig {
    pub resource_identity: ResourceIdentity,
    pub url: GraphUrl,
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

    pub fn query<T: serde::Serialize + ?Sized>(mut self, query: &T) -> Self {
        self.request_builder = self.request_builder.query(query);
        self
    }

    /// Filters properties (columns).
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#select-parameter)
    pub fn select(mut self, value: &[&str]) -> Self {
        let s = value.join(",");
        self.request_builder = self.request_builder.query(&[("$select", &s)]);
        self
    }

    pub fn url(self) -> reqwest::Url {
        let request = self.request_builder.build().unwrap();
        request.url().clone()
    }
}

impl From<RequestBuilder> for ResponseHandler {
    fn from(request_builder: RequestBuilder) -> Self {
        ResponseHandler::new(request_builder, None)
    }
}
