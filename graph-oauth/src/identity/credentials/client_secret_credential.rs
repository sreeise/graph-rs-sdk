use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use async_trait::async_trait;
use http::{HeaderMap, HeaderName, HeaderValue};

use uuid::Uuid;

use graph_core::cache::{CacheStore, InMemoryCacheStore, TokenCache};
use graph_core::http::{AsyncResponseConverterExt, ResponseConverterExt};
use graph_core::identity::ForceTokenRefresh;
use graph_error::{AuthExecutionError, AuthExecutionResult, AuthorizationFailure, IdentityResult};

use crate::identity::{
    credentials::app_config::AppConfig, tracing_targets::CREDENTIAL_EXECUTOR, Authority,
    AzureCloudInstance, ClientCredentialsAuthorizationUrlParameterBuilder,
    ConfidentialClientApplication, Token, TokenCredentialExecutor,
};
use crate::oauth_serializer::{OAuthParameter, OAuthSerializer};

credential_builder!(
    ClientSecretCredentialBuilder,
    ConfidentialClientApplication<ClientSecretCredential>
);

/// Client Credentials flow using a client secret.
///
/// The OAuth 2.0 client credentials grant flow permits a web service (confidential client)
/// to use its own credentials, instead of impersonating a user, to authenticate when calling
/// another web service. The grant specified in RFC 6749, sometimes called two-legged OAuth,
/// can be used to access web-hosted resources by using the identity of an application.
/// This type is commonly used for server-to-server interactions that must run in the background,
/// without immediate interaction with a user, and is often referred to as daemons or service accounts.
///
/// See [Microsoft identity platform and the OAuth 2.0 client credentials flow](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow)
#[derive(Clone)]
pub struct ClientSecretCredential {
    pub(crate) app_config: AppConfig,
    /// Required
    /// The application secret that you created in the app registration portal for your app.
    /// Don't use the application secret in a native app or single page app because a
    /// client_secret can't be reliably stored on devices or web pages. It's required for web
    /// apps and web APIs, which can store the client_secret securely on the server side. Like
    /// all parameters here, the client secret must be URL-encoded before being sent. This step
    /// is done by the SDK. For more information on URI encoding, see the URI Generic Syntax
    /// specification. The Basic auth pattern of instead providing credentials in the Authorization
    /// header, per RFC 6749 is also supported.
    pub(crate) client_secret: String,
    token_cache: InMemoryCacheStore<Token>,
}

impl Debug for ClientSecretCredential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientSecretCredential")
            .field("app_config", &self.app_config)
            .finish()
    }
}

impl ClientSecretCredential {
    pub fn new(
        client_id: impl AsRef<str>,
        client_secret: impl AsRef<str>,
    ) -> ClientSecretCredential {
        ClientSecretCredential {
            app_config: AppConfig::builder(client_id.as_ref())
                .scope(vec!["https://graph.microsoft.com/.default"])
                .build(),
            client_secret: client_secret.as_ref().to_owned(),
            token_cache: InMemoryCacheStore::new(),
        }
    }

    pub fn new_with_tenant(
        tenant_id: impl AsRef<str>,
        client_id: impl AsRef<str>,
        client_secret: impl AsRef<str>,
    ) -> ClientSecretCredential {
        ClientSecretCredential {
            app_config: AppConfig::builder(client_id.as_ref())
                .tenant(tenant_id.as_ref())
                .scope(vec!["https://graph.microsoft.com/.default"])
                .build(),
            client_secret: client_secret.as_ref().to_owned(),
            token_cache: InMemoryCacheStore::new(),
        }
    }

    pub fn authorization_url_builder<T: AsRef<str>>(
        client_id: T,
    ) -> ClientCredentialsAuthorizationUrlParameterBuilder {
        ClientCredentialsAuthorizationUrlParameterBuilder::new(client_id)
    }

    fn execute_cached_token_refresh(&mut self, cache_id: String) -> AuthExecutionResult<Token> {
        let response = self.execute()?;

        if !response.status().is_success() {
            return Err(AuthExecutionError::silent_token_auth(
                response.into_http_response()?,
            ));
        }

        let new_token: Token = response.json()?;
        self.token_cache.store(cache_id, new_token.clone());
        Ok(new_token)
    }

    async fn execute_cached_token_refresh_async(
        &mut self,
        cache_id: String,
    ) -> AuthExecutionResult<Token> {
        let response = self.execute_async().await?;

        if !response.status().is_success() {
            return Err(AuthExecutionError::silent_token_auth(
                response.into_http_response_async().await?,
            ));
        }

        let new_token: Token = response.json().await?;
        self.token_cache.store(cache_id, new_token.clone());
        Ok(new_token)
    }
}

