use crate::blocking::BlockingRequestHandler;
use crate::internal::{
    BodyRead, Client, GraphClientConfiguration, HttpResponseBuilderExt, ODataNextLink, ODataQuery,
    RequestComponents,
};
use async_stream::try_stream;
use futures::Stream;
use graph_error::{AuthExecutionResult, ErrorMessage, GraphFailure, GraphResult};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE};
use reqwest::{Request, Response};
use serde::de::DeserializeOwned;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::time::Duration;
use tower::util::BoxCloneService;
use tower::{Service, ServiceExt};
use url::Url;

pub struct RequestHandler {
    pub(crate) inner: Client,
    pub(crate) request_components: RequestComponents,
    pub(crate) error: Option<GraphFailure>,
    pub(crate) body: Option<BodyRead>,
    pub(crate) client_builder: GraphClientConfiguration,
    pub(crate) service:
        BoxCloneService<Request, Response, Box<dyn std::error::Error + Send + Sync>>,
}

impl RequestHandler {
    pub fn new(
        inner: Client,
        mut request_components: RequestComponents,
        err: Option<GraphFailure>,
        body: Option<BodyRead>,
    ) -> RequestHandler {
        let service = inner.builder.build_tower_service(&inner.inner);
        let client_builder = inner.builder.clone();
        let mut original_headers = inner.headers.clone();
        original_headers.extend(request_components.headers.clone());
        request_components.headers = original_headers;

        let mut error = None;
        if let Some(err) = err {
            let message = err.to_string();
            error = Some(GraphFailure::PreFlightError {
                url: Some(request_components.url.clone()),
                headers: Some(request_components.headers.clone()),
                error: Some(Box::new(err)),
                message,
            });
        }

        RequestHandler {
            inner,
            request_components,
            error,
            body,
            client_builder,
            service,
        }
    }

    pub fn into_blocking(self) -> BlockingRequestHandler {
        BlockingRequestHandler::new(
            self.client_builder.build_blocking(),
            self.request_components,
            self.error,
            self.body,
        )
    }

    /// Returns true if any errors occurred prior to sending the request.
    ///
    /// # Example
    /// ```rust,ignore
    /// let client = Graph::new("ACCESS_TOKEN");
    /// let request_handler = client.groups().list_group();
    /// println!("{:#?}", request_handler.is_err());
    /// ```
    pub fn is_err(&self) -> bool {
        self.error.is_some()
    }

    /// Returns any error wrapped in an Option that occurred prior to sending a request
    ///
    /// # Example
    /// ```rust,ignore
    /// let client = Graph::new("ACCESS_TOKEN");
    /// let request_handler = client.groups().list_group();
    /// println!("{:#?}", request_handler.err());
    /// ```
    pub fn err(&self) -> Option<&GraphFailure> {
        self.error.as_ref()
    }

    #[inline]
    pub fn url(&self) -> Url {
        self.request_components.url.clone()
    }

    #[inline]
    pub fn query<T: serde::Serialize + ?Sized>(mut self, query: &T) -> Self {
        if let Err(err) = self.request_components.query(query) {
            if self.error.is_none() {
                self.error = Some(err);
            }
        }

        if let Some("") = self.request_components.url.query() {
            self.request_components.url.set_query(None);
        }
        self
    }

    #[inline]
    pub fn append_query_pair<KV: AsRef<str>>(mut self, key: KV, value: KV) -> Self {
        self.request_components
            .url
            .query_pairs_mut()
            .append_pair(key.as_ref(), value.as_ref());
        self
    }

    #[inline]
    pub fn extend_path<I: AsRef<str>>(mut self, path: &[I]) -> Self {
        if let Ok(mut p) = self.request_components.url.path_segments_mut() {
            p.extend(path);
        }
        self
    }

    /// Insert a header for the request.
    #[inline]
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
    #[inline]
    pub fn headers(mut self, header_map: HeaderMap) -> Self {
        self.request_components.headers.extend(header_map);
        self
    }

