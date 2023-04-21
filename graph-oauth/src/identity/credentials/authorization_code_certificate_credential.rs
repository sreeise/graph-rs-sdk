use crate::auth::{OAuth, OAuthCredential};
use crate::grants::GrantType;
use crate::identity::form_credential::FormCredential;
use crate::identity::{
    AuthCodeAuthorizationUrl, Authority, AuthorizationSerializer, AzureAuthorityHost,
};
use graph_error::{AuthorizationFailure, AuthorizationResult, GraphFailure, GraphResult};
use std::collections::HashMap;
use url::Url;

#[derive(Clone)]
pub struct AuthorizationCodeCertificateCredential {
    /// The authorization code obtained from a call to authorize. The code should be obtained with all required scopes.
    pub(crate) authorization_code: Option<String>,
    /// The refresh token needed to make an access token request using a refresh token.
    /// Do not include an authorization code when using a refresh token.
    pub(crate) refresh_token: Option<String>,
    /// The client (application) ID of the service principal
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) redirect_uri: String,
    pub(crate) code_verifier: Option<String>,
    pub(crate) client_assertion_type: String,
    pub(crate) client_assertion: String,
    pub(crate) scopes: Vec<String>,
    /// The Azure Active Directory tenant (directory) Id of the service principal.
    pub(crate) authority: Authority,
    serializer: OAuth,
}

impl AuthorizationCodeCertificateCredential {
    pub fn new<T: AsRef<str>>(
        client_id: T,
        client_secret: T,
        authorization_code: T,
        redirect_uri: T,
        client_assertion: T,
    ) -> AuthorizationCodeCertificateCredential {
        AuthorizationCodeCertificateCredential {
            authorization_code: Some(authorization_code.as_ref().to_owned()),
            refresh_token: None,
            client_id: client_id.as_ref().to_owned(),
            client_secret: client_secret.as_ref().to_owned(),
            redirect_uri: redirect_uri.as_ref().to_owned(),
            code_verifier: None,
            client_assertion_type: "urn:ietf:params:oauth:client-assertion-type:jwt-bearer"
                .to_owned(),
            client_assertion: client_assertion.as_ref().to_owned(),
            scopes: vec![],
            authority: Default::default(),
            serializer: OAuth::new(),
        }
    }

    pub fn grant_type(&self) -> GrantType {
        GrantType::AuthorizationCode
    }

    pub fn builder() -> AuthorizationCodeCertificateCredentialBuilder {
        AuthorizationCodeCertificateCredentialBuilder::new()
    }
}

impl AuthorizationSerializer for AuthorizationCodeCertificateCredential {
    fn uri(&mut self, azure_authority_host: &AzureAuthorityHost) -> GraphResult<Url> {
        self.serializer
            .authority(azure_authority_host, &self.authority);

        let uri = self
            .serializer
            .get_or_else(OAuthCredential::AccessTokenUrl)?;
        Url::parse(uri.as_str()).map_err(GraphFailure::from)
    }

    fn form(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        if self.authorization_code.is_some() && self.refresh_token.is_some() {
            return AuthorizationFailure::required_value_msg(
                &format!(
                    "{} or {}",
                    OAuthCredential::AuthorizationCode.alias(),
                    OAuthCredential::RefreshToken.alias()
                ),
                Some("Authorization code and refresh token cannot be set at the same time - choose one or the other"),
            );
        }

        if self.client_id.trim().is_empty() {
            return AuthorizationFailure::required_value_msg(
                OAuthCredential::ClientId.alias(),
                None,
            );
        }

        if self.client_secret.trim().is_empty() {
            return AuthorizationFailure::required_value_msg(
                OAuthCredential::ClientSecret.alias(),
                None,
            );
        }

        if self.client_assertion.trim().is_empty() {
            return AuthorizationFailure::required_value_msg(
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
            .client_secret(self.client_secret.as_str())
            .redirect_uri(self.redirect_uri.as_str())
            .client_assertion(self.client_assertion.as_str())
            .client_assertion_type(self.client_assertion_type.as_str())
            .extend_scopes(self.scopes.clone());

        if let Some(code_verifier) = self.code_verifier.as_ref() {
            self.serializer.code_verifier(code_verifier.as_ref());
        }

        if let Some(refresh_token) = self.refresh_token.as_ref() {
            if refresh_token.trim().is_empty() {
                return AuthorizationFailure::required_value_msg(
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
                return AuthorizationFailure::required_value_msg(
                    OAuthCredential::AuthorizationCode.alias(),
                    None,
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

        AuthorizationFailure::required_value_msg(
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
                client_secret: String::new(),
                redirect_uri: String::new(),
                code_verifier: None,
                client_assertion_type: String::new(),
                client_assertion: String::new(),
                scopes: vec![],
                authority: Default::default(),
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

    pub fn with_client_secret<T: AsRef<str>>(&mut self, client_secret: T) -> &mut Self {
        self.credential.client_secret = client_secret.as_ref().to_owned();
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
        self.credential.scopes = scopes.into_iter().map(|s| s.to_string()).collect();
        self
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
