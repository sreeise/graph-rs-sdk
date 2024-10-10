use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::{Debug, Formatter};

use async_trait::async_trait;
use http::{HeaderMap, HeaderName, HeaderValue};

use url::Url;
use uuid::Uuid;

use graph_core::cache::{CacheStore, InMemoryCacheStore, TokenCache};
use graph_core::crypto::ProofKeyCodeExchange;
use graph_core::http::{AsyncResponseConverterExt, ResponseConverterExt};
use graph_core::identity::ForceTokenRefresh;
use graph_error::{AuthExecutionError, AuthExecutionResult, IdentityResult, AF};

use crate::identity::credentials::app_config::{AppConfig, AppConfigBuilder};
use crate::identity::{
    tracing_targets::CREDENTIAL_EXECUTOR, Authority, AuthorizationResponse, AzureCloudInstance,
    Token, TokenCredentialExecutor,
};
use crate::oauth_serializer::{AuthParameter, AuthSerializer};
use crate::{AuthCodeAuthorizationUrlParameterBuilder, PublicClientApplication};

credential_builder!(
    AuthorizationCodeSpaCredentialBuilder,
    PublicClientApplication<AuthorizationCodeSpaCredential>
);

/// The OAuth 2.0 authorization code grant type, or auth code flow, enables a client application
/// to obtain authorized access to protected resources like web APIs. The auth code flow requires
/// a user-agent that supports redirection from the authorization server (the Microsoft
/// identity platform) back to your application. For example, a web browser, desktop, or mobile
/// application operated by a user to sign in to your app and access their data.
/// https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow
#[derive(Clone)]
pub struct AuthorizationCodeSpaCredential {
    app_config: AppConfig,
    /// Required unless requesting a refresh token
    /// The authorization code obtained from a call to authorize.
    /// The code should be obtained with all required scopes.
    pub(crate) authorization_code: Option<String>,
    /// Required when requesting a new access token using a refresh token
    /// The refresh token needed to make an access token request using a refresh token.
    /// Do not include an authorization code when using a refresh token.
    pub(crate) refresh_token: Option<String>,
    /// The same code_verifier that was used to obtain the authorization_code.
    /// Required if PKCE was used in the authorization code grant request. For more information,
    /// see the PKCE RFC https://datatracker.ietf.org/doc/html/rfc7636.
    pub(crate) code_verifier: Option<String>,
    token_cache: InMemoryCacheStore<Token>,
}

impl Debug for AuthorizationCodeSpaCredential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthorizationCodeSpaCredential")
            .field("app_config", &self.app_config)
            .finish()
    }
}

impl AuthorizationCodeSpaCredential {
    pub fn new(
        tenant_id: impl AsRef<str>,
        client_id: impl AsRef<str>,
        authorization_code: impl AsRef<str>,
    ) -> IdentityResult<AuthorizationCodeSpaCredential> {
        Ok(AuthorizationCodeSpaCredential {
            app_config: AppConfig::builder(client_id.as_ref())
                .tenant(tenant_id.as_ref())
                .build(),
            authorization_code: Some(authorization_code.as_ref().to_owned()),
            refresh_token: None,
            code_verifier: None,
            token_cache: Default::default(),
        })
    }

    pub fn new_with_redirect_uri(
        tenant_id: impl AsRef<str>,
        client_id: impl AsRef<str>,
        authorization_code: impl AsRef<str>,
        redirect_uri: Url,
    ) -> IdentityResult<AuthorizationCodeSpaCredential> {
        Ok(AuthorizationCodeSpaCredential {
            app_config: AppConfigBuilder::new(client_id.as_ref())
                .tenant(tenant_id.as_ref())
                .redirect_uri(redirect_uri)
                .build(),
            authorization_code: Some(authorization_code.as_ref().to_owned()),
            refresh_token: None,
            code_verifier: None,
            token_cache: Default::default(),
        })
    }

    pub fn with_refresh_token<T: AsRef<str>>(&mut self, refresh_token: T) {
        self.refresh_token = Some(refresh_token.as_ref().to_owned());
    }