    /// Get a mutable reference to the headers.
    #[inline]
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        self.request_components.as_mut()
    }

    pub fn paging(self) -> Paging {
        Paging(self)
    }

    pub(crate) async fn default_request_builder_with_token(
        &mut self,
    ) -> AuthExecutionResult<(String, reqwest::RequestBuilder)> {
        let access_token = self
            .inner
            .client_application
            .get_token_silent_async()
            .await?;

        let request_builder = self
            .inner
            .inner
            .request(
                self.request_components.method.clone(),
                self.request_components.url.clone(),
            )
            .bearer_auth(access_token.as_str())
            .headers(self.request_components.headers.clone());

        if let Some(body) = self.body.take() {
            if body.has_byte_buf() {
                self.request_components
                    .headers
                    .entry(CONTENT_TYPE)
                    .or_insert(HeaderValue::from_static("application/octet-stream"));
            } else if body.has_string_buf() {
                self.request_components
                    .headers
                    .entry(CONTENT_TYPE)
                    .or_insert(HeaderValue::from_static("application/json"));
            }
            return Ok((
                access_token,
                request_builder
                    .body::<reqwest::Body>(body.into())
                    .headers(self.request_components.headers.clone()),
            ));
        }
        Ok((access_token, request_builder))
    }

    pub(crate) async fn default_request_builder(&mut self) -> GraphResult<reqwest::RequestBuilder> {
        let access_token = self
            .inner
            .client_application
            .get_token_silent_async()
            .await?;

        let request_builder = self
            .inner
            .inner
            .request(
                self.request_components.method.clone(),
                self.request_components.url.clone(),
            )
            .bearer_auth(access_token.as_str())
            .headers(self.request_components.headers.clone());

        if let Some(body) = self.body.take() {
            if body.has_byte_buf() {
                self.request_components
                    .headers
                    .entry(CONTENT_TYPE)
                    .or_insert(HeaderValue::from_static("application/octet-stream"));
            } else if body.has_string_buf() {
                self.request_components
                    .headers
                    .entry(CONTENT_TYPE)
                    .or_insert(HeaderValue::from_static("application/json"));
            }
            return Ok(request_builder
                .body::<reqwest::Body>(body.into())
                .headers(self.request_components.headers.clone()));
        }

        Ok(request_builder)
    }

    /// Builds the request and returns a [`reqwest::RequestBuilder`].
    #[inline]
    pub async fn build(mut self) -> GraphResult<reqwest::RequestBuilder> {
        if let Some(err) = self.error {
            return Err(err);
        }
        self.default_request_builder().await
    }

    #[inline]
    pub async fn send(self) -> GraphResult<reqwest::Response> {
        let mut service = self.service.clone();
        let request_builder = self.build().await?;
        let request = request_builder.build()?;
        service
            .ready()
            .await
            .map_err(GraphFailure::from)?
            .call(request)
            .await
            .map_err(GraphFailure::from)
    }
}

impl ODataQuery for RequestHandler {
    fn append_query_pair<KV: AsRef<str>>(self, key: KV, value: KV) -> Self {
        self.append_query_pair(key.as_ref(), value.as_ref())
    }
}

impl AsRef<Url> for RequestHandler {
    fn as_ref(&self) -> &Url {
        self.request_components.as_ref()
    }
}

impl AsMut<Url> for RequestHandler {
    fn as_mut(&mut self) -> &mut Url {
        self.request_components.as_mut()
    }
}

pub type PagingResponse<T> = http::Response<Result<T, ErrorMessage>>;
pub type PagingResult<T> = GraphResult<PagingResponse<T>>;

pub struct Paging(RequestHandler);

