use crate::client::Client;
use crate::odata_query::ODataQuery;
use crate::traits::HttpResponseBuilderExt;
use crate::traits::{AsBytesMut, ODataNextLink, ResponseExt};
use crate::url::GraphUrl;
use crate::FileConfig;
use async_stream::{stream, try_stream};
use bytes::BytesMut;
use futures_core::Stream;
use futures_util::StreamExt;
use graph_core::resource::ResourceIdentity;
use graph_error::{GraphFailure, GraphResult, WithGraphErrorAsync};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE};
use reqwest::Method;
use serde::de::DeserializeOwned;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::path::PathBuf;
use std::time::Duration;

/// Provides components for storing resource id's and helps build the current request URL.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ResourceConfig {
    pub resource_identity: ResourceIdentity,
    pub url: GraphUrl,
    pub resource_identity_id: Option<String>,
}

impl ResourceConfig {
    pub fn new(ri: ResourceIdentity, url: GraphUrl, ri_id: Option<String>) -> ResourceConfig {
        ResourceConfig {
            resource_identity: ri,
            url,
            resource_identity_id: ri_id,
        }
    }
}

impl ResourceConfig {
    pub fn resource_identity(&self) -> ResourceIdentity {
        self.resource_identity
    }
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

#[derive(Default, Debug)]
pub struct RequestHandler {
    pub(crate) inner: reqwest::Client,
    pub(crate) access_token: String,
    pub(crate) request_components: RequestComponents,
    pub(crate) error: Option<GraphFailure>,
}

impl RequestHandler {
    pub fn new(
        inner: Client,
        mut request_components: RequestComponents,
        err: Option<GraphFailure>,
    ) -> RequestHandler {
        request_components.headers.extend(inner.headers.into_iter());

        let mut error = None;
        if let Some(err) = err {
            error = Some(GraphFailure::PreFlightError {
                url: Some(request_components.to_reqwest_url()),
                headers: request_components.headers.clone(),
                error: Box::new(err),
            });
        }

        RequestHandler {
            inner: inner.inner.clone(),
            access_token: inner.access_token,
            request_components,
            error,
        }
    }

    /// Returns true if any errors occurred prior to sending the request.
    ///
    /// # Example
    /// ```rust,ignore
    /// let client = Graph::new("ACCESS_TOKEN");
    /// let response_handler = client.groups().list_group();
    /// println!("{:#?}", response_handler.is_err());
    /// ```
    pub fn is_err(&self) -> bool {
        self.error.is_some()
    }

    /// Returns any error wrapped in an Option that occurred prior to sending a request
    ///
    /// # Example
    /// ```rust,ignore
    /// let client = Graph::new("ACCESS_TOKEN");
    /// let response_handler = client.groups().list_group();
    /// println!("{:#?}", response_handler.err());
    /// ```
    pub fn err(&self) -> Option<&GraphFailure> {
        self.error.as_ref()
    }

    pub fn url(&self) -> reqwest::Url {
        self.request_components.url.to_reqwest_url()
    }

    fn query(mut self, key: &str, value: &str) -> Self {
        self.request_components.url.append_query_pair(key, value);
        self
    }

    pub fn extend_path(mut self, value: &[&str]) -> Self {
        self.request_components.url.extend_path(value);
        self
    }

    /// Insert a header for the request.
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

    /// Set the headers for the request using reqwest::HeaderMap
    pub fn headers(mut self, header_map: HeaderMap) -> Self {
        self.request_components.headers.extend(header_map);
        self
    }

    /// Get a mutable reference to the headers.
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        self.request_components.as_mut()
    }

    pub fn paging(self) -> Paging {
        Paging(self)
    }

    pub(crate) fn default_request_builder(&mut self) -> reqwest::RequestBuilder {
        let request_builder = self
            .inner
            .request(
                self.request_components.method.clone(),
                self.request_components.url.to_reqwest_url(),
            )
            .bearer_auth(self.access_token.as_str())
            .headers(self.request_components.headers.clone());

        if let Some(body) = self.request_components.body.take() {
            self.request_components
                .headers
                .entry(CONTENT_TYPE)
                .or_insert(HeaderValue::from_static("application/json"));
            return request_builder
                .body(body)
                .headers(self.request_components.headers.clone());
        }
        request_builder
    }

    /// Builds the request and returns a [`reqwest::RequestBuilder`].
    pub async fn build(mut self) -> GraphResult<reqwest::RequestBuilder> {
        if let Some(err) = self.error {
            return Err(err);
        }
        Ok(self.default_request_builder())
    }

