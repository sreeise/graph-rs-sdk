use async_trait::async_trait;
use graph_core::cache::AsBearer;
use graph_core::identity::ClientApplication;
use graph_error::AuthExecutionResult;

#[derive(Clone)]
pub struct BearerTokenCredential(String);

impl BearerTokenCredential {
    pub fn new(access_token: impl ToString) -> BearerTokenCredential {
        BearerTokenCredential(access_token.to_string())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl ToString for BearerTokenCredential {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl AsBearer for BearerTokenCredential {
    fn as_bearer(&self) -> String {
        self.0.clone()
    }
}

impl AsRef<str> for BearerTokenCredential {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&str> for BearerTokenCredential {
    fn from(value: &str) -> Self {
        BearerTokenCredential(value.to_string())
    }
}

impl From<String> for BearerTokenCredential {
    fn from(value: String) -> Self {
        BearerTokenCredential(value)
    }
}

#[async_trait]
impl ClientApplication for BearerTokenCredential {
    fn get_token_silent(&mut self) -> AuthExecutionResult<String> {
        Ok(self.0.clone())
    }

    async fn get_token_silent_async(&mut self) -> AuthExecutionResult<String> {
        Ok(self.0.clone())
    }
}