impl Paging {
    async fn http_response<T: DeserializeOwned>(
        response: reqwest::Response,
    ) -> GraphResult<(Option<String>, PagingResponse<T>)> {
        let status = response.status();
        let url = response.url().clone();
        let headers = response.headers().clone();
        let version = response.version();

        let body: serde_json::Value = response.json().await?;
        let next_link = body.odata_next_link();
        let json = body.clone();
        let body_result: Result<T, ErrorMessage> = serde_json::from_value(body)
            .map_err(|_| serde_json::from_value(json.clone()).unwrap_or(ErrorMessage::default()));

        let mut builder = http::Response::builder()
            .url(url)
            .json(&json)
            .status(http::StatusCode::from(&status))
            .version(version);

        for builder_header in builder.headers_mut().iter_mut() {
            builder_header.extend(headers.clone());
        }

        Ok((next_link, builder.body(body_result)?))
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
    /// #[derive(Debug, Serialize, Deserialize)]
    /// pub struct User {
    ///     pub(crate) id: Option<String>,
    ///     #[serde(rename = "userPrincipalName")]
    ///     user_principal_name: Option<String>,
    /// }
    ///
    /// #[derive(Debug, Serialize, Deserialize)]
    /// pub struct Users {
    ///     pub value: Vec<User>,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> GraphResult<()> {
    ///     let client = GraphClient::new("ACCESS_TOKEN");
    ///
    ///     let deque = client
    ///         .users()
    ///         .list_user()
    ///         .select(&["id", "userPrincipalName"])
    ///         .paging()
    ///         .json::<Users>()
    ///         .await?;
    ///
    ///     for response in deque.iter() {
    ///         let users = response.into_body()?;
    ///         println!("{users:#?}");
    ///     }
    ///     Ok(())
    /// }
    ///
    /// ```
    pub async fn json<T: DeserializeOwned>(mut self) -> GraphResult<VecDeque<PagingResponse<T>>> {
        if let Some(err) = self.0.error {
            return Err(err);
        }

        let (access_token, request) = self.0.default_request_builder_with_token().await?;
        let response = request.send().await?;

        let (next, http_response) = Paging::http_response(response).await?;
        let mut next_link = next;
        let mut vec = VecDeque::new();
        vec.push_back(http_response);

        let client = self.0.inner.inner.clone();
        while let Some(next) = next_link {
            let response = client
                .get(next)
                .bearer_auth(access_token.as_str())
                .send()
                .await?;

            let (next, http_response) = Paging::http_response(response).await?;

            next_link = next;
            vec.push_back(http_response);
        }

        Ok(vec)
    }

    fn try_stream<'a, T: DeserializeOwned + 'a>(
        mut self,
    ) -> impl Stream<Item = PagingResult<T>> + 'a {
        try_stream! {
            let (access_token, request) = self.0.default_request_builder_with_token().await?;
            let response = request.send().await?;
            let (next, http_response) = Paging::http_response(response).await?;
            let mut next_link = next;
            yield http_response;

            while let Some(url) = next_link {
                let response = self.0
                    .inner
                    .inner
                    .get(url)
                    .bearer_auth(access_token.as_str())
                    .send()
                    .await?;
                let (next, http_response) = Paging::http_response(response).await?;
                next_link = next;
                yield http_response;
            }
        }
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
        mut self,
    ) -> GraphResult<impl Stream<Item = PagingResult<T>> + 'a> {
        if let Some(err) = self.0.error.take() {
            return Err(err);
        }

