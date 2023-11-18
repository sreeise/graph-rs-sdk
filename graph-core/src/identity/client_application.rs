use async_trait::async_trait;
use dyn_clone::DynClone;
use graph_error::AuthExecutionResult;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForceTokenRefresh {
    /// Always use the token cache first to when returning tokens.
    /// Expired tokens will still cause an authorization request to
    /// be called.
    #[default]
    Never,
    /// ForceRefreshToken::Once will cause only the next authorization request
    /// to ignore any tokens in cache and request a new token. Authorization
    /// requests after this are treated as ForceRefreshToken::Never
    Once,
    /// Always make an authorization request regardless of any tokens in cache.
    Always,
}

dyn_clone::clone_trait_object!(ClientApplication);

#[async_trait]
pub trait ClientApplication: DynClone + Send + Sync {
    fn get_token_silent(&mut self) -> AuthExecutionResult<String>;

    async fn get_token_silent_async(&mut self) -> AuthExecutionResult<String>;

    fn with_force_token_refresh(&mut self, force_token_refresh: ForceTokenRefresh);
}

#[async_trait]
impl ClientApplication for String {
    fn get_token_silent(&mut self) -> AuthExecutionResult<String> {
        Ok(self.clone())
    }

    async fn get_token_silent_async(&mut self) -> AuthExecutionResult<String> {
        Ok(self.clone())
    }

    fn with_force_token_refresh(&mut self, _force_token_refresh: ForceTokenRefresh) {}
}
