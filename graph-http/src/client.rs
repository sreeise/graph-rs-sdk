use crate::blocking::BlockingClient;
use graph_core::identity::{ClientApplication, ForceTokenRefresh};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use reqwest::redirect::Policy;
use reqwest::tls::Version;
use std::env::VarError;
use std::ffi::OsStr;
use std::fmt::{Debug, Formatter};
use std::time::Duration;

fn user_agent_header_from_env() -> Option<HeaderValue> {
    let header = std::option_env!("GRAPH_CLIENT_USER_AGENT")?;
    HeaderValue::from_str(header).ok()
}

#[derive(Clone)]
struct ClientConfiguration {
    client_application: Option<Box<dyn ClientApplication>>,
    headers: HeaderMap,
    referer: bool,
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    connection_verbose: bool,
    https_only: bool,
    /// TLS 1.2 required to support all features in Microsoft Graph
    /// See [Reliability and Support](https://learn.microsoft.com/en-us/graph/best-practices-concept#reliability-and-support)
    min_tls_version: Version,
}

impl ClientConfiguration {
    pub fn new() -> ClientConfiguration {
        let mut headers: HeaderMap<HeaderValue> = HeaderMap::with_capacity(2);
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));

        if let Some(user_agent) = user_agent_header_from_env() {
            headers.insert(USER_AGENT, user_agent);
        }

        ClientConfiguration {
            client_application: None,
            headers,
            referer: true,
            timeout: None,
            connect_timeout: None,
            connection_verbose: false,
            https_only: true,
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
        self.config.client_application = Some(Box::new(access_token.to_string()));
        self
    }

    pub fn client_application<CA: ClientApplication + 'static>(mut self, client_app: CA) -> Self {
        self.config.client_application = Some(Box::new(client_app));
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

    /// TLS 1.2 required to support all features in Microsoft Graph
    /// See [Reliability and Support](https://learn.microsoft.com/en-us/graph/best-practices-concept#reliability-and-support)
    pub fn min_tls_version(mut self, version: Version) -> GraphClientConfiguration {
        self.config.min_tls_version = version;
        self
    }

    #[cfg(feature = "test-util")]
    pub fn https_only(mut self, https_only: bool) -> GraphClientConfiguration {
        self.config.https_only = https_only;
        self
    }

    pub fn build(self) -> Client {
        let config = self.clone();
        let headers = self.config.headers.clone();
        let mut builder = reqwest::ClientBuilder::new()
            .referer(self.config.referer)
            .connection_verbose(self.config.connection_verbose)
            .https_only(self.config.https_only)
            .min_tls_version(self.config.min_tls_version)
            .redirect(Policy::limited(2))
            .default_headers(self.config.headers);

        if let Some(timeout) = self.config.timeout {
            builder = builder.timeout(timeout);
        }

        if let Some(connect_timeout) = self.config.connect_timeout {
            builder = builder.connect_timeout(connect_timeout);
        }

        if let Some(client_application) = self.config.client_application {
            Client {
                client_application,
                inner: builder.build().unwrap(),
                headers,
                builder: config,
            }
        } else {
            Client {
                client_application: Box::<String>::default(),
                inner: builder.build().unwrap(),
                headers,
                builder: config,
            }
        }
    }

    pub(crate) fn build_blocking(self) -> BlockingClient {
        let headers = self.config.headers.clone();
        let mut builder = reqwest::blocking::ClientBuilder::new()
            .referer(self.config.referer)
            .connection_verbose(self.config.connection_verbose)
            .https_only(self.config.https_only)
            .min_tls_version(self.config.min_tls_version)
            .redirect(Policy::limited(2))
            .default_headers(self.config.headers);

        if let Some(timeout) = self.config.timeout {
            builder = builder.timeout(timeout);
        }

        if let Some(connect_timeout) = self.config.connect_timeout {
            builder = builder.connect_timeout(connect_timeout);
        }

        if let Some(client_application) = self.config.client_application {
            BlockingClient {
                client_application,
                inner: builder.build().unwrap(),
                headers,
            }
        } else {
            BlockingClient {
                client_application: Box::<String>::default(),
                inner: builder.build().unwrap(),
                headers,
            }
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
    pub(crate) client_application: Box<dyn ClientApplication>,
    pub(crate) inner: reqwest::Client,
    pub(crate) headers: HeaderMap,
    pub(crate) builder: GraphClientConfiguration,
}

impl Client {
    pub fn new<CA: ClientApplication + 'static>(client_app: CA) -> Self {
        GraphClientConfiguration::new()
            .client_application(client_app)
            .build()
    }

    pub fn from_access_token<T: AsRef<str>>(access_token: T) -> Self {
        GraphClientConfiguration::new()
            .access_token(access_token.as_ref())
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

    pub fn with_force_token_refresh(&mut self, force_token_refresh: ForceTokenRefresh) {
        self.client_application
            .with_force_token_refresh(force_token_refresh);
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

impl From<GraphClientConfiguration> for Client {
    fn from(value: GraphClientConfiguration) -> Self {
        value.build()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compile_time_user_agent_header() {
        let client = GraphClientConfiguration::new()
            .access_token("access_token")
            .build();

        assert!(client.builder.config.headers.contains_key(USER_AGENT));
    }

    #[test]
    fn update_user_agent_header() {
        let client = GraphClientConfiguration::new()
            .access_token("access_token")
            .user_agent(HeaderValue::from_static("user_agent"))
            .build();

        assert!(client.builder.config.headers.contains_key(USER_AGENT));
        let user_agent_header = client.builder.config.headers.get(USER_AGENT).unwrap();
        assert_eq!("user_agent", user_agent_header.to_str().unwrap());
    }
}
