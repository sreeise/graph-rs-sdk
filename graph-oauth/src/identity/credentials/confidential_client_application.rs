use crate::identity::credentials::app_config::AppConfig;
use crate::identity::credentials::application_builder::ConfidentialClientApplicationBuilder;
use crate::identity::credentials::client_assertion_credential::ClientAssertionCredential;
use crate::identity::{
    Authority, AuthorizationCodeCertificateCredential, AuthorizationCodeCredential,
    AzureCloudInstance, ClientCertificateCredential, ClientSecretCredential, OpenIdCredential,
    TokenCredentialExecutor,
};

use async_trait::async_trait;
use graph_error::{AuthExecutionResult, AuthorizationResult};

use reqwest::header::{HeaderValue, CONTENT_TYPE};
use reqwest::tls::Version;
use reqwest::{ClientBuilder, Response};
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;
use wry::http::HeaderMap;

/// Clients capable of maintaining the confidentiality of their credentials
/// (e.g., client implemented on a secure server with restricted access to the client credentials),
/// or capable of secure client authentication using other means.
pub struct ConfidentialClientApplication {
    http_client: reqwest::Client,
    credential: Box<dyn TokenCredentialExecutor + Send>,
}

impl ConfidentialClientApplication {
    pub(crate) fn new<T>(credential: T) -> ConfidentialClientApplication
    where
        T: Into<ConfidentialClientApplication>,
    {
        credential.into()
    }

    pub(crate) fn credential<T>(credential: T) -> ConfidentialClientApplication
    where
        T: TokenCredentialExecutor + Send + 'static,
    {
        ConfidentialClientApplication {
            http_client: ClientBuilder::new()
                .min_tls_version(Version::TLS_1_2)
                .https_only(true)
                .build()
                .unwrap(),
            credential: Box::new(credential),
        }
    }

    pub fn builder(client_id: &str) -> ConfidentialClientApplicationBuilder {
        ConfidentialClientApplicationBuilder::new(client_id)
    }
}

#[async_trait]
impl TokenCredentialExecutor for ConfidentialClientApplication {
    fn uri(&mut self, azure_cloud_instance: &AzureCloudInstance) -> AuthorizationResult<Url> {
        self.credential.uri(azure_cloud_instance)
    }

    fn form_urlencode(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        self.credential.form_urlencode()
    }

    fn client_id(&self) -> &Uuid {
        self.credential.client_id()
    }

    fn authority(&self) -> Authority {
        self.credential.authority()
    }

    fn azure_cloud_instance(&self) -> AzureCloudInstance {
        self.credential.azure_cloud_instance()
    }

    fn basic_auth(&self) -> Option<(String, String)> {
        self.credential.basic_auth()
    }

    fn app_config(&self) -> &AppConfig {
        self.credential.app_config()
    }

    fn execute(&mut self) -> AuthExecutionResult<reqwest::blocking::Response> {
        let azure_cloud_instance = self.azure_cloud_instance();
        let uri = self.credential.uri(&azure_cloud_instance)?;
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
                // Reqwest adds these automatically but this is here in case that changes.
                .headers(headers)
                .form(&form)
                .send()?)
        } else {
            Ok(http_client.post(uri).headers(headers).form(&form).send()?)
        }
    }

    async fn execute_async(&mut self) -> AuthExecutionResult<Response> {
        let azure_cloud_instance = self.azure_cloud_instance();
        let uri = self.credential.uri(&azure_cloud_instance)?;
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

impl From<AuthorizationCodeCredential> for ConfidentialClientApplication {
    fn from(value: AuthorizationCodeCredential) -> Self {
        ConfidentialClientApplication::credential(value)
    }
}

impl From<AuthorizationCodeCertificateCredential> for ConfidentialClientApplication {
    fn from(value: AuthorizationCodeCertificateCredential) -> Self {
        ConfidentialClientApplication::credential(value)
    }
}

impl From<ClientSecretCredential> for ConfidentialClientApplication {
    fn from(value: ClientSecretCredential) -> Self {
        ConfidentialClientApplication::credential(value)
    }
}

impl From<ClientCertificateCredential> for ConfidentialClientApplication {
    fn from(value: ClientCertificateCredential) -> Self {
        ConfidentialClientApplication::credential(value)
    }
}

impl From<ClientAssertionCredential> for ConfidentialClientApplication {
    fn from(value: ClientAssertionCredential) -> Self {
        ConfidentialClientApplication::credential(value)
    }
}

impl From<OpenIdCredential> for ConfidentialClientApplication {
    fn from(value: OpenIdCredential) -> Self {
        ConfidentialClientApplication::credential(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::identity::{Authority, AzureCloudInstance};

    #[test]
    fn confidential_client_new() {
        let credential = AuthorizationCodeCredential::builder(
            Uuid::new_v4().to_string(),
            "ALDSKFJLKERLKJALSDKJF2209LAKJGFL",
        )
        .with_client_secret("CLDIE3F")
        .with_scope(vec!["Read.Write", "Fall.Down"])
        .with_redirect_uri("http://localhost:8888/redirect")
        .unwrap()
        .build();

        let mut confidential_client = credential;
        let credential_uri = confidential_client
            .credential
            .uri(&AzureCloudInstance::AzurePublic)
            .unwrap();

        assert_eq!(
            "https://login.microsoftonline.com/common/oauth2/v2.0/token",
            credential_uri.as_str()
        );
    }

    #[test]
    fn confidential_client_tenant() {
        let mut confidential_client = AuthorizationCodeCredential::builder(
            Uuid::new_v4().to_string(),
            "ALDSKFJLKERLKJALSDKJF2209LAKJGFL",
        )
        .with_client_id("bb301aaa-1201-4259-a230923fds32")
        .with_client_secret("CLDIE3F")
        .with_redirect_uri("http://localhost:8888/redirect")
        .unwrap()
        .with_authority(Authority::Consumers)
        .build();
        let credential_uri = confidential_client
            .credential
            .uri(&AzureCloudInstance::AzurePublic)
            .unwrap();

        assert_eq!(
            "https://login.microsoftonline.com/consumers/oauth2/v2.0/token",
            credential_uri.as_str()
        );
    }
}