    pub fn builder(
        authorization_code: impl AsRef<str>,
        client_id: impl AsRef<str>,
    ) -> AuthorizationCodeSpaCredentialBuilder {
        AuthorizationCodeSpaCredentialBuilder::new(authorization_code, client_id)
    }

    pub fn authorization_url_builder(
        client_id: impl TryInto<Uuid>,
    ) -> AuthCodeAuthorizationUrlParameterBuilder {
        AuthCodeAuthorizationUrlParameterBuilder::new(client_id)
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

        if new_token.refresh_token.is_some() {
            self.refresh_token = new_token.refresh_token.clone();
        }

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

        if new_token.refresh_token.is_some() {
            self.refresh_token = new_token.refresh_token.clone();
        }
        Ok(new_token)
    }
}

#[async_trait]
impl TokenCache for AuthorizationCodeSpaCredential {
    type Token = Token;

    #[tracing::instrument]
    fn get_token_silent(&mut self) -> Result<Self::Token, AuthExecutionError> {
        let cache_id = self.app_config.cache_id.to_string();

        match self.app_config.force_token_refresh {
            ForceTokenRefresh::Never => {
                // Attempt to bypass a read on the token store by using previous
                // refresh token stored outside of RwLock
                if self.refresh_token.is_some() {
                    tracing::debug!(target: CREDENTIAL_EXECUTOR, "executing silent token request; refresh_token=Some");
                    if let Ok(token) = self.execute_cached_token_refresh(cache_id.clone()) {
                        return Ok(token);
                    }
                }

                if let Some(token) = self.token_cache.get(cache_id.as_str()) {
                    if token.is_expired_sub(time::Duration::minutes(5)) {
                        tracing::debug!(target: CREDENTIAL_EXECUTOR, "executing silent token request; refresh_token=Some");
                        if let Some(refresh_token) = token.refresh_token.as_ref() {
                            self.refresh_token = Some(refresh_token.to_owned());
                        }

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
            ForceTokenRefresh::Once | ForceTokenRefresh::Always => {
                tracing::debug!(target: CREDENTIAL_EXECUTOR, "executing silent token request; refresh_token=None");
                let token_result = self.execute_cached_token_refresh(cache_id);
                if self.app_config.force_token_refresh == ForceTokenRefresh::Once {
                    self.app_config.force_token_refresh = ForceTokenRefresh::Never;
                }
                token_result
            }
        }
    }

    #[tracing::instrument]
    async fn get_token_silent_async(&mut self) -> Result<Self::Token, AuthExecutionError> {
        let cache_id = self.app_config.cache_id.to_string();

        match self.app_config.force_token_refresh {
            ForceTokenRefresh::Never => {
                // Attempt to bypass a read on the token store by using previous
                // refresh token stored outside of RwLock
                if self.refresh_token.is_some() {
                    tracing::debug!(target: CREDENTIAL_EXECUTOR, "executing silent token request; refresh_token=Some");
                    if let Ok(token) = self
                        .execute_cached_token_refresh_async(cache_id.clone())
                        .await
                    {
                        return Ok(token);
                    }
                }

                if let Some(old_token) = self.token_cache.get(cache_id.as_str()) {
                    if old_token.is_expired_sub(time::Duration::minutes(5)) {
                        if let Some(refresh_token) = old_token.refresh_token.as_ref() {
                            self.refresh_token = Some(refresh_token.to_owned());
                        }
                        tracing::debug!(target: CREDENTIAL_EXECUTOR, "executing silent token request; refresh_token=Some");
                        self.execute_cached_token_refresh_async(cache_id).await
                    } else {
                        tracing::debug!(target: CREDENTIAL_EXECUTOR, "using token from cache");
                        Ok(old_token.clone())
                    }
                } else {
                    tracing::debug!(target: CREDENTIAL_EXECUTOR, "executing silent token request; refresh_token=None");
                    self.execute_cached_token_refresh_async(cache_id).await
                }
            }
            ForceTokenRefresh::Once | ForceTokenRefresh::Always => {
                let token_result = self.execute_cached_token_refresh_async(cache_id).await;
                if self.app_config.force_token_refresh == ForceTokenRefresh::Once {
                    self.app_config.force_token_refresh = ForceTokenRefresh::Never;
                }
                token_result
            }
        }
    }

    fn with_force_token_refresh(&mut self, force_token_refresh: ForceTokenRefresh) {
        self.app_config.force_token_refresh = force_token_refresh;
    }
}

#[derive(Clone)]
pub struct AuthorizationCodeSpaCredentialBuilder {
    credential: AuthorizationCodeSpaCredential,
}

impl AuthorizationCodeSpaCredentialBuilder {
    fn new(
        authorization_code: impl AsRef<str>,
        client_id: impl AsRef<str>,
    ) -> AuthorizationCodeSpaCredentialBuilder {
        Self {
            credential: AuthorizationCodeSpaCredential {
                app_config: AppConfig::new(client_id.as_ref()),
                authorization_code: Some(authorization_code.as_ref().to_owned()),
                refresh_token: None,
                code_verifier: None,
                token_cache: Default::default(),
            },
        }
    }

    pub(crate) fn new_with_token(
        app_config: AppConfig,
        token: Token,
    ) -> AuthorizationCodeSpaCredentialBuilder {
        let cache_id = app_config.cache_id.clone();
        let mut token_cache = InMemoryCacheStore::new();
        token_cache.store(cache_id, token);

        Self {
            credential: AuthorizationCodeSpaCredential {
                app_config,
                authorization_code: None,
                refresh_token: None,
                code_verifier: None,
                token_cache,
            },
        }
    }

    pub(crate) fn new_with_auth_code(
        authorization_code: impl AsRef<str>,
        app_config: AppConfig,
    ) -> AuthorizationCodeSpaCredentialBuilder {
        Self {
            credential: AuthorizationCodeSpaCredential {
                app_config,
                authorization_code: Some(authorization_code.as_ref().to_owned()),
                refresh_token: None,
                code_verifier: None,
                token_cache: Default::default(),
            },
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_secret(
        authorization_code: String,
        app_config: AppConfig,
    ) -> AuthorizationCodeSpaCredentialBuilder {
        Self {
            credential: AuthorizationCodeSpaCredential {
                app_config,
                authorization_code: Some(authorization_code),
                refresh_token: None,
                code_verifier: None,
                token_cache: Default::default(),
            },
        }
    }

    pub fn with_authorization_code<T: AsRef<str>>(&mut self, authorization_code: T) -> &mut Self {
        self.credential.authorization_code = Some(authorization_code.as_ref().to_owned());
        self.credential.refresh_token = None;
        self
    }

    pub fn with_refresh_token<T: AsRef<str>>(&mut self, refresh_token: T) -> &mut Self {
        self.credential.refresh_token = Some(refresh_token.as_ref().to_owned());
        self
    }

    /// Defaults to http://localhost
    pub fn with_redirect_uri(&mut self, redirect_uri: Url) -> &mut Self {
        self.credential.app_config.redirect_uri = Some(redirect_uri);
        self
    }

    fn with_code_verifier<T: AsRef<str>>(&mut self, code_verifier: T) -> &mut Self {
        self.credential.code_verifier = Some(code_verifier.as_ref().to_owned());
        self
    }

    pub fn with_pkce(&mut self, proof_key_for_code_exchange: &ProofKeyCodeExchange) -> &mut Self {
        self.with_code_verifier(proof_key_for_code_exchange.code_verifier.as_str());
        self
    }
}

impl From<AuthorizationCodeSpaCredential> for AuthorizationCodeSpaCredentialBuilder {
    fn from(credential: AuthorizationCodeSpaCredential) -> Self {
        AuthorizationCodeSpaCredentialBuilder { credential }
    }
}

#[async_trait]
impl TokenCredentialExecutor for AuthorizationCodeSpaCredential {
    fn form_urlencode(&mut self) -> IdentityResult<HashMap<String, String>> {
        let mut serializer = AuthSerializer::new();
        let client_id = self.app_config.client_id.to_string();
        if client_id.is_empty() || self.app_config.client_id.is_nil() {
            return AF::result(AuthParameter::ClientId.alias());
        }

        serializer
            .client_id(client_id.as_str())
            .set_scope(self.app_config.scope.clone());

        let cache_id = self.app_config.cache_id.to_string();
        if let Some(token) = self.token_cache.get(cache_id.as_str()) {
            if let Some(refresh_token) = token.refresh_token.as_ref() {
                serializer
                    .grant_type("refresh_token")
                    .refresh_token(refresh_token.as_ref());

                return serializer.as_credential_map(
                    vec![AuthParameter::Scope],
                    vec![
                        AuthParameter::ClientId,
                        AuthParameter::RefreshToken,
                        AuthParameter::GrantType,
                    ],
                );
            }
        }

        let should_attempt_refresh = self.refresh_token.is_some()
            && self.app_config.force_token_refresh != ForceTokenRefresh::Once
            && self.app_config.force_token_refresh != ForceTokenRefresh::Always;

        if should_attempt_refresh {
            let refresh_token = self.refresh_token.clone().unwrap_or_default();
            if refresh_token.trim().is_empty() {
                return AF::msg_result(AuthParameter::RefreshToken, "Refresh token is empty");
            }

            serializer
                .grant_type("refresh_token")
                .refresh_token(refresh_token.as_ref());

            return serializer.as_credential_map(
                vec![AuthParameter::Scope],
                vec![
                    AuthParameter::ClientId,
                    AuthParameter::RefreshToken,
                    AuthParameter::GrantType,
                ],
            );
        } else if let Some(authorization_code) = self.authorization_code.as_ref() {
            if authorization_code.trim().is_empty() {
                return AF::msg_result(
                    AuthParameter::AuthorizationCode.alias(),
                    "Authorization code is empty",
                );
            }

            if let Some(redirect_uri) = self.app_config.redirect_uri.as_ref() {
                serializer.redirect_uri(redirect_uri.as_str());
            }

            serializer
                .authorization_code(authorization_code.as_ref())
                .grant_type("authorization_code");

            if let Some(code_verifier) = self.code_verifier.as_ref() {
                if code_verifier.trim().is_empty() {
                    return AF::msg_result(
                        AuthParameter::CodeVerifier.alias(),
                        "Code verifier is empty - Spa apps require Proof Key Code Exchange",
                    );
                }

                serializer.code_verifier(code_verifier.as_str());
            }

            return serializer.as_credential_map(
                vec![AuthParameter::Scope],
                vec![
                    AuthParameter::ClientId,
                    AuthParameter::RedirectUri,
                    AuthParameter::AuthorizationCode,
                    AuthParameter::CodeVerifier,
                    AuthParameter::GrantType,
                ],
            );
        }

        AF::msg_result(
            format!(
                "{} or {}",
                AuthParameter::AuthorizationCode.alias(),
                AuthParameter::RefreshToken.alias()
            ),
            "Either authorization code or refresh token is required",
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

    /// Basic auth is not used in Spas because they do not use a client secret.
    fn basic_auth(&self) -> Option<(String, String)> {
        None
    }

    fn app_config(&self) -> &AppConfig {
        &self.app_config
    }
}

impl Debug for AuthorizationCodeSpaCredentialBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.credential.fmt(f)
    }
}

impl From<(AppConfig, AuthorizationResponse)> for AuthorizationCodeSpaCredentialBuilder {
    fn from(value: (AppConfig, AuthorizationResponse)) -> Self {
        let (app_config, authorization_response) = value;
        if let Some(authorization_code) = authorization_response.code.as_ref() {
            AuthorizationCodeSpaCredentialBuilder::new_with_auth_code(
                authorization_code,
                app_config,
            )
        } else {
            AuthorizationCodeSpaCredentialBuilder::new_with_token(
                app_config,
                Token::try_from(authorization_response.clone()).unwrap_or_default(),
            )
        }
    }
}
