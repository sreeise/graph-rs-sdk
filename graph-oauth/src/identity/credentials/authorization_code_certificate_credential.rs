use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use async_trait::async_trait;
use http::{HeaderMap, HeaderName, HeaderValue};
use reqwest::IntoUrl;
use url::Url;

use uuid::Uuid;

use graph_core::cache::{CacheStore, InMemoryCacheStore, TokenCache};
use graph_core::http::{AsyncResponseConverterExt, ResponseConverterExt};
use graph_core::identity::ForceTokenRefresh;
use graph_error::{AuthExecutionError, AuthExecutionResult, IdentityResult, AF};

#[cfg(feature = "openssl")]
use crate::identity::X509Certificate;

use crate::identity::{
    AppConfig, AuthCodeAuthorizationUrlParameterBuilder, Authority, AzureCloudInstance,
    ConfidentialClientApplication, Token, TokenCredentialExecutor, CLIENT_ASSERTION_TYPE,
};
use crate::oauth_serializer::{AuthParameter, AuthSerializer};

credential_builder!(
    AuthorizationCodeCertificateCredentialBuilder,
    ConfidentialClientApplication<AuthorizationCodeCertificateCredential>
);

/// The OAuth 2.0 authorization code grant type, or auth code flow, enables a client application
/// to obtain authorized access to protected resources like web APIs. The auth code flow requires
/// a user-agent that supports redirection from the authorization server (the Microsoft
/// identity platform) back to your application. For example, a web browser, desktop, or mobile
/// application operated by a user to sign in to your app and access their data.
/// https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow'
///
/// [X509Certificate] requires features = \["openssl"\]
/// ```rust,ignore
/// use graph_rs_sdk::oauth::{
///     ClientCertificateCredential, ConfidentialClientApplication, PKey, X509Certificate, X509,
/// };
/// use std::fs::File;
/// use std::io::Read;
/// use std::path::Path;
///
/// pub fn x509_certificate(
///     client_id: &str,
///     tenant: &str,
///     public_key_path: impl AsRef<Path>,
///     private_key_path: impl AsRef<Path>,
/// ) -> anyhow::Result<X509Certificate> {
///     // Use include_bytes!(file_path) if the files are local
///     let mut cert_file = File::open(public_key_path)?;
///     let mut certificate: Vec<u8> = Vec::new();
///     cert_file.read_to_end(&mut certificate)?;
///
///     let mut private_key_file = File::open(private_key_path)?;
///     let mut private_key: Vec<u8> = Vec::new();
///     private_key_file.read_to_end(&mut private_key)?;
///
///     let cert = X509::from_pem(certificate.as_slice())?;
///     let pkey = PKey::private_key_from_pem(private_key.as_slice())?;
///     Ok(X509Certificate::new_with_tenant(
///         client_id, tenant, cert, pkey,
///     ))
/// }
///
/// fn build_confidential_client(
///     client_id: &str,
///     tenant: &str,
///     scope: Vec<&str>,
///     x509certificate: X509Certificate,
/// ) -> anyhow::Result<ConfidentialClientApplication<ClientCertificateCredential>> {
///     Ok(ConfidentialClientApplication::builder(client_id)
///         .with_client_x509_certificate(&x509certificate)?
///         .with_tenant(tenant)
///         .with_scope(scope)
///         .build())
/// }
///
/// ```
#[derive(Clone)]
pub struct AuthorizationCodeCertificateCredential {
    pub(crate) app_config: AppConfig,
    /// The authorization code obtained from a call to authorize. The code should be obtained with all required scopes.
    pub(crate) authorization_code: Option<String>,
    /// The refresh token needed to make an access token request using a refresh token.
    /// Do not include an authorization code when using a refresh token.
    pub(crate) refresh_token: Option<String>,
    /// The same code_verifier that was used to obtain the authorization_code.
    /// Required if PKCE was used in the authorization code grant request. For more information,
    /// see the PKCE RFC https://datatracker.ietf.org/doc/html/rfc7636.
    pub(crate) code_verifier: Option<String>,
    /// The value must be set to urn:ietf:params:oauth:client-assertion-type:jwt-bearer.
    pub(crate) client_assertion_type: String,
    /// An assertion (a JSON web token) that you need to create and sign with the certificate
    /// you registered as credentials for your application. Read about certificate credentials
    /// to learn how to register your certificate and the format of the assertion.
    pub(crate) client_assertion: String,
    token_cache: InMemoryCacheStore<Token>,
}

