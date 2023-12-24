use reqwest::IntoUrl;

use url::Url;
use uuid::Uuid;

use graph_error::{AuthorizationFailure, IdentityResult};

use crate::identity::{credentials::app_config::AppConfig, Authority, AzureCloudInstance};
use crate::oauth_serializer::{OAuthParameter, OAuthSerializer};
use crate::{ClientAssertionCredentialBuilder, ClientSecretCredentialBuilder};

#[cfg(feature = "openssl")]
use crate::identity::{ClientCertificateCredentialBuilder, X509Certificate};

#[derive(Clone)]
pub struct ClientCredentialsAuthorizationUrlParameters {
    /// The client (application) ID of the service principal
    pub(crate) app_config: AppConfig,
    pub(crate) state: Option<String>,
}

impl ClientCredentialsAuthorizationUrlParameters {
    pub fn new(
        client_id: impl AsRef<str>,
        redirect_uri: impl IntoUrl,
    ) -> IdentityResult<ClientCredentialsAuthorizationUrlParameters> {
        let redirect_uri_result = Url::parse(redirect_uri.as_str());
        let redirect_uri = redirect_uri.into_url().or(redirect_uri_result)?;

        Ok(ClientCredentialsAuthorizationUrlParameters {
            app_config: AppConfig::builder(client_id.as_ref())
                .redirect_uri(redirect_uri)
                .build(),
            state: None,
        })
    }

    pub fn builder<T: AsRef<str>>(
        client_id: T,
    ) -> ClientCredentialsAuthorizationUrlParameterBuilder {
        ClientCredentialsAuthorizationUrlParameterBuilder::new(client_id)
    }

    pub fn with_client_secret(
        self,
        client_secret: impl AsRef<str>,
    ) -> ClientSecretCredentialBuilder {
        ClientSecretCredentialBuilder::new_with_client_secret(client_secret, self.app_config)
    }

    pub fn with_client_assertion(
        self,
        signed_assertion: impl AsRef<str>,
    ) -> ClientAssertionCredentialBuilder {
        ClientAssertionCredentialBuilder::new_with_signed_assertion(
            signed_assertion,
            self.app_config,
        )
    }

    #[cfg(feature = "openssl")]
    pub fn with_client_x509_certificate(
        self,
        _client_secret: impl AsRef<str>,
        x509: &X509Certificate,
    ) -> IdentityResult<ClientCertificateCredentialBuilder> {
        ClientCertificateCredentialBuilder::new_with_certificate(x509, self.app_config)
    }

    pub fn url(&self) -> IdentityResult<Url> {
        self.url_with_host(&self.app_config.azure_cloud_instance)
    }

    pub fn url_with_host(&self, azure_cloud_instance: &AzureCloudInstance) -> IdentityResult<Url> {
        let mut serializer = OAuthSerializer::new();
        let client_id = self.app_config.client_id.to_string();
        if client_id.trim().is_empty() || self.app_config.client_id.is_nil() {
            return AuthorizationFailure::result(OAuthParameter::ClientId.alias());
        }

        if self.app_config.redirect_uri.is_none() {
            return AuthorizationFailure::result(OAuthParameter::RedirectUri.alias());
        }

        if let Some(redirect_uri) = self.app_config.redirect_uri.as_ref() {
            serializer.redirect_uri(redirect_uri.as_str());
        }

        serializer.client_id(client_id.as_str());

        if let Some(state) = self.state.as_ref() {
            serializer.state(state.as_ref());
        }

        let mut uri = azure_cloud_instance.admin_consent_uri(&self.app_config.authority)?;
        let query = serializer.encode_query(
            vec![OAuthParameter::State],
            vec![OAuthParameter::ClientId, OAuthParameter::RedirectUri],
        )?;
        uri.set_query(Some(query.as_str()));
        Ok(uri)
    }
}

#[derive(Clone)]
pub struct ClientCredentialsAuthorizationUrlParameterBuilder {
    credential: ClientCredentialsAuthorizationUrlParameters,
}

impl ClientCredentialsAuthorizationUrlParameterBuilder {
    pub fn new(client_id: impl AsRef<str>) -> Self {
        Self {
            credential: ClientCredentialsAuthorizationUrlParameters {
                app_config: AppConfig::new(client_id.as_ref()),
                state: None,
            },
        }
    }

    pub(crate) fn new_with_app_config(app_config: AppConfig) -> Self {
        Self {
            credential: ClientCredentialsAuthorizationUrlParameters {
                app_config,
                state: None,
            },
        }
    }

    pub fn with_client_id<T: AsRef<str>>(&mut self, client_id: T) -> IdentityResult<&mut Self> {
        self.credential.app_config.client_id = Uuid::try_parse(client_id.as_ref())?;
        Ok(self)
    }

    pub fn with_redirect_uri<T: IntoUrl>(&mut self, redirect_uri: T) -> IdentityResult<&mut Self> {
        let redirect_uri_result = Url::parse(redirect_uri.as_str());
        let redirect_uri = redirect_uri.into_url().or(redirect_uri_result)?;
        self.credential.app_config.redirect_uri = Some(redirect_uri);
        Ok(self)
    }

    /// Convenience method. Same as calling [with_authority(Authority::TenantId("tenant_id"))]
    pub fn with_tenant<T: AsRef<str>>(&mut self, tenant: T) -> &mut Self {
        self.credential.app_config.authority = Authority::TenantId(tenant.as_ref().to_owned());
        self
    }

    pub fn with_authority<T: Into<Authority>>(&mut self, authority: T) -> &mut Self {
        self.credential.app_config.authority = authority.into();
        self
    }

    pub fn with_state<T: AsRef<str>>(&mut self, state: T) -> &mut Self {
        self.credential.state = Some(state.as_ref().to_owned());
        self
    }

    pub fn build(&self) -> ClientCredentialsAuthorizationUrlParameters {
        self.credential.clone()
    }

    pub fn url(&self) -> IdentityResult<Url> {
        self.credential.url()
    }

    pub fn with_client_secret(
        self,
        client_secret: impl AsRef<str>,
    ) -> ClientSecretCredentialBuilder {
        ClientSecretCredentialBuilder::new_with_client_secret(
            client_secret,
            self.credential.app_config,
        )
    }

    pub fn with_client_assertion(
        self,
        signed_assertion: impl AsRef<str>,
    ) -> ClientAssertionCredentialBuilder {
        ClientAssertionCredentialBuilder::new_with_signed_assertion(
            signed_assertion,
            self.credential.app_config,
        )
    }

    #[cfg(feature = "openssl")]
    pub fn with_client_x509_certificate(
        self,
        _client_secret: impl AsRef<str>,
        x509: &X509Certificate,
    ) -> IdentityResult<ClientCertificateCredentialBuilder> {
        ClientCertificateCredentialBuilder::new_with_certificate(x509, self.credential.app_config)
    }
}
