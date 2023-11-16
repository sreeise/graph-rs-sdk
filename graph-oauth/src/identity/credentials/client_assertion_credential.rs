use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use async_trait::async_trait;
use http::{HeaderMap, HeaderName, HeaderValue};

use uuid::Uuid;

use crate::oauth_serializer::{OAuthParameter, OAuthSerializer};
use graph_core::cache::{CacheStore, InMemoryCacheStore, TokenCache};
use graph_error::{AuthExecutionError, IdentityResult, AF};

use crate::identity::credentials::app_config::AppConfig;
use crate::identity::{
    Authority, AzureCloudInstance, ConfidentialClientApplication, ForceTokenRefresh, Token,
    TokenCredentialExecutor, CLIENT_ASSERTION_TYPE,
};

credential_builder!(
    ClientAssertionCredentialBuilder,
    ConfidentialClientApplication<ClientAssertionCredential>
);

/// Client Credentials Using an Assertion.
///
/// The OAuth 2.0 client credentials grant flow permits a web service (confidential client) to use
/// its own credentials, instead of impersonating a user, to authenticate when calling another
/// web service.
///
/// Everything in the request is the same as the certificate-based flow, with the crucial exception
/// of the source of the client_assertion. In this flow, your application does not create the JWT
/// assertion itself. Instead, your app uses a JWT created by another identity provider.
/// This is called workload identity federation, where your apps identity in another identity
/// platform is used to acquire tokens inside the Microsoft identity platform. This is best
/// suited for cross-cloud scenarios, such as hosting your compute outside Azure but accessing
/// APIs protected by Microsoft identity platform. For information about the required format
/// of JWTs created by other identity providers, read about the assertion format.
/// https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-client-creds-grant-flow#third-case-access-token-request-with-a-federated-credential
#[derive(Clone)]
pub struct ClientAssertionCredential {
    pub(crate) app_config: AppConfig,
    /// The value must be set to urn:ietf:params:oauth:client-assertion-type:jwt-bearer.
    /// This is automatically set by the SDK.
    pub(crate) client_assertion_type: String,
    /// An assertion (a JWT, or JSON web token) that your application gets from another identity
    /// provider outside of Microsoft identity platform, like Kubernetes. The specifics of this
    /// JWT must be registered on your application as a federated identity credential. Read about
    /// workload identity federation to learn how to setup and use assertions generated from
    /// other identity providers.
    pub(crate) client_assertion: String,
    token_cache: InMemoryCacheStore<Token>,
}

impl ClientAssertionCredential {
    pub fn new(
        assertion: impl AsRef<str>,
        tenant_id: impl AsRef<str>,
        client_id: impl AsRef<str>,
    ) -> ClientAssertionCredential {
        ClientAssertionCredential {
            app_config: AppConfig::builder(client_id.as_ref())
                .tenant(tenant_id.as_ref())
                .scope(vec!["https://graph.microsoft.com/.default"])
                .build(),
            client_assertion_type: CLIENT_ASSERTION_TYPE.to_owned(),
            client_assertion: assertion.as_ref().to_string(),
            token_cache: Default::default(),
        }
    }
}

impl Debug for ClientAssertionCredential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientAssertionCredential")
            .field("app_config", &self.app_config)
            .finish()
    }
}

#[async_trait]
impl TokenCache for ClientAssertionCredential {
    type Token = Token;

    fn get_token_silent(&mut self) -> Result<Self::Token, AuthExecutionError> {
        let cache_id = self.app_config.cache_id.to_string();
        if let Some(token) = self.token_cache.get(cache_id.as_str()) {
            if token.is_expired_sub(time::Duration::minutes(5)) {
                let response = self.execute()?;
                let msal_token: Token = response.json()?;
                self.token_cache.store(cache_id, msal_token.clone());
                Ok(msal_token)
            } else {
                Ok(token)
            }
        } else {
            let response = self.execute()?;
            let msal_token: Token = response.json()?;
            self.token_cache.store(cache_id, msal_token.clone());
            Ok(msal_token)
        }
    }

