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
pub trait ClientApplication: DynClone {
    fn get_token_silent(&mut self) -> AuthExecutionResult<String>;

    async fn get_token_silent_async(&mut self) -> AuthExecutionResult<String>;
}
