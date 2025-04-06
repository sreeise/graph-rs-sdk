use crate::identity::credentials::app_config::AppConfig;
use crate::identity::{
    tracing_targets::CREDENTIAL_EXECUTOR, Authority, AzureCloudInstance, Token,
    TokenCredentialExecutor,
};
use crate::oauth_serializer::{AuthParameter, AuthSerializer};
use async_trait::async_trait;
use graph_core::cache::{CacheStore, InMemoryCacheStore, TokenCache};
use graph_core::http::{AsyncResponseConverterExt, ResponseConverterExt};
use graph_core::identity::ForceTokenRefresh;
use graph_error::{AuthExecutionError, AuthExecutionResult, IdentityResult, AF};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use uuid::Uuid;

/// Allows an application to sign in the user by directly handling their password.
/// Not recommended. ROPC can also be done using a client secret or assertion,
/// however this client implementation does not offer this use case. This is the
/// same as all MSAL clients.
/// https://datatracker.ietf.org/doc/html/rfc6749#section-1.3.3
///
/// The Microsoft identity platform only supports the ROPC grant within Microsoft Entra tenants,
/// not personal accounts. This means that you must use a tenant-specific endpoint
/// (https://login.microsoftonline.com/{TenantId_or_Name}) or the organizations endpoint.
#[derive(Clone)]
pub struct ResourceOwnerPasswordCredential {
    pub(crate) app_config: AppConfig,
    /// Required
    /// The user's email address.
    pub(crate) username: String,
    /// Required
    /// The user's password.
    pub(crate) password: String,
    token_cache: InMemoryCacheStore<Token>,
}

impl Debug for ResourceOwnerPasswordCredential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientAssertionCredential")
            .field("app_config", &self.app_config)
            .finish()
    }
}

