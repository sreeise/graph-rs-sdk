use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::credentials::app_config::AppConfig;
use crate::identity::{
    Authority, AzureCloudInstance, OpenIdAuthorizationUrl, ProofKeyForCodeExchange,
    TokenCredentialExecutor,
};
use crate::oauth::{ConfidentialClientApplication, OpenIdAuthorizationUrlBuilder};
use async_trait::async_trait;
use graph_error::{AuthorizationResult, AF};
use http::{HeaderMap, HeaderName, HeaderValue};
use reqwest::IntoUrl;
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

credential_builder!(OpenIdCredentialBuilder, ConfidentialClientApplication);

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
    /// The same redirect_uri value that was used to acquire the authorization_code.
    // pub(crate) redirect_uri: Url,
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
    /// Used only when the client generates the pkce itself when the generate method
    /// is called.
    pub(crate) pkce: Option<ProofKeyForCodeExchange>,
    serializer: OAuthSerializer,
}

impl OpenIdCredential {
    pub fn new<T: AsRef<str>, U: IntoUrl>(
        client_id: T,
        client_secret: T,
        authorization_code: T,
        redirect_uri: U,
    ) -> AuthorizationResult<OpenIdCredential> {
        let redirect_uri_result = Url::parse(redirect_uri.as_str());
        Ok(OpenIdCredential {
            app_config: AppConfig {
                tenant_id: None,
                client_id: Uuid::try_parse(client_id.as_ref()).unwrap_or_default(),
                authority: Default::default(),
                azure_cloud_instance: Default::default(),
                extra_query_parameters: Default::default(),
                extra_header_parameters: Default::default(),
                redirect_uri: Some(redirect_uri.into_url().or(redirect_uri_result)?),
            },
            authorization_code: Some(authorization_code.as_ref().to_owned()),
            refresh_token: None,
            client_secret: client_secret.as_ref().to_owned(),
            scope: vec!["openid".to_owned()],
            code_verifier: None,
            pkce: None,
            serializer: OAuthSerializer::new(),
        })
    }

    pub fn with_refresh_token<T: AsRef<str>>(&mut self, refresh_token: T) {
        self.authorization_code = None;
        self.refresh_token = Some(refresh_token.as_ref().to_owned());
    }

    pub fn builder() -> OpenIdCredentialBuilder {
        OpenIdCredentialBuilder::new()
    }

    pub fn authorization_url_builder() -> AuthorizationResult<OpenIdAuthorizationUrlBuilder> {
        OpenIdAuthorizationUrlBuilder::new()
    }

    pub fn pkce(&self) -> Option<&ProofKeyForCodeExchange> {
        self.pkce.as_ref()
    }
}

#[async_trait]
impl TokenCredentialExecutor for OpenIdCredential {
    fn uri(&mut self, azure_authority_host: &AzureCloudInstance) -> AuthorizationResult<Url> {
        self.serializer
            .authority(azure_authority_host, &self.app_config.authority);

        if self.refresh_token.is_none() {
            let uri = self
                .serializer
                .get(OAuthParameter::TokenUrl)
                .ok_or(AF::msg_internal_err("access_token_url"))?;
            Url::parse(uri.as_str()).map_err(AF::from)
        } else {
            let uri = self
                .serializer
                .get(OAuthParameter::RefreshTokenUrl)
                .ok_or(AF::msg_internal_err("refresh_token_url"))?;
            Url::parse(uri.as_str()).map_err(AF::from)
        }
    }

    fn form_urlencode(&mut self) -> AuthorizationResult<HashMap<String, String>> {
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

#[derive(Clone)]
pub struct OpenIdCredentialBuilder {
    credential: OpenIdCredential,
}

impl OpenIdCredentialBuilder {
    fn new() -> OpenIdCredentialBuilder {
        let redirect_url = Url::parse("http://localhost").expect("Internal Error - please report");
        let mut app_config = AppConfig::new();
        app_config.redirect_uri = Some(redirect_url);
        Self {
            credential: OpenIdCredential {
                app_config,
                authorization_code: None,
                refresh_token: None,
                client_secret: String::new(),
                scope: vec!["openid".to_owned()],
                code_verifier: None,
                pkce: None,
                serializer: OAuthSerializer::new(),
            },
        }
    }

    pub(crate) fn new_with_auth_code_and_secret(
        authorization_code: impl AsRef<str>,
        client_secret: impl AsRef<str>,
        app_config: AppConfig,
    ) -> OpenIdCredentialBuilder {
        OpenIdCredentialBuilder {
            credential: OpenIdCredential {
                app_config,
                authorization_code: Some(authorization_code.as_ref().to_owned()),
                refresh_token: None,
                client_secret: client_secret.as_ref().to_owned(),
                scope: vec![],
                code_verifier: None,
                pkce: None,
                serializer: Default::default(),
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

    pub fn with_pkce(
        &mut self,
        proof_key_for_code_exchange: &ProofKeyForCodeExchange,
    ) -> &mut Self {
        self.with_code_verifier(proof_key_for_code_exchange.code_verifier.as_str());
        self
    }

    pub fn generate_pkce(&mut self) -> AuthorizationResult<&mut Self> {
        let pkce = ProofKeyForCodeExchange::generate()?;
        self.with_code_verifier(pkce.code_verifier.as_str());
        self.credential.pkce = Some(pkce);
        Ok(self)
    }

    pub fn credential(&self) -> &OpenIdCredential {
        &self.credential
    }
}

impl From<OpenIdAuthorizationUrl> for OpenIdCredentialBuilder {
    fn from(value: OpenIdAuthorizationUrl) -> Self {
        let mut builder = OpenIdCredentialBuilder::new();
        builder.credential.app_config = value.app_config;
        builder.with_scope(value.scope);

        builder
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
        let credential = OpenIdCredential::builder()
            .with_authority(Authority::TenantId("common".into()))
            .build();

        assert_eq!(credential.authority(), Authority::TenantId("common".into()))
    }

    #[test]
    fn with_tenant_id_adfs() {
        let credential = OpenIdCredential::builder()
            .with_authority(Authority::AzureDirectoryFederatedServices)
            .build();

        assert_eq!(credential.authority().as_ref(), "adfs");
    }
}
