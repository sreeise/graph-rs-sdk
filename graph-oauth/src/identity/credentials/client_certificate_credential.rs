use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use async_trait::async_trait;
use http::{HeaderMap, HeaderName, HeaderValue};

use uuid::Uuid;

use graph_core::cache::{InMemoryCacheStore, TokenCache};
use graph_error::{AuthExecutionError, AuthorizationFailure, IdentityResult};

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

/// Client Credentials Using A Certificate
///
/// The OAuth 2.0 client credentials grant flow permits a web service (confidential client) to use
/// its own credentials, instead of impersonating a user, to authenticate when calling another
/// web service. The grant specified in RFC 6749, sometimes called two-legged OAuth, can be used
/// to access web-hosted resources by using the identity of an application. This type is commonly
/// used for server-to-server interactions that must run in the background, without immediate
/// interaction with a user, and is often referred to as daemons or service accounts.
/// For more information on the flow see
/// [Token Request With a Certificate](https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-client-creds-grant-flow#second-case-access-token-request-with-a-certificate)
///
/// The SDK handles certificates and creating the assertion automatically using the
/// openssl crate. This is significantly easier than having to format the assertion from
/// the certificate yourself. If you need to use your own assertion see
/// [ClientAssertionCredential](crate::identity::ClientAssertionCredential)
#[derive(Clone)]
#[allow(dead_code)]
pub struct ClientCertificateCredential {
    pub(crate) app_config: AppConfig,
    /// The value passed for the scope parameter in this request should be the resource identifier
    /// (application ID URI) of the resource you want, affixed with the .default suffix.
    /// All scopes included must be for a single resource. Including scopes for multiple
    /// resources will result in an error.
    ///
    /// For the Microsoft Graph example, the value is https://graph.microsoft.com/.default.
    /// This value tells the Microsoft identity platform that of all the direct application
    /// permissions you have configured for your app, the endpoint should issue a token for the
    /// ones associated with the resource you want to use. To learn more about the /.default scope,
    /// see the [consent documentation](https://learn.microsoft.com/en-us/entra/identity-platform/permissions-consent-overview#the-default-scope).
    pub(crate) scope: Vec<String>,
    /// The value must be set to urn:ietf:params:oauth:client-assertion-type:jwt-bearer.
    /// This value is automatically set by the SDK.
    pub(crate) client_assertion_type: String,
    /// An assertion (a JSON web token) that you need to create and sign with the certificate
    /// you registered as credentials for your application. Read about
    /// [certificate credentials](https://learn.microsoft.com/en-us/entra/identity-platform/certificate-credentials)
    /// to learn how to register your certificate and the format of the assertion.
    ///
    /// The SDK handles certificates and creating the assertion automatically using the
    /// openssl crate. This is significantly easier than having to format the assertion from
    /// the certificate yourself.
    pub(crate) client_assertion: String,
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
        f.debug_struct("ClientCertificateCredential")
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
            .client_assertion_type(self.client_assertion_type.as_str())
            .grant_type("client_credentials");

        if self.scope.is_empty() {
            self.serializer
                .add_scope("https://graph.microsoft.com/.default");
        }

        self.serializer.as_credential_map(
            vec![OAuthParameter::Scope],
            vec![
                OAuthParameter::ClientId,
                OAuthParameter::GrantType,
                OAuthParameter::ClientAssertion,
                OAuthParameter::ClientAssertionType,
            ],
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
                client_assertion: Default::default(),
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
                client_assertion: Default::default(),
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
