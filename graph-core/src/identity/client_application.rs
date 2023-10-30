use async_trait::async_trait;
use dyn_clone::DynClone;
use graph_error::AuthExecutionResult;

dyn_clone::clone_trait_object!(ClientApplication);

#[async_trait]
pub trait ClientApplication: DynClone + Send + Sync {
    fn get_token_silent(&mut self) -> AuthExecutionResult<String>;

    async fn get_token_silent_async(&mut self) -> AuthExecutionResult<String>;
}

#[async_trait]
impl ClientApplication for String {
    fn get_token_silent(&mut self) -> AuthExecutionResult<String> {
        Ok(self.clone())
    }

    async fn get_token_silent_async(&mut self) -> AuthExecutionResult<String> {
        Ok(self.clone())
    }
}
