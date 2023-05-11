use crate::blocking::BlockingClient;
use crate::traits::ODataQuery;

use backoff::exponential::ExponentialBackoff;
use backoff::{ExponentialBackoffBuilder, SystemClock};
use graph_error::GraphResult;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use reqwest::redirect::Policy;
use reqwest::tls::Version;
use std::env::VarError;
use std::ffi::OsStr;
use std::fmt::{Debug, Formatter};
use std::time::{Duration, Instant};
use url::Url;

#[derive(Debug, Clone)]
pub struct RetriesConfig {
    /// The current retry interval.
    pub current_interval: Duration,
    /// The initial retry interval.
    pub initial_interval: Duration,
    /// The randomization factor to use for creating a range around the retry interval.
    ///
    /// A randomization factor of 0.5 results in a random period ranging between 50% below and 50%
    /// above the retry interval.
    pub randomization_factor: f64,
    /// The value to multiply the current interval with for each retry attempt.
    pub multiplier: f64,
    /// The maximum value of the back off period. Once the retry interval reaches this
    /// value it stops increasing.
    pub max_interval: Duration,
    /// The system time. It is calculated when an [`ExponentialBackoff`](struct.ExponentialBackoff.html) instance is
    /// created and is reset when [`retry`](../trait.Operation.html#method.retry) is called.
    pub start_time: Instant,
    /// The maximum elapsed time after instantiating [`ExponentialBackoff`](struct.ExponentialBackoff.html) or calling
    /// [`reset`](trait.Backoff.html#method.reset) after which [`next_backoff`](../trait.Backoff.html#method.reset) returns `None`.
    pub max_elapsed_time: Option<Duration>,
}

impl Default for RetriesConfig {
    fn default() -> RetriesConfig {
        RetriesConfig {
            current_interval: Duration::from_millis(500),
            initial_interval: Duration::from_millis(500),
            randomization_factor: 0.0,
            multiplier: 1.5,
            max_interval: Duration::from_secs(450),
            max_elapsed_time: Some(Duration::from_millis(450)),
            start_time: Instant::now(),
        }
    }
}

#[derive(Clone)]
struct ClientConfiguration {
    access_token: Option<String>,
    headers: HeaderMap,
    referer: bool,
    timeout: Option<Duration>,
    retries: bool,
    retries_config: RetriesConfig,
    connect_timeout: Option<Duration>,
    connection_verbose: bool,
    https_only: bool,
    min_tls_version: Version,
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
            retries: false,
            retries_config: RetriesConfig::default(),
            connect_timeout: None,
            connection_verbose: false,
            https_only: true,
            /// TLS 1.2 required to support all features in Microsoft Graph
            /// See [Reliability and Support](https://learn.microsoft.com/en-us/graph/best-practices-concept#reliability-and-support)
            min_tls_version: Version::TLS_1_2,
        }
    }
}

impl Debug for ClientConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientConfiguration")
            .field("headers", &self.headers)
            .field("referer", &self.referer)
            .field("timeout", &self.timeout)
            .field("retries", &self.retries)
            .field("retries_config", &self.retries_config)
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

    /// Enables the client to perform retries if the request fail.
    ///
    /// Default is no retries.
    pub fn retries(mut self, retries: bool) -> GraphClientConfiguration {
        self.config.retries = retries;
        self
    }

    pub(crate) fn get_retries(&self) -> bool {
        self.config.retries
    }

    /// Set the Retries Config to be used by the Exponential Backoff Builder
    /// Has no effect if the retries are not enabled
    ///
    /// # Note
    ///
    /// If Graph API returns an Error 429 Too Many Requests with a Retry-After header
    /// it will be preferred over the configured exponential backoff
    pub fn retries_config(mut self, config: RetriesConfig) -> GraphClientConfiguration {
        self.config.retries_config = config;
        self
    }

    pub fn get_retries_config(self) -> RetriesConfig {
        self.config.retries_config
    }

    pub(crate) fn get_retries_exponential_backoff(&self) -> ExponentialBackoff<SystemClock> {
        ExponentialBackoffBuilder::new()
            .with_initial_interval(self.config.retries_config.initial_interval)
            .with_multiplier(self.config.retries_config.multiplier)
            .with_randomization_factor(self.config.retries_config.randomization_factor)
            .with_max_interval(self.config.retries_config.max_interval)
            .with_max_elapsed_time(self.config.retries_config.max_elapsed_time)
            .build()
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

        Client {
            access_token: self.config.access_token.unwrap_or_default(),
            inner: builder.build().unwrap(),
            headers,
            builder: config,
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
