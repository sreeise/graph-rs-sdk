use std::collections::HashMap;
use std::fmt::Debug;

use async_trait::async_trait;

use reqwest::Response;
use url::Url;
use uuid::Uuid;

use graph_core::cache::{AsBearer, TokenCache};
use graph_core::identity::ClientApplication;
use graph_error::{AuthExecutionResult, IdentityResult, AF};

use crate::identity::{
    credentials::app_config::AppConfig,
    credentials::application_builder::ConfidentialClientApplicationBuilder,
    credentials::client_assertion_credential::ClientAssertionCredential, Authority,
    AuthorizationCodeAssertionCredential, AuthorizationCodeCertificateCredential,
    AuthorizationCodeCredential, AuthorizationQueryResponse, AzureCloudInstance,
    ClientCertificateCredential, ClientSecretCredential, OpenIdCredential, TokenCredentialExecutor,
};
use crate::oauth::AuthCodeAuthorizationUrlParameters;
use crate::web::{
    InteractiveAuthEvent, InteractiveAuthenticator, WebViewOptions, WindowCloseReason,
};

/// Clients capable of maintaining the confidentiality of their credentials
/// (e.g., client implemented on a secure server with restricted access to the client credentials),
/// or capable of secure client authentication using other means.
///
///
/// # Build a confidential client for the authorization code grant.
/// Use [with_authorization_code](crate::identity::ConfidentialClientApplicationBuilder::with_auth_code) to set the authorization code received from
/// the authorization step, see [Request an authorization code](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow#request-an-authorization-code)
/// You can use the [AuthCodeAuthorizationUrlParameterBuilder](crate::identity::AuthCodeAuthorizationUrlParameterBuilder)
/// to build the url that the user will be directed to authorize at.
///
/// ```rust
#[derive(Clone, Debug)]
pub struct ConfidentialClientApplication<Credential> {
    credential: Credential,
}

impl ConfidentialClientApplication<()> {
    pub fn builder(client_id: impl AsRef<str>) -> ConfidentialClientApplicationBuilder {
        ConfidentialClientApplicationBuilder::new(client_id)
    }
}

impl ConfidentialClientApplication<AuthCodeAuthorizationUrlParameters> {
    pub fn parameter_builder(
        credential: AuthCodeAuthorizationUrlParameters,
    ) -> ConfidentialClientApplication<AuthCodeAuthorizationUrlParameters> {
        ConfidentialClientApplication { credential }
    }

    pub async fn interactive_auth(
        &self,
        options: Option<WebViewOptions>,
    ) -> anyhow::Result<AuthorizationQueryResponse> {
        let result = self
            .credential
            .interactive_webview_authentication(options)?;
        Ok(result)
    }
}

impl<Credential: Clone + Debug + Send + Sync + TokenCredentialExecutor>
    ConfidentialClientApplication<Credential>
{
    pub(crate) fn new(credential: Credential) -> ConfidentialClientApplication<Credential> {
        ConfidentialClientApplication { credential }
    }

    pub(crate) fn credential(credential: Credential) -> ConfidentialClientApplication<Credential> {
        ConfidentialClientApplication { credential }
    }

    pub fn into_inner(self) -> Credential {
        self.credential
    }
}

#[async_trait]
impl<Credential: Clone + Debug + Send + Sync + TokenCache> ClientApplication
    for ConfidentialClientApplication<Credential>
{
    fn get_token_silent(&mut self) -> AuthExecutionResult<String> {
        let token = self.credential.get_token_silent()?;
        Ok(token.as_bearer())
    }

    async fn get_token_silent_async(&mut self) -> AuthExecutionResult<String> {
        let token = self.credential.get_token_silent_async().await?;
        Ok(token.as_bearer())
    }
}