impl Debug for AuthorizationCodeCertificateCredential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthorizationCodeCertificateCredential")
            .field("app_config", &self.app_config)
            .finish()
    }
}
impl AuthorizationCodeCertificateCredential {
    pub fn new<T: AsRef<str>, U: IntoUrl>(
        client_id: T,
        authorization_code: T,
        client_assertion: T,
        redirect_uri: Option<U>,
    ) -> IdentityResult<AuthorizationCodeCertificateCredential> {
        let redirect_uri = {
            if let Some(redirect_uri) = redirect_uri {
                redirect_uri.into_url().ok()
            } else {
                None
            }
        };

        Ok(AuthorizationCodeCertificateCredential {
            app_config: AppConfig::builder(client_id.as_ref())
                .redirect_uri_option(redirect_uri)
                .build(),
            authorization_code: Some(authorization_code.as_ref().to_owned()),
            refresh_token: None,
            code_verifier: None,
            client_assertion_type: CLIENT_ASSERTION_TYPE.to_owned(),
            client_assertion: client_assertion.as_ref().to_owned(),
            token_cache: Default::default(),
        })
    }

    #[cfg(feature = "openssl")]
    pub fn builder(
        client_id: impl AsRef<str>,
        authorization_code: impl AsRef<str>,
        x509: &X509Certificate,
    ) -> IdentityResult<AuthorizationCodeCertificateCredentialBuilder> {
        AuthorizationCodeCertificateCredentialBuilder::new_with_auth_code_and_x509(
            authorization_code,
            x509,
            AppConfig::new(client_id.as_ref()),
        )
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

        if new_token.refresh_token.is_some() {
            self.refresh_token = new_token.refresh_token.clone();
        }

        self.token_cache.store(cache_id, new_token.clone());
        Ok(new_token)
    }
}

