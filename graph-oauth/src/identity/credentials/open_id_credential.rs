use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use async_trait::async_trait;
use graph_core::cache::{CacheStore, InMemoryCacheStore, TokenCache};
use http::{HeaderMap, HeaderName, HeaderValue};
use reqwest::IntoUrl;
use url::Url;
use uuid::Uuid;

use graph_core::crypto::{GenPkce, ProofKeyCodeExchange};
use graph_core::http::{AsyncResponseConverterExt, ResponseConverterExt};
use graph_core::identity::ForceTokenRefresh;
use graph_error::{AuthExecutionError, AuthExecutionResult, IdentityResult, AF};

use crate::identity::credentials::app_config::{AppConfig, AppConfigBuilder};
use crate::identity::{
    Authority, AzureCloudInstance, ConfidentialClientApplication,
    OpenIdAuthorizationUrlParameterBuilder, OpenIdAuthorizationUrlParameters, Token,
    TokenCredentialExecutor,
};
use crate::internal::{OAuthParameter, OAuthSerializer};

credential_builder!(
    OpenIdCredentialBuilder,
    ConfidentialClientApplication<OpenIdCredential>
);

/// OpenID Connect (OIDC) extends the OAuth 2.0 authorization protocol for use as an additional
/// authentication protocol. You can use OIDC to enable single sign-on (SSO) between your
/// OAuth-enabled applications by using a security token called an ID token.
/// https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc
#[derive(Clone)]
pub struct OpenIdCredential {
    pub(crate) app_config: AppConfig,

    /// Required unless requesting a refresh token
    /// The authorization code obtained from a call to authorize.
    /// The code should be obtained with all required scopes.
    pub(crate) authorization_code: Option<String>,
    /// Required when requesting a new access token using a refresh token
    /// The refresh token needed to make an access token request using a refresh token.
    /// Do not include an authorization code when using a refresh token.
    pub(crate) refresh_token: Option<String>,
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
    /// The same code_verifier that was used to obtain the authorization_code.
    /// Required if PKCE was used in the authorization code grant request. For more information,
    /// see the PKCE RFC https://datatracker.ietf.org/doc/html/rfc7636.
    pub(crate) code_verifier: Option<String>,
    /// Used only when the client generates the pkce itself when the generate method
    /// is called.
    pub(crate) pkce: Option<ProofKeyCodeExchange>,
    serializer: OAuthSerializer,
    token_cache: InMemoryCacheStore<Token>,
}
impl Debug for OpenIdCredential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenIdCredential")
            .field("app_config", &self.app_config)
            .finish()
    }
}

impl OpenIdCredential {
    pub fn new<T: AsRef<str>, U: IntoUrl>(
        client_id: T,
        client_secret: T,
        authorization_code: T,
        redirect_uri: U,
    ) -> IdentityResult<OpenIdCredential> {
        let redirect_uri_result = Url::parse(redirect_uri.as_str());
        Ok(OpenIdCredential {
            app_config: AppConfigBuilder::new(client_id.as_ref())
                .redirect_uri(redirect_uri.into_url().or(redirect_uri_result)?)
                .scope(vec!["openid"])
                .build(),
            authorization_code: Some(authorization_code.as_ref().to_owned()),
            refresh_token: None,
            client_secret: client_secret.as_ref().to_owned(),
            code_verifier: None,
            pkce: None,
            serializer: Default::default(),
            token_cache: Default::default(),
        })
    }

    pub fn with_refresh_token<T: AsRef<str>>(&mut self, refresh_token: T) {
        self.authorization_code = None;
        self.refresh_token = Some(refresh_token.as_ref().to_owned());
    }

    pub fn builder(client_id: impl TryInto<Uuid>) -> OpenIdCredentialBuilder {
        OpenIdCredentialBuilder::new(client_id)
    }

    pub fn authorization_url_builder(
        client_id: impl AsRef<str>,
    ) -> OpenIdAuthorizationUrlParameterBuilder {
        OpenIdAuthorizationUrlParameterBuilder::new_with_app_config(AppConfig::new(
            client_id.as_ref(),
        ))
    }

    pub fn pkce(&self) -> Option<&ProofKeyCodeExchange> {
        self.pkce.as_ref()
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

        if new_token.refresh_token.is_some() {
            self.refresh_token = new_token.refresh_token.clone();
        }

        self.token_cache.store(cache_id, new_token.clone());
        Ok(new_token)
    }
}

#[async_trait]
impl TokenCache for OpenIdCredential {
    type Token = Token;

