use crate::identity::credentials::app_config::AppConfig;
use crate::identity::credentials::application_builder::PublicClientApplicationBuilder;
use crate::identity::{
    Authority, AzureCloudInstance, DeviceCodeCredential, ResourceOwnerPasswordCredential,
    TokenCredentialExecutor,
};
use async_trait::async_trait;
use graph_error::{AuthExecutionResult, AuthorizationResult};
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
    credential: Box<dyn TokenCredentialExecutor + Send>,
}

impl PublicClientApplication {
    pub fn new<T>(credential: T) -> PublicClientApplication
    where
        T: Into<PublicClientApplication>,
    {
        credential.into()
    }

    pub(crate) fn credential<T>(credential: T) -> PublicClientApplication
    where
        T: TokenCredentialExecutor + Send + 'static,
    {
        PublicClientApplication {
            http_client: ClientBuilder::new()
                .min_tls_version(Version::TLS_1_2)
                .https_only(true)
                .build()
                .unwrap(),
            credential: Box::new(credential),
        }
    }

    pub fn builder(client_id: impl AsRef<str>) -> PublicClientApplicationBuilder {
        PublicClientApplicationBuilder::new(client_id.as_ref())
    }
}

#[async_trait]
impl TokenCredentialExecutor for PublicClientApplication {
    fn uri(&mut self, azure_cloud_instance: &AzureCloudInstance) -> AuthorizationResult<Url> {
        self.credential.uri(azure_cloud_instance)
    }

    fn form_urlencode(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        self.credential.form_urlencode()
    }

    fn client_id(&self) -> &String {
        self.credential.client_id()
    }

    fn authority(&self) -> Authority {
        self.credential.authority()
    }

    fn azure_cloud_instance(&self) -> AzureCloudInstance {
        self.credential.azure_cloud_instance()
    }

    fn app_config(&self) -> &AppConfig {
        self.credential.app_config()
    }

    fn execute(&mut self) -> AuthExecutionResult<reqwest::blocking::Response> {
        let azure_authority_host = self.azure_cloud_instance();
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

    async fn execute_async(&mut self) -> AuthExecutionResult<Response> {
        let azure_cloud_instance = self.credential.azure_cloud_instance();
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

impl From<ResourceOwnerPasswordCredential> for PublicClientApplication {
    fn from(value: ResourceOwnerPasswordCredential) -> Self {
        PublicClientApplication::credential(value)
    }
}

impl From<DeviceCodeCredential> for PublicClientApplication {
    fn from(value: DeviceCodeCredential) -> Self {
        PublicClientApplication::credential(value)
    }
}
