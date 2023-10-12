use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use async_trait::async_trait;
use http::{HeaderMap, HeaderName, HeaderValue};
use reqwest::IntoUrl;
use url::Url;
use uuid::Uuid;

use graph_error::{IdentityResult, AF};
use graph_extensions::crypto::ProofKeyCodeExchange;

use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::credentials::app_config::AppConfig;
use crate::identity::{
    Authority, AzureCloudInstance, ConfidentialClientApplication, TokenCredentialExecutor,
};
use crate::oauth::AuthCodeAuthorizationUrlParameterBuilder;

credential_builder!(
    AuthorizationCodeCredentialBuilder,
    ConfidentialClientApplication
);

/// The OAuth 2.0 authorization code grant type, or auth code flow, enables a client application
/// to obtain authorized access to protected resources like web APIs. The auth code flow requires
/// a user-agent that supports redirection from the authorization server (the Microsoft
/// identity platform) back to your application. For example, a web browser, desktop, or mobile
/// application operated by a user to sign in to your app and access their data.
/// https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow
#[derive(Clone)]
pub struct AuthorizationCodeCredential {
    app_config: AppConfig,
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
    /// A space-separated list of scopes. The scopes must all be from a single resource,
    /// along with OIDC scopes (profile, openid, email). For more information, see Permissions
    /// and consent in the Microsoft identity platform. This parameter is a Microsoft extension
    /// to the authorization code flow, intended to allow apps to declare the resource they want
    /// the token for during token redemption.
    pub(crate) scope: Vec<String>,
    /// The same code_verifier that was used to obtain the authorization_code.
    /// Required if PKCE was used in the authorization code grant request. For more information,
    /// see the PKCE RFC https://datatracker.ietf.org/doc/html/rfc7636.
    pub(crate) code_verifier: Option<String>,
    serializer: OAuthSerializer,
}

impl Debug for AuthorizationCodeCredential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthorizationCodeCredential")
            .field("app_config", &self.app_config)
            .field("scope", &self.scope)
            .finish()
    }
}

impl AuthorizationCodeCredential {
    pub fn new<T: AsRef<str>, U: IntoUrl>(
        tenant_id: T,
        client_id: T,
        client_secret: T,
        authorization_code: T,
    ) -> IdentityResult<AuthorizationCodeCredential> {
        Ok(AuthorizationCodeCredential {
            app_config: AppConfig::new_with_tenant_and_client_id(tenant_id, client_id),
            authorization_code: Some(authorization_code.as_ref().to_owned()),
            refresh_token: None,
            client_secret: client_secret.as_ref().to_owned(),
            scope: vec![],
            code_verifier: None,
            serializer: OAuthSerializer::new(),
        })
    }

    pub fn new_with_redirect_uri<T: AsRef<str>, U: IntoUrl>(
        tenant_id: T,
        client_id: T,
        client_secret: T,
        authorization_code: T,
        redirect_uri: U,
    ) -> IdentityResult<AuthorizationCodeCredential> {
        let redirect_uri_result = Url::parse(redirect_uri.as_str());
        let redirect_uri = redirect_uri.into_url().or(redirect_uri_result)?;

        Ok(AuthorizationCodeCredential {
            app_config: AppConfig::new_init(
                Uuid::try_parse(client_id.as_ref()).unwrap_or_default(),
                Some(tenant_id.as_ref().to_owned()),
                Some(redirect_uri),
            ),
            authorization_code: Some(authorization_code.as_ref().to_owned()),
            refresh_token: None,
            client_secret: client_secret.as_ref().to_owned(),
            scope: vec![],
            code_verifier: None,
            serializer: OAuthSerializer::new(),
        })
    }

    pub fn with_refresh_token<T: AsRef<str>>(&mut self, refresh_token: T) {
        self.refresh_token = Some(refresh_token.as_ref().to_owned());
    }

    pub fn builder<T: AsRef<str>, U: AsRef<str>>(
        client_id: T,
        client_secret: T,
        authorization_code: U,
    ) -> AuthorizationCodeCredentialBuilder {
        AuthorizationCodeCredentialBuilder::new(client_id, client_secret, authorization_code)
    }

