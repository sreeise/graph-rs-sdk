use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use async_trait::async_trait;
use graph_core::cache::{CacheStore, InMemoryCacheStore, TokenCache};
use http::{HeaderMap, HeaderName, HeaderValue};

use jsonwebtoken::TokenData;
use reqwest::IntoUrl;
use url::{ParseError, Url};
use uuid::Uuid;

use graph_core::{
    crypto::{GenPkce, ProofKeyCodeExchange},
    http::{AsyncResponseConverterExt, ResponseConverterExt},
    identity::{Claims, DecodedJwt, ForceTokenRefresh, JwksKeySet},
};

use graph_error::{
    AuthExecutionError, AuthExecutionResult, AuthorizationFailure, IdentityResult, AF,
};

use crate::identity::credentials::app_config::{AppConfig, AppConfigBuilder};
use crate::identity::{
    tracing_targets::CREDENTIAL_EXECUTOR, Authority, AuthorizationResponse, AzureCloudInstance,
    ConfidentialClientApplication, IdToken, OpenIdAuthorizationUrlParameterBuilder,
    OpenIdAuthorizationUrlParameters, Token, TokenCredentialExecutor,
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
    verify_id_token: bool,
    id_token_jwt: Option<DecodedJwt>,
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
            verify_id_token: Default::default(),
            id_token_jwt: None,
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

    pub(crate) fn get_decoded_jwt(&self) -> Option<&TokenData<Claims>> {
        self.id_token_jwt.as_ref()
    }

    pub fn get_openid_config(&self) -> AuthExecutionResult<reqwest::blocking::Response> {
        let uri = self
            .app_config
            .azure_cloud_instance
            .openid_configuration_uri(&self.app_config.authority)
            .map_err(AuthorizationFailure::from)?;
        Ok(reqwest::blocking::get(uri)?)
    }

    pub async fn get_openid_config_async(&self) -> AuthExecutionResult<reqwest::Response> {
        let uri = self
            .app_config
            .azure_cloud_instance
            .openid_configuration_uri(&self.app_config.authority)
            .map_err(AuthorizationFailure::from)?;
        reqwest::get(uri).await.map_err(AuthExecutionError::from)
    }

    pub fn get_jwks(&self) -> AuthExecutionResult<reqwest::blocking::Response> {
        let config_response = self.get_openid_config()?;
        let json: serde_json::Value = config_response.json()?;
        let jwks_uri = json["jwks_uri"]
            .as_str()
            .ok_or(AuthExecutionError::Authorization(AF::msg_err(
                "jwks_uri",
                "not found in openid configuration",
            )))?;
        Ok(reqwest::blocking::get(jwks_uri)?)
    }

    pub async fn get_jwks_async(&self) -> AuthExecutionResult<reqwest::Response> {
        let config_response = self.get_openid_config_async().await?;
        let json: serde_json::Value = config_response.json().await?;
        let jwks_uri = json["jwks_uri"]
            .as_str()
            .ok_or(AuthExecutionError::Authorization(AF::msg_err(
                "jwks_uri",
                "not found in openid configuration",
            )))?;
        reqwest::get(jwks_uri)
            .await
            .map_err(AuthExecutionError::from)
    }

    pub fn verify_jwks(&self) -> AuthExecutionResult<TokenData<Claims>> {
        let cache_id = self.app_config.cache_id.to_string();
        let token = self
            .token_cache
            .get(cache_id.as_str())
            .ok_or(AF::msg_err("token", "no cached token"))?;
        let mut id_token = token
            .id_token
            .ok_or(AF::msg_err("id_token", "no cached id_token"))?;
        self.verify_jwks_from_token(&mut id_token)
    }

    pub async fn verify_jwks_async(&self) -> AuthExecutionResult<TokenData<Claims>> {
        let cache_id = self.app_config.cache_id.to_string();
        let token = self
            .token_cache
            .get(cache_id.as_str())
            .ok_or(AF::msg_err("token", "no cached token"))?;
        let mut id_token = token
            .id_token
            .clone()
            .ok_or(AF::msg_err("id_token", "no cached id_token"))?;
        self.verify_jwks_from_token_async(&mut id_token).await
    }

    fn verify_jwks_from_token(
        &self,
        id_token: &mut IdToken,
    ) -> AuthExecutionResult<TokenData<Claims>> {
        let headers = id_token.decode_header()?;
        let kid = headers
            .kid
            .as_ref()
            .ok_or(AF::msg_err("id_token", "id_token header does not have kid"))?;

        let response = self.get_jwks()?;
        let status = response.status();

        tracing::debug!(target: CREDENTIAL_EXECUTOR, "jwks key set response received; status={status:#?}");

        let key_set: JwksKeySet = response.json()?;
        let jwks_key = key_set
            .keys
            .iter()
            .find(|key| key.kid.eq(kid))
            .cloned()
            .ok_or(AF::msg_err(
                "kid",
                "no match found for kid in json web keys",
            ))
            .map_err(AuthExecutionError::from)?;

        tracing::debug!(target: CREDENTIAL_EXECUTOR, "found matching kid in jwks key set");

        if self.app_config.tenant_id.is_some() {
            Ok(id_token.decode(
                jwks_key.modulus.as_str(),
                jwks_key.exponent.as_str(),
                &self.app_config.client_id.to_string(),
                Some(self.issuer().map_err(AuthorizationFailure::from)?.as_str()),
            )?)
        } else {
            Ok(id_token.decode(
                jwks_key.modulus.as_str(),
                jwks_key.exponent.as_str(),
                &self.app_config.client_id.to_string(),
                None,
            )?)
        }
    }

    async fn verify_jwks_from_token_async(
        &self,
        id_token: &mut IdToken,
    ) -> AuthExecutionResult<TokenData<Claims>> {
        let headers = id_token.decode_header()?;
        let value2 = serde_json::to_string(&headers).unwrap();
        tracing::debug!(
             target: CREDENTIAL_EXECUTOR,
            value2
        );

        let kid = headers
            .kid
            .as_ref()
            .ok_or(AF::msg_err("id_token", "id_token header does not have kid"))?;

        let response = self.get_jwks_async().await?;
        let key_set: JwksKeySet = response.json().await?;
        let jwks_key = key_set
            .keys
            .iter()
            .find(|key| key.kid.eq(kid))
            .cloned()
            .ok_or(AF::msg_err(
                "kid",
                "no match found for kid in json web keys",
            ))
            .map_err(AuthExecutionError::from)?;

        if self.app_config.tenant_id.is_some() {
            Ok(id_token.decode(
                jwks_key.modulus.as_str(),
                jwks_key.exponent.as_str(),
                &self.app_config.client_id.to_string(),
                Some(self.issuer().map_err(AuthorizationFailure::from)?.as_str()),
            )?)
        } else {
            Ok(id_token.decode(
                jwks_key.modulus.as_str(),
                jwks_key.exponent.as_str(),
                &self.app_config.client_id.to_string(),
                None,
            )?)
        }
    }

    #[allow(unused)]
    async fn verify_authorization_id_token_async(
        &mut self,
    ) -> Option<AuthExecutionResult<TokenData<Claims>>> {
        if let Some(id_token) = self.app_config.id_token.as_ref() {
            let mut id_token_clone = id_token.clone();
            if !id_token_clone.verified {
                return match self.verify_jwks_from_token_async(&mut id_token_clone).await {
                    Ok(token_data) => {
                        self.app_config.with_id_token(id_token_clone);
                        Some(Ok(token_data))
                    }
                    Err(err) => Some(Err(err)),
                };
                // return Some(self.verify_jwks_from_token_async(&mut id_token_clone).await)
            }
        }
        None
    }

    fn execute_cached_token_refresh(&mut self, cache_id: String) -> AuthExecutionResult<Token> {
        let response = self.execute()?;

        if !response.status().is_success() {
            return Err(AuthExecutionError::silent_token_auth(
                response.into_http_response()?,
            ));
        }

        let new_token: Token = response.json()?;

        if self.verify_id_token {
            if let Some(mut id_token) = new_token.id_token.clone() {
                tracing::debug!(target: CREDENTIAL_EXECUTOR, "performing jwks verification");

                let id_token_verification_result = self.verify_jwks_from_token(&mut id_token);
                if let Ok(token_data) = id_token_verification_result {
                    self.id_token_jwt = Some(token_data);
                    dbg!(&self.id_token_jwt);
                    tracing::debug!(target: CREDENTIAL_EXECUTOR, "jwks verification successful");
                } else if let Err(err) = id_token_verification_result {
                    tracing::debug!(target: CREDENTIAL_EXECUTOR, "jwks verification failed - evicting token from cache");

                    // The new token has not been stored in the cache but we still need evict any previous tokens.
                    self.refresh_token = None;
                    self.token_cache.evict(cache_id.as_str());
                    return Err(err);
                }
            }
        }

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

        if self.verify_id_token {
            if let Some(mut id_token) = new_token.id_token.clone() {
                tracing::debug!(
                    target: CREDENTIAL_EXECUTOR,
                    verify_id_token = self.verify_id_token,
                    "performing jwks verification:"
                );

                let id_token_verification_result =
                    self.verify_jwks_from_token_async(&mut id_token).await;
                if let Ok(token_data) = id_token_verification_result {
                    self.id_token_jwt = Some(token_data);
                    dbg!(&self.id_token_jwt);
                    tracing::debug!(target: CREDENTIAL_EXECUTOR, "jwks verification successful");
                } else if let Err(err) = id_token_verification_result {
                    tracing::debug!(target: CREDENTIAL_EXECUTOR, "jwks verification failed - evicting token from cache");

                    // The new token has not been stored in the cache but we still need evict any previous tokens.
                    self.refresh_token = None;
                    self.token_cache.evict(cache_id.as_str());
                    return Err(err);
                }
            }
        }

        self.token_cache.store(cache_id, new_token.clone());

        if new_token.refresh_token.is_some() {
            self.refresh_token = new_token.refresh_token.clone();
        }

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

    fn decoded_jwt(&self) -> Option<&DecodedJwt> {
        self.id_token_jwt.as_ref()
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

impl Debug for OpenIdCredentialBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.credential.fmt(f)
    }
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
                verify_id_token: Default::default(),
                id_token_jwt: None,
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
                verify_id_token: Default::default(),
                id_token_jwt: None,
            },
        }
    }

    pub(crate) fn new_with_auth_code(
        mut app_config: AppConfig,
        authorization_code: impl AsRef<str>,
        verify_id_token: bool,
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
                verify_id_token,
                id_token_jwt: None,
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
                verify_id_token: Default::default(),
                id_token_jwt: None,
            },
        }
    }

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
                verify_id_token: Default::default(),
                id_token_jwt: None,
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

    pub fn with_id_token_verification(&mut self, verify_id_token: bool) -> &mut Self {
        self.credential.verify_id_token = verify_id_token;
        self
    }

    pub fn issuer(&self) -> Result<Url, ParseError> {
        self.credential.issuer()
    }

    pub fn get_openid_config(&self) -> AuthExecutionResult<reqwest::blocking::Response> {
        self.credential.get_openid_config()
    }

    pub async fn get_openid_config_async(&self) -> AuthExecutionResult<reqwest::Response> {
        self.credential.get_openid_config_async().await
    }

    pub fn get_jwks(&self) -> AuthExecutionResult<reqwest::blocking::Response> {
        self.credential.get_jwks()
    }

    pub async fn get_jwks_async(&self) -> AuthExecutionResult<reqwest::Response> {
        self.credential.get_jwks_async().await
    }

    #[allow(dead_code)]
    pub(crate) async fn verify_jwks_async(&self) -> AuthExecutionResult<TokenData<Claims>> {
        self.credential.verify_jwks_async().await
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

impl From<(AppConfig, AuthorizationResponse)> for OpenIdCredentialBuilder {
    fn from(value: (AppConfig, AuthorizationResponse)) -> Self {
        let (mut app_config, authorization_response) = value;
        if let Some(authorization_code) = authorization_response.code.as_ref() {
            if let Some(id_token) = authorization_response.id_token.as_ref() {
                app_config.with_id_token(IdToken::new(
                    id_token.as_ref(),
                    None,
                    Some(authorization_code.as_ref()),
                    None,
                ));
                OpenIdCredentialBuilder::new_with_auth_code(app_config, authorization_code, true)
            } else {
                OpenIdCredentialBuilder::new_with_auth_code(app_config, authorization_code, false)
            }
        } else {
            OpenIdCredentialBuilder::new_with_token(
                app_config,
                Token::try_from(authorization_response.clone()).unwrap_or_default(),
            )
        }
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
