use async_trait::async_trait;
use graph_error::GraphResult;

#[async_trait]
pub trait TokenRequest {
    async fn get_token_silent(&self) -> GraphResult<reqwest::Response>;
}