    async fn get_token_silent_async(&mut self) -> Result<Self::Token, AuthExecutionError> {
        let cache_id = self.app_config.cache_id.to_string();
        if let Some(token) = self.token_cache.get(cache_id.as_str()) {
            if token.is_expired_sub(time::Duration::minutes(5)) {
                let response = self.execute_async().await?;
                let msal_token: Token = response.json().await?;
                self.token_cache.store(cache_id, msal_token.clone());
                Ok(msal_token)
            } else {
                Ok(token.clone())
            }
        } else {
            let response = self.execute_async().await?;
            let msal_token: Token = response.json().await?;
            self.token_cache.store(cache_id, msal_token.clone());
            Ok(msal_token)
        }
    }
}

#[derive(Clone)]
pub struct ClientAssertionCredentialBuilder {
    credential: ClientAssertionCredential,
}

impl ClientAssertionCredentialBuilder {
    pub fn new(
        client_id: impl AsRef<str>,
        signed_assertion: impl AsRef<str>,
    ) -> ClientAssertionCredentialBuilder {
        ClientAssertionCredentialBuilder {
            credential: ClientAssertionCredential {
                app_config: AppConfig::builder(client_id.as_ref())
                    .scope(vec!["https://graph.microsoft.com/.default"])
                    .build(),
                client_assertion_type: CLIENT_ASSERTION_TYPE.to_string(),
                client_assertion: signed_assertion.as_ref().to_owned(),
                token_cache: Default::default(),
            },
        }
    }

    pub(crate) fn new_with_signed_assertion(
        signed_assertion: String,
        mut app_config: AppConfig,
    ) -> ClientAssertionCredentialBuilder {
        app_config
            .scope
            .insert("https://graph.microsoft.com/.default".to_string());
        ClientAssertionCredentialBuilder {
            credential: ClientAssertionCredential {
                app_config,
                client_assertion_type: CLIENT_ASSERTION_TYPE.to_string(),
                client_assertion: signed_assertion,
                token_cache: Default::default(),
            },
        }
    }

    pub fn with_client_assertion<T: AsRef<str>>(&mut self, client_assertion: T) -> &mut Self {
        self.credential.client_assertion = client_assertion.as_ref().to_owned();
        self
    }
}

#[async_trait]
impl TokenCredentialExecutor for ClientAssertionCredential {
    fn form_urlencode(&mut self) -> IdentityResult<HashMap<String, String>> {
        let mut serializer = OAuthSerializer::new();
        let client_id = self.client_id().to_string();
        if client_id.trim().is_empty() {
            return AF::result(OAuthParameter::ClientId.alias());
        }

        if self.client_assertion.trim().is_empty() {
            return AF::result(OAuthParameter::ClientAssertion.alias());
        }

        if self.client_assertion_type.trim().is_empty() {
            self.client_assertion_type = CLIENT_ASSERTION_TYPE.to_owned();
        }

        serializer
            .client_id(client_id.as_str())
            .client_assertion(self.client_assertion.as_str())
            .client_assertion_type(self.client_assertion_type.as_str())
            .set_scope(self.app_config.scope.clone())
            .grant_type("client_credentials");

        serializer.as_credential_map(
            vec![OAuthParameter::Scope],
            vec![
                OAuthParameter::ClientId,
                OAuthParameter::GrantType,
                OAuthParameter::ClientAssertion,
                OAuthParameter::ClientAssertionType,
            ],
        )
    }

    fn client_id(&self) -> &Uuid {
        &self.app_config.client_id
    }

    fn authority(&self) -> Authority {
        self.app_config.authority.clone()
    }

    fn azure_cloud_instance(&self) -> AzureCloudInstance {
        self.app_config.azure_cloud_instance
    }

    fn app_config(&self) -> &AppConfig {
        &self.app_config
    }
}
