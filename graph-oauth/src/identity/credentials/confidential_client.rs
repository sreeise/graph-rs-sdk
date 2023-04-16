use crate::identity::{
    AuthorizationCodeCertificateCredential, AuthorizationCodeCredential, AuthorizationSerializer,
    TokenCredentialOptions, TokenRequest,
};
use async_trait::async_trait;
use graph_error::GraphResult;
use reqwest::Response;

pub struct ConfidentialClient {
    http_client: reqwest::Client,
    credential: Box<dyn AuthorizationSerializer + Send>,
    token_credential_options: TokenCredentialOptions,
}

impl ConfidentialClient {
    pub fn new<T>(credential: T, options: TokenCredentialOptions) -> GraphResult<ConfidentialClient>
    where
        T: Into<ConfidentialClient>,
    {
        let mut confidential_client = credential.into();
        confidential_client.token_credential_options = options;
        Ok(confidential_client)
    }
}

#[async_trait]
impl TokenRequest for ConfidentialClient {
    fn get_token_silent(&mut self) -> anyhow::Result<reqwest::blocking::Response> {
        let uri = self
            .credential
            .uri(&self.token_credential_options.azure_authority_host)?;
        let form = self.credential.form()?;
        let http_client = reqwest::blocking::Client::new();
        Ok(http_client.post(uri).form(&form).send()?)
    }

    async fn get_token_silent_async(&mut self) -> anyhow::Result<Response> {
        let uri = self
            .credential
            .uri(&self.token_credential_options.azure_authority_host)?;
        let form = self.credential.form()?;
        Ok(self.http_client.post(uri).form(&form).send().await?)
    }
}

impl From<AuthorizationCodeCredential> for ConfidentialClient {
    fn from(value: AuthorizationCodeCredential) -> Self {
        ConfidentialClient {
            http_client: reqwest::Client::new(),
            credential: Box::new(value),
            token_credential_options: Default::default(),
        }
    }
}

impl From<AuthorizationCodeCertificateCredential> for ConfidentialClient {
    fn from(value: AuthorizationCodeCertificateCredential) -> Self {
        ConfidentialClient {
            http_client: reqwest::Client::new(),
            credential: Box::new(value),
            token_credential_options: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::identity::{Authority, AzureAuthorityHost};

    #[test]
    fn confidential_client_new() {
        let credential = AuthorizationCodeCredential::builder()
            .with_authorization_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL")
            .with_client_id("bb301aaa-1201-4259-a230923fds32")
            .with_client_secret("CLDIE3F")
            .with_scopes(vec!["Read.Write", "Fall.Down"])
            .with_redirect_uri("http://localhost:8888/redirect")
            .build();

        let mut confidential_client =
            ConfidentialClient::new(credential, TokenCredentialOptions::default()).unwrap();
        let credential_uri = confidential_client
            .credential
            .uri(&AzureAuthorityHost::AzurePublic)
            .unwrap();

        assert_eq!(
            "https://login.microsoftonline.com/common/oauth2/v2.0/token",
            credential_uri.as_str()
        );
    }

    #[test]
    fn confidential_client_tenant() {
        let credential = AuthorizationCodeCredential::builder()
            .with_authorization_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL")
            .with_client_id("bb301aaa-1201-4259-a230923fds32")
            .with_client_secret("CLDIE3F")
            .with_scopes(vec!["Read.Write", "Fall.Down"])
            .with_redirect_uri("http://localhost:8888/redirect")
            .with_authority(Authority::Consumers)
            .build();
        let mut confidential_client =
            ConfidentialClient::new(credential, TokenCredentialOptions::default()).unwrap();
        let credential_uri = confidential_client
            .credential
            .uri(&AzureAuthorityHost::AzurePublic)
            .unwrap();

        assert_eq!(
            "https://login.microsoftonline.com/consumers/oauth2/v2.0/token",
            credential_uri.as_str()
        );
    }
}
