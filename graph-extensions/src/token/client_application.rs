use crate::cache::{StoredToken, TokenStore};
use async_trait::async_trait;
use dyn_clone::DynClone;
use graph_error::AuthExecutionResult;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ClientApplicationType {
    ConfidentialClientApplication,
    PublicClientApplication,
}

dyn_clone::clone_trait_object!(ClientApplication);

#[async_trait]
pub trait ClientApplication: TokenStore + DynClone {
    fn client_application_type(&self) -> ClientApplicationType;

    fn get_token_silent(&mut self) -> AuthExecutionResult<String>;

    async fn get_token_silent_async(&mut self) -> AuthExecutionResult<String>;

    fn get_stored_application_token(&mut self) -> Option<&StoredToken>;
}