    pub async fn send(self) -> GraphResult<reqwest::Response> {
        let request_builder = self.build().await?;
        request_builder
            .send()
            .await?
            .with_graph_error()
            .await
            .map_err(GraphFailure::from)
    }

    pub async fn download(mut self, file_config: &FileConfig) -> GraphResult<PathBuf> {
        if let Some(err) = self.error {
            return Err(err);
        }

        let request_builder = self.default_request_builder();
        let response = request_builder.send().await.map_err(GraphFailure::from)?;
        response
            .with_graph_error()
            .await?
            .download(file_config)
            .await
            .map_err(GraphFailure::from)
    }

    pub async fn upload<T: AsBytesMut>(self, input: &mut T) -> GraphResult<reqwest::Response> {
        if let Some(err) = self.error {
            return Err(err);
        }

        let bytes_mut = input.as_bytes_mut()?;

        let request_builder = self
            .inner
            .request(
                self.request_components.method.clone(),
                self.request_components.url.to_reqwest_url(),
            )
            .bearer_auth(self.access_token.as_str())
            .headers(self.request_components.headers.clone())
            .body(bytes_mut.to_vec());

        request_builder.send().await?.with_graph_error().await
    }
}

impl ODataQuery for RequestHandler {
    fn append_query_pair<KV: AsRef<str>>(self, key: KV, value: KV) -> Self {
        self.query(key.as_ref(), value.as_ref())
    }
}

impl AsRef<GraphUrl> for RequestHandler {
    fn as_ref(&self) -> &GraphUrl {
        self.request_components.as_ref()
    }
}

impl AsMut<GraphUrl> for RequestHandler {
    fn as_mut(&mut self) -> &mut GraphUrl {
        self.request_components.as_mut()
    }
}

impl From<(RequestComponents, &Client)> for RequestHandler {
    fn from(value: (RequestComponents, &Client)) -> Self {
        RequestHandler {
            inner: value.1.inner.clone(),
            access_token: value.1.access_token.clone(),
            request_components: value.0,
            error: None,
        }
    }
}

#[derive(Debug)]
pub enum ChannelResponse<T: Debug> {
    Next(GraphResult<http::Response<T>>),
    Done,
}

pub struct Paging(RequestHandler);

impl Paging {
    async fn http_response<T: DeserializeOwned>(
        response: reqwest::Response,
    ) -> GraphResult<(Option<String>, http::Response<T>)> {
        let status = response.status();
        let url = response.url().clone();
        let headers = response.headers().clone();
        let version = response.version();
        dbg!(&version);

        let json: serde_json::Value = response.json().await?;
        dbg!(&json);
        let next_link = json.odata_next_link();
        let body: T = serde_json::from_value(json)?;

        let mut builder = http::Response::builder()
            .url(url)
            .status(http::StatusCode::from(&status))
            .version(version);

        for builder_header in builder.headers_mut().iter_mut() {
            builder_header.extend(headers.clone());
        }

        Ok((next_link, builder.body(body)?))
    }

    /// Returns all next links as [`VecDeque<http::Response<T>>`]. This method may
    /// cause significant delay in returning when there is a high volume of next links.
    ///
    /// This method is mainly provided for convenience in cases where the caller is sure
    /// the requests will return successful without issue or where the caller is ok with
    /// a return delay or does not care if errors occur. It is not recommended to use this
    /// method in production environments.
    ///
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut stream = client
    ///     .users()
    ///     .delta()
    ///     .paging()
    ///     .stream::<serde_json::Value>()
    ///     .unwrap();
    ///
    ///  while let Some(result) = stream.next().await {
    ///     println!("{result:#?}");
    ///  }
    /// ```
    pub async fn json<T: DeserializeOwned>(mut self) -> GraphResult<VecDeque<http::Response<T>>> {
        if let Some(err) = self.0.error {
            return Err(err);
        }

        let request = self.0.default_request_builder();
        let response = request.send().await?.with_graph_error().await?;

        let (next, http_response) = Paging::http_response(response).await?;
        let mut next_link = next;
        let mut vec = VecDeque::new();
        vec.push_back(http_response);

        let client = self.0.inner.clone();
        let access_token = self.0.access_token.clone();
        while let Some(next) = next_link {
            let response = client
                .get(next)
                .bearer_auth(access_token.as_str())
                .send()
                .await
                .map_err(GraphFailure::from)?
                .with_graph_error()
                .await?;

            let (next, http_response) = Paging::http_response(response).await?;

            next_link = next;
            vec.push_back(http_response);
        }

        Ok(vec)
    }

