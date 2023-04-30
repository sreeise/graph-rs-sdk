use crate::auth::{OAuth, OAuthCredential};
use crate::identity::form_credential::FormCredential;
use crate::identity::{
    Authority, AuthorizationSerializer, AzureAuthorityHost, TokenCredentialOptions, TokenRequest,
};
use async_trait::async_trait;
use graph_error::{AuthorizationFailure, AuthorizationResult};
use std::collections::HashMap;
use url::Url;

#[cfg(feature = "openssl")]
use crate::identity::ClientAssertion;
use crate::oauth::ClientCredentialsAuthorizationUrlBuilder;

#[derive(Clone)]
#[allow(dead_code)]
pub struct ClientCertificateCredential {
    /// The client (application) ID of the service principal
    pub(crate) client_id: String,
    /// The value passed for the scope parameter in this request should be the resource
    /// identifier (application ID URI) of the resource you want, affixed with the .default
    /// suffix. For the Microsoft Graph example, the value is https://graph.microsoft.com/.default.
    /// Default is https://graph.microsoft.com/.default.
    pub(crate) scopes: Vec<String>,
    pub(crate) authority: Authority,
    pub(crate) client_assertion_type: String,
    pub(crate) client_assertion: String,
    pub(crate) refresh_token: Option<String>,
    pub(crate) token_credential_options: TokenCredentialOptions,
    serializer: OAuth,
}

impl ClientCertificateCredential {
    pub fn builder() -> ClientCertificateCredentialBuilder {
        ClientCertificateCredentialBuilder::new()
    }

    pub fn authorization_url_builder() -> ClientCredentialsAuthorizationUrlBuilder {
        ClientCredentialsAuthorizationUrlBuilder::new()
    }
}

#[async_trait]
impl TokenRequest for ClientCertificateCredential {
    fn token_credential_options(&self) -> &TokenCredentialOptions {
        &self.token_credential_options
    }
}

impl AuthorizationSerializer for ClientCertificateCredential {
    fn uri(&mut self, azure_authority_host: &AzureAuthorityHost) -> AuthorizationResult<Url> {
        self.serializer
            .authority(azure_authority_host, &self.authority);

        let uri = self.serializer.get(OAuthCredential::AccessTokenUrl).ok_or(
            AuthorizationFailure::required_value_msg("access_token_url", Some("Internal Error")),
        )?;
        Url::parse(uri.as_str()).map_err(AuthorizationFailure::from)
    }

    fn form(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        if self.client_id.trim().is_empty() {
            return AuthorizationFailure::required_value_result(OAuthCredential::ClientId.alias());
        }

        if self.client_assertion.trim().is_empty() {
            return AuthorizationFailure::required_value_result(
                OAuthCredential::ClientAssertion.alias(),
            );
        }

        if self.client_assertion_type.trim().is_empty() {
            self.client_assertion_type =
                "urn:ietf:params:oauth:client-assertion-type:jwt-bearer".to_owned();
        }

        self.serializer
            .client_id(self.client_id.as_str())
            .client_assertion(self.client_assertion.as_str())
            .client_assertion_type(self.client_assertion_type.as_str());

        if self.scopes.is_empty() {
            self.serializer
                .add_scope("https://graph.microsoft.com/.default");
        }

        return if let Some(refresh_token) = self.refresh_token.as_ref() {
            if refresh_token.trim().is_empty() {
                return AuthorizationFailure::required_value_msg_result(
                    OAuthCredential::RefreshToken.alias(),
                    Some("refresh_token is set but is empty"),
                );
            }

            self.serializer
                .refresh_token(refresh_token.as_ref())
                .grant_type("refresh_token");

            self.serializer.authorization_form(vec![
                FormCredential::Required(OAuthCredential::RefreshToken),
                FormCredential::Required(OAuthCredential::ClientId),
                FormCredential::Required(OAuthCredential::GrantType),
                FormCredential::NotRequired(OAuthCredential::Scope),
                FormCredential::Required(OAuthCredential::ClientAssertion),
                FormCredential::Required(OAuthCredential::ClientAssertionType),
            ])
        } else {
            self.serializer.grant_type("client_credentials");
            self.serializer.authorization_form(vec![
                FormCredential::Required(OAuthCredential::ClientId),
                FormCredential::Required(OAuthCredential::GrantType),
                FormCredential::NotRequired(OAuthCredential::Scope),
                FormCredential::Required(OAuthCredential::ClientAssertion),
                FormCredential::Required(OAuthCredential::ClientAssertionType),
            ])
        };
    }
}

pub struct ClientCertificateCredentialBuilder {
    credential: ClientCertificateCredential,
}

impl ClientCertificateCredentialBuilder {
    fn new() -> ClientCertificateCredentialBuilder {
        ClientCertificateCredentialBuilder {
            credential: ClientCertificateCredential {
                client_id: String::new(),
                scopes: vec![],
                authority: Default::default(),
                client_assertion_type: "urn:ietf:params:oauth:client-assertion-type:jwt-bearer"
                    .to_owned(),
                client_assertion: String::new(),
                refresh_token: None,
                token_credential_options: TokenCredentialOptions::default(),
                serializer: OAuth::new(),
            },
        }
    }

    pub fn with_client_id<T: AsRef<str>>(&mut self, client_id: T) -> &mut Self {
        self.credential.client_id = client_id.as_ref().to_owned();
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

    pub fn with_refresh_token<T: AsRef<str>>(&mut self, refresh_token: T) -> &mut Self {
        self.credential.refresh_token = Some(refresh_token.as_ref().to_owned());
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

    /// Defaults to "https://graph.microsoft.com/.default"
    pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scopes: I) -> &mut Self {
        self.credential.scopes = scopes.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_token_credential_options(
        &mut self,
        token_credential_options: TokenCredentialOptions,
    ) {
        self.credential.token_credential_options = token_credential_options;
    }

    pub fn build(&self) -> ClientCertificateCredential {
        self.credential.clone()
    }
}

impl From<ClientCertificateCredential> for ClientCertificateCredentialBuilder {
    fn from(credential: ClientCertificateCredential) -> Self {
        ClientCertificateCredentialBuilder { credential }
    }
}
