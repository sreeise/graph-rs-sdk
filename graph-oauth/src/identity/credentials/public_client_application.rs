use crate::identity::{
    AzureCloudInstance, DeviceCodeCredential, ResourceOwnerPasswordCredential, TokenCredential,
    TokenCredentialOptions,
};
use async_trait::async_trait;
use graph_error::AuthorizationResult;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::tls::Version;
use reqwest::{ClientBuilder, Response};
use std::collections::HashMap;
use url::Url;

/// Clients incapable of maintaining the confidentiality of their credentials
/// (e.g., clients executing on the device used by the resource owner, such as an
/// installed native application or a web browser-based application), and incapable of
/// secure client authentication via any other means.
/// https://datatracker.ietf.org/doc/html/rfc6749#section-2.1
pub struct PublicClientApplication {
    http_client: reqwest::Client,
    token_credential_options: TokenCredentialOptions,
    credential: Box<dyn TokenCredential + Send>,
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

#[async_trait]
impl TokenCredential for PublicClientApplication {
    fn uri(&mut self, azure_authority_host: &AzureCloudInstance) -> AuthorizationResult<Url> {
        self.credential.uri(azure_authority_host)
    }

    fn form_urlencode(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        self.credential.form_urlencode()
    }

    fn client_id(&self) -> &String {
        self.credential.client_id()
    }

    fn token_credential_options(&self) -> &TokenCredentialOptions {
        &self.token_credential_options
    }

    fn get_token(&mut self) -> anyhow::Result<reqwest::blocking::Response> {
        let azure_authority_host = self.token_credential_options.azure_authority_host;
        let uri = self.credential.uri(&azure_authority_host)?;
        let form = self.credential.form_urlencode()?;
        let http_client = reqwest::blocking::ClientBuilder::new()
            .min_tls_version(Version::TLS_1_2)
            .https_only(true)
            .build()?;
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        let basic_auth = self.credential.basic_auth();
        if let Some((client_identifier, secret)) = basic_auth {
            Ok(http_client
                .post(uri)
                .basic_auth(client_identifier, Some(secret))
                .headers(headers)
                .form(&form)
                .send()?)
        } else {
            Ok(http_client.post(uri).headers(headers).form(&form).send()?)
        }
    }

    async fn get_token_async(&mut self) -> anyhow::Result<Response> {
        let azure_authority_host = self.token_credential_options.azure_authority_host;
        let uri = self.credential.uri(&azure_authority_host)?;
        let form = self.credential.form_urlencode()?;
        let basic_auth = self.credential.basic_auth();
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        if let Some((client_identifier, secret)) = basic_auth {
            Ok(self
                .http_client
                .post(uri)
                // https://datatracker.ietf.org/doc/html/rfc6749#section-2.3.1
                .basic_auth(client_identifier, Some(secret))
                .headers(headers)
                .form(&form)
                .send()
                .await?)
        } else {
            Ok(self
                .http_client
                .post(uri)
                .headers(headers)
                .form(&form)
                .send()
                .await?)
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

impl From<DeviceCodeCredential> for PublicClientApplication {
    fn from(value: DeviceCodeCredential) -> Self {
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
