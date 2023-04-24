use crate::oauth::{AuthorizationSerializer, AzureAuthorityHost};
use async_trait::async_trait;

#[async_trait]
pub trait TokenRequest: AuthorizationSerializer {
    fn azure_authority_host(&self) -> &AzureAuthorityHost;

    fn get_token(&mut self) -> anyhow::Result<reqwest::blocking::Response> {
        let azure_authority_host = self.azure_authority_host().clone();
        let uri = self.uri(&azure_authority_host)?;
        let form = self.form()?;
        let http_client = reqwest::blocking::Client::new();
        Ok(http_client.post(uri).form(&form).send()?)
    }
    async fn get_token_async(&mut self) -> anyhow::Result<reqwest::Response> {
        let azure_authority_host = self.azure_authority_host().clone();
        let uri = self.uri(&azure_authority_host)?;
        let form = self.form()?;
        let http_client = reqwest::Client::new();
        Ok(http_client.post(uri).form(&form).send().await?)
    }
}