    pub fn authorization_url_builder<T: AsRef<str>>(
        client_id: T,
    ) -> AuthCodeAuthorizationUrlParameterBuilder {
        AuthCodeAuthorizationUrlParameterBuilder::new(client_id)
    }
}

#[derive(Clone)]
pub struct AuthorizationCodeCredentialBuilder {
    credential: AuthorizationCodeCredential,
}

impl AuthorizationCodeCredentialBuilder {
    fn new<T: AsRef<str>, U: AsRef<str>>(
        client_id: T,
        client_secret: T,
        authorization_code: U,
    ) -> AuthorizationCodeCredentialBuilder {
        Self {
            credential: AuthorizationCodeCredential {
                app_config: AppConfig::new_with_client_id(client_id.as_ref()),
                authorization_code: Some(authorization_code.as_ref().to_owned()),
                refresh_token: None,
                client_secret: client_secret.as_ref().to_owned(),
                scope: vec![],
                code_verifier: None,
                serializer: OAuthSerializer::new(),
            },
        }
    }

    pub(crate) fn new_with_auth_code(
        app_config: AppConfig,
        authorization_code: impl AsRef<str>,
    ) -> AuthorizationCodeCredentialBuilder {
        Self {
            credential: AuthorizationCodeCredential {
                app_config,
                authorization_code: Some(authorization_code.as_ref().to_owned()),
                refresh_token: None,
                client_secret: String::new(),
                scope: vec![],
                code_verifier: None,
                serializer: OAuthSerializer::new(),
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

    pub fn with_pkce(&mut self, proof_key_for_code_exchange: &ProofKeyCodeExchange) -> &mut Self {
        self.with_code_verifier(proof_key_for_code_exchange.code_verifier.as_str());
        self
    }
}

impl From<AuthorizationCodeCredential> for AuthorizationCodeCredentialBuilder {
    fn from(credential: AuthorizationCodeCredential) -> Self {
        AuthorizationCodeCredentialBuilder { credential }
    }
}

#[async_trait]
impl TokenCredentialExecutor for AuthorizationCodeCredential {
    fn uri(&mut self) -> IdentityResult<Url> {
        let azure_cloud_instance = self.azure_cloud_instance();
        self.serializer
            .authority(&azure_cloud_instance, &self.authority());

        let uri = self
            .serializer
            .get(OAuthParameter::TokenUrl)
            .ok_or(AF::msg_err("access_token_url", "Internal Error"))?;
        Url::parse(uri.as_str()).map_err(AF::from)
    }

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
            .extend_scopes(self.scope.clone());

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn with_tenant_id_common() {
        let credential = AuthorizationCodeCredential::builder(
            Uuid::new_v4().to_string(),
            "secret".to_string(),
            "code",
        )
        .with_authority(Authority::TenantId("common".into()))
        .build();

        assert_eq!(credential.authority(), Authority::TenantId("common".into()))
    }

    #[test]
    fn with_tenant_id_adfs() {
        let credential = AuthorizationCodeCredential::builder(
            Uuid::new_v4().to_string(),
            "secret".to_string(),
            "code",
        )
        .with_authority(Authority::AzureDirectoryFederatedServices)
        .build();

        assert_eq!(credential.authority().as_ref(), "adfs");
    }

    #[test]
    #[should_panic]
    fn required_value_missing_client_id() {
        let mut credential_builder = AuthorizationCodeCredential::builder(
            Uuid::default().to_string(),
            "secret".to_string(),
            "code",
        );
        credential_builder
            .with_authorization_code("code")
            .with_refresh_token("token");
        let mut credential = credential_builder.build();
        let _ = credential.form_urlencode().unwrap();
    }

    #[test]
    fn serialization() {
        let uuid_value = Uuid::new_v4().to_string();
        let mut credential_builder =
            AuthorizationCodeCredential::builder(uuid_value.clone(), "secret".to_string(), "code");
        let mut credential = credential_builder
            .with_redirect_uri("https://localhost")
            .unwrap()
            .with_client_secret("client_secret")
            .with_scope(vec!["scope"])
            .with_tenant("tenant_id")
            .build();

        let map = credential.form_urlencode().unwrap();
        assert_eq!(map.get("client_id"), Some(&uuid_value))
    }
}
