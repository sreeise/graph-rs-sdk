use crate::auth::{OAuth, OAuthCredential};
use crate::identity::form_credential::FormCredential;
use crate::identity::{
    AuthCodeAuthorizationUrl, Authority, AuthorizationSerializer, AzureAuthorityHost,
    TokenCredentialOptions, TokenRequest,
};
use async_trait::async_trait;
use graph_error::{AuthorizationFailure, AuthorizationResult};
use std::collections::HashMap;
use url::Url;

#[cfg(feature = "openssl")]
use crate::oauth::ClientAssertion;

/// The OAuth 2.0 authorization code grant type, or auth code flow, enables a client application
/// to obtain authorized access to protected resources like web APIs. The auth code flow requires
/// a user-agent that supports redirection from the authorization server (the Microsoft
/// identity platform) back to your application. For example, a web browser, desktop, or mobile
/// application operated by a user to sign in to your app and access their data.
/// https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow
#[derive(Clone)]
pub struct AuthorizationCodeCertificateCredential {
    /// The authorization code obtained from a call to authorize. The code should be obtained with all required scopes.
    pub(crate) authorization_code: Option<String>,
    /// The refresh token needed to make an access token request using a refresh token.
    /// Do not include an authorization code when using a refresh token.
    pub(crate) refresh_token: Option<String>,
    /// Required.
    /// The Application (client) ID that the Azure portal - App registrations page assigned
    /// to your app
    pub(crate) client_id: String,
    /// Optional
    /// The redirect_uri of your app, where authentication responses can be sent and received
    /// by your app. It must exactly match one of the redirect_uris you registered in the portal,
    /// except it must be URL-encoded.
    pub(crate) redirect_uri: String,
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
    /// Required
    /// A space-separated list of scopes. For OpenID Connect (id_tokens), it must include the
    /// scope openid, which translates to the "Sign you in" permission in the consent UI.
    /// Optionally you may also want to include the email and profile scopes for gaining access
    /// to additional user data. You may also include other scopes in this request for requesting
    /// consent to various resources, if an access token is requested.
    pub(crate) scope: Vec<String>,
    /// The Azure Active Directory tenant (directory) Id of the service principal.
    pub(crate) authority: Authority,
    pub(crate) token_credential_options: TokenCredentialOptions,
    serializer: OAuth,
}

impl AuthorizationCodeCertificateCredential {
    pub fn new<T: AsRef<str>>(
        client_id: T,
        _client_secret: T,
        authorization_code: T,
        redirect_uri: T,
        client_assertion: T,
    ) -> AuthorizationCodeCertificateCredential {
        AuthorizationCodeCertificateCredential {
            authorization_code: Some(authorization_code.as_ref().to_owned()),
            refresh_token: None,
            client_id: client_id.as_ref().to_owned(),
            redirect_uri: redirect_uri.as_ref().to_owned(),
            code_verifier: None,
            client_assertion_type: "urn:ietf:params:oauth:client-assertion-type:jwt-bearer"
                .to_owned(),
            client_assertion: client_assertion.as_ref().to_owned(),
            scope: vec![],
            authority: Default::default(),
            token_credential_options: TokenCredentialOptions::default(),
            serializer: OAuth::new(),
        }
    }

    pub fn builder() -> AuthorizationCodeCertificateCredentialBuilder {
        AuthorizationCodeCertificateCredentialBuilder::new()
    }
}

#[async_trait]
impl TokenRequest for AuthorizationCodeCertificateCredential {
    fn azure_authority_host(&self) -> &AzureAuthorityHost {
        &self.token_credential_options.azure_authority_host
    }
}

impl AuthorizationSerializer for AuthorizationCodeCertificateCredential {
    fn uri(&mut self, azure_authority_host: &AzureAuthorityHost) -> AuthorizationResult<Url> {
        self.serializer
            .authority(azure_authority_host, &self.authority);

        let uri = self.serializer.get(OAuthCredential::AccessTokenUrl).ok_or(
            AuthorizationFailure::required_value_msg("access_token_url", Some("Internal Error")),
        )?;
        Url::parse(uri.as_str()).map_err(AuthorizationFailure::from)
    }

