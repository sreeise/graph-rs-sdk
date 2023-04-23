use async_trait::async_trait;

#[async_trait]
pub trait TokenRequest {
    fn get_token(&mut self) -> anyhow::Result<reqwest::blocking::Response>;
    async fn get_token_async(&mut self) -> anyhow::Result<reqwest::Response>;
}
