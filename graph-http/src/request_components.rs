use crate::api_impl::GraphUrl;
use graph_core::resource::ResourceIdentity;
use graph_error::{GraphFailure, GraphResult};
use http::header::ACCEPT;
use http::{HeaderMap, HeaderValue, Method};

/// Provides the necessary components for building a request.
#[derive(Clone, Debug, Default)]
pub struct RequestComponents {
    pub resource_identity: ResourceIdentity,
    pub url: GraphUrl,
    pub method: Method,
    pub headers: HeaderMap,
}

impl RequestComponents {
    pub fn to_reqwest_url(&self) -> reqwest::Url {
        self.url.to_reqwest_url()
    }
}

impl AsRef<GraphUrl> for RequestComponents {
    fn as_ref(&self) -> &GraphUrl {
        &self.url
    }
}

impl AsMut<GraphUrl> for RequestComponents {
    fn as_mut(&mut self) -> &mut GraphUrl {
        &mut self.url
    }
}

impl AsMut<HeaderMap> for RequestComponents {
    fn as_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }
}

impl RequestComponents {
    pub fn new(
        resource_identity: ResourceIdentity,
        url: GraphUrl,
        method: reqwest::Method,
    ) -> RequestComponents {
        let mut headers: HeaderMap<HeaderValue> = HeaderMap::with_capacity(2);
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        RequestComponents {
            resource_identity,
            url,
            method,
            headers,
        }
    }
}

impl TryFrom<(ResourceIdentity, reqwest::Method, GraphResult<GraphUrl>)> for RequestComponents {
    type Error = GraphFailure;

    fn try_from(
        value: (ResourceIdentity, Method, GraphResult<GraphUrl>),
    ) -> Result<Self, Self::Error> {
        Ok(RequestComponents::new(value.0, value.2?, value.1))
    }
}
