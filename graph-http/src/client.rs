use crate::url::GraphUrl;
use graph_core::resource::ResourceIdentity;
use graph_error::{GraphFailure, GraphResult};
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT};
use reqwest::redirect::Policy;
use reqwest::{Body, Certificate, ClientBuilder, Identity, Method, RequestBuilder, Response};
use std::env::VarError;
use std::ffi::OsStr;
use std::io::Write;
use std::time::Duration;
use tokio::runtime::Runtime;

pub type GraphRequest = reqwest::Request;

struct ClientConfiguration {
    access_token: Option<String>,
    headers: HeaderMap,
    referer: bool,
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    connection_verbose: bool,
    gzip: bool,
    deflate: bool,
    brotli: bool,
    https_only: bool,
    trust_dns: bool,
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
            gzip: false,
            deflate: false,
            brotli: false,
            https_only: false,
            trust_dns: false,
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

    /// Enable auto gzip decompression by checking the `Content-Encoding` response header.
    ///
    /// If auto gzip decompression is turned on:
    ///
    /// - When sending a request and if the request's headers do not already contain
    ///   an `Accept-Encoding` **and** `Range` values, the `Accept-Encoding` header is set to `gzip`.
    ///   The request body is **not** automatically compressed.
    /// - When receiving a response, if its headers contain a `Content-Encoding` value of
    ///   `gzip`, both `Content-Encoding` and `Content-Length` are removed from the
    ///   headers' set. The response body is automatically decompressed.
    ///
    /// If the `gzip` feature is turned on, the default option is enabled.
    ///
    /// # Optional
    ///
    /// This requires the optional `gzip` feature to be enabled
    pub fn gzip(mut self, enable: bool) -> GraphClientBuilder {
        self.config.gzip = enable;
        self
    }

    /// Enable auto deflate decompression by checking the `Content-Encoding` response header.
    ///
    /// If auto deflate decompression is turned on:
    ///
    /// - When sending a request and if the request's headers do not already contain
    ///   an `Accept-Encoding` **and** `Range` values, the `Accept-Encoding` header is set to `deflate`.
    ///   The request body is **not** automatically compressed.
    /// - When receiving a response, if it's headers contain a `Content-Encoding` value that
    ///   equals to `deflate`, both values `Content-Encoding` and `Content-Length` are removed from the
    ///   headers' set. The response body is automatically decompressed.
    ///
    /// If the `deflate` feature is turned on, the default option is enabled.
    ///
    /// # Optional
    ///
    /// This requires the optional `deflate` feature to be enabled
    pub fn deflate(mut self, enable: bool) -> GraphClientBuilder {
        self.config.deflate = enable;
        self
    }

    /// Enable auto brotli decompression by checking the `Content-Encoding` response header.
    ///
    /// If auto brotli decompression is turned on:
    ///
    /// - When sending a request and if the request's headers do not already contain
    ///   an `Accept-Encoding` **and** `Range` values, the `Accept-Encoding` header is set to `br`.
    ///   The request body is **not** automatically compressed.
    /// - When receiving a response, if its headers contain a `Content-Encoding` value of
    ///   `br`, both `Content-Encoding` and `Content-Length` are removed from the
    ///   headers' set. The response body is automatically decompressed.
    ///
    /// If the `brotli` feature is turned on, the default option is enabled.
    ///
    /// # Optional
    ///
    /// This requires the optional `brotli` feature to be enabled
    pub fn brotli(mut self, enable: bool) -> GraphClientBuilder {
        self.config.brotli = enable;
        self
    }

    /// Restrict the Client to be used with HTTPS only requests.
    ///
    /// Defaults to false.
    pub fn https_only(mut self, enabled: bool) -> GraphClientBuilder {
        self.config.https_only = enabled;
        self
    }

    /// Enables the [trust-dns](trust_dns_resolver) async resolver instead of a default threadpool using `getaddrinfo`.
    ///
    /// If the `trust-dns` feature is turned on, the default option is enabled.
    ///
    /// # Optional
    ///
    /// This requires the optional `trust-dns` feature to be enabled
    pub fn trust_dns(mut self, enable: bool) -> GraphClientBuilder {
        self.config.trust_dns = enable;
        self
    }

    pub fn user_agent(mut self, value: HeaderValue) -> GraphClientBuilder {
        self.config.headers.insert(USER_AGENT, value);
        self
    }

    pub fn build(mut self) -> Client {
        let mut builder = reqwest::ClientBuilder::new()
            .default_headers(self.config.headers)
            .referer(self.config.referer)
            .connection_verbose(self.config.connection_verbose)
            .gzip(self.config.gzip)
            .brotli(self.config.brotli)
            .https_only(self.config.https_only)
            .trust_dns(self.config.trust_dns);

        if let Some(timeout) = self.config.timeout.clone() {
            builder = builder.timeout(timeout)
        }

        if let Some(connect_timeout) = self.config.connect_timeout.clone() {
            builder = builder.connect_timeout(connect_timeout)
        }

        Client {
            access_token: self.config.access_token.unwrap_or_default(),
            inner: builder.build().unwrap(),
        }
    }
}

impl Default for GraphClientBuilder {
    fn default() -> Self {
        GraphClientBuilder::new()
    }
}

#[derive(Clone)]
pub struct Client {
    access_token: String,
    inner: reqwest::Client,
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
}

pub trait ApiClientImpl {
    fn url(&self) -> GraphUrl;

    fn render_template<S: AsRef<str>>(
        &self,
        path: S,
        path_params_map: &serde_json::Value,
    ) -> GraphResult<String>;

    fn build_url<S: AsRef<str>>(
        &self,
        path: S,
        path_params_map: &serde_json::Value,
    ) -> GraphResult<GraphUrl> {
        let mut path = self.render_template(path.as_ref(), path_params_map)?;
        let mut vec: Vec<&str> = path.split("/").collect();
        vec.retain(|s| !s.is_empty());
        let mut url = self.url();
        url.extend_path(&vec);
        Ok(url)
    }
}
