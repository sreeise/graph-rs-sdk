use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::{
    Authority, AuthorizationSerializer, AzureAuthorityHost, CredentialBuilder,
    OpenIdAuthorizationUrl, ProofKeyForCodeExchange, TokenCredential, TokenCredentialOptions,
    TokenRequest,
};
use crate::oauth::OpenIdAuthorizationUrlBuilder;
use async_trait::async_trait;
use graph_error::{AuthorizationResult, AF};
use reqwest::IntoUrl;
use std::collections::HashMap;
use url::Url;

credential_builder_impl!(OpenIdCredentialBuilder, OpenIdCredential);

/// OpenID Connect (OIDC) extends the OAuth 2.0 authorization protocol for use as an additional
/// authentication protocol. You can use OIDC to enable single sign-on (SSO) between your
/// OAuth-enabled applications by using a security token called an ID token.
/// https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc
#[derive(Clone)]
pub struct OpenIdCredential {
    /// Required unless requesting a refresh token
    /// The authorization code obtained from a call to authorize.
    /// The code should be obtained with all required scopes.
    pub(crate) authorization_code: Option<String>,
    /// Required when requesting a new access token using a refresh token
    /// The refresh token needed to make an access token request using a refresh token.
    /// Do not include an authorization code when using a refresh token.
    pub(crate) refresh_token: Option<String>,
    /// Required.
    /// The Application (client) ID that the Azure portal - App registrations page assigned
    /// to your app
    pub(crate) client_id: String,
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
    pub(crate) redirect_uri: Url,
    /// A space-separated list of scopes. The scopes must all be from a single resource,
    /// along with OIDC scopes (profile, openid, email). For more information, see Permissions
    /// and consent in the Microsoft identity platform. This parameter is a Microsoft extension
    /// to the authorization code flow, intended to allow apps to declare the resource they want
    /// the token for during token redemption.
    pub(crate) scope: Vec<String>,
    /// The Azure Active Directory tenant (directory) Id of the service principal.
    pub(crate) authority: Authority,
    /// The same code_verifier that was used to obtain the authorization_code.
    /// Required if PKCE was used in the authorization code grant request. For more information,
    /// see the PKCE RFC https://datatracker.ietf.org/doc/html/rfc7636.
    pub(crate) code_verifier: Option<String>,
    pub(crate) token_credential_options: TokenCredentialOptions,
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
            authorization_code: Some(authorization_code.as_ref().to_owned()),
            refresh_token: None,
            client_id: client_id.as_ref().to_owned(),
            client_secret: client_secret.as_ref().to_owned(),
            redirect_uri: redirect_uri.into_url().or(redirect_uri_result)?,
            scope: vec!["openid".to_owned()],
            authority: Default::default(),
            code_verifier: None,
            token_credential_options: TokenCredentialOptions::default(),
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

    pub fn authorization_url_builder() -> anyhow::Result<OpenIdAuthorizationUrlBuilder> {
        OpenIdAuthorizationUrlBuilder::new()
    }
}

#[async_trait]
impl TokenRequest for OpenIdCredential {
    fn token_credential_options(&self) -> &TokenCredentialOptions {
        &self.token_credential_options
    }
}

impl AuthorizationSerializer for OpenIdCredential {
    fn uri(&mut self, azure_authority_host: &AzureAuthorityHost) -> AuthorizationResult<Url> {
        self.serializer
            .authority(azure_authority_host, &self.authority);

        if self.refresh_token.is_none() {
            let uri = self
                .serializer
                .get(OAuthParameter::AccessTokenUrl)
                .ok_or(AF::msg_err("access_token_url", "Internal Error"))?;
            Url::parse(uri.as_str()).map_err(AF::from)
        } else {
            let uri = self
                .serializer
                .get(OAuthParameter::RefreshTokenUrl)
                .ok_or(AF::msg_err("refresh_token_url", "Internal Error"))?;
            Url::parse(uri.as_str()).map_err(AF::from)
        }
    }

    fn form_urlencode(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        if self.client_id.trim().is_empty() {
            return AF::result(OAuthParameter::ClientId.alias());
        }

        if self.client_secret.trim().is_empty() {
            return AF::result(OAuthParameter::ClientSecret.alias());
        }

        self.serializer
            .client_id(self.client_id.as_str())
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

            self.serializer
                .authorization_code(authorization_code.as_ref())
                .grant_type("authorization_code")
                .redirect_uri(self.redirect_uri.as_str());

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

    fn basic_auth(&self) -> Option<(String, String)> {
        Some((self.client_id.clone(), self.client_secret.clone()))
    }
}

impl TokenCredential for OpenIdCredential {
    fn client_id(&self) -> &String {
        &self.client_id
    }
}

#[derive(Clone)]
pub struct OpenIdCredentialBuilder {
    credential: OpenIdCredential,
}

impl OpenIdCredentialBuilder {
    fn new() -> OpenIdCredentialBuilder {
        Self {
            credential: OpenIdCredential {
                authorization_code: None,
                refresh_token: None,
                client_id: String::with_capacity(32),
                client_secret: String::new(),
                redirect_uri: Url::parse("http://localhost")
                    .expect("Internal Error - please report"),
                scope: vec![],
                authority: Default::default(),
                code_verifier: None,
                token_credential_options: TokenCredentialOptions::default(),
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
        self.credential.authorization_code = None;
        self.credential.refresh_token = Some(refresh_token.as_ref().to_owned());
        self
    }

    /// Defaults to http://localhost
    pub fn with_redirect_uri<U: IntoUrl>(&mut self, redirect_uri: U) -> anyhow::Result<&mut Self> {
        self.credential.redirect_uri = redirect_uri.into_url()?;
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

    pub fn with_proof_key_for_code_exchange(
        &mut self,
        proof_key_for_code_exchange: &ProofKeyForCodeExchange,
    ) -> &mut Self {
        self.with_code_verifier(proof_key_for_code_exchange.code_verifier.as_str());
        self
    }
}

impl From<OpenIdAuthorizationUrl> for OpenIdCredentialBuilder {
    fn from(value: OpenIdAuthorizationUrl) -> Self {
        let mut builder = OpenIdCredentialBuilder::new();
        if let Some(redirect_uri) = value.redirect_uri.as_ref() {
            let _ = builder.with_redirect_uri(redirect_uri);
        }
        builder
            .with_scope(value.scope)
            .with_client_secret(value.client_id)
            .with_authority(value.authority);

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

        assert_eq!(credential.authority, Authority::TenantId("common".into()))
    }

    #[test]
    fn with_tenant_id_adfs() {
        let credential = OpenIdCredential::builder()
            .with_authority(Authority::AzureDirectoryFederatedServices)
            .build();

        assert_eq!(credential.authority.as_ref(), "adfs");
    }
}