#[async_trait]
impl TokenCache for AuthorizationCodeCertificateCredential {
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
impl TokenCredentialExecutor for AuthorizationCodeCertificateCredential {
    fn form_urlencode(&mut self) -> IdentityResult<HashMap<String, String>> {
        let mut serializer = AuthSerializer::new();
        let client_id = self.app_config.client_id.to_string();
        if client_id.is_empty() || self.app_config.client_id.is_nil() {
            return AF::result(AuthParameter::ClientId);
        }

        if self.client_assertion.trim().is_empty() {
            return AF::result(AuthParameter::ClientAssertion);
        }

        if self.client_assertion_type.trim().is_empty() {
            self.client_assertion_type = CLIENT_ASSERTION_TYPE.to_owned();
        }

        serializer
            .client_id(client_id.as_str())
            .client_assertion(self.client_assertion.as_str())
            .client_assertion_type(self.client_assertion_type.as_str())
            .set_scope(self.app_config.scope.clone());

        if let Some(redirect_uri) = self.app_config.redirect_uri.as_ref() {
            serializer.redirect_uri(redirect_uri.as_str());
        }

        if let Some(code_verifier) = self.code_verifier.as_ref() {
            serializer.code_verifier(code_verifier.as_ref());
        }

        if let Some(refresh_token) = self.refresh_token.as_ref() {
            if refresh_token.trim().is_empty() {
                return AF::msg_result(
                    AuthParameter::RefreshToken.alias(),
                    "refresh_token is empty - cannot be an empty string",
                );
            }

            serializer
                .refresh_token(refresh_token.as_ref())
                .grant_type("refresh_token");

            return serializer.as_credential_map(
                vec![AuthParameter::Scope],
                vec![
                    AuthParameter::RefreshToken,
                    AuthParameter::ClientId,
                    AuthParameter::GrantType,
                    AuthParameter::ClientAssertion,
                    AuthParameter::ClientAssertionType,
                ],
            );
        } else if let Some(authorization_code) = self.authorization_code.as_ref() {
            if authorization_code.trim().is_empty() {
                return AF::msg_result(
                    AuthParameter::AuthorizationCode.alias(),
                    "authorization_code is empty - cannot be an empty string",
                );
            }

            serializer
                .authorization_code(authorization_code.as_str())
                .grant_type("authorization_code");

            return serializer.as_credential_map(
                vec![AuthParameter::Scope, AuthParameter::CodeVerifier],
                vec![
                    AuthParameter::AuthorizationCode,
                    AuthParameter::ClientId,
                    AuthParameter::GrantType,
                    AuthParameter::RedirectUri,
                    AuthParameter::ClientAssertion,
                    AuthParameter::ClientAssertionType,
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

    fn app_config(&self) -> &AppConfig {
        &self.app_config
    }
}

#[derive(Clone)]
pub struct AuthorizationCodeCertificateCredentialBuilder {
    credential: AuthorizationCodeCertificateCredential,
}

impl AuthorizationCodeCertificateCredentialBuilder {
    #[cfg(feature = "openssl")]
    pub(crate) fn new_with_auth_code_and_x509(
        authorization_code: impl AsRef<str>,
        x509: &X509Certificate,
        app_config: AppConfig,
    ) -> IdentityResult<AuthorizationCodeCertificateCredentialBuilder> {
        let mut builder = Self {
            credential: AuthorizationCodeCertificateCredential {
                app_config,
                authorization_code: Some(authorization_code.as_ref().to_owned()),
                refresh_token: None,
                code_verifier: None,
                client_assertion_type: CLIENT_ASSERTION_TYPE.to_owned(),
                client_assertion: String::new(),
                token_cache: Default::default(),
            },
        };

        builder.with_x509(x509)?;
        Ok(builder)
    }

    #[cfg(all(feature = "openssl", feature = "interactive-auth"))]
    pub(crate) fn new_with_token(
        token: Token,
        x509: &X509Certificate,
        app_config: AppConfig,
    ) -> IdentityResult<AuthorizationCodeCertificateCredentialBuilder> {
        let cache_id = app_config.cache_id.clone();
        let mut token_cache = InMemoryCacheStore::new();
        token_cache.store(cache_id, token);

        let mut builder = Self {
            credential: AuthorizationCodeCertificateCredential {
                app_config,
                authorization_code: None,
                refresh_token: None,
                code_verifier: None,
                client_assertion_type: CLIENT_ASSERTION_TYPE.to_owned(),
                client_assertion: String::new(),
                token_cache,
            },
        };

        builder.with_x509(x509)?;
        Ok(builder)
    }

    pub fn with_authorization_code<T: AsRef<str>>(&mut self, authorization_code: T) -> &mut Self {
        self.credential.authorization_code = Some(authorization_code.as_ref().to_owned());
        self
    }

    pub fn with_refresh_token<T: AsRef<str>>(&mut self, refresh_token: T) -> &mut Self {
        self.credential.authorization_code = None;
        self.credential.refresh_token = Some(refresh_token.as_ref().to_owned());
        self
    }

    pub fn with_redirect_uri(&mut self, redirect_uri: Url) -> &mut Self {
        self.credential.app_config.redirect_uri = Some(redirect_uri);
        self
    }

    pub fn with_code_verifier<T: AsRef<str>>(&mut self, code_verifier: T) -> &mut Self {
        self.credential.code_verifier = Some(code_verifier.as_ref().to_owned());
        self
    }

    #[cfg(feature = "openssl")]
    pub fn with_x509(
        &mut self,
        certificate_assertion: &X509Certificate,
    ) -> IdentityResult<&mut Self> {
        if let Some(tenant_id) = self.credential.authority().tenant_id() {
            self.with_client_assertion(
                certificate_assertion.sign_with_tenant(Some(tenant_id.clone()))?,
            );
        } else {
            self.with_client_assertion(certificate_assertion.sign_with_tenant(None)?);
        }
        Ok(self)
    }

    pub fn with_client_assertion<T: AsRef<str>>(&mut self, client_assertion: T) -> &mut Self {
        self.credential.client_assertion = client_assertion.as_ref().to_owned();
        self
    }

    pub fn with_client_assertion_type<T: AsRef<str>>(
        &mut self,
        client_assertion_type: T,
    ) -> &mut Self {
        self.credential.client_assertion_type = client_assertion_type.as_ref().to_owned();
        self
    }

    pub fn credential(self) -> AuthorizationCodeCertificateCredential {
        self.credential
    }
}

impl From<AuthorizationCodeCertificateCredential>
    for AuthorizationCodeCertificateCredentialBuilder
{
    fn from(credential: AuthorizationCodeCertificateCredential) -> Self {
        AuthorizationCodeCertificateCredentialBuilder { credential }
    }
}

impl From<AuthorizationCodeCertificateCredentialBuilder>
    for AuthorizationCodeCertificateCredential
{
    fn from(builder: AuthorizationCodeCertificateCredentialBuilder) -> Self {
        builder.credential
    }
}

impl Debug for AuthorizationCodeCertificateCredentialBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.credential.fmt(f)
    }
}