impl ResourceOwnerPasswordCredential {
    pub fn new(
        client_id: impl AsRef<str>,
        username: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> ResourceOwnerPasswordCredential {
        ResourceOwnerPasswordCredential {
            app_config: AppConfig::builder(client_id.as_ref())
                .authority(Authority::Organizations)
                .build(),
            username: username.as_ref().to_owned(),
            password: password.as_ref().to_owned(),
            token_cache: Default::default(),
        }
    }

    pub fn new_with_tenant(
        tenant_id: impl AsRef<str>,
        client_id: impl AsRef<str>,
        username: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> ResourceOwnerPasswordCredential {
        ResourceOwnerPasswordCredential {
            app_config: AppConfig::builder(client_id.as_ref())
                .tenant(tenant_id.as_ref())
                .build(),
            username: username.as_ref().to_owned(),
            password: password.as_ref().to_owned(),
            token_cache: Default::default(),
        }
    }

    pub fn builder<T: AsRef<str>>(client_id: T) -> ResourceOwnerPasswordCredentialBuilder {
        ResourceOwnerPasswordCredentialBuilder::new(client_id)
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
impl TokenCache for ResourceOwnerPasswordCredential {
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
impl TokenCredentialExecutor for ResourceOwnerPasswordCredential {
    fn form_urlencode(&mut self) -> IdentityResult<HashMap<String, String>> {
        let mut serializer = AuthSerializer::new();
        let client_id = self.app_config.client_id.to_string();
        if client_id.is_empty() || self.app_config.client_id.is_nil() {
            return AF::result(AuthParameter::ClientId.alias());
        }

        if self.username.trim().is_empty() {
            return AF::result(AuthParameter::Username.alias());
        }

        if self.password.trim().is_empty() {
            return AF::result(AuthParameter::Password.alias());
        }

        serializer
            .client_id(client_id.as_str())
            .grant_type("password")
            .set_scope(self.app_config.scope.clone())
            .username(&self.username)
            .password(&self.password);

        serializer.as_credential_map(
            vec![AuthParameter::Scope],
            vec![
                AuthParameter::ClientId,
                AuthParameter::GrantType,
                AuthParameter::Username,
                AuthParameter::Password,
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

    fn basic_auth(&self) -> Option<(String, String)> {
        Some((self.username.clone(), self.password.clone()))
    }

    fn app_config(&self) -> &AppConfig {
        &self.app_config
    }
}

#[derive(Clone)]
pub struct ResourceOwnerPasswordCredentialBuilder {
    credential: ResourceOwnerPasswordCredential,
}

impl ResourceOwnerPasswordCredentialBuilder {
    fn new(client_id: impl AsRef<str>) -> ResourceOwnerPasswordCredentialBuilder {
        ResourceOwnerPasswordCredentialBuilder {
            credential: ResourceOwnerPasswordCredential {
                app_config: AppConfig::new(client_id.as_ref()),
                username: Default::default(),
                password: Default::default(),
                token_cache: Default::default(),
            },
        }
    }

    pub(crate) fn new_with_username_password(
        username: impl AsRef<str>,
        password: impl AsRef<str>,
        app_config: AppConfig,
    ) -> ResourceOwnerPasswordCredentialBuilder {
        ResourceOwnerPasswordCredentialBuilder {
            credential: ResourceOwnerPasswordCredential {
                app_config,
                username: username.as_ref().to_owned(),
                password: password.as_ref().to_owned(),
                token_cache: Default::default(),
            },
        }
    }

    pub fn with_client_id<T: AsRef<str>>(&mut self, client_id: T) -> &mut Self {
        self.credential.app_config.client_id =
            Uuid::try_parse(client_id.as_ref()).unwrap_or_default();
        self
    }

    pub fn with_username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
        self.credential.username = username.as_ref().to_owned();
        self
    }

    pub fn with_password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
        self.credential.password = password.as_ref().to_owned();
        self
    }

    /// The grant type isn't supported on the /common or /consumers authentication contexts.
    /// Use /organizations or a tenant ID instead.
    /// Convenience method. Same as calling [with_authority(Authority::TenantId("tenant_id"))]
    pub fn with_tenant<T: AsRef<str>>(&mut self, tenant: T) -> &mut Self {
        self.credential.app_config.authority = Authority::TenantId(tenant.as_ref().to_owned());
        self
    }

    pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scope: I) -> &mut Self {
        self.credential.app_config.scope = scope.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// The grant type isn't supported on the /common or /consumers authentication contexts.
    /// Use /organizations or a tenant ID instead.
    /// Authority defaults to /organizations if no tenant id or authority is given.
    pub fn with_authority<T: Into<Authority>>(
        &mut self,
        authority: T,
    ) -> IdentityResult<&mut Self> {
        let authority = authority.into();
        if [
            Authority::Common,
            Authority::Consumers,
            Authority::AzureActiveDirectory,
        ]
        .contains(&authority)
        {
            return AF::msg_result(
                "tenant_id",
                "AzureActiveDirectory, Common, and Consumers are not supported authentication contexts for ROPC"
            );
        }

        self.credential.app_config.authority = authority;
        Ok(self)
    }

    pub fn build(&self) -> ResourceOwnerPasswordCredential {
        self.credential.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_on_authority_common() {
        let _ = ResourceOwnerPasswordCredential::builder(Uuid::new_v4().to_string())
            .with_authority(Authority::Common)
            .unwrap()
            .build();
    }

    #[test]
    #[should_panic]
    fn fail_on_authority_adfs() {
        let _ = ResourceOwnerPasswordCredential::builder(Uuid::new_v4().to_string())
            .with_authority(Authority::AzureActiveDirectory)
            .unwrap()
            .build();
    }

    #[test]
    #[should_panic]
    fn fail_on_authority_consumers() {
        let _ = ResourceOwnerPasswordCredential::builder(Uuid::new_v4().to_string())
            .with_authority(Authority::Consumers)
            .unwrap()
            .build();
    }
}