    fn get_token_silent(&mut self) -> Result<Self::Token, AuthExecutionError> {
        let cache_id = self.app_config.cache_id.to_string();

        match self.app_config.force_token_refresh {
            ForceTokenRefresh::Never => {
                // Attempt to bypass a read on the token store by using previous
                // refresh token stored outside of RwLock
                if self.refresh_token.is_some() {
                    if let Ok(token) = self.execute_cached_token_refresh(cache_id.clone()) {
                        return Ok(token);
                    }
                }

                if let Some(token) = self.token_cache.get(cache_id.as_str()) {
                    if token.is_expired_sub(time::Duration::minutes(5)) {
                        if let Some(refresh_token) = token.refresh_token.as_ref() {
                            self.refresh_token = Some(refresh_token.to_owned());
                        }

                        self.execute_cached_token_refresh(cache_id)
                    } else {
                        Ok(token)
                    }
                } else {
                    self.execute_cached_token_refresh(cache_id)
                }
            }
            ForceTokenRefresh::Once | ForceTokenRefresh::Always => {
                let token_result = self.execute_cached_token_refresh(cache_id);
                if self.app_config.force_token_refresh == ForceTokenRefresh::Once {
                    self.app_config.force_token_refresh = ForceTokenRefresh::Never;
                }
                token_result
            }
        }
    }

