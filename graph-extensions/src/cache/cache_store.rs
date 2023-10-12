use crate::token::MsalToken;
use async_trait::async_trait;
use graph_error::AuthExecutionError;

pub trait AsBearer<RHS = Self> {
    fn as_bearer(&self) -> String;
}

pub struct BearerToken(String);

impl AsBearer for BearerToken {
    fn as_bearer(&self) -> String {
        self.0.clone()
    }
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

impl AsBearer for MsalToken {
    fn as_bearer(&self) -> String {
        self.access_token.to_string()
    }
}

#[async_trait]
pub trait TokenCacheStore {
    type Token: AsBearer;

    fn get_token_silent(&mut self) -> Result<Self::Token, AuthExecutionError>;

    async fn get_token_silent_async(&mut self) -> Result<Self::Token, AuthExecutionError>;
}
