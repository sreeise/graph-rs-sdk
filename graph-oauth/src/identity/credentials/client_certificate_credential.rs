use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use async_trait::async_trait;
use http::{HeaderMap, HeaderName, HeaderValue};
use url::Url;
use uuid::Uuid;

use graph_core::cache::{InMemoryCacheStore, TokenCache};
use graph_error::{AuthExecutionError, AuthorizationFailure, IdentityResult, AF};

use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::credentials::app_config::AppConfig;
#[cfg(feature = "openssl")]
use crate::identity::X509Certificate;
use crate::identity::{
    Authority, AzureCloudInstance, ClientCredentialsAuthorizationUrlParameterBuilder,
    ConfidentialClientApplication, ForceTokenRefresh, Token, TokenCredentialExecutor,
};

pub(crate) static CLIENT_ASSERTION_TYPE: &str =
    "urn:ietf:params:oauth:client-assertion-type:jwt-bearer";

credential_builder!(
    ClientCertificateCredentialBuilder,
    ConfidentialClientApplication<ClientCertificateCredential>
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
    token_cache: InMemoryCacheStore<Token>,
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
            token_cache: Default::default(),
        }
    }

    #[cfg(feature = "openssl")]
    pub fn x509<T: AsRef<str>>(
        client_id: T,
        x509: &X509Certificate,
    ) -> anyhow::Result<ClientCertificateCredential> {
        let mut builder = ClientCertificateCredentialBuilder::new(client_id.as_ref());
        builder.with_certificate(x509)?;
        Ok(builder.credential)
    }

    pub fn with_refresh_token<T: AsRef<str>>(&mut self, refresh_token: T) -> &mut Self {
        self.refresh_token = Some(refresh_token.as_ref().to_owned());
        self
    }

    pub fn builder<T: AsRef<str>>(client_id: T) -> ClientCertificateCredentialBuilder {
        ClientCertificateCredentialBuilder::new(client_id)
    }

    pub fn authorization_url_builder<T: AsRef<str>>(
        client_id: T,
    ) -> ClientCredentialsAuthorizationUrlParameterBuilder {
        ClientCredentialsAuthorizationUrlParameterBuilder::new(client_id)
    }
}

impl Debug for ClientCertificateCredential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientAssertionCredential")
            .field("app_config", &self.app_config)
            .field("scope", &self.scope)
            .finish()
    }
}

#[async_trait]
impl TokenCache for ClientCertificateCredential {
    type Token = Token;

    fn get_token_silent(&mut self) -> Result<Self::Token, AuthExecutionError> {
        let cache_id = self.app_config.cache_id.to_string();
        if let Some(token) = self.token_cache.get(cache_id.as_str()) {
            if token.is_expired_sub(time::Duration::minutes(5)) {
                let response = self.execute()?;
                let msal_token: Token = response.json()?;
                self.token_cache.store(cache_id, msal_token.clone());
                Ok(msal_token)
            } else {
                Ok(token)
            }
        } else {
            let response = self.execute()?;
            let msal_token: Token = response.json()?;
            self.token_cache.store(cache_id, msal_token.clone());
            Ok(msal_token)
        }
    }

    async fn get_token_silent_async(&mut self) -> Result<Self::Token, AuthExecutionError> {
        let cache_id = self.app_config.cache_id.to_string();
        if let Some(token) = self.token_cache.get(cache_id.as_str()) {
            if token.is_expired_sub(time::Duration::minutes(5)) {
                let response = self.execute_async().await?;
                let msal_token: Token = response.json().await?;
                self.token_cache.store(cache_id, msal_token.clone());
                Ok(msal_token)
            } else {
                Ok(token.clone())
            }
        } else {
            let response = self.execute_async().await?;
            let msal_token: Token = response.json().await?;
            self.token_cache.store(cache_id, msal_token.clone());
            Ok(msal_token)
        }
    }
}

#[async_trait]
impl TokenCredentialExecutor for ClientCertificateCredential {
    fn uri(&mut self) -> IdentityResult<Url> {
        let azure_cloud_instance = self.azure_cloud_instance();
        self.serializer
            .authority(&azure_cloud_instance, &self.authority());

        let uri = self
            .serializer
            .get(OAuthParameter::TokenUrl)
            .ok_or(AF::msg_err("token_url", "Internal Error"))?;
        Url::parse(uri.as_str()).map_err(AF::from)
    }

    fn form_urlencode(&mut self) -> IdentityResult<HashMap<String, String>> {
        let client_id = self.app_config.client_id.to_string();
        if client_id.is_empty() || self.app_config.client_id.is_nil() {
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
pub struct ClientCertificateCredentialBuilder {
    credential: ClientCertificateCredential,
}

impl ClientCertificateCredentialBuilder {
    fn new<T: AsRef<str>>(client_id: T) -> ClientCertificateCredentialBuilder {
        ClientCertificateCredentialBuilder {
            credential: ClientCertificateCredential {
                app_config: AppConfig::new_with_client_id(client_id.as_ref()),
                scope: vec!["https://graph.microsoft.com/.default".into()],
                client_assertion_type: CLIENT_ASSERTION_TYPE.to_owned(),
                client_assertion: String::new(),
                refresh_token: None,
                serializer: OAuthSerializer::new(),
                token_cache: Default::default(),
            },
        }
    }

    #[cfg(feature = "openssl")]
    pub(crate) fn new_with_certificate(
        x509: &X509Certificate,
        app_config: AppConfig,
    ) -> anyhow::Result<ClientCertificateCredentialBuilder> {
        let mut credential_builder = ClientCertificateCredentialBuilder {
            credential: ClientCertificateCredential {
                app_config,
                scope: vec!["https://graph.microsoft.com/.default".into()],
                client_assertion_type: CLIENT_ASSERTION_TYPE.to_owned(),
                client_assertion: String::new(),
                refresh_token: None,
                serializer: OAuthSerializer::new(),
                token_cache: Default::default(),
            },
        };
        credential_builder.with_certificate(x509)?;
        Ok(credential_builder)
    }

    #[cfg(feature = "openssl")]
    pub fn with_certificate(&mut self, certificate: &X509Certificate) -> anyhow::Result<&mut Self> {
        if let Some(tenant_id) = self.credential.app_config.authority.tenant_id() {
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

    pub fn credential(self) -> ClientCertificateCredential {
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

    #[test]
    fn test_uuid_fake() {
        let client_id_uuid = Uuid::new_v4();
        let builder = ClientCertificateCredentialBuilder::new(client_id_uuid.to_string());
        assert_eq!(builder.credential.app_config.client_id, client_id_uuid);
    }

    #[test]
    #[should_panic]
    fn test_123_uuid() {
        let builder = ClientCertificateCredentialBuilder::new("123");
        assert_eq!(
            builder.credential.app_config.client_id,
            Uuid::try_parse("123").unwrap()
        );
    }

    #[test]
    fn credential_builder() {
        let builder =
            ClientCertificateCredentialBuilder::new("4ef900be-dfd9-4da6-b224-0011e46c54dd");
        assert_eq!(
            builder.credential.app_config.client_id.to_string(),
            "4ef900be-dfd9-4da6-b224-0011e46c54dd"
        );
    }
}
