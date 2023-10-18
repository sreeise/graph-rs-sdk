use crate::internal::GraphClientConfiguration;
use graph_extensions::token::ClientApplication;
use reqwest::header::HeaderMap;
use std::env::VarError;
use std::ffi::OsStr;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct BlockingClient {
    pub(crate) inner: reqwest::blocking::Client,
    pub(crate) client_application: Box<dyn ClientApplication>,
    pub(crate) headers: HeaderMap,
}

impl BlockingClient {
    pub fn new<AT: ToString>(access_token: AT) -> BlockingClient {
        GraphClientConfiguration::new()
            .access_token(access_token)
            .build_blocking()
    }

    /// Create a new client and use the given environment variable
    /// for the access token.
    pub fn new_env<K: AsRef<OsStr>>(env_var: K) -> Result<BlockingClient, VarError> {
        Ok(GraphClientConfiguration::new()
            .access_token(std::env::var(env_var)?)
            .build_blocking())
    }

    pub fn builder() -> GraphClientConfiguration {
        GraphClientConfiguration::new()
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }
}

impl Default for BlockingClient {
    fn default() -> Self {
        GraphClientConfiguration::new().build_blocking()
    }
}

impl Debug for BlockingClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockingClient")
            .field("inner", &self.inner)
            .field("headers", &self.headers)
            .finish()
    }
}