    async fn get_token_silent_async(&mut self) -> Result<Self::Token, AuthExecutionError> {
        let cache_id = self.app_config.cache_id.to_string();

        match self.app_config.force_token_refresh {
            ForceTokenRefresh::Never => {
                // Attempt to bypass a read on the token store by using previous
                // refresh token stored outside of RwLock
                if self.refresh_token.is_some() {
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

                        self.execute_cached_token_refresh_async(cache_id).await
                    } else {
                        Ok(old_token.clone())
                    }
                } else {
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

#[async_trait]
impl TokenCredentialExecutor for OpenIdCredential {
    fn form_urlencode(&mut self) -> IdentityResult<HashMap<String, String>> {
        let client_id = self.app_config.client_id.to_string();
        if client_id.is_empty() || self.app_config.client_id.is_nil() {
            return AF::result(OAuthParameter::ClientId.alias());
        }

        if self.client_secret.trim().is_empty() {
            return AF::result(OAuthParameter::ClientSecret.alias());
        }

        self.serializer
            .client_id(client_id.as_str())
            .client_secret(self.client_secret.as_str())
            .set_scope(self.app_config.scope.clone());

        if let Some(refresh_token) = self.refresh_token.as_ref() {
            if refresh_token.trim().is_empty() {
                return AF::msg_result(OAuthParameter::RefreshToken, "Refresh token is empty");
            }

            self.serializer
                .grant_type("refresh_token")
                .refresh_token(refresh_token.as_ref());

            return self.serializer.as_credential_map(
                vec![OAuthParameter::Scope],
                vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::ClientSecret,
                    OAuthParameter::RefreshToken,
                    OAuthParameter::GrantType,
                ],
            );
        } else if let Some(authorization_code) = self.authorization_code.as_ref() {
            if authorization_code.trim().is_empty() {
                return AF::msg_result(
                    OAuthParameter::AuthorizationCode.alias(),
                    "Authorization code is empty",
                );
            }

            if let Some(redirect_uri) = self.app_config.redirect_uri.as_ref() {
                self.serializer.redirect_uri(redirect_uri.as_str());
            }

            self.serializer
                .authorization_code(authorization_code.as_ref())
                .grant_type("authorization_code");

            // Authorization codes can only be used once. Remove it from the configuration.
            self.authorization_code = None;

            if let Some(code_verifier) = self.code_verifier.as_ref() {
                self.serializer.code_verifier(code_verifier.as_str());
            }

            return self.serializer.as_credential_map(
                vec![OAuthParameter::Scope, OAuthParameter::CodeVerifier],
                vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::ClientSecret,
                    OAuthParameter::RedirectUri,
                    OAuthParameter::AuthorizationCode,
                    OAuthParameter::GrantType,
                ],
            );
        }

        AF::msg_result(
            format!(
                "{} or {}",
                OAuthParameter::AuthorizationCode.alias(),
                OAuthParameter::RefreshToken.alias()
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

#[derive(Clone)]
pub struct OpenIdCredentialBuilder {
    credential: OpenIdCredential,
}

impl OpenIdCredentialBuilder {
    fn new(client_id: impl TryInto<Uuid>) -> OpenIdCredentialBuilder {
        Self {
            credential: OpenIdCredential {
                app_config: AppConfig::builder(client_id)
                    .redirect_uri(
                        Url::parse("http://localhost").expect("Internal Error - please report"),
                    )
                    .scope(vec!["openid"])
                    .build(),
                authorization_code: None,
                refresh_token: None,
                client_secret: String::new(),
                code_verifier: None,
                pkce: None,
                serializer: Default::default(),
                token_cache: Default::default(),
            },
        }
    }

    fn new_with_app_config(mut app_config: AppConfig) -> OpenIdCredentialBuilder {
        app_config.scope.insert("openid".to_string());
        Self {
            credential: OpenIdCredential {
                app_config,
                authorization_code: None,
                refresh_token: None,
                client_secret: String::new(),
                code_verifier: None,
                pkce: None,
                serializer: Default::default(),
                token_cache: Default::default(),
            },
        }
    }

    pub(crate) fn new_with_auth_code(
        mut app_config: AppConfig,
        authorization_code: impl AsRef<str>,
    ) -> OpenIdCredentialBuilder {
        app_config.scope.insert("openid".to_string());
        OpenIdCredentialBuilder {
            credential: OpenIdCredential {
                app_config,
                authorization_code: Some(authorization_code.as_ref().to_owned()),
                refresh_token: None,
                client_secret: Default::default(),
                code_verifier: None,
                pkce: None,
                serializer: Default::default(),
                token_cache: Default::default(),
            },
        }
    }

    pub(crate) fn new_with_auth_code_and_secret(
        authorization_code: impl AsRef<str>,
        client_secret: impl AsRef<str>,
        mut app_config: AppConfig,
    ) -> OpenIdCredentialBuilder {
        app_config.scope.insert("openid".to_string());
        OpenIdCredentialBuilder {
            credential: OpenIdCredential {
                app_config,
                authorization_code: Some(authorization_code.as_ref().to_owned()),
                refresh_token: None,
                client_secret: client_secret.as_ref().to_owned(),
                code_verifier: None,
                pkce: None,
                serializer: Default::default(),
                token_cache: Default::default(),
            },
        }
    }

    #[cfg(feature = "interactive-auth")]
    pub(crate) fn new_with_token(app_config: AppConfig, token: Token) -> OpenIdCredentialBuilder {
        let cache_id = app_config.cache_id.clone();
        let mut token_cache = InMemoryCacheStore::new();
        token_cache.store(cache_id, token);

        Self {
            credential: OpenIdCredential {
                app_config,
                authorization_code: None,
                refresh_token: None,
                client_secret: Default::default(),
                code_verifier: None,
                pkce: None,
                serializer: Default::default(),
                token_cache,
            },
        }
    }

    pub fn with_authorization_code<T: AsRef<str>>(&mut self, authorization_code: T) -> &mut Self {
        self.credential.authorization_code = Some(authorization_code.as_ref().to_owned());
        self.credential.refresh_token = None;
        self
    }

    pub fn with_refresh_token<T: AsRef<str>>(&mut self, refresh_token: T) -> &mut Self {
        self.credential.authorization_code = None;
        self.credential.refresh_token = Some(refresh_token.as_ref().to_owned());
        self
    }

    /// Defaults to http://localhost
    pub fn with_redirect_uri<U: IntoUrl>(&mut self, redirect_uri: U) -> anyhow::Result<&mut Self> {
        self.credential.app_config.redirect_uri = Some(redirect_uri.into_url()?);
        Ok(self)
    }

    pub fn with_client_secret<T: AsRef<str>>(&mut self, client_secret: T) -> &mut Self {
        self.credential.client_secret = client_secret.as_ref().to_owned();
        self
    }

    fn with_code_verifier<T: AsRef<str>>(&mut self, code_verifier: T) -> &mut Self {
        self.credential.code_verifier = Some(code_verifier.as_ref().to_owned());
        self
    }

    pub fn with_pkce(&mut self, pkce: ProofKeyCodeExchange) -> &mut Self {
        self.with_code_verifier(pkce.code_verifier.as_str());
        self.credential.pkce = Some(pkce);
        self
    }

    pub fn with_pkce_oneshot(&mut self) -> IdentityResult<&mut Self> {
        let pkce = ProofKeyCodeExchange::oneshot()?;
        self.with_code_verifier(pkce.code_verifier.as_str());
        self.credential.pkce = Some(pkce);
        Ok(self)
    }

    pub fn credential(&self) -> &OpenIdCredential {
        &self.credential
    }
}

impl From<OpenIdAuthorizationUrlParameters> for OpenIdCredentialBuilder {
    fn from(value: OpenIdAuthorizationUrlParameters) -> Self {
        OpenIdCredentialBuilder::new_with_app_config(value.app_config)
    }
}

impl From<OpenIdCredential> for OpenIdCredentialBuilder {
    fn from(credential: OpenIdCredential) -> Self {
        OpenIdCredentialBuilder { credential }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn with_tenant_id_common() {
        let credential = OpenIdCredential::builder(Uuid::new_v4())
            .with_authority(Authority::TenantId("common".into()))
            .build();

        assert_eq!(credential.authority(), Authority::TenantId("common".into()))
    }

    #[test]
    fn with_tenant_id_adfs() {
        let credential = OpenIdCredential::builder(Uuid::new_v4())
            .with_authority(Authority::AzureDirectoryFederatedServices)
            .build();

        assert_eq!(credential.authority().as_ref(), "adfs");
    }
}
