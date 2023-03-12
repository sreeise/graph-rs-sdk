use crate::client::Client;
use crate::odata_query::ODataQuery;
use crate::traits::BodyFromBytes;
use crate::traits::ODataNextLink;
use crate::url::GraphUrl;
use crate::{DownloadConfig, DownloadTask};
use async_stream::{stream, try_stream};
use futures_core::Stream;
use graph_core::resource::ResourceIdentity;
use graph_error::{GraphFailure, GraphResult};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT};
use std::path::PathBuf;

#[derive(Clone, Builder, Debug, Default, Eq, PartialEq)]
#[builder(
    pattern = "mutable",
    derive(Debug),
    setter(into, strip_option),
    default
)]
pub struct ResourceConfig {
    pub resource_identity: ResourceIdentity,
    pub url: GraphUrl,
    pub resource_identity_id: Option<String>,
}

impl AsRef<GraphUrl> for ResourceConfig {
    fn as_ref(&self) -> &GraphUrl {
        &self.url
    }
}

impl AsMut<GraphUrl> for ResourceConfig {
    fn as_mut(&mut self) -> &mut GraphUrl {
        &mut self.url
    }
}

#[derive(Clone, Builder, Debug, Default, Eq, PartialEq)]
#[builder(
    pattern = "mutable",
    derive(Debug, Eq, PartialEq),
    setter(into, strip_option),
    default
)]
pub struct RequestComponents {
    pub resource_identity: ResourceIdentity,
    pub url: GraphUrl,
    pub method: reqwest::Method,
    pub body: Option<String>,
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

impl RequestComponents {
    pub fn new(
        ri: ResourceIdentity,
        url: GraphUrl,
        method: reqwest::Method,
        body: Option<String>,
    ) -> RequestComponents {
        let mut headers: HeaderMap<HeaderValue> = HeaderMap::with_capacity(2);
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        RequestComponents {
            resource_identity: ri,
            url,
            method,
            body,
            headers,
        }
    }

    pub fn builder(ri: ResourceIdentity, method: reqwest::Method) -> RequestComponentsBuilder {
        let mut headers: HeaderMap<HeaderValue> = HeaderMap::with_capacity(2);
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        let mut builder = RequestComponentsBuilder::default();
        builder
            .resource_identity(ri)
            .method(method)
            .headers(headers);
        builder
    }

    pub fn builder_with_body(
        ri: ResourceIdentity,
        method: reqwest::Method,
        body: String,
    ) -> RequestComponentsBuilder {
        let mut headers: HeaderMap<HeaderValue> = HeaderMap::with_capacity(2);
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        let mut builder = RequestComponentsBuilder::default();
        builder
            .resource_identity(ri)
            .method(method)
            .body(body)
            .headers(headers);
        builder
    }
}

impl From<(ResourceIdentity, reqwest::Method, GraphResult<GraphUrl>)> for RequestComponents {
    fn from(value: (ResourceIdentity, reqwest::Method, GraphResult<GraphUrl>)) -> Self {
        RequestComponents::new(value.0, value.2.unwrap_or_default(), value.1, None)
    }
}

impl
    From<(
        ResourceIdentity,
        reqwest::Method,
        GraphResult<GraphUrl>,
        GraphResult<String>,
    )> for RequestComponents
{
    fn from(
        value: (
            ResourceIdentity,
            reqwest::Method,
            GraphResult<GraphUrl>,
            GraphResult<String>,
        ),
    ) -> Self {
        RequestComponents::new(value.0, value.2.unwrap_or_default(), value.1, value.3.ok())
    }
}

#[derive(Builder, Default, Debug)]
#[builder(pattern = "owned", derive(Debug), setter(into, strip_option), default)]
pub struct ResponseHandler {
    pub(crate) inner: reqwest::Client,
    pub(crate) access_token: String,
    pub(crate) request_components: RequestComponents,
    pub(crate) error: Option<GraphFailure>,
}

impl ResponseHandler {
    pub fn new(
        inner: Client,
        request_components: RequestComponents,
        error: Option<GraphFailure>,
    ) -> ResponseHandler {
        ResponseHandler {
            inner: inner.inner.clone(),
            access_token: inner.access_token.clone(),
            request_components,
            error,
        }
    }

