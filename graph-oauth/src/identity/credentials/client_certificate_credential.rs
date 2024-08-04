use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use async_trait::async_trait;
use http::{HeaderMap, HeaderName, HeaderValue};

use uuid::Uuid;

use graph_core::cache::{CacheStore, InMemoryCacheStore, TokenCache};
use graph_core::http::{AsyncResponseConverterExt, ResponseConverterExt};
use graph_core::identity::ForceTokenRefresh;
use graph_error::{AuthExecutionError, AuthExecutionResult, AuthorizationFailure, IdentityResult};

use crate::identity::credentials::app_config::AppConfig;
#[cfg(feature = "openssl")]
use crate::identity::X509Certificate;
use crate::identity::{
    tracing_targets::CREDENTIAL_EXECUTOR, Authority, AzureCloudInstance,
    ClientCredentialsAuthorizationUrlParameterBuilder, ConfidentialClientApplication, Token,
    TokenCredentialExecutor,
};
use crate::oauth_serializer::{AuthParameter, AuthSerializer};

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
pub struct ClientCertificateCredential {
    pub(crate) app_config: AppConfig,
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
    token_cache: InMemoryCacheStore<Token>,
}

impl ClientCertificateCredential {
    #[cfg(feature = "openssl")]
    pub fn new<T: AsRef<str>>(
        client_id: T,
        x509: &X509Certificate,
    ) -> IdentityResult<ClientCertificateCredential> {
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

    fn execute_cached_token_refresh(&mut self, cache_id: String) -> AuthExecutionResult<Token> {
        let response = self.execute()?;

        if !response.status().is_success() {
            return Err(AuthExecutionError::silent_token_auth(
                response.into_http_response()?,
            ));
        }

        let new_token: Token = response.json()?;
        self.token_cache.store(cache_id, new_token.clone());
        Ok(new_token)
    }

    async fn execute_cached_token_refresh_async(
        &mut self,
        cache_id: String,
    ) -> AuthExecutionResult<Token> {
        let response = self.execute_async().await?;

        if !response.status().is_success() {
            return Err(AuthExecutionError::silent_token_auth(
                response.into_http_response_async().await?,
            ));
        }

        let new_token: Token = response.json().await?;
        self.token_cache.store(cache_id, new_token.clone());
        Ok(new_token)
    }
}

impl Debug for ClientCertificateCredential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientCertificateCredential")
            .field("app_config", &self.app_config)
            .finish()
    }
}

#[async_trait]
impl TokenCache for ClientCertificateCredential {
    type Token = Token;

    #[tracing::instrument]
    fn get_token_silent(&mut self) -> Result<Self::Token, AuthExecutionError> {
        let cache_id = self.app_config.cache_id.to_string();
        if let Some(token) = self.token_cache.get(cache_id.as_str()) {
            if token.is_expired_sub(time::Duration::minutes(5)) {
                tracing::debug!(target: CREDENTIAL_EXECUTOR, "executing silent token request; refresh_token=None");
                self.execute_cached_token_refresh(cache_id)
            } else {
                tracing::debug!(target: CREDENTIAL_EXECUTOR, "using token from cache");
                Ok(token)
            }
        } else {
            tracing::debug!(target: CREDENTIAL_EXECUTOR, "executing silent token request; refresh_token=None");
            self.execute_cached_token_refresh(cache_id)
        }
    }

    #[tracing::instrument]
    async fn get_token_silent_async(&mut self) -> Result<Self::Token, AuthExecutionError> {
        let cache_id = self.app_config.cache_id.to_string();
        if let Some(token) = self.token_cache.get(cache_id.as_str()) {
            if token.is_expired_sub(time::Duration::minutes(5)) {
                tracing::debug!(target: CREDENTIAL_EXECUTOR, "executing silent token refresh");
                self.execute_cached_token_refresh_async(cache_id).await
            } else {
                tracing::debug!(target: CREDENTIAL_EXECUTOR, "using token from cache");
                Ok(token.clone())
            }
        } else {
            tracing::debug!(target: CREDENTIAL_EXECUTOR, "executing silent token request");
            self.execute_cached_token_refresh_async(cache_id).await
        }
    }

    fn with_force_token_refresh(&mut self, force_token_refresh: ForceTokenRefresh) {
        self.app_config.force_token_refresh = force_token_refresh;
    }
}

#[async_trait]
impl TokenCredentialExecutor for ClientCertificateCredential {
    fn form_urlencode(&mut self) -> IdentityResult<HashMap<String, String>> {
        let mut serializer = AuthSerializer::new();
        let client_id = self.app_config.client_id.to_string();
        if client_id.is_empty() || self.app_config.client_id.is_nil() {
            return AuthorizationFailure::result(AuthParameter::ClientId.alias());
        }

        if self.client_assertion.trim().is_empty() {
            return AuthorizationFailure::result(AuthParameter::ClientAssertion.alias());
        }

        if self.client_assertion_type.trim().is_empty() {
            self.client_assertion_type = CLIENT_ASSERTION_TYPE.to_owned();
        }

        serializer
            .client_id(client_id.as_str())
            .client_assertion(self.client_assertion.as_str())
            .client_assertion_type(self.client_assertion_type.as_str())
            .grant_type("client_credentials")
            .set_scope(self.app_config.scope.clone());

        serializer.as_credential_map(
            vec![AuthParameter::Scope],
            vec![
                AuthParameter::ClientId,
                AuthParameter::GrantType,
                AuthParameter::ClientAssertion,
                AuthParameter::ClientAssertionType,
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
                app_config: AppConfig::builder(client_id.as_ref())
                    .scope(vec!["https://graph.microsoft.com/.default"])
                    .build(),
                client_assertion_type: CLIENT_ASSERTION_TYPE.to_owned(),
                client_assertion: Default::default(),
                token_cache: Default::default(),
            },
        }
    }

    #[cfg(feature = "openssl")]
    pub(crate) fn new_with_certificate(
        x509: &X509Certificate,
        mut app_config: AppConfig,
    ) -> IdentityResult<ClientCertificateCredentialBuilder> {
        app_config
            .scope
            .insert("https://graph.microsoft.com/.default".into());
        let mut credential_builder = ClientCertificateCredentialBuilder {
            credential: ClientCertificateCredential {
                app_config,
                client_assertion_type: CLIENT_ASSERTION_TYPE.to_owned(),
                client_assertion: Default::default(),
                token_cache: Default::default(),
            },
        };
        credential_builder.with_certificate(x509)?;
        Ok(credential_builder)
    }

    #[cfg(feature = "openssl")]
    pub fn with_certificate(&mut self, certificate: &X509Certificate) -> IdentityResult<&mut Self> {
        if let Some(tenant_id) = self.credential.app_config.authority.tenant_id() {
            self.with_client_assertion(certificate.sign_with_tenant(Some(tenant_id.clone()))?);
        } else {
            self.with_client_assertion(certificate.sign_with_tenant(None)?);
        }
        Ok(self)
    }

    #[allow(dead_code)]
    fn with_client_assertion<T: AsRef<str>>(&mut self, client_assertion: T) -> &mut Self {
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
