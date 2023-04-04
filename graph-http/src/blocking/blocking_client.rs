use crate::internal::GraphClientConfiguration;
use reqwest::header::HeaderMap;
use std::env::VarError;
use std::ffi::OsStr;

#[derive(Clone, Debug)]
pub struct BlockingClient {
    pub(crate) inner: reqwest::blocking::Client,
    pub(crate) access_token: String,
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
