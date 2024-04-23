use crate::blocking::BlockingClient;
use crate::traits::ODataQuery;

use graph_error::GraphResult;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use reqwest::redirect::Policy;
use reqwest::tls::Version;
use reqwest::{Request, Response};
use std::env::VarError;
use std::ffi::OsStr;
use std::fmt::{Debug, Formatter};
use std::time::Duration;
use tower::limit::ConcurrencyLimitLayer;
use tower::retry::RetryLayer;
use tower::util::BoxCloneService;
use tower::ServiceExt;

use url::Url;

#[derive(Default, Clone)]
struct ServiceLayersConfiguration {
    concurrency_limit: Option<usize>,
    retry: Option<usize>,
    wait_for_retry_after_headers: Option<()>,
}

#[derive(Clone)]
struct ClientConfiguration {
    access_token: Option<String>,
    headers: HeaderMap,
    referer: bool,
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    connection_verbose: bool,
    https_only: bool,
    /// TLS 1.2 required to support all features in Microsoft Graph
    /// See [Reliability and Support](https://learn.microsoft.com/en-us/graph/best-practices-concept#reliability-and-support)
    min_tls_version: Version,
    service_layers_configuration: ServiceLayersConfiguration,
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
            min_tls_version: Version::TLS_1_2,
            service_layers_configuration: ServiceLayersConfiguration::default(),
        }
    }
}

impl Debug for ClientConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientConfiguration")
            .field("headers", &self.headers)
            .field("referer", &self.referer)
            .field("timeout", &self.timeout)
            .field("connect_timeout", &self.connect_timeout)
            .field("https_only", &self.https_only)
            .field("min_tls_version", &self.min_tls_version)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct GraphClientConfiguration {
    config: ClientConfiguration,
}

impl GraphClientConfiguration {
    pub fn new() -> GraphClientConfiguration {
        GraphClientConfiguration {
            config: ClientConfiguration::new(),
        }
    }

    pub fn access_token<AT: ToString>(mut self, access_token: AT) -> GraphClientConfiguration {
        self.config.access_token = Some(access_token.to_string());
        self
    }

    pub fn default_headers(mut self, headers: HeaderMap) -> GraphClientConfiguration {
        for (key, value) in headers.iter() {
            self.config.headers.insert(key, value.clone());
        }
        self
    }

    /// Enable or disable automatic setting of the `Referer` header.
    ///
    /// Default is `true`.
    pub fn referer(mut self, enable: bool) -> GraphClientConfiguration {
        self.config.referer = enable;
        self
    }

