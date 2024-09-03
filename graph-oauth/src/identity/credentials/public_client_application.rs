use crate::identity::credentials::app_config::AppConfig;
use crate::identity::credentials::application_builder::PublicClientApplicationBuilder;
use crate::identity::{
    Authority, AuthorizationCodeCredential, AzureCloudInstance, DeviceCodeCredential,
    ResourceOwnerPasswordCredential, TokenCredentialExecutor,
};
use async_trait::async_trait;
use graph_core::cache::{AsBearer, TokenCache};
use graph_core::identity::{ClientApplication, ForceTokenRefresh};
use graph_error::{AuthExecutionResult, IdentityResult};
use reqwest::Response;
use std::collections::HashMap;
use std::fmt::Debug;
use url::Url;
use uuid::Uuid;

/// Clients incapable of maintaining the confidentiality of their credentials
/// (e.g., clients executing on the device used by the resource owner, such as an
/// installed native application or a web browser-based application), and incapable of
/// secure client authentication via any other means.
///
/// See [Client Types](https://datatracker.ietf.org/doc/html/rfc6749#section-2.1) in the specification.
#[derive(Clone, Debug)]
pub struct PublicClientApplication<Credential> {
    credential: Credential,
}

impl PublicClientApplication<()> {
    pub fn builder(client_id: impl AsRef<str>) -> PublicClientApplicationBuilder {
        PublicClientApplicationBuilder::new(client_id)
    }
}

impl<Credential: Clone + Debug + Send + Sync + TokenCredentialExecutor>
    PublicClientApplication<Credential>
{
    pub(crate) fn new(credential: Credential) -> PublicClientApplication<Credential> {
        PublicClientApplication { credential }
    }

    pub(crate) fn credential(credential: Credential) -> PublicClientApplication<Credential> {
        PublicClientApplication { credential }
    }
}

#[async_trait]
impl<Credential: Clone + Debug + Send + Sync + TokenCache> ClientApplication
    for PublicClientApplication<Credential>
{
    fn get_token_silent(&mut self) -> AuthExecutionResult<String> {
        let token = self.credential.get_token_silent()?;
        Ok(token.as_bearer())
    }

    async fn get_token_silent_async(&mut self) -> AuthExecutionResult<String> {
        let token = self.credential.get_token_silent_async().await?;
        Ok(token.as_bearer())
    }

    fn with_force_token_refresh(&mut self, force_token_refresh: ForceTokenRefresh) {
        self.credential
            .with_force_token_refresh(force_token_refresh);
    }
}

#[async_trait]
impl<Credential: Clone + Debug + Send + Sync + TokenCredentialExecutor> TokenCredentialExecutor
    for PublicClientApplication<Credential>
{
    fn uri(&mut self) -> IdentityResult<Url> {
        self.credential.uri()
    }

    fn form_urlencode(&mut self) -> IdentityResult<HashMap<String, String>> {
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

    fn app_config(&self) -> &AppConfig {
        self.credential.app_config()
    }

    fn execute(&mut self) -> AuthExecutionResult<reqwest::blocking::Response> {
        self.credential.execute()
    }

    async fn execute_async(&mut self) -> AuthExecutionResult<Response> {
        self.credential.execute_async().await
    }
}

impl From<ResourceOwnerPasswordCredential>
    for PublicClientApplication<ResourceOwnerPasswordCredential>
{
    fn from(value: ResourceOwnerPasswordCredential) -> Self {
        PublicClientApplication::credential(value)
    }
}

impl From<DeviceCodeCredential> for PublicClientApplication<DeviceCodeCredential> {
    fn from(value: DeviceCodeCredential) -> Self {
        PublicClientApplication::credential(value)
    }
}

impl From<AuthorizationCodeCredential> for PublicClientApplication<AuthorizationCodeCredential> {
    fn from(value: AuthorizationCodeCredential) -> Self {
        PublicClientApplication::credential(value)
    }
}