        Ok(Box::pin(self.try_stream()))
    }

    /// Get next link responses using a channel Receiver [`tokio::sync::mpsc::Receiver<Option<GraphResult<http::Response<T>>>>`].
    ///
    /// By default channels use [`tokio::sync::mpsc::Sender::send_timeout`] with a buffer of 100
    /// and a timeout of 60. Use [`Paging::channel_timeout`] to set a custom timeout and use
    /// [`Paging::channel_buffer_timeout`] to set both the buffer and timeout.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let client = Graph::new("ACCESS_TOKEN");
    ///
    ///  let mut receiver = client
    ///     .users()
    ///     .list_user()
    ///     .top("5")
    ///     .paging()
    ///     .channel::<serde_json::Value>()
    ///     .await?;
    ///
    ///  while let Some(result) = receiver.recv().await {
    ///     let response = result?;
    ///     println!("{:#?}", response);
    ///  }
    /// ```
    pub async fn channel<T: DeserializeOwned + Debug + Send + 'static>(
        self,
    ) -> GraphResult<tokio::sync::mpsc::Receiver<PagingResult<T>>> {
        self.channel_buffer_timeout(100, Duration::from_secs(60))
            .await
    }

    /// Get next link responses using a channel Receiver,
    /// [`tokio::sync::mpsc::Receiver<Option<GraphResult<http::Response<T>>>>`].
    /// using a custom timeout [`Duration`]
    ///
    /// By default channels use [`tokio::sync::mpsc::Sender::send_timeout`] with a buffer of 100
    /// and a timeout of 60. Use [`Paging::channel_timeout`] to set a custom timeout and use
    /// [`Paging::channel_buffer_timeout`] to set both the buffer and timeout.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let client = Graph::new("ACCESS_TOKEN");
    ///
    ///  let mut receiver = client
    ///     .users()
    ///     .list_user()
    ///     .top("5")
    ///     .paging()
    ///     .channel_timeout::<serde_json::Value>(Duration::from_secs(60))
    ///     .await?;
    ///
    ///  while let Some(result) = receiver.recv().await {
    ///     let response = result?;
    ///     println!("{:#?}", response);
    ///  }
    /// ```
    pub async fn channel_timeout<T: DeserializeOwned + Debug + Send + 'static>(
        self,
        timeout: Duration,
    ) -> GraphResult<tokio::sync::mpsc::Receiver<PagingResult<T>>> {
        self.channel_buffer_timeout(100, timeout).await
    }

    async fn send_channel_request<T: DeserializeOwned>(
        client: &reqwest::Client,
        url: &str,
        access_token: &str,
    ) -> GraphResult<(Option<String>, PagingResponse<T>)> {
        let response = client.get(url).bearer_auth(access_token).send().await?;

        Paging::http_response(response).await
    }

    /// Get next link responses using a channel Receiver,
    /// [`tokio::sync::mpsc::Receiver<Option<GraphResult<http::Response<T>>>>`].
    /// with a custom timeout [`Duration`] and buffer.
    ///
    /// By default channels use [`tokio::sync::mpsc::Sender::send_timeout`] with a buffer of 100
    /// and a timeout of 60. Use [`Paging::channel_timeout`] to set a custom timeout and use
    /// [`Paging::channel_buffer_timeout`] to set both the buffer and timeout.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let client = Graph::new("ACCESS_TOKEN");
    ///
    ///  let mut receiver = client
    ///     .users()
    ///     .list_user()
    ///     .top("5")
    ///     .paging()
    ///     .channel_buffer_timeout::<serde_json::Value>(100, Duration::from_secs(60))
    ///     .await?;
    ///
    ///  while let Some(result) = receiver.recv().await {
    ///     let response = result?;
    ///     println!("{:#?}", response);
    ///  }
    /// ```
    #[allow(unused_assignments)] // Issue with Clippy not seeing next_link get assigned.
    pub async fn channel_buffer_timeout<T: DeserializeOwned + Debug + Send + 'static>(
        mut self,
        buffer: usize,
        timeout: Duration,
    ) -> GraphResult<tokio::sync::mpsc::Receiver<PagingResult<T>>> {
        let (sender, receiver) = tokio::sync::mpsc::channel(buffer);

        let (access_token, request) = self.0.default_request_builder_with_token().await?;
        let response = request.send().await?;
        let (next, http_response) = Paging::http_response(response).await?;
        let mut next_link = next;
        sender
            .send_timeout(Ok(http_response), timeout)
            .await
            .unwrap();

        let client = self.0.inner.inner.clone();
        tokio::spawn(async move {
            while let Some(next) = next_link {
                let result =
                    Paging::send_channel_request(&client, next.as_str(), access_token.as_str())
                        .await;

                match result {
                    Ok((next, response)) => {
                        next_link = next;
                        sender.send_timeout(Ok(response), timeout).await.unwrap();
                    }
                    Err(err) => {
                        sender.send_timeout(Err(err), timeout).await.unwrap();
                        next_link = None;
                        break;
                    }
                }
            }
        });

        Ok(receiver)
    }
}
