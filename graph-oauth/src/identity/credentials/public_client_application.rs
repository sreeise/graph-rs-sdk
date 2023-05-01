use crate::identity::{
    AuthorizationSerializer, AzureAuthorityHost, ResourceOwnerPasswordCredential,
    TokenCredentialOptions, TokenRequest,
};
use async_trait::async_trait;
use graph_error::AuthorizationResult;
use reqwest::tls::Version;
use reqwest::{ClientBuilder, Response};
use std::collections::HashMap;
use url::Url;

/// Clients incapable of maintaining the confidentiality of their credentials
/// (e.g., clients executing on the device used by the resource owner, such as an
/// installed native application or a web browser-based application), and incapable of
/// secure client authentication via any other means.
pub struct PublicClientApplication {
    http_client: reqwest::Client,
    token_credential_options: TokenCredentialOptions,
    credential: Box<dyn AuthorizationSerializer + Send>,
}

impl PublicClientApplication {
    pub fn new<T>(
        credential: T,
        options: TokenCredentialOptions,
    ) -> anyhow::Result<PublicClientApplication>
    where
        T: Into<PublicClientApplication>,
    {
        let mut public_client_application = credential.into();
        public_client_application.token_credential_options = options;
        Ok(public_client_application)
    }
}

impl AuthorizationSerializer for PublicClientApplication {
    fn uri(&mut self, azure_authority_host: &AzureAuthorityHost) -> AuthorizationResult<Url> {
        self.credential.uri(azure_authority_host)
    }

    fn form(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        self.credential.form()
    }
}

#[async_trait]
impl TokenRequest for PublicClientApplication {
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

impl From<ResourceOwnerPasswordCredential> for PublicClientApplication {
    fn from(value: ResourceOwnerPasswordCredential) -> Self {
        PublicClientApplication {
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
