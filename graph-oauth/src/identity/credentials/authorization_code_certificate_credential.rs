use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::credentials::app_config::AppConfig;
use crate::identity::{
    AuthCodeAuthorizationUrlParameterBuilder, AuthCodeAuthorizationUrlParameters, Authority,
    AzureCloudInstance, ConfidentialClientApplication, TokenCredentialExecutor,
    CLIENT_ASSERTION_TYPE,
};
use async_trait::async_trait;
use graph_error::{AuthorizationResult, AF};
use http::{HeaderMap, HeaderName, HeaderValue};
use reqwest::IntoUrl;
use std::collections::HashMap;
use url::Url;

#[cfg(feature = "openssl")]
use crate::oauth::X509Certificate;

credential_builder!(
    AuthorizationCodeCertificateCredentialBuilder,
    ConfidentialClientApplication
);

/// The OAuth 2.0 authorization code grant type, or auth code flow, enables a client application
/// to obtain authorized access to protected resources like web APIs. The auth code flow requires
/// a user-agent that supports redirection from the authorization server (the Microsoft
/// identity platform) back to your application. For example, a web browser, desktop, or mobile
/// application operated by a user to sign in to your app and access their data.
/// https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow
#[derive(Clone, Debug)]
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
    /// Required
    /// A space-separated list of scopes. For OpenID Connect (id_tokens), it must include the
    /// scope openid, which translates to the "Sign you in" permission in the consent UI.
    /// Optionally you may also want to include the email and profile scopes for gaining access
    /// to additional user data. You may also include other scopes in this request for requesting
    /// consent to various resources, if an access token is requested.
    pub(crate) scope: Vec<String>,
    serializer: OAuthSerializer,
}

impl AuthorizationCodeCertificateCredential {
    pub fn new<T: AsRef<str>, U: IntoUrl>(
        client_id: T,
        authorization_code: T,
        client_assertion: T,
        redirect_uri: Option<U>,
    ) -> AuthorizationResult<AuthorizationCodeCertificateCredential> {
        let redirect_uri = {
            if let Some(redirect_uri) = redirect_uri {
                redirect_uri.into_url().ok()
            } else {
                None
            }
        };

        let app_config = AppConfig {
            client_id: client_id.as_ref().to_owned(),
            tenant_id: None,
            authority: Default::default(),
            authority_url: Default::default(),
            extra_query_parameters: Default::default(),
            extra_header_parameters: Default::default(),
            redirect_uri,
        };

        Ok(AuthorizationCodeCertificateCredential {
            app_config,
            authorization_code: Some(authorization_code.as_ref().to_owned()),
            refresh_token: None,
            code_verifier: None,
            client_assertion_type: CLIENT_ASSERTION_TYPE.to_owned(),
            client_assertion: client_assertion.as_ref().to_owned(),
            scope: vec![],
            serializer: OAuthSerializer::new(),
        })
    }

    pub fn builder(
        authorization_code: impl AsRef<str>,
    ) -> AuthorizationCodeCertificateCredentialBuilder {
        AuthorizationCodeCertificateCredentialBuilder::new_with_auth_code(
            Default::default(),
            authorization_code,
        )
    }

    pub fn authorization_url_builder() -> AuthCodeAuthorizationUrlParameterBuilder {
        AuthCodeAuthorizationUrlParameterBuilder::new()
    }
}

#[async_trait]
impl TokenCredentialExecutor for AuthorizationCodeCertificateCredential {
    fn uri(&mut self, azure_authority_host: &AzureCloudInstance) -> AuthorizationResult<Url> {
        self.serializer
            .authority(azure_authority_host, &self.authority());

        let uri = self
            .serializer
            .get(OAuthParameter::TokenUrl)
            .ok_or(AF::msg_internal_err("token_url"))?;
        Url::parse(uri.as_str()).map_err(AF::from)
    }

