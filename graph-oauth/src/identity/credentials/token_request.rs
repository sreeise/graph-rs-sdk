use crate::oauth::{AuthorizationSerializer, TokenCredentialOptions};
use async_trait::async_trait;

#[async_trait]
pub trait TokenRequest: AuthorizationSerializer {
    fn token_credential_options(&self) -> &TokenCredentialOptions;

    fn get_token(&mut self) -> anyhow::Result<reqwest::blocking::Response> {
        let options = self.token_credential_options().clone();
        let uri = self.uri(&options.azure_authority_host)?;
        let form = self.form()?;
        let http_client = reqwest::blocking::Client::new();
        Ok(http_client.post(uri).form(&form).send()?)
    }
    async fn get_token_async(&mut self) -> anyhow::Result<reqwest::Response> {
        let options = self.token_credential_options().clone();
        let uri = self.uri(&options.azure_authority_host)?;
        let form = self.form()?;
        let http_client = reqwest::Client::new();
        Ok(http_client.post(uri).form(&form).send().await?)
    }
}
