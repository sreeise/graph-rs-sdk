use crate::blocking::blocking_client::BlockingClient;
use crate::internal::*;
use graph_error::{GraphFailure, GraphResult};
use http::header::CONTENT_TYPE;
use http::{HeaderMap, HeaderName, HeaderValue};
use serde::de::DeserializeOwned;
use std::collections::VecDeque;
use url::Url;

#[derive(Default)]
pub struct BlockingRequestHandler {
    pub(crate) inner: reqwest::blocking::Client,
    pub(crate) access_token: String,
    pub(crate) request_components: RequestComponents,
    pub(crate) error: Option<GraphFailure>,
    pub(crate) body: Option<BodyRead>,
}

impl BlockingRequestHandler {
    pub fn new(
        inner: BlockingClient,
        mut request_components: RequestComponents,
        err: Option<GraphFailure>,
        body: Option<BodyRead>,
    ) -> BlockingRequestHandler {
        let mut original_headers = inner.headers;
        original_headers.extend(request_components.headers.clone());
        request_components.headers = original_headers;

        let mut error = None;
        if let Some(err) = err {
            error = Some(GraphFailure::PreFlightError {
                url: Some(request_components.url.clone()),
                headers: request_components.headers.clone(),
                error: Box::new(err),
            });
        }

        BlockingRequestHandler {
            inner: inner.inner.clone(),
            access_token: inner.access_token,
            request_components,
            error,
            body,
        }
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

    pub fn paging(self) -> BlockingPaging {
        BlockingPaging(self)
    }

    #[inline]
    fn default_request_builder(&mut self) -> reqwest::blocking::RequestBuilder {
        let request_builder = self
            .inner
            .request(
                self.request_components.method.clone(),
                self.request_components.url.clone(),
            )
            .bearer_auth(self.access_token.as_str())
            .headers(self.request_components.headers.clone());

        if let Some(body) = self.body.take() {
            self.request_components
                .headers
                .entry(CONTENT_TYPE)
                .or_insert(HeaderValue::from_static("application/json"));
            return request_builder
                .body::<reqwest::blocking::Body>(body.into())
                .headers(self.request_components.headers.clone());
        }
        request_builder
    }

    /// Builds the request and returns a [`reqwest::blocking::RequestBuilder`].
    #[inline]
    pub fn build(mut self) -> GraphResult<reqwest::blocking::RequestBuilder> {
        if let Some(err) = self.error {
            return Err(err);
        }
        Ok(self.default_request_builder())
    }

    #[inline]
    pub fn send(self) -> GraphResult<reqwest::blocking::Response> {
        let request_builder = self.build()?;
        request_builder.send().map_err(GraphFailure::from)
    }
}

impl ODataQuery for BlockingRequestHandler {
    fn append_query_pair<KV: AsRef<str>>(self, key: KV, value: KV) -> Self {
        self.append_query_pair(key.as_ref(), value.as_ref())
    }
}

impl AsRef<Url> for BlockingRequestHandler {
    fn as_ref(&self) -> &Url {
        self.request_components.as_ref()
    }
}

impl AsMut<Url> for BlockingRequestHandler {
    fn as_mut(&mut self) -> &mut Url {
        self.request_components.as_mut()
    }
}

pub type BlockingPagingResult<T> = GraphResult<http::Response<GraphResult<T>>>;

pub struct BlockingPaging(BlockingRequestHandler);

impl BlockingPaging {
    fn http_response<T: DeserializeOwned>(
        response: reqwest::blocking::Response,
    ) -> GraphResult<(Option<String>, http::Response<GraphResult<T>>)> {
        let status = response.status();
        let url = response.url().clone();
        let headers = response.headers().clone();
        let version = response.version();

        let body: serde_json::Value = response.json()?;
        let next_link = body.odata_next_link();
        let json = body.clone();
        let body: GraphResult<T> = serde_json::from_value(body).map_err(GraphFailure::from);

        let mut builder = http::Response::builder()
            .url(url)
            .json(json)
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
    /// let response = client
    ///     .users()
    ///     .delta()
    ///     .into_blocking()
    ///     .paging()
    ///     .json::<serde_json::Value>()
    ///     .unwrap();
    ///
    /// println!("{response:#?}");
    /// println!("{:#?}", response.body());
    /// ```
    pub fn json<T: DeserializeOwned>(
        mut self,
    ) -> GraphResult<VecDeque<http::Response<GraphResult<T>>>> {
        if let Some(err) = self.0.error {
            return Err(err);
        }

        let request = self.0.default_request_builder();
        let response = request.send()?;

        let (next, http_response) = BlockingPaging::http_response(response)?;
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
                .map_err(GraphFailure::from)?;

            let (next, http_response) = BlockingPaging::http_response(response)?;

            next_link = next;
            vec.push_back(http_response);
        }

        Ok(vec)
    }

    fn send_channel_request<T: DeserializeOwned>(
        client: &reqwest::blocking::Client,
        next: &str,
        access_token: &str,
    ) -> GraphResult<(Option<String>, http::Response<GraphResult<T>>)> {
        let response = client
            .get(next)
            .bearer_auth(access_token)
            .send()
            .map_err(GraphFailure::from)?;

        BlockingPaging::http_response(response)
    }

    pub fn channel<T: DeserializeOwned + Send + 'static>(
        mut self,
    ) -> GraphResult<std::sync::mpsc::Receiver<Option<BlockingPagingResult<T>>>> {
        let (sender, receiver) = std::sync::mpsc::channel();
        let request = self.0.default_request_builder();
        let response = request.send()?;

        let (next, http_response) = BlockingPaging::http_response(response)?;
        let mut next_link = next;
        sender.send(Some(Ok(http_response))).unwrap();

        let client = self.0.inner.clone();
        let access_token = self.0.access_token.clone();

        std::thread::spawn(move || {
            while let Some(next) = next_link.as_ref() {
                let result = BlockingPaging::send_channel_request(
                    &client,
                    next.as_str(),
                    access_token.as_str(),
                );
                if let Ok((next_option, http_response)) = result {
                    next_link = next_option;
                    sender.send(Some(Ok(http_response))).unwrap();
                } else if let Err(err) = result {
                    sender.send(Some(Err(err))).unwrap();
                    break;
                }
            }
            sender.send(None).unwrap();
        });

        Ok(receiver)
    }
}