#[async_trait]
impl<Credential: Clone + Debug + Send + Sync + TokenCredentialExecutor> TokenCredentialExecutor
    for ConfidentialClientApplication<Credential>
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

    fn basic_auth(&self) -> Option<(String, String)> {
        self.credential.basic_auth()
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

impl From<AuthorizationCodeCredential>
    for ConfidentialClientApplication<AuthorizationCodeCredential>
{
    fn from(value: AuthorizationCodeCredential) -> Self {
        ConfidentialClientApplication::credential(value)
    }
}

impl From<AuthorizationCodeAssertionCredential>
    for ConfidentialClientApplication<AuthorizationCodeAssertionCredential>
{
    fn from(value: AuthorizationCodeAssertionCredential) -> Self {
        ConfidentialClientApplication::credential(value)
    }
}

impl From<AuthorizationCodeCertificateCredential>
    for ConfidentialClientApplication<AuthorizationCodeCertificateCredential>
{
    fn from(value: AuthorizationCodeCertificateCredential) -> Self {
        ConfidentialClientApplication::credential(value)
    }
}

impl From<ClientSecretCredential> for ConfidentialClientApplication<ClientSecretCredential> {
    fn from(value: ClientSecretCredential) -> Self {
        ConfidentialClientApplication::credential(value)
    }
}

impl From<ClientCertificateCredential>
    for ConfidentialClientApplication<ClientCertificateCredential>
{
    fn from(value: ClientCertificateCredential) -> Self {
        ConfidentialClientApplication::credential(value)
    }
}

impl From<ClientAssertionCredential> for ConfidentialClientApplication<ClientAssertionCredential> {
    fn from(value: ClientAssertionCredential) -> Self {
        ConfidentialClientApplication::credential(value)
    }
}

impl From<OpenIdCredential> for ConfidentialClientApplication<OpenIdCredential> {
    fn from(value: OpenIdCredential) -> Self {
        ConfidentialClientApplication::credential(value)
    }
}

impl From<AuthCodeAuthorizationUrlParameters>
    for ConfidentialClientApplication<AuthCodeAuthorizationUrlParameters>
{
    fn from(value: AuthCodeAuthorizationUrlParameters) -> Self {
        ConfidentialClientApplication::parameter_builder(value)
    }
}

impl ConfidentialClientApplication<AuthCodeAuthorizationUrlParameters> {
    pub fn interactive_webview_authentication(
        &self,
        interactive_web_view_options: Option<WebViewOptions>,
    ) -> anyhow::Result<AuthorizationQueryResponse> {
        let receiver = self
            .credential
            .interactive_authentication(interactive_web_view_options)?;
        let mut iter = receiver.try_iter();
        let mut next = iter.next();
        while next.is_none() {
            next = iter.next();
        }

        return match next {
            None => Err(anyhow::anyhow!("Unknown")),
            Some(auth_event) => match auth_event {
                InteractiveAuthEvent::InvalidRedirectUri(reason) => {
                    Err(anyhow::anyhow!("Invalid Redirect Uri - {reason}"))
                }
                InteractiveAuthEvent::ReachedRedirectUri(uri) => {
                    let url_str = uri.as_str();
                    let query = uri.query().or(uri.fragment()).ok_or(AF::msg_err(
                        "query | fragment",
                        &format!("No query or fragment returned on redirect uri: {url_str}"),
                    ))?;

                    let response_query: AuthorizationQueryResponse =
                        serde_urlencoded::from_str(query)?;
                    Ok(response_query)
                }
                InteractiveAuthEvent::ClosingWindow(window_close_reason) => {
                    match window_close_reason {
                        WindowCloseReason::CloseRequested => Err(anyhow::anyhow!("CloseRequested")),
                        WindowCloseReason::InvalidWindowNavigation => {
                            Err(anyhow::anyhow!("InvalidWindowNavigation"))
                        }
                        WindowCloseReason::TimedOut {
                            start: _,
                            requested_resume: _,
                        } => Err(anyhow::anyhow!("TimedOut")),
                    }
                }
            },
        };
    }
}

#[cfg(test)]
mod test {
    use crate::identity::Authority;

    use super::*;

    #[test]
    fn confidential_client_new() {
        let client_id = Uuid::new_v4();
        let client_id_string = client_id.to_string();
        let mut confidential_client =
            ConfidentialClientApplication::builder(client_id_string.as_str())
                .with_auth_code("code")
                .with_client_secret("ALDSKFJLKERLKJALSDKJF2209LAKJGFL")
                .with_scope(vec!["Read.Write"])
                .with_redirect_uri("http://localhost:8888/redirect")
                .unwrap()
                .build();

        let credential_uri = confidential_client.credential.uri().unwrap();

        assert_eq!(
            "https://login.microsoftonline.com/common/oauth2/v2.0/token",
            credential_uri.as_str()
        );
    }

    #[test]
    fn confidential_client_authority_tenant() {
        let client_id = Uuid::new_v4();
        let client_id_string = client_id.to_string();
        let mut confidential_client =
            ConfidentialClientApplication::builder(client_id_string.as_str())
                .with_auth_code("code")
                .with_tenant("tenant")
                .with_client_secret("ALDSKFJLKERLKJALSDKJF2209LAKJGFL")
                .with_scope(vec!["Read.Write"])
                .with_redirect_uri("http://localhost:8888/redirect")
                .unwrap()
                .build();

        let credential_uri = confidential_client.credential.uri().unwrap();

        assert_eq!(
            "https://login.microsoftonline.com/tenant/oauth2/v2.0/token",
            credential_uri.as_str()
        );
    }

    #[test]
    fn confidential_client_authority_consumers() {
        let client_id = Uuid::new_v4();
        let client_id_string = client_id.to_string();
        let mut confidential_client =
            ConfidentialClientApplication::builder(client_id_string.as_str())
                .with_auth_code("code")
                .with_authority(Authority::Consumers)
                .with_client_secret("ALDSKFJLKERLKJALSDKJF2209LAKJGFL")
                .with_scope(vec!["Read.Write", "Fall.Down"])
                .with_redirect_uri("http://localhost:8888/redirect")
                .unwrap()
                .build();

        let credential_uri = confidential_client.credential.uri().unwrap();

        assert_eq!(
            "https://login.microsoftonline.com/consumers/oauth2/v2.0/token",
            credential_uri.as_str()
        );
    }
}
