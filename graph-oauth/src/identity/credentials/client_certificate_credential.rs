use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::{
    Authority, AuthorizationSerializer, AzureAuthorityHost, CredentialBuilder,
    TokenCredentialOptions, TokenRequest,
};
use async_trait::async_trait;
use graph_error::{AuthorizationFailure, AuthorizationResult};
use std::collections::HashMap;
use url::Url;

#[cfg(feature = "openssl")]
use crate::identity::X509Certificate;
use crate::oauth::ClientCredentialsAuthorizationUrlBuilder;

pub(crate) static CLIENT_ASSERTION_TYPE: &str =
    "urn:ietf:params:oauth:client-assertion-type:jwt-bearer";

credential_builder_impl!(
    ClientCertificateCredentialBuilder,
    ClientCertificateCredential
);

/// https://learn.microsoft.com/en-us/azure/active-directory/develop/active-directory-certificate-credentials
#[derive(Clone)]
#[allow(dead_code)]
pub struct ClientCertificateCredential {
    /// The client (application) ID of the service principal
    pub(crate) client_id: String,
    /// The value passed for the scope parameter in this request should be the resource
    /// identifier (application ID URI) of the resource you want, affixed with the .default
    /// suffix. For the Microsoft Graph example, the value is https://graph.microsoft.com/.default.
    /// Default is https://graph.microsoft.com/.default.
    pub(crate) scope: Vec<String>,
    pub(crate) authority: Authority,
    pub(crate) client_assertion_type: String,
    pub(crate) client_assertion: String,
    pub(crate) refresh_token: Option<String>,
    pub(crate) token_credential_options: TokenCredentialOptions,
    serializer: OAuthSerializer,
}

impl ClientCertificateCredential {
    pub fn new<T: AsRef<str>>(client_id: T, client_assertion: T) -> ClientCertificateCredential {
        ClientCertificateCredential {
            client_id: client_id.as_ref().to_owned(),
            scope: vec!["https://graph.microsoft.com/.default".into()],
            authority: Default::default(),
            client_assertion_type: CLIENT_ASSERTION_TYPE.to_owned(),
            client_assertion: client_assertion.as_ref().to_owned(),
            refresh_token: None,
            token_credential_options: Default::default(),
            serializer: Default::default(),
        }
    }

    pub fn with_refresh_token<T: AsRef<str>>(&mut self, refresh_token: T) -> &mut Self {
        self.refresh_token = Some(refresh_token.as_ref().to_owned());
        self
    }

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

        if self.refresh_token.is_none() {
            let uri = self.serializer.get(OAuthParameter::AccessTokenUrl).ok_or(
                AuthorizationFailure::msg_err("access_token_url", "Internal Error"),
            )?;
            Url::parse(uri.as_str()).map_err(AuthorizationFailure::from)
        } else {
            let uri = self.serializer.get(OAuthParameter::RefreshTokenUrl).ok_or(
                AuthorizationFailure::msg_err("refresh_token_url", "Internal Error"),
            )?;
            Url::parse(uri.as_str()).map_err(AuthorizationFailure::from)
        }
    }

    fn form_urlencode(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        if self.client_id.trim().is_empty() {
            return AuthorizationFailure::result(OAuthParameter::ClientId.alias());
        }

        if self.client_assertion.trim().is_empty() {
            return AuthorizationFailure::result(OAuthParameter::ClientAssertion.alias());
        }

        if self.client_assertion_type.trim().is_empty() {
            self.client_assertion_type = CLIENT_ASSERTION_TYPE.to_owned();
        }

        self.serializer
            .client_id(self.client_id.as_str())
            .client_assertion(self.client_assertion.as_str())
            .client_assertion_type(self.client_assertion_type.as_str());

        if self.scope.is_empty() {
            self.serializer
                .add_scope("https://graph.microsoft.com/.default");
        }

        return if let Some(refresh_token) = self.refresh_token.as_ref() {
            if refresh_token.trim().is_empty() {
                return AuthorizationFailure::msg_result(
                    OAuthParameter::RefreshToken.alias(),
                    "refresh_token is set but is empty",
                );
            }

            self.serializer
                .refresh_token(refresh_token.as_ref())
                .grant_type("refresh_token");

            self.serializer.as_credential_map(
                vec![OAuthParameter::Scope],
                vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::GrantType,
                    OAuthParameter::ClientAssertion,
                    OAuthParameter::ClientAssertionType,
                    OAuthParameter::RefreshToken,
                ],
            )
        } else {
            self.serializer.grant_type("client_credentials");

            self.serializer.as_credential_map(
                vec![OAuthParameter::Scope],
                vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::GrantType,
                    OAuthParameter::ClientAssertion,
                    OAuthParameter::ClientAssertionType,
                ],
            )
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
                client_id: String::with_capacity(32),
                scope: vec!["https://graph.microsoft.com/.default".into()],
                authority: Default::default(),
                client_assertion_type: CLIENT_ASSERTION_TYPE.to_owned(),
                client_assertion: String::new(),
                refresh_token: None,
                token_credential_options: TokenCredentialOptions::default(),
                serializer: OAuthSerializer::new(),
            },
        }
    }

    #[cfg(feature = "openssl")]
    pub fn with_certificate(
        &mut self,
        certificate_assertion: &X509Certificate,
    ) -> anyhow::Result<&mut Self> {
        if let Some(tenant_id) = self.credential.authority.tenant_id() {
            self.with_client_assertion(certificate_assertion.sign(Some(tenant_id.clone()))?);
        } else {
            self.with_client_assertion(certificate_assertion.sign(None)?);
        }
        Ok(self)
    }

    pub fn with_client_assertion<T: AsRef<str>>(&mut self, client_assertion: T) -> &mut Self {
        self.credential.client_assertion = client_assertion.as_ref().to_owned();
        self
    }

    pub fn with_refresh_token<T: AsRef<str>>(&mut self, refresh_token: T) -> &mut Self {
        self.credential.refresh_token = Some(refresh_token.as_ref().to_owned());
        self
    }
}

impl From<ClientCertificateCredential> for ClientCertificateCredentialBuilder {
    fn from(credential: ClientCertificateCredential) -> Self {
        ClientCertificateCredentialBuilder { credential }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_CLIENT_ID: &str = "671a21bd-b91b-8ri7-94cb-e2cea49f30e1";

    #[test]
    fn credential_builder() {
        let mut builder = ClientCertificateCredentialBuilder::new();
        builder.with_client_id(TEST_CLIENT_ID);
        assert_eq!(builder.credential.client_id, TEST_CLIENT_ID);

        builder.with_client_id("123");
        assert_eq!(builder.credential.client_id, "123");

        builder.credential.client_id = "".into();
        assert!(builder.credential.client_id.is_empty());

        builder.with_client_id(TEST_CLIENT_ID);
        assert_eq!(builder.credential.client_id, TEST_CLIENT_ID);
    }
}
