use crate::api_impl::GraphUrl;
use bytes::BytesMut;
use graph_core::resource::ResourceIdentity;
use graph_error::{GraphFailure, GraphResult};
use http::header::ACCEPT;
use http::{HeaderMap, HeaderValue, Method};

/// Provides the necessary components for building a request.
#[derive(Debug, Default)]
pub struct RequestComponents {
    pub resource_identity: ResourceIdentity,
    pub url: GraphUrl,
    pub method: reqwest::Method,
    pub body: Option<reqwest::Body>,
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
        body: Option<String>,
    ) -> RequestComponents {
        let mut headers: HeaderMap<HeaderValue> = HeaderMap::with_capacity(2);
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        RequestComponents {
            resource_identity,
            url,
            method,
            body: body.map(|s| s.into()),
            headers,
        }
    }
}

impl TryFrom<(ResourceIdentity, reqwest::Method, GraphResult<GraphUrl>)> for RequestComponents {
    type Error = GraphFailure;

    fn try_from(
        value: (ResourceIdentity, Method, GraphResult<GraphUrl>),
    ) -> Result<Self, Self::Error> {
        Ok(RequestComponents::new(value.0, value.2?, value.1, None))
    }
}

impl
    TryFrom<(
        ResourceIdentity,
        reqwest::Method,
        GraphResult<GraphUrl>,
        GraphResult<String>,
    )> for RequestComponents
{
    type Error = GraphFailure;

    fn try_from(
        value: (
            ResourceIdentity,
            Method,
            GraphResult<GraphUrl>,
            GraphResult<String>,
        ),
    ) -> Result<Self, Self::Error> {
        Ok(RequestComponents::new(
            value.0,
            value.2.map_err(GraphFailure::from)?,
            value.1,
            Some(value.3.map_err(GraphFailure::from)?),
        ))
    }
}

impl
    TryFrom<(
        ResourceIdentity,
        reqwest::Method,
        GraphResult<GraphUrl>,
        BytesMut,
    )> for RequestComponents
{
    type Error = GraphFailure;

    fn try_from(
        value: (ResourceIdentity, Method, GraphResult<GraphUrl>, BytesMut),
    ) -> Result<Self, Self::Error> {
        Ok(RequestComponents {
            resource_identity: value.0,
            url: value.2.map_err(GraphFailure::from)?,
            method: value.1,
            body: Some(reqwest::Body::from(value.3.to_vec())),
            headers: Default::default(),
        })
    }
}

impl
    TryFrom<(
        ResourceIdentity,
        reqwest::Method,
        GraphResult<GraphUrl>,
        GraphResult<BytesMut>,
    )> for RequestComponents
{
    type Error = GraphFailure;

    fn try_from(
        value: (
            ResourceIdentity,
            Method,
            GraphResult<GraphUrl>,
            GraphResult<BytesMut>,
        ),
    ) -> Result<Self, Self::Error> {
        Ok(RequestComponents {
            resource_identity: value.0,
            url: value.2.map_err(GraphFailure::from)?,
            method: value.1,
            body: Some(reqwest::Body::from(
                value
                    .3
                    .map(|bytes_mut| bytes_mut.to_vec())
                    .map_err(GraphFailure::from)?,
            )),
            headers: Default::default(),
        })
    }
}

impl
    TryFrom<(
        ResourceIdentity,
        reqwest::Method,
        GraphResult<GraphUrl>,
        GraphResult<reqwest::Body>,
    )> for RequestComponents
{
    type Error = GraphFailure;

    fn try_from(
        value: (
            ResourceIdentity,
            Method,
            GraphResult<GraphUrl>,
            GraphResult<reqwest::Body>,
        ),
    ) -> Result<Self, Self::Error> {
        Ok(RequestComponents {
            resource_identity: value.0,
            url: value.2.map_err(GraphFailure::from)?,
            method: value.1,
            body: Some(value.3?),
            headers: Default::default(),
        })
    }
}