    pub fn is_err(&self) -> bool {
        self.error.is_some()
    }

    pub fn err(&self) -> Option<&GraphFailure> {
        self.error.as_ref()
    }

    pub async fn build(self) -> GraphResult<reqwest::RequestBuilder> {
        if let Some(err) = self.error {
            return Err(GraphFailure::PreFlightError {
                url: Some(self.request_components.to_reqwest_url()),
                headers: self.request_components.headers.clone(),
                error: Box::new(err),
            })
            .map_err(GraphFailure::from);
        }

        Ok(self.default_request_builder())
    }

    pub async fn send(self) -> GraphResult<reqwest::Response> {
        let request_builder = self.build().await?;
        request_builder.send().await.map_err(GraphFailure::from)
    }

    pub async fn download(self, download_config: DownloadConfig) -> GraphResult<PathBuf> {
        if let Some(err) = self.error {
            return Err(GraphFailure::PreFlightError {
                url: Some(self.request_components.to_reqwest_url()),
                headers: self.request_components.headers.clone(),
                error: Box::new(err),
            });
        }

        let request_builder = self.default_request_builder();
        let response = request_builder.send().await.map_err(GraphFailure::from)?;
        response
            .download(download_config)
            .await
            .map_err(GraphFailure::from)
    }

    pub fn url(&self) -> reqwest::Url {
        self.request_components.url.to_reqwest_url()
    }

    fn query(mut self, key: &str, value: &str) -> Self {
        self.request_components.url.append_query_pair(key, value);
        self
    }

    pub fn header<K: Into<HeaderName>, V: Into<HeaderValue>>(
        mut self,
        header_name: K,
        header_value: V,
    ) -> Self {
        self.request_components
            .headers
            .insert(header_name.into(), header_value.into());
        self
    }

    pub fn headers(mut self, header_map: HeaderMap) -> Self {
        self.request_components.headers.extend(header_map);
        self
    }

    pub(crate) fn default_request_builder(&self) -> reqwest::RequestBuilder {
        let mut request_builder = self
            .inner
            .request(
                self.request_components.method.clone(),
                self.request_components.url.to_reqwest_url(),
            )
            .bearer_auth(self.access_token.as_str())
            .headers(self.request_components.headers.clone());

        if let Some(body) = self.request_components.body.as_ref() {
            return request_builder.body(body.to_string());
        }
        request_builder
    }

    fn try_stream(&self) -> impl Stream<Item = GraphResult<reqwest::Response>> + '_ {
        try_stream! {
            let request = self.default_request_builder();
            let mut response = request.send().await?;
            let json: serde_json::Value = response.body_from_bytes().await?;
            let mut next_link = json.odata_next_link();
            yield response;

            while let Some(url) = next_link {
                let mut response = self.inner.get(url).send().await?;
                let json: serde_json::Value = response.body_from_bytes().await?;
                next_link = json.odata_next_link();
                yield response;
            }
        }
    }

    fn into_stream(self) -> GraphResult<impl Stream<Item = GraphResult<reqwest::Response>>> {
        Ok(stream! {
            let stream = self.try_stream();
            for await value in stream {
                yield value
            }
        })
    }

    /// Stream the current request along with any next link requests from the response body.
    pub fn stream(self) -> GraphResult<impl Stream<Item = GraphResult<reqwest::Response>>> {
        if let Some(err) = self.error {
            return Err(GraphFailure::PreFlightError {
                url: Some(self.request_components.to_reqwest_url()),
                headers: self.request_components.headers.clone(),
                error: Box::new(err),
            });
        }

        Ok(Box::pin(self.into_stream()?))
    }
}

impl ODataQuery for ResponseHandler {
    fn append_query_pair<KV: AsRef<str>>(self, key: KV, value: KV) -> Self {
        self.query(key.as_ref(), value.as_ref())
    }
}

impl AsRef<GraphUrl> for ResponseHandler {
    fn as_ref(&self) -> &GraphUrl {
        self.request_components.as_ref()
    }
}

impl AsMut<GraphUrl> for ResponseHandler {
    fn as_mut(&mut self) -> &mut GraphUrl {
        self.request_components.as_mut()
    }
}
