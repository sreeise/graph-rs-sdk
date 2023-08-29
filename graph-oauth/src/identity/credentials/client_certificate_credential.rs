use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::credentials::app_config::AppConfig;
use crate::identity::{
    Authority, AzureCloudInstance, TokenCredentialExecutor, TokenCredentialOptions,
};
use async_trait::async_trait;
use graph_error::{AuthorizationFailure, AuthorizationResult, AF};
use http_body_util::BodyExt;
use std::collections::HashMap;
use url::Url;

#[cfg(feature = "openssl")]
use crate::identity::X509Certificate;
use crate::oauth::{ClientCredentialsAuthorizationUrlBuilder, ConfidentialClientApplication};

pub(crate) static CLIENT_ASSERTION_TYPE: &str =
    "urn:ietf:params:oauth:client-assertion-type:jwt-bearer";

credential_builder!(
    ClientCertificateCredentialBuilder,
    ConfidentialClientApplication
);

/// https://learn.microsoft.com/en-us/azure/active-directory/develop/active-directory-certificate-credentials
#[derive(Clone)]
#[allow(dead_code)]
pub struct ClientCertificateCredential {
    pub(crate) app_config: AppConfig,
    /// The value passed for the scope parameter in this request should be the resource
    /// identifier (application ID URI) of the resource you want, affixed with the .default
    /// suffix. For the Microsoft Graph example, the value is https://graph.microsoft.com/.default.
    /// Default is https://graph.microsoft.com/.default.
    pub(crate) scope: Vec<String>,
    pub(crate) client_assertion_type: String,
    pub(crate) client_assertion: String,
    pub(crate) refresh_token: Option<String>,
    serializer: OAuthSerializer,
}

impl ClientCertificateCredential {
    pub fn new<T: AsRef<str>>(client_id: T, client_assertion: T) -> ClientCertificateCredential {
        ClientCertificateCredential {
            app_config: AppConfig::new_with_client_id(client_id),
            scope: vec!["https://graph.microsoft.com/.default".into()],
            client_assertion_type: CLIENT_ASSERTION_TYPE.to_owned(),
            client_assertion: client_assertion.as_ref().to_owned(),
            refresh_token: None,
            serializer: Default::default(),
        }
    }

    #[cfg(feature = "openssl")]
    pub fn x509<T: AsRef<str>>(
        client_id: T,
        x509: &X509Certificate,
    ) -> anyhow::Result<ClientCertificateCredential> {
        let mut builder = ClientCertificateCredentialBuilder::new();
        builder
            .with_client_id(client_id.as_ref().to_owned())
            .with_certificate(x509)?;
        Ok(builder.credential)
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
impl TokenCredentialExecutor for ClientCertificateCredential {
    fn uri(&mut self, azure_authority_host: &AzureCloudInstance) -> AuthorizationResult<Url> {
        self.serializer
            .authority(azure_authority_host, &self.app_config.authority);

        let uri = self
            .serializer
            .get(OAuthParameter::TokenUrl)
            .ok_or(AF::msg_err("token_url", "Internal Error"))?;
        Url::parse(uri.as_str()).map_err(AF::from)
    }

    fn form_urlencode(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        let client_id = self.client_id().clone();
        if client_id.trim().is_empty() {
            return AuthorizationFailure::result(OAuthParameter::ClientId.alias());
        }

        if self.client_assertion.trim().is_empty() {
            return AuthorizationFailure::result(OAuthParameter::ClientAssertion.alias());
        }

        if self.client_assertion_type.trim().is_empty() {
            self.client_assertion_type = CLIENT_ASSERTION_TYPE.to_owned();
        }

        self.serializer
            .client_id(client_id.as_str())
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

    fn client_id(&self) -> &String {
        &self.app_config.client_id
    }

    fn authority(&self) -> Authority {
        self.app_config.authority.clone()
    }

    fn app_config(&self) -> &AppConfig {
        &self.app_config
    }
}

pub struct ClientCertificateCredentialBuilder {
    credential: ClientCertificateCredential,
}

impl ClientCertificateCredentialBuilder {
    fn new() -> ClientCertificateCredentialBuilder {
        ClientCertificateCredentialBuilder {
            credential: ClientCertificateCredential {
                app_config: Default::default(),
                scope: vec!["https://graph.microsoft.com/.default".into()],
                client_assertion_type: CLIENT_ASSERTION_TYPE.to_owned(),
                client_assertion: String::new(),
                refresh_token: None,
                serializer: OAuthSerializer::new(),
            },
        }
    }

    fn builder<T: ToString, I: IntoIterator<Item = T>>(
        scopes: I,
    ) -> ClientCertificateCredentialBuilder {
        ClientCertificateCredentialBuilder {
            credential: ClientCertificateCredential {
                app_config: Default::default(),
                scope: scopes.into_iter().map(|s| s.to_string()).collect(),
                client_assertion_type: CLIENT_ASSERTION_TYPE.to_owned(),
                client_assertion: String::new(),
                refresh_token: None,
                serializer: OAuthSerializer::new(),
            },
        }
    }

    #[cfg(feature = "openssl")]
    pub(crate) fn new_with_certificate(
        x509: &X509Certificate,
        app_config: AppConfig,
    ) -> anyhow::Result<ClientCertificateCredentialBuilder> {
        let mut builder = ClientCertificateCredentialBuilder::new();
        builder.credential.app_config = app_config;
        builder.with_certificate(x509)?;
        Ok(builder)
    }

    #[cfg(feature = "openssl")]
    pub fn with_certificate(&mut self, certificate: &X509Certificate) -> anyhow::Result<&mut Self> {
        if let Some(tenant_id) = self.credential.authority.tenant_id() {
            self.with_client_assertion(certificate.sign_with_tenant(Some(tenant_id.clone()))?);
        } else {
            self.with_client_assertion(certificate.sign_with_tenant(None)?);
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

    pub(crate) fn credential(self) -> ClientCertificateCredential {
        self.credential
    }
}

impl From<ClientCertificateCredential> for ClientCertificateCredentialBuilder {
    fn from(credential: ClientCertificateCredential) -> Self {
        ClientCertificateCredentialBuilder { credential }
    }
}

impl From<ClientCertificateCredentialBuilder> for ClientCertificateCredential {
    fn from(builder: ClientCertificateCredentialBuilder) -> Self {
        builder.credential
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
        assert_eq!(builder.credential.app_config.client_id, TEST_CLIENT_ID);

        builder.with_client_id("123");
        assert_eq!(builder.credential.app_config.client_id, "123");

        builder.credential.app_config.client_id = "".into();
        assert!(builder.credential.app_config.client_id.is_empty());

        builder.with_client_id(TEST_CLIENT_ID);
        assert_eq!(builder.credential.app_config.client_id, TEST_CLIENT_ID);
    }
}
