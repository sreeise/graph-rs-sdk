use crate::identity::{
    AuthorizationCodeCertificateCredential, AuthorizationCodeCredential, AuthorizationSerializer,
    AzureAuthorityHost, ClientCertificateCredential, ClientSecretCredential,
    TokenCredentialOptions, TokenRequest,
};
use async_trait::async_trait;
use graph_error::{AuthorizationResult, GraphResult};
use reqwest::tls::Version;
use reqwest::{ClientBuilder, Response};
use std::collections::HashMap;
use url::Url;

/// Clients capable of maintaining the confidentiality of their credentials
/// (e.g., client implemented on a secure server with restricted access to the client credentials),
/// or capable of secure client authentication using other means.
pub struct ConfidentialClientApplication {
    http_client: reqwest::Client,
    token_credential_options: TokenCredentialOptions,
    credential: Box<dyn AuthorizationSerializer + Send>,
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
    fn token_credential_options(&self) -> &TokenCredentialOptions {
        &self.token_credential_options
    }

    fn get_token(&mut self) -> anyhow::Result<reqwest::blocking::Response> {
        let azure_authority_host = self.token_credential_options.azure_authority_host.clone();
        let uri = self.credential.uri(&azure_authority_host)?;
        let form = self.credential.form()?;
        let http_client = reqwest::blocking::ClientBuilder::new()
            .min_tls_version(Version::TLS_1_2)
            .https_only(true)
            .build()?;

        let basic_auth = self.credential.basic_auth();
        if let Some((client_identifier, secret)) = basic_auth {
            Ok(http_client
                .post(uri)
                .basic_auth(client_identifier, Some(secret))
                .form(&form)
                .send()?)
        } else {
            Ok(http_client.post(uri).form(&form).send()?)
        }
    }

    async fn get_token_async(&mut self) -> anyhow::Result<Response> {
        let azure_authority_host = self.token_credential_options.azure_authority_host.clone();
        let uri = self.credential.uri(&azure_authority_host)?;
        let form = self.credential.form()?;
        let basic_auth = self.credential.basic_auth();
        if let Some((client_identifier, secret)) = basic_auth {
            Ok(self
                .http_client
                .post(uri)
                // https://datatracker.ietf.org/doc/html/rfc6749#section-2.3.1
                .basic_auth(client_identifier, Some(secret))
                .form(&form)
                .send()
                .await?)
        } else {
            Ok(self.http_client.post(uri).form(&form).send().await?)
        }
    }
}

impl From<AuthorizationCodeCredential> for ConfidentialClientApplication {
    fn from(value: AuthorizationCodeCredential) -> Self {
        ConfidentialClientApplication {
            http_client: ClientBuilder::new()
                .min_tls_version(Version::TLS_1_2)
                .https_only(true)
                .build()
                .unwrap(),
            token_credential_options: value.token_credential_options.clone(),
            credential: Box::new(value),
        }
    }
}

impl From<AuthorizationCodeCertificateCredential> for ConfidentialClientApplication {
    fn from(value: AuthorizationCodeCertificateCredential) -> Self {
        ConfidentialClientApplication {
            http_client: ClientBuilder::new()
                .min_tls_version(Version::TLS_1_2)
                .https_only(true)
                .build()
                .unwrap(),
            token_credential_options: value.token_credential_options.clone(),
            credential: Box::new(value),
        }
    }
}

impl From<ClientSecretCredential> for ConfidentialClientApplication {
    fn from(value: ClientSecretCredential) -> Self {
        ConfidentialClientApplication {
            http_client: ClientBuilder::new()
                .min_tls_version(Version::TLS_1_2)
                .https_only(true)
                .build()
                .unwrap(),
            token_credential_options: value.token_credential_options.clone(),
            credential: Box::new(value),
        }
    }
}

impl From<ClientCertificateCredential> for ConfidentialClientApplication {
    fn from(value: ClientCertificateCredential) -> Self {
        ConfidentialClientApplication {
            http_client: ClientBuilder::new()
                .min_tls_version(Version::TLS_1_2)
                .https_only(true)
                .build()
                .unwrap(),
            token_credential_options: value.token_credential_options.clone(),
            credential: Box::new(value),
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
