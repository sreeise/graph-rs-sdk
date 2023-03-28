use crate::client::Client;
use crate::next_link::{NextLinkResponse, NextLinkValues};
use crate::odata_query::ODataQuery;
use crate::traits::{AsBytesMut, ODataNextLink, ResponseExt};
use crate::url::GraphUrl;
use crate::FileConfig;
use async_stream::{stream, try_stream};
use bytes::BytesMut;
use futures_core::Stream;
use futures_util::TryFutureExt;
use graph_core::resource::ResourceIdentity;
use graph_error::{GraphFailure, GraphResult, WithGraphErrorAsync};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE};
use reqwest::{Method, RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::io::Read;
use std::path::PathBuf;
use tokio::runtime::Handle;

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

async fn map_next_link_response<V: DeserializeOwned>(
    response: Response,
) -> (Option<String>, NextLinkResponse<Vec<V>>) {
    let headers = response.headers().clone();
    let status = response.status();
    let url = response.url().clone();
    let result_with_mapped_error = response
        .with_graph_error()
        .await
        .map_err(GraphFailure::from);

    match result_with_mapped_error {
        Ok(response) => {
            let mut result: GraphResult<NextLinkValues<V>> =
                response.json().await.map_err(GraphFailure::from);

            match result {
                Ok(body) => (
                    body.next_link.clone(),
                    NextLinkResponse::new(url, status, headers, body.value, None),
                ),
                Err(err) => (
                    None,
                    NextLinkResponse::new(url, status, headers, vec![], Some(err)),
                ),
            }
        }
        Err(err) => (
            None,
            NextLinkResponse::new(url, status, headers, vec![], Some(err)),
        ),
    }
}

async fn map_next_link_response_from_value<V: DeserializeOwned + ODataNextLink>(
    response: Response,
) -> (Option<String>, NextLinkResponse<Option<V>>) {
    let headers = response.headers().clone();
    let status = response.status();
    let url = response.url().clone();
    let result_with_mapped_error = response
        .with_graph_error()
        .await
        .map_err(GraphFailure::from);

    match result_with_mapped_error {
        Ok(response) => {
            let mut result: GraphResult<V> = response.json().await.map_err(GraphFailure::from);

            match result {
                Ok(body) => (
                    body.odata_next_link(),
                    NextLinkResponse::new(url, status, headers, Some(body), None),
                ),
                Err(err) => (
                    None,
                    NextLinkResponse::new(url, status, headers, None, Some(err)),
                ),
            }
        }
        Err(err) => (
            None,
            NextLinkResponse::new(url, status, headers, None, Some(err)),
        ),
    }
}

#[derive(Default, Debug)]
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
            access_token: inner.access_token,
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

    pub async fn build(mut self) -> GraphResult<reqwest::RequestBuilder> {
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

    pub async fn send(mut self) -> GraphResult<reqwest::Response> {
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
            return Err(GraphFailure::PreFlightError {
                url: Some(self.request_components.to_reqwest_url()),
                headers: self.request_components.headers.clone(),
                error: Box::new(err),
            });
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

    pub async fn upload<T: AsBytesMut>(mut self, input: &mut T) -> GraphResult<reqwest::Response> {
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

    fn try_stream<'a, T>(
        &'a mut self,
    ) -> impl Stream<Item = GraphResult<NextLinkResponse<Option<T>>>> + 'a
    where
        for<'de> T: serde::Deserialize<'de> + ODataNextLink + 'a,
    {
        try_stream! {
            let request = self.default_request_builder();
            let mut response = request.send().await?;
            let next_value = map_next_link_response_from_value(response).await;
            let mut next_link = next_value.0;
            yield next_value.1;

            while let Some(url) = next_link {
                let response = self.inner.get(url)
                .bearer_auth(self.access_token.as_str())
                .send()
                .await?;
                let next_value = map_next_link_response_from_value(response).await;
                next_link = next_value.0;
                yield next_value.1;
            }
        }
    }

    fn into_stream<'a, T>(
        mut self,
    ) -> GraphResult<impl Stream<Item = GraphResult<NextLinkResponse<Option<T>>>> + 'a>
    where
        for<'de> T: serde::Deserialize<'de> + ODataNextLink + 'a,
    {
        Ok(stream! {
            let stream = self.try_stream();
            for await value in stream {
                yield value
            }
        })
    }

    /// Stream the current request along with any next link requests from the response body.
    pub fn stream_next_links<'a, T>(
        mut self,
    ) -> GraphResult<impl Stream<Item = GraphResult<NextLinkResponse<Option<T>>>> + 'a>
    where
        for<'de> T: serde::Deserialize<'de> + ODataNextLink + 'a,
    {
        if let Some(err) = self.error {
            return Err(GraphFailure::PreFlightError {
                url: Some(self.request_components.to_reqwest_url()),
                headers: self.request_components.headers.clone(),
                error: Box::new(err),
            });
        }

        Ok(Box::pin(self.into_stream()?))
    }

    pub async fn json_next_links<V>(mut self) -> GraphResult<Vec<NextLinkResponse<Vec<V>>>>
    where
        for<'de> V: serde::Deserialize<'de>,
    {
        if let Some(err) = self.error {
            return Err(GraphFailure::PreFlightError {
                url: Some(self.request_components.to_reqwest_url()),
                headers: self.request_components.headers.clone(),
                error: Box::new(err),
            });
        }

        let mut next_link = None;
        let mut values = vec![];

        let request = self.default_request_builder();
        let mut response = request.send().await?;
        let next_value = map_next_link_response(response).await;
        next_link = next_value.0;
        values.push(next_value.1);

        while let Some(url) = next_link {
            let response = self
                .inner
                .get(url.as_str())
                .bearer_auth(self.access_token.as_str())
                .send()
                .await?;
            let next_value = map_next_link_response(response).await;
            next_link = next_value.0;
            values.push(next_value.1);
        }

        Ok(values)
    }

    pub async fn channel_next_links<V>(
        mut self,
    ) -> GraphResult<tokio::sync::mpsc::Receiver<NextLinkResponse<Vec<V>>>>
    where
        for<'de> V: serde::Deserialize<'de> + Send + ODataNextLink + Debug + 'static,
    {
        if let Some(err) = self.error {
            return Err(GraphFailure::PreFlightError {
                url: Some(self.request_components.to_reqwest_url()),
                headers: self.request_components.headers.clone(),
                error: Box::new(err),
            });
        }

        let (sender, receiver) = tokio::sync::mpsc::channel(100);
        let request = self.default_request_builder();
        let mut response = request.send().await?.with_graph_error().await?;

        let next_value = map_next_link_response(response).await;
        let mut next_link = next_value.0;
        sender.send(next_value.1).await.unwrap();

        let client = self.inner.clone();
        let access_token = self.access_token.clone();
        tokio::spawn(async move {
            while let Some(next) = next_link.as_ref() {
                let result = client
                    .get(next)
                    .bearer_auth(access_token.as_str())
                    .send()
                    .await
                    .map_err(GraphFailure::from);

                match result {
                    Ok(response) => {
                        let next_value = map_next_link_response(response).await;
                        next_link = next_value.0;
                        sender.send(next_value.1).await.unwrap();
                    }
                    Err(err) => {
                        sender
                            .send(NextLinkResponse::from((next.clone(), err)))
                            .await
                            .unwrap();
                        next_link = None;
                    }
                }
            }
        })
        .await
        .unwrap();

        Ok(receiver)
    }

    pub async fn channel<V>(
        mut self,
    ) -> GraphResult<tokio::sync::mpsc::Receiver<NextLinkResponse<Option<V>>>>
    where
        for<'de> V: serde::Deserialize<'de> + Send + ODataNextLink + Debug + 'static,
    {
        if let Some(err) = self.error {
            return Err(GraphFailure::PreFlightError {
                url: Some(self.request_components.to_reqwest_url()),
                headers: self.request_components.headers.clone(),
                error: Box::new(err),
            });
        }

        let (sender, receiver) = tokio::sync::mpsc::channel(100);
        let request = self.default_request_builder();
        let mut response = request.send().await?.with_graph_error().await?;

        let next_value = map_next_link_response_from_value(response).await;
        let mut next_link = next_value.0;
        sender.send(next_value.1).await.unwrap();

        let client = self.inner.clone();
        let access_token = self.access_token.clone();
        tokio::spawn(async move {
            while let Some(next) = next_link.as_ref() {
                let result = client
                    .get(next)
                    .bearer_auth(access_token.as_str())
                    .send()
                    .await
                    .map_err(GraphFailure::from);

                match result {
                    Ok(response) => {
                        let next_value = map_next_link_response_from_value(response).await;
                        next_link = next_value.0;
                        sender.send(next_value.1).await.unwrap();
                    }
                    Err(err) => {
                        sender
                            .send(NextLinkResponse::from((next.clone(), err)))
                            .await
                            .unwrap();
                        next_link = None;
                    }
                }
            }
        })
        .await
        .unwrap();

        Ok(receiver)
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

impl From<(RequestComponents, &Client)> for ResponseHandler {
    fn from(value: (RequestComponents, &Client)) -> Self {
        ResponseHandler {
            inner: value.1.inner.clone(),
            access_token: value.1.access_token.clone(),
            request_components: value.0,
            error: None,
        }
    }
}