    /// Enables a request timeout.
    ///
    /// The timeout is applied from when the request starts connecting until the
    /// response body has finished.
    ///
    /// Default is no timeout.
    pub fn timeout(mut self, timeout: Duration) -> GraphClientConfiguration {
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
    pub fn connect_timeout(mut self, timeout: Duration) -> GraphClientConfiguration {
        self.config.connect_timeout = Some(timeout);
        self
    }

    /// Set whether connections should emit verbose logs.
    ///
    /// Enabling this option will emit [log][] messages at the `TRACE` level
    /// for read and write operations on connections.
    ///
    /// [log]: https://crates.io/crates/log
    pub fn connection_verbose(mut self, verbose: bool) -> GraphClientConfiguration {
        self.config.connection_verbose = verbose;
        self
    }

    pub fn user_agent(mut self, value: HeaderValue) -> GraphClientConfiguration {
        self.config.headers.insert(USER_AGENT, value);
        self
    }

    pub fn min_tls_version(mut self, version: Version) -> GraphClientConfiguration {
        self.config.min_tls_version = version;
        self
    }

    #[cfg(feature = "test-util")]
    pub fn https_only(mut self, https_only: bool) -> GraphClientConfiguration {
        self.config.https_only = https_only;
        self
    }

    /// Enable a request retry in case of failure and how many times we should retry before giving up
    ///
    /// Some requests may fail on GraphAPI side and should be retried. Only server errors (HTTP code between 500 and 599) with be retried.
    ///
    /// Default is no retry
    pub fn retry(mut self, retry: Option<usize>) -> GraphClientConfiguration {
        self.config.service_layers_configuration.retry = retry;
        self
    }

    /// Enable a request retry if we reach the throttling limits and GraphAPI returns a 429 Too Many Requests with a Retry-After header
    ///
    /// Be careful with this parameter as some API endpoints have quite low limits (reports for example) and the request may hang for hundreds of seconds.
    /// For maximum throughput you may want to not respect the Retry-After header as hitting another server thanks to load-balancing may lead to a successful response.
    ///
    /// Default is no retry
    pub fn wait_for_retry_after_headers(mut self, retry: bool) -> GraphClientConfiguration {
        self.config
            .service_layers_configuration
            .wait_for_retry_after_headers = match retry {
            true => Some(()),
            false => None,
        };
        self
    }

    /// Enable a concurrency limit on the client
    ///
    /// Every request through this client will be subject to a concurrency limit. Can be useful to stay under the API limits set by GraphAPI
    ///
    /// Default is no concurrency limit
    pub fn concurrency_limit(
        mut self,
        concurrency_limit: Option<usize>,
    ) -> GraphClientConfiguration {
        self.config.service_layers_configuration.concurrency_limit = concurrency_limit;
        self
    }

    pub fn build(self) -> Client {
        let config = self.clone();
        let headers = self.config.headers.clone();
        let mut builder = reqwest::ClientBuilder::new()
            .default_headers(self.config.headers)
            .referer(self.config.referer)
            .connection_verbose(self.config.connection_verbose)
            .https_only(self.config.https_only)
            .min_tls_version(self.config.min_tls_version)
            .redirect(Policy::limited(2));

        if let Some(timeout) = self.config.timeout {
            builder = builder.timeout(timeout);
        }

        if let Some(connect_timeout) = self.config.connect_timeout {
            builder = builder.connect_timeout(connect_timeout);
        }

        let client = builder.build().unwrap();

        let service = tower::ServiceBuilder::new()
            .option_layer(
                self.config
                    .service_layers_configuration
                    .retry
                    .map(|num| RetryLayer::new(crate::tower_services::Attempts(num))),
            )
            .option_layer(
                self.config
                    .service_layers_configuration
                    .wait_for_retry_after_headers
                    .map(|_| RetryLayer::new(crate::tower_services::WaitFor())),
            )
            .option_layer(
                self.config
                    .service_layers_configuration
                    .concurrency_limit
                    .map(ConcurrencyLimitLayer::new),
            )
            .service(client.clone())
            .boxed_clone();

        Client {
            access_token: self.config.access_token.unwrap_or_default(),
            inner: client,
            headers,
            builder: config,
            service,
        }
    }

    pub(crate) fn build_blocking(self) -> BlockingClient {
        let headers = self.config.headers.clone();
        let mut builder = reqwest::blocking::ClientBuilder::new()
            .default_headers(self.config.headers)
            .referer(self.config.referer)
            .connection_verbose(self.config.connection_verbose)
            .https_only(self.config.https_only)
            .min_tls_version(self.config.min_tls_version)
            .redirect(Policy::limited(2));

        if let Some(timeout) = self.config.timeout {
            builder = builder.timeout(timeout);
        }

        if let Some(connect_timeout) = self.config.connect_timeout {
            builder = builder.connect_timeout(connect_timeout);
        }

        BlockingClient {
            access_token: self.config.access_token.unwrap_or_default(),
            inner: builder.build().unwrap(),
            headers,
        }
    }
}

impl Default for GraphClientConfiguration {
    fn default() -> Self {
        GraphClientConfiguration::new()
    }
}

#[derive(Clone)]
pub struct Client {
    pub(crate) access_token: String,
    pub(crate) inner: reqwest::Client,
    pub(crate) headers: HeaderMap,
    pub(crate) builder: GraphClientConfiguration,
    pub(crate) service:
        BoxCloneService<Request, Response, Box<dyn std::error::Error + Send + Sync>>,
}

impl Client {
    pub fn new<AT: ToString>(access_token: AT) -> Client {
        GraphClientConfiguration::new()
            .access_token(access_token)
            .build()
    }

    /// Create a new client and use the given environment variable
    /// for the access token.
    pub fn new_env<K: AsRef<OsStr>>(env_var: K) -> Result<Client, VarError> {
        Ok(GraphClientConfiguration::new()
            .access_token(std::env::var(env_var)?)
            .build())
    }

    pub fn builder() -> GraphClientConfiguration {
        GraphClientConfiguration::new()
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }
}

impl Default for Client {
    fn default() -> Self {
        GraphClientConfiguration::new().build()
    }
}

impl Debug for Client {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("inner", &self.inner)
            .field("headers", &self.headers)
            .field("builder", &self.builder)
            .finish()
    }
}

pub trait ApiClientImpl: ODataQuery + Sized {
    fn url(&self) -> Url;

    fn render_path<S: AsRef<str>>(
        &self,
        path: S,
        path_params_map: &serde_json::Value,
    ) -> GraphResult<String>;

    fn build_url<S: AsRef<str>>(
        &self,
        path: S,
        path_params_map: &serde_json::Value,
    ) -> GraphResult<Url> {
        let path = self.render_path(path.as_ref(), path_params_map)?;
        let mut vec: Vec<&str> = path.split('/').collect();
        vec.retain(|s| !s.is_empty());
        let mut url = self.url();
        if let Ok(mut p) = url.path_segments_mut() {
            p.extend(&vec);
        }
        Ok(url)
    }
}
