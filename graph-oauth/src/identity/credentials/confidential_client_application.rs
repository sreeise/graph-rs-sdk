use crate::identity::{
    AuthorizationCodeCertificateCredential, AuthorizationCodeCredential, AuthorizationSerializer,
    AzureAuthorityHost, ClientCertificateCredential, ClientSecretCredential,
    TokenCredentialOptions, TokenRequest,
};
use async_trait::async_trait;
use graph_error::{AuthorizationResult, GraphResult};
use reqwest::Response;
use std::collections::HashMap;
use url::Url;

pub struct ConfidentialClientApplication {
    http_client: reqwest::Client,
    credential: Box<dyn AuthorizationSerializer + Send>,
    token_credential_options: TokenCredentialOptions,
}

impl ConfidentialClientApplication {
    pub fn new<T>(
        credential: T,
        options: TokenCredentialOptions,
    ) -> GraphResult<ConfidentialClientApplication>
    where
        T: Into<ConfidentialClientApplication>,
    {
        let mut confidential_client = credential.into();
        confidential_client.token_credential_options = options;
        Ok(confidential_client)
    }
}

impl AuthorizationSerializer for ConfidentialClientApplication {
    fn uri(&mut self, azure_authority_host: &AzureAuthorityHost) -> AuthorizationResult<Url> {
        self.credential.uri(azure_authority_host)
    }

    fn form(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        self.credential.form()
    }
}

#[async_trait]
impl TokenRequest for ConfidentialClientApplication {
    fn azure_authority_host(&self) -> &AzureAuthorityHost {
        &self.token_credential_options.azure_authority_host
    }

    fn get_token(&mut self) -> anyhow::Result<reqwest::blocking::Response> {
        let uri = self
            .credential
            .uri(&self.token_credential_options.azure_authority_host)?;
        let form = self.credential.form()?;
        let http_client = reqwest::blocking::Client::new();
        Ok(http_client.post(uri).form(&form).send()?)
    }

    async fn get_token_async(&mut self) -> anyhow::Result<Response> {
        let uri = self
            .credential
            .uri(&self.token_credential_options.azure_authority_host)?;
        let form = self.credential.form()?;
        Ok(self.http_client.post(uri).form(&form).send().await?)
    }
}

impl From<AuthorizationCodeCredential> for ConfidentialClientApplication {
    fn from(value: AuthorizationCodeCredential) -> Self {
        ConfidentialClientApplication {
            http_client: reqwest::Client::new(),
            credential: Box::new(value),
            token_credential_options: Default::default(),
        }
    }
}

impl From<AuthorizationCodeCertificateCredential> for ConfidentialClientApplication {
    fn from(value: AuthorizationCodeCertificateCredential) -> Self {
        ConfidentialClientApplication {
            http_client: reqwest::Client::new(),
            credential: Box::new(value),
            token_credential_options: Default::default(),
        }
    }
}

impl From<ClientSecretCredential> for ConfidentialClientApplication {
    fn from(value: ClientSecretCredential) -> Self {
        ConfidentialClientApplication {
            http_client: reqwest::Client::new(),
            credential: Box::new(value),
            token_credential_options: Default::default(),
        }
    }
}

impl From<ClientCertificateCredential> for ConfidentialClientApplication {
    fn from(value: ClientCertificateCredential) -> Self {
        ConfidentialClientApplication {
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
            .with_scope(vec!["Read.Write", "Fall.Down"])
            .with_redirect_uri("http://localhost:8888/redirect")
            .build();

        let mut confidential_client =
            ConfidentialClientApplication::new(credential, TokenCredentialOptions::default())
                .unwrap();
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
            .with_scope(vec!["Read.Write", "Fall.Down"])
            .with_redirect_uri("http://localhost:8888/redirect")
            .with_authority(Authority::Consumers)
            .build();
        let mut confidential_client =
            ConfidentialClientApplication::new(credential, TokenCredentialOptions::default())
                .unwrap();
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