    fn form(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        if self.authorization_code.is_some() && self.refresh_token.is_some() {
            return AuthorizationFailure::required_value_msg_result(
                &format!(
                    "{} or {}",
                    OAuthCredential::AuthorizationCode.alias(),
                    OAuthCredential::RefreshToken.alias()
                ),
                Some("Authorization code and refresh token cannot be set at the same time - choose one or the other"),
            );
        }

        if self.client_id.trim().is_empty() {
            return AuthorizationFailure::required_value_msg_result(
                OAuthCredential::ClientId.alias(),
                None,
            );
        }

        if self.client_assertion.trim().is_empty() {
            return AuthorizationFailure::required_value_msg_result(
                OAuthCredential::ClientAssertion.alias(),
                None,
            );
        }

        if self.client_assertion_type.trim().is_empty() {
            self.client_assertion_type =
                "urn:ietf:params:oauth:client-assertion-type:jwt-bearer".to_owned();
        }

        self.serializer
            .client_id(self.client_id.as_str())
            .redirect_uri(self.redirect_uri.as_str())
            .client_assertion(self.client_assertion.as_str())
            .client_assertion_type(self.client_assertion_type.as_str())
            .extend_scopes(self.scope.clone());

        if let Some(code_verifier) = self.code_verifier.as_ref() {
            self.serializer.code_verifier(code_verifier.as_ref());
        }

        if let Some(refresh_token) = self.refresh_token.as_ref() {
            if refresh_token.trim().is_empty() {
                return AuthorizationFailure::required_value_msg_result(
                    OAuthCredential::RefreshToken.alias(),
                    None,
                );
            }

            self.serializer
                .refresh_token(refresh_token.as_ref())
                .grant_type("refresh_token");

            return self.serializer.authorization_form(vec![
                FormCredential::Required(OAuthCredential::RefreshToken),
                FormCredential::Required(OAuthCredential::ClientId),
                FormCredential::Required(OAuthCredential::GrantType),
                FormCredential::NotRequired(OAuthCredential::Scope),
                FormCredential::Required(OAuthCredential::ClientAssertion),
                FormCredential::Required(OAuthCredential::ClientAssertionType),
            ]);
        } else if let Some(authorization_code) = self.authorization_code.as_ref() {
            if authorization_code.trim().is_empty() {
                return AuthorizationFailure::required_value_msg_result(
                    OAuthCredential::AuthorizationCode.alias(),
                    Some("refresh_token is set but is empty"),
                );
            }

            self.serializer
                .authorization_code(authorization_code.as_str())
                .grant_type("authorization_code");

            return self.serializer.authorization_form(vec![
                FormCredential::Required(OAuthCredential::AuthorizationCode),
                FormCredential::Required(OAuthCredential::ClientId),
                FormCredential::Required(OAuthCredential::RedirectUri),
                FormCredential::Required(OAuthCredential::GrantType),
                FormCredential::NotRequired(OAuthCredential::Scope),
                FormCredential::NotRequired(OAuthCredential::CodeVerifier),
                FormCredential::Required(OAuthCredential::ClientAssertion),
                FormCredential::Required(OAuthCredential::ClientAssertionType),
            ]);
        }

        AuthorizationFailure::required_value_msg_result(
            &format!(
                "{} or {}",
                OAuthCredential::AuthorizationCode.alias(),
                OAuthCredential::RefreshToken.alias()
            ),
            Some("Either authorization code or refresh token is required"),
        )
    }
}

#[derive(Clone)]
pub struct AuthorizationCodeCertificateCredentialBuilder {
    credential: AuthorizationCodeCertificateCredential,
}

impl AuthorizationCodeCertificateCredentialBuilder {
    fn new() -> AuthorizationCodeCertificateCredentialBuilder {
        Self {
            credential: AuthorizationCodeCertificateCredential {
                authorization_code: None,
                refresh_token: None,
                client_id: String::new(),
                redirect_uri: String::new(),
                code_verifier: None,
                client_assertion_type: String::new(),
                client_assertion: String::new(),
                scope: vec![],
                authority: Default::default(),
                token_credential_options: TokenCredentialOptions::default(),
                serializer: OAuth::new(),
            },
        }
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

    pub fn with_redirect_uri<T: AsRef<str>>(&mut self, redirect_uri: T) -> &mut Self {
        self.credential.redirect_uri = redirect_uri.as_ref().to_owned();
        self
    }

    pub fn with_client_id<T: AsRef<str>>(&mut self, client_id: T) -> &mut Self {
        self.credential.client_id = client_id.as_ref().to_owned();
        self
    }
    /// Convenience method. Same as calling [with_authority(Authority::TenantId("tenant_id"))]
    pub fn with_tenant<T: AsRef<str>>(&mut self, tenant: T) -> &mut Self {
        self.credential.authority = Authority::TenantId(tenant.as_ref().to_owned());
        self
    }

    pub fn with_authority<T: Into<Authority>>(&mut self, authority: T) -> &mut Self {
        self.credential.authority = authority.into();
        self
    }

    pub fn with_code_verifier<T: AsRef<str>>(&mut self, code_verifier: T) -> &mut Self {
        self.credential.code_verifier = Some(code_verifier.as_ref().to_owned());
        self
    }

    #[cfg(feature = "openssl")]
    pub fn with_certificate(
        &mut self,
        certificate_assertion: &ClientAssertion,
    ) -> anyhow::Result<&mut Self> {
        self.with_client_assertion(certificate_assertion.sign()?);
        self.with_client_assertion_type("urn:ietf:params:oauth:client-assertion-type:jwt-bearer");
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

    pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scopes: I) -> &mut Self {
        self.credential.scope = scopes.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_token_credential_options(
        &mut self,
        token_credential_options: TokenCredentialOptions,
    ) {
        self.credential.token_credential_options = token_credential_options;
    }

    pub fn build(&self) -> AuthorizationCodeCertificateCredential {
        self.credential.clone()
    }
}

impl From<AuthCodeAuthorizationUrl> for AuthorizationCodeCertificateCredentialBuilder {
    fn from(value: AuthCodeAuthorizationUrl) -> Self {
        let mut builder = AuthorizationCodeCertificateCredentialBuilder::new();
        builder
            .with_scope(value.scope)
            .with_client_id(value.client_id)
            .with_redirect_uri(value.redirect_uri)
            .with_authority(value.authority);

        builder
    }
}

impl From<AuthorizationCodeCertificateCredential>
    for AuthorizationCodeCertificateCredentialBuilder
{
    fn from(credential: AuthorizationCodeCertificateCredential) -> Self {
        AuthorizationCodeCertificateCredentialBuilder { credential }
    }
}