    fn try_stream<'a, T: DeserializeOwned + 'a>(
        &'a mut self,
    ) -> impl Stream<Item = GraphResult<http::Response<T>>> + 'a {
        try_stream! {
            let request = self.0.default_request_builder();
            let response = request.send().await?;
            let (next, http_response) = Paging::http_response(response).await?;
            let mut next_link = next;
            yield http_response;

            while let Some(url) = next_link {
                let response = self.0
                    .inner
                    .get(url)
                    .bearer_auth(self.0.access_token.as_str())
                    .send()
                    .await?;
                let (next, http_response) = Paging::http_response(response).await?;
                next_link = next;
                yield http_response;
            }
        }
    }

    fn into_stream<'a, T: DeserializeOwned + 'a>(
        mut self,
    ) -> GraphResult<impl Stream<Item = GraphResult<http::Response<T>>> + 'a> {
        Ok(stream! {
            let stream = self.try_stream();
            for await value in stream {
                yield value
            }
        })
    }

    /// Stream the current request along with any next link requests from the response body.
    /// Each stream.next() returns a [`GraphResult<http::Response<T>>`].
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut stream = client
    ///     .users()
    ///     .delta()
    ///     .paging()
    ///     .stream::<serde_json::Value>()
    ///     .unwrap();
    ///
    ///  while let Some(result) = stream.next().await {
    ///     println!("{result:#?}");
    ///  }
    /// ```
    pub fn stream<'a, T: DeserializeOwned + 'a>(
        self,
    ) -> GraphResult<impl Stream<Item = GraphResult<http::Response<T>>> + 'a> {
        if let Some(err) = self.0.error {
            return Err(err);
        }

        Ok(Box::pin(self.into_stream()?))
    }

    /// Get next link responses using a channel Receiver [`tokio::sync::mpsc::Receiver<ChannelResponse<T>>`].
    /// The channel Senders have a default timeout of 30 seconds when attempting a send.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use graph_http::ChannelResponse;
    /// let client = Graph::new("ACCESS_TOKEN");
    ///
    /// let mut receiver = client
    ///     .users()
    ///     .delta()
    ///     .paging()
    ///     .channel::<serde_json::Value>()
    ///     .await
    ///     .unwrap();
    ///
    ///  while let Some(channel_response) = receiver.recv().await {
    ///     match channel_response {
    ///         ChannelResponse::Next(result) => {
    ///             match result {
    ///                 Ok(response) => {
    ///                     println!("{:#?}", response);
    ///                 }
    ///                 Err(err) => {
    ///                     println!("{:#?}", err);
    ///                     break;
    ///                 }
    ///             }
    ///         }
    ///         ChannelResponse::Done => break,
    ///     }
    ///  }
    /// ```
    pub async fn channel<T: DeserializeOwned + Debug + Send + 'static>(
        self,
    ) -> GraphResult<tokio::sync::mpsc::Receiver<ChannelResponse<T>>> {
        self.channel_timeout(Duration::from_secs(30)).await
    }

    /// Get next link responses using a channel Receiver [`tokio::sync::mpsc::Receiver<ChannelResponse<T>>`] with a custom timeout [`Duration`].
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use graph_http::ChannelResponse;
    /// let client = Graph::new("ACCESS_TOKEN");
    ///
    /// let mut receiver = client
    ///     .users()
    ///     .delta()
    ///     .paging()
    ///     .channel::<serde_json::Value>()
    ///     .await
    ///     .unwrap();
    ///
    ///  while let Some(channel_response) = receiver.recv().await {
    ///     match channel_response {
    ///         ChannelResponse::Next(result) => {
    ///             match result {
    ///                 Ok(response) => {
    ///                     println!("{:#?}", response);
    ///                 }
    ///                 Err(err) => {
    ///                     println!("{:#?}", err);
    ///                     break;
    ///                 }
    ///             }
    ///         }
    ///         ChannelResponse::Done => break,
    ///     }
    ///  }
    /// ```
    pub async fn channel_timeout<T: DeserializeOwned + Debug + Send + 'static>(
        self,
        timeout: Duration,
    ) -> GraphResult<tokio::sync::mpsc::Receiver<ChannelResponse<T>>> {
        let mut stream = self.stream()?;
        let (sender, receiver) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            while let Some(result) = stream.next().await {
                sender
                    .send_timeout(ChannelResponse::Next(result), timeout)
                    .await
                    .unwrap();
            }
            sender
                .send_timeout(ChannelResponse::Done, timeout)
                .await
                .unwrap();
        })
        .await
        .unwrap();

        Ok(receiver)
    }
}