#[async_trait]
impl TokenCache for ClientSecretCredential {
    type Token = Token;

    fn get_token_silent(&mut self) -> Result<Self::Token, AuthExecutionError> {
        let cache_id = self.app_config.cache_id.to_string();
        if let Some(token) = self.token_cache.get(cache_id.as_str()) {
            if token.is_expired_sub(time::Duration::minutes(5)) {
                tracing::debug!(target: CREDENTIAL_EXECUTOR, "executing silent token request; refresh_token=None");
                self.execute_cached_token_refresh(cache_id)
            } else {
                tracing::debug!(target: CREDENTIAL_EXECUTOR, "using token from cache");
                Ok(token)
            }
        } else {
            tracing::debug!(target: CREDENTIAL_EXECUTOR, "executing silent token request; refresh_token=None");
            self.execute_cached_token_refresh(cache_id)
        }
    }

    async fn get_token_silent_async(&mut self) -> Result<Self::Token, AuthExecutionError> {
        let cache_id = self.app_config.cache_id.to_string();
        if let Some(token) = self.token_cache.get(cache_id.as_str()) {
            if token.is_expired_sub(time::Duration::minutes(5)) {
                tracing::debug!(target: CREDENTIAL_EXECUTOR, "executing silent token request; refresh_token=None");
                self.execute_cached_token_refresh_async(cache_id).await
            } else {
                tracing::debug!(target: CREDENTIAL_EXECUTOR, "using token from cache");
                Ok(token.clone())
            }
        } else {
            tracing::debug!(target: CREDENTIAL_EXECUTOR, "executing silent token request; refresh_token=None");
            self.execute_cached_token_refresh_async(cache_id).await
        }
    }

    fn with_force_token_refresh(&mut self, force_token_refresh: ForceTokenRefresh) {
        self.app_config.force_token_refresh = force_token_refresh;
    }
}

#[async_trait]
impl TokenCredentialExecutor for ClientSecretCredential {
    fn form_urlencode(&mut self) -> IdentityResult<HashMap<String, String>> {
        let mut serializer = OAuthSerializer::new();
        let client_id = self.app_config.client_id.to_string();
        if client_id.is_empty() || self.app_config.client_id.is_nil() {
            return AuthorizationFailure::result(OAuthParameter::ClientId);
        }

        if self.client_secret.trim().is_empty() {
            return AuthorizationFailure::result(OAuthParameter::ClientSecret);
        }

        serializer
            .client_id(client_id.as_str())
            .client_secret(self.client_secret.as_str())
            .grant_type("client_credentials")
            .set_scope(self.app_config.scope.clone());

        // Don't include ClientId and Client Secret in the fields for form url encode because
        // Client Id and Client Secret are already included as basic auth.
        serializer.as_credential_map(vec![OAuthParameter::Scope], vec![OAuthParameter::GrantType])
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

    fn basic_auth(&self) -> Option<(String, String)> {
        Some((
            self.app_config.client_id.to_string(),
            self.client_secret.clone(),
        ))
    }

    fn app_config(&self) -> &AppConfig {
        &self.app_config
    }
}

#[derive(Clone, Debug)]
pub struct ClientSecretCredentialBuilder {
    credential: ClientSecretCredential,
}

impl ClientSecretCredentialBuilder {
    pub fn new(client_id: impl AsRef<str>, client_secret: impl AsRef<str>) -> Self {
        ClientSecretCredentialBuilder {
            credential: ClientSecretCredential::new(client_id, client_secret),
        }
    }

    pub(crate) fn new_with_client_secret(
        client_secret: impl AsRef<str>,
        mut app_config: AppConfig,
    ) -> ClientSecretCredentialBuilder {
        app_config
            .scope
            .insert("https://graph.microsoft.com/.default".into());
        Self {
            credential: ClientSecretCredential {
                app_config,
                client_secret: client_secret.as_ref().to_string(),
                token_cache: InMemoryCacheStore::new(),
            },
        }
    }

    pub fn with_client_secret<T: AsRef<str>>(&mut self, client_secret: T) -> &mut Self {
        self.credential.client_secret = client_secret.as_ref().to_owned();
        self
    }

    pub fn credential(&self) -> ClientSecretCredential {
        self.credential.clone()
    }
}
