use async_trait::async_trait;

#[async_trait]
pub trait TokenRequest {
    fn get_token_silent(&mut self) -> anyhow::Result<reqwest::blocking::Response>;
    async fn get_token_silent_async(&mut self) -> anyhow::Result<reqwest::Response>;
}