    fn form_urlencode(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        let client_id = self.app_config.client_id.trim();
        if client_id.is_empty() {
            return AF::result(OAuthParameter::ClientId);
        }

        if self.client_assertion.trim().is_empty() {
            return AF::result(OAuthParameter::ClientAssertion);
        }

        if self.client_assertion_type.trim().is_empty() {
            self.client_assertion_type = CLIENT_ASSERTION_TYPE.to_owned();
        }

        self.serializer
            .client_id(client_id)
            .client_assertion(self.client_assertion.as_str())
            .client_assertion_type(self.client_assertion_type.as_str())
            .extend_scopes(self.scope.clone());

        if let Some(redirect_uri) = self.app_config.redirect_uri.as_ref() {
            self.serializer.redirect_uri(redirect_uri.as_str());
        }

        if let Some(code_verifier) = self.code_verifier.as_ref() {
            self.serializer.code_verifier(code_verifier.as_ref());
        }

        if let Some(refresh_token) = self.refresh_token.as_ref() {
            if refresh_token.trim().is_empty() {
                return AF::msg_result(
                    OAuthParameter::RefreshToken.alias(),
                    "refresh_token is empty - cannot be an empty string",
                );
            }

            self.serializer
                .refresh_token(refresh_token.as_ref())
                .grant_type("refresh_token");

            return self.serializer.as_credential_map(
                vec![OAuthParameter::Scope],
                vec![
                    OAuthParameter::RefreshToken,
                    OAuthParameter::ClientId,
                    OAuthParameter::GrantType,
                    OAuthParameter::ClientAssertion,
                    OAuthParameter::ClientAssertionType,
                ],
            );
        } else if let Some(authorization_code) = self.authorization_code.as_ref() {
            if authorization_code.trim().is_empty() {
                return AF::msg_result(
                    OAuthParameter::AuthorizationCode.alias(),
                    "authorization_code is empty - cannot be an empty string",
                );
            }

            self.serializer
                .authorization_code(authorization_code.as_str())
                .grant_type("authorization_code");

            return self.serializer.as_credential_map(
                vec![OAuthParameter::Scope, OAuthParameter::CodeVerifier],
                vec![
                    OAuthParameter::AuthorizationCode,
                    OAuthParameter::ClientId,
                    OAuthParameter::GrantType,
                    OAuthParameter::RedirectUri,
                    OAuthParameter::ClientAssertion,
                    OAuthParameter::ClientAssertionType,
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

    fn client_id(&self) -> &String {
        &self.app_config.client_id
    }

    fn app_config(&self) -> &AppConfig {
        &self.app_config
    }

    fn authority(&self) -> Authority {
        self.app_config.authority.clone()
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
                app_config: Default::default(),
                authorization_code: None,
                refresh_token: None,
                code_verifier: None,
                client_assertion_type: CLIENT_ASSERTION_TYPE.to_owned(),
                client_assertion: String::new(),
                scope: vec![],
                serializer: OAuthSerializer::new(),
            },
        }
    }

    pub(crate) fn new_with_auth_code(
        app_config: AppConfig,
        authorization_code: impl AsRef<str>,
    ) -> AuthorizationCodeCertificateCredentialBuilder {
        Self {
            credential: AuthorizationCodeCertificateCredential {
                app_config,
                authorization_code: Some(authorization_code.as_ref().to_owned()),
                refresh_token: None,
                code_verifier: None,
                client_assertion_type: CLIENT_ASSERTION_TYPE.to_owned(),
                client_assertion: String::new(),
                scope: vec![],
                serializer: OAuthSerializer::new(),
            },
        }
    }

    pub(crate) fn new_with_auth_code_and_assertion(
        app_config: AppConfig,
        authorization_code: impl AsRef<str>,
        assertion: impl AsRef<str>,
    ) -> AuthorizationCodeCertificateCredentialBuilder {
        Self {
            credential: AuthorizationCodeCertificateCredential {
                app_config,
                authorization_code: Some(authorization_code.as_ref().to_owned()),
                refresh_token: None,
                code_verifier: None,
                client_assertion_type: CLIENT_ASSERTION_TYPE.to_owned(),
                client_assertion: assertion.as_ref().to_owned(),
                scope: vec![],
                serializer: OAuthSerializer::new(),
            },
        }
    }

    #[cfg(feature = "openssl")]
    pub(crate) fn new_with_auth_code_and_x509(
        app_config: AppConfig,
        authorization_code: impl AsRef<str>,
        x509: &X509Certificate,
    ) -> anyhow::Result<AuthorizationCodeCertificateCredentialBuilder> {
        let mut builder = Self {
            credential: AuthorizationCodeCertificateCredential {
                app_config,
                authorization_code: Some(authorization_code.as_ref().to_owned()),
                refresh_token: None,
                code_verifier: None,
                client_assertion_type: CLIENT_ASSERTION_TYPE.to_owned(),
                client_assertion: String::new(),
                scope: vec![],
                serializer: OAuthSerializer::new(),
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

    pub fn with_redirect_uri(&mut self, redirect_uri: impl IntoUrl) -> anyhow::Result<&mut Self> {
        self.credential.app_config.redirect_uri = Some(redirect_uri.into_url()?);
        Ok(self)
    }

    pub fn with_code_verifier<T: AsRef<str>>(&mut self, code_verifier: T) -> &mut Self {
        self.credential.code_verifier = Some(code_verifier.as_ref().to_owned());
        self
    }

    #[cfg(feature = "openssl")]
    pub fn with_x509(
        &mut self,
        certificate_assertion: &X509Certificate,
    ) -> anyhow::Result<&mut Self> {
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

impl From<AuthCodeAuthorizationUrlParameters> for AuthorizationCodeCertificateCredentialBuilder {
    fn from(value: AuthCodeAuthorizationUrlParameters) -> Self {
        let mut builder = AuthorizationCodeCertificateCredentialBuilder::new();
        let _ = builder.with_redirect_uri(value.redirect_uri);
        builder
            .with_scope(value.scope)
            .with_client_id(value.client_id)
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

impl From<AuthorizationCodeCertificateCredentialBuilder>
    for AuthorizationCodeCertificateCredential
{
    fn from(builder: AuthorizationCodeCertificateCredentialBuilder) -> Self {
        builder.credential
    }
}
