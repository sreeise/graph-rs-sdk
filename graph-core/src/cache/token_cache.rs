use crate::identity::ForceTokenRefresh;
use async_trait::async_trait;
use graph_error::AuthExecutionError;

pub trait AsBearer<RHS = Self> {
    fn as_bearer(&self) -> String;
}

impl AsBearer for String {
    fn as_bearer(&self) -> String {
        self.clone()
    }
}

impl AsBearer for &str {
    fn as_bearer(&self) -> String {
        self.to_string()
    }
}

#[async_trait]
pub trait TokenCache {
    type Token: AsBearer;

    fn get_token_silent(&mut self) -> Result<Self::Token, AuthExecutionError>;

    async fn get_token_silent_async(&mut self) -> Result<Self::Token, AuthExecutionError>;

    fn with_force_token_refresh(&mut self, force_token_refresh: ForceTokenRefresh);
}
