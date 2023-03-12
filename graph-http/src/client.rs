use crate::odata_query::ODataQuery;
use crate::url::GraphUrl;
use crate::RequestComponents;
use graph_error::GraphResult;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use reqwest::{Body, Method};
use std::env::VarError;
use std::ffi::OsStr;
use std::time::Duration;

pub type GraphRequest = reqwest::Request;

struct ClientConfiguration {
    access_token: Option<String>,
    headers: HeaderMap,
    referer: bool,
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    connection_verbose: bool,
    https_only: bool,
}

impl ClientConfiguration {
    pub fn new() -> ClientConfiguration {
        let mut headers: HeaderMap<HeaderValue> = HeaderMap::with_capacity(2);
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));

        ClientConfiguration {
            access_token: None,
            headers,
            referer: true,
            timeout: None,
            connect_timeout: None,
            connection_verbose: false,
            https_only: true,
        }
    }
}

pub struct GraphClientBuilder {
    config: ClientConfiguration,
}

impl GraphClientBuilder {
    pub fn new() -> GraphClientBuilder {
        GraphClientBuilder {
            config: ClientConfiguration::new(),
        }
    }

    pub fn access_token<AT: ToString>(mut self, access_token: AT) -> GraphClientBuilder {
        self.config.access_token = Some(access_token.to_string());
        self
    }

    pub fn default_headers(mut self, headers: HeaderMap) -> GraphClientBuilder {
        for (key, value) in headers.iter() {
            self.config.headers.insert(key, value.clone());
        }
        self
    }

    /// Enable or disable automatic setting of the `Referer` header.
    ///
    /// Default is `true`.
    pub fn referer(mut self, enable: bool) -> GraphClientBuilder {
        self.config.referer = enable;
        self
    }

    /// Enables a request timeout.
    ///
    /// The timeout is applied from when the request starts connecting until the
    /// response body has finished.
    ///
    /// Default is no timeout.
    pub fn timeout(mut self, timeout: Duration) -> GraphClientBuilder {
        self.config.timeout = Some(timeout);
        self
    }

    /// Set a timeout for only the connect phase of a `Client`.
    ///
    /// Default is `None`.
    ///
    /// # Note
    ///
    /// This **requires** the futures be executed in a tokio runtime with
    /// a tokio timer enabled.
    pub fn connect_timeout(mut self, timeout: Duration) -> GraphClientBuilder {
        self.config.connect_timeout = Some(timeout);
        self
    }

    /// Set whether connections should emit verbose logs.
    ///
    /// Enabling this option will emit [log][] messages at the `TRACE` level
    /// for read and write operations on connections.
    ///
    /// [log]: https://crates.io/crates/log
    pub fn connection_verbose(mut self, verbose: bool) -> GraphClientBuilder {
        self.config.connection_verbose = verbose;
        self
    }

    pub fn user_agent(mut self, value: HeaderValue) -> GraphClientBuilder {
        self.config.headers.insert(USER_AGENT, value);
        self
    }

    pub fn build(self) -> Client {
        let headers = self.config.headers.clone();
        let mut builder = reqwest::ClientBuilder::new()
            .default_headers(self.config.headers)
            .referer(self.config.referer)
            .connection_verbose(self.config.connection_verbose)
            .https_only(self.config.https_only);

        if let Some(timeout) = self.config.timeout {
            builder = builder.timeout(timeout);
        }

        if let Some(connect_timeout) = self.config.connect_timeout {
            builder = builder.connect_timeout(connect_timeout);
        }

        Client {
            access_token: self.config.access_token.unwrap_or_default(),
            inner: builder.build().unwrap(),
            headers,
        }
    }
}

impl Default for GraphClientBuilder {
    fn default() -> Self {
        GraphClientBuilder::new()
    }
}

#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) access_token: String,
    pub(crate) inner: reqwest::Client,
    pub(crate) headers: HeaderMap,
}

impl Client {
    pub fn new<AT: ToString>(access_token: AT) -> Client {
        GraphClientBuilder::new().access_token(access_token).build()
    }

    /// Create a new client and use the given environment variable
    /// for the access token.
    pub fn new_env<K: AsRef<OsStr>>(env_var: K) -> Result<Client, VarError> {
        Ok(GraphClientBuilder::new()
            .access_token(std::env::var(env_var)?)
            .build())
    }

    pub fn builder() -> GraphClientBuilder {
        GraphClientBuilder::new()
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn default_builder(&self, method: Method, url: GraphUrl) -> reqwest::RequestBuilder {
        self.inner
            .request(method, url.to_reqwest_url())
            .bearer_auth(self.access_token.as_str())
    }

    pub fn default_builder_with_body<T: Into<Body>>(
        &self,
        method: Method,
        url: GraphUrl,
        body: T,
    ) -> reqwest::RequestBuilder {
        self.inner
            .request(method, url.to_reqwest_url())
            .bearer_auth(self.access_token.as_str())
            .body(body)
    }

    pub fn default_request_builder(
        &self,
        request_components: RequestComponents,
    ) -> reqwest::RequestBuilder {
        if let Some(body) = request_components.body {
            self.inner
                .request(
                    request_components.method,
                    request_components.url.to_reqwest_url(),
                )
                .bearer_auth(self.access_token.as_str())
                .body(body)
        } else {
            self.inner
                .request(
                    request_components.method,
                    request_components.url.to_reqwest_url(),
                )
                .bearer_auth(self.access_token.as_str())
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        GraphClientBuilder::new().build()
    }
}

pub trait ApiClientImpl: ODataQuery + Sized {
    fn url(&self) -> GraphUrl;

    fn render_path<S: AsRef<str>>(
        &self,
        path: S,
        path_params_map: &serde_json::Value,
    ) -> GraphResult<String>;

    fn build_url<S: AsRef<str>>(
        &self,
        path: S,
        path_params_map: &serde_json::Value,
    ) -> GraphResult<GraphUrl> {
        let path = self.render_path(path.as_ref(), path_params_map)?;
        let mut vec: Vec<&str> = path.split("/").collect();
        vec.retain(|s| !s.is_empty());
        let mut url = self.url();
        url.extend_path(&vec);
        Ok(url)
    }
}
