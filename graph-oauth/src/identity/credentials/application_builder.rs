use crate::identity::{
    application_options::ApplicationOptions, credentials::app_config::AppConfig,
    AuthCodeAuthorizationUrlParameterBuilder, Authority,
    AuthorizationCodeAssertionCredentialBuilder, AuthorizationCodeCredentialBuilder,
    AzureCloudInstance, ClientAssertionCredentialBuilder,
    ClientCredentialsAuthorizationUrlParameterBuilder, ClientSecretCredentialBuilder,
    DeviceCodeCredentialBuilder, DeviceCodePollingExecutor, EnvironmentCredential,
    OpenIdAuthorizationUrlParameterBuilder, OpenIdCredentialBuilder, PublicClientApplication,
    ResourceOwnerPasswordCredential, ResourceOwnerPasswordCredentialBuilder,
};
use graph_error::{IdentityResult, AF};
use http::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::env::VarError;
use uuid::Uuid;

#[cfg(feature = "openssl")]
use crate::identity::{
    AuthorizationCodeCertificateCredentialBuilder, ClientCertificateCredentialBuilder,
    X509Certificate,
};

pub struct ConfidentialClientApplicationBuilder {
    pub(crate) app_config: AppConfig,
}

impl ConfidentialClientApplicationBuilder {
    pub fn new(client_id: impl TryInto<Uuid>) -> Self {
        ConfidentialClientApplicationBuilder {
            app_config: AppConfig::new(client_id),
        }
    }

    pub fn new_with_application_options(
        application_options: ApplicationOptions,
    ) -> IdentityResult<ConfidentialClientApplicationBuilder> {
        ConfidentialClientApplicationBuilder::try_from(application_options)
    }

    pub fn with_tenant(&mut self, tenant_id: impl AsRef<str>) -> &mut Self {
        self.app_config.with_tenant(tenant_id);
        self
    }

    pub fn with_authority<T: Into<Authority>>(&mut self, authority: T) -> &mut Self {
        self.app_config.with_authority(authority.into());
        self
    }

    pub fn with_azure_cloud_instance(
        &mut self,
        azure_cloud_instance: AzureCloudInstance,
    ) -> &mut Self {
        self.app_config
            .with_azure_cloud_instance(azure_cloud_instance);
        self
    }

    /// Extends the query parameters of both the default query params and user defined params.
    /// Does not overwrite default params.
    pub fn with_extra_query_param(&mut self, query_param: (String, String)) -> &mut Self {
        self.app_config.with_extra_query_param(query_param);
        self
    }

    /// Extends the query parameters of both the default query params and user defined params.
    /// Does not overwrite default params.
    pub fn with_extra_query_parameters(
        &mut self,
        query_parameters: HashMap<String, String>,
    ) -> &mut Self {
        self.app_config
            .with_extra_query_parameters(query_parameters);
        self
    }

    /// Extends the header parameters of both the default header params and user defined params.
    /// Does not overwrite default params.
    pub fn with_extra_header_param<K: Into<HeaderName>, V: Into<HeaderValue>>(
        &mut self,
        header_name: K,
        header_value: V,
    ) -> &mut Self {
        self.app_config
            .with_extra_header_param(header_name, header_value);
        self
    }

    /// Extends the header parameters of both the default header params and user defined params.
    /// Does not overwrite default params.
    pub fn with_extra_header_parameters(&mut self, header_parameters: HeaderMap) -> &mut Self {
        self.app_config
            .with_extra_header_parameters(header_parameters);
        self
    }

    pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scope: I) -> &mut Self {
        self.app_config.with_scope(scope);
        self
    }

    /// Auth Code Authorization Url Builder
    pub fn auth_code_url_builder(&mut self) -> AuthCodeAuthorizationUrlParameterBuilder {
        AuthCodeAuthorizationUrlParameterBuilder::new_with_app_config(self.app_config.clone())
    }

    /// Client Credentials Authorization Url Builder
    pub fn client_credential_url_builder(
        &mut self,
    ) -> ClientCredentialsAuthorizationUrlParameterBuilder {
        ClientCredentialsAuthorizationUrlParameterBuilder::new_with_app_config(
            self.app_config.clone(),
        )
    }

    /// OpenId Authorization Url Builder
    pub fn openid_url_builder(&mut self) -> OpenIdAuthorizationUrlParameterBuilder {
        OpenIdAuthorizationUrlParameterBuilder::new_with_app_config(self.app_config.clone())
    }

    /// Client Credentials Using X509 Certificate
    #[cfg(feature = "openssl")]
    pub fn with_client_x509_certificate(
        &mut self,
        certificate: &X509Certificate,
    ) -> IdentityResult<ClientCertificateCredentialBuilder> {
        ClientCertificateCredentialBuilder::new_with_certificate(
            certificate,
            self.app_config.clone(),
        )
    }

    /// Client Credentials Using Client Secret.
    pub fn with_client_secret(
        &mut self,
        client_secret: impl AsRef<str>,
    ) -> ClientSecretCredentialBuilder {
        ClientSecretCredentialBuilder::new_with_client_secret(
            client_secret,
            self.app_config.clone(),
        )
    }

    /// Client Credentials Using Assertion.
    pub fn with_client_assertion(
        &mut self,
        signed_assertion: impl AsRef<str>,
    ) -> ClientAssertionCredentialBuilder {
        ClientAssertionCredentialBuilder::new_with_signed_assertion(
            signed_assertion,
            self.app_config.clone(),
        )
    }

    /// Client Credentials Authorization Url Builder
    pub fn with_auth_code(
        &mut self,
        authorization_code: impl AsRef<str>,
    ) -> AuthorizationCodeCredentialBuilder {
        AuthorizationCodeCredentialBuilder::new_with_auth_code(
            authorization_code,
            self.app_config.clone(),
        )
    }

    /// Auth Code Using Assertion
    pub fn with_auth_code_assertion(
        &mut self,
        authorization_code: impl AsRef<str>,
        assertion: impl AsRef<str>,
    ) -> AuthorizationCodeAssertionCredentialBuilder {
        AuthorizationCodeAssertionCredentialBuilder::from_assertion(
            authorization_code,
            assertion,
            self.app_config.clone(),
        )
    }

    /// Auth Code Using X509 Certificate
    #[cfg(feature = "openssl")]
    pub fn with_auth_code_x509_certificate(
        &mut self,
        authorization_code: impl AsRef<str>,
        x509: &X509Certificate,
    ) -> IdentityResult<AuthorizationCodeCertificateCredentialBuilder> {
        AuthorizationCodeCertificateCredentialBuilder::new_with_auth_code_and_x509(
            authorization_code,
            x509,
            self.app_config.clone(),
        )
    }

    //#[cfg(feature = "interactive-auth")]

    /// Auth Code Using OpenId.
    pub fn with_openid(
        &mut self,
        authorization_code: impl AsRef<str>,
        client_secret: impl AsRef<str>,
    ) -> OpenIdCredentialBuilder {
        OpenIdCredentialBuilder::new_with_auth_code_and_secret(
            authorization_code,
            client_secret,
            self.app_config.clone(),
        )
    }
}

impl From<ConfidentialClientApplicationBuilder> for AppConfig {
    fn from(value: ConfidentialClientApplicationBuilder) -> Self {
        value.app_config
    }
}

impl TryFrom<ApplicationOptions> for ConfidentialClientApplicationBuilder {
    type Error = AF;

    fn try_from(value: ApplicationOptions) -> Result<Self, Self::Error> {
        AF::condition(
            !value.client_id.to_string().is_empty(),
            "Client Id",
            "Client Id cannot be empty",
        )?;
        AF::condition(
            !(value.instance.is_some() && value.azure_cloud_instance.is_some()),
            "Instance | AzureCloudInstance",
            "Both specify the azure cloud instance and cannot be set at the same time",
        )?;
        AF::condition(
            !(value.tenant_id.is_some() && value.aad_authority_audience.is_some()),
            "TenantId | AadAuthorityAudience",
            "Both represent an authority audience and cannot be set at the same time",
        )?;

        Ok(ConfidentialClientApplicationBuilder {
            app_config: AppConfig::try_from(value)?,
        })
    }
}

#[allow(dead_code)]
pub struct PublicClientApplicationBuilder {
    app_config: AppConfig,
}

impl PublicClientApplicationBuilder {
    #[allow(dead_code)]
    pub fn new(client_id: impl AsRef<str>) -> PublicClientApplicationBuilder {
        PublicClientApplicationBuilder {
            app_config: AppConfig::new(client_id.as_ref()),
        }
    }

    #[allow(dead_code)]
    pub fn create_with_application_options(
        application_options: ApplicationOptions,
    ) -> IdentityResult<PublicClientApplicationBuilder> {
        PublicClientApplicationBuilder::try_from(application_options)
    }

    pub fn with_tenant(&mut self, tenant_id: impl AsRef<str>) -> &mut Self {
        self.app_config.with_tenant(tenant_id);
        self
    }

    pub fn with_authority<T: Into<Authority>>(&mut self, authority: T) -> &mut Self {
        self.app_config.with_authority(authority.into());
        self
    }

    pub fn with_azure_cloud_instance(
        &mut self,
        azure_cloud_instance: AzureCloudInstance,
    ) -> &mut Self {
        self.app_config
            .with_azure_cloud_instance(azure_cloud_instance);
        self
    }

    /// Extends the query parameters of both the default query params and user defined params.
    /// Does not overwrite default params.
    pub fn with_extra_query_param(&mut self, query_param: (String, String)) -> &mut Self {
        self.app_config.with_extra_query_param(query_param);
        self
    }

    /// Extends the query parameters of both the default query params and user defined params.
    /// Does not overwrite default params.
    pub fn with_extra_query_parameters(
        &mut self,
        query_parameters: HashMap<String, String>,
    ) -> &mut Self {
        self.app_config
            .with_extra_query_parameters(query_parameters);
        self
    }

    /// Extends the header parameters of both the default header params and user defined params.
    /// Does not overwrite default params.
    pub fn with_extra_header_param<K: Into<HeaderName>, V: Into<HeaderValue>>(
        &mut self,
        header_name: K,
        header_value: V,
    ) -> &mut Self {
        self.app_config
            .with_extra_header_param(header_name, header_value);
        self
    }

    /// Extends the header parameters of both the default header params and user defined params.
    /// Does not overwrite default params.
    pub fn with_extra_header_parameters(&mut self, header_parameters: HeaderMap) -> &mut Self {
        self.app_config
            .with_extra_header_parameters(header_parameters);
        self
    }

    pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scope: I) -> &mut Self {
        self.app_config.with_scope(scope);
        self
    }

    pub fn with_device_code_executor(&mut self) -> DeviceCodePollingExecutor {
        DeviceCodePollingExecutor::new_with_app_config(self.app_config.clone())
    }

    pub fn with_device_code(
        &mut self,
        device_code: impl AsRef<str>,
    ) -> DeviceCodeCredentialBuilder {
        DeviceCodeCredentialBuilder::new_with_device_code(
            device_code.as_ref(),
            self.app_config.clone(),
        )
    }

    pub fn with_username_password(
        &mut self,
        username: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> ResourceOwnerPasswordCredentialBuilder {
        ResourceOwnerPasswordCredentialBuilder::new_with_username_password(
            username.as_ref(),
            password.as_ref(),
            self.app_config.clone(),
        )
    }

    pub fn with_username_password_from_environment(
    ) -> Result<PublicClientApplication<ResourceOwnerPasswordCredential>, VarError> {
        EnvironmentCredential::resource_owner_password_credential()
    }
}

impl TryFrom<ApplicationOptions> for PublicClientApplicationBuilder {
    type Error = AF;

    fn try_from(value: ApplicationOptions) -> Result<Self, Self::Error> {
        AF::condition(
            !value.client_id.is_nil(),
            "client_id",
            "Client id cannot be empty",
        )?;
        AF::condition(
            !(value.instance.is_some() && value.azure_cloud_instance.is_some()),
            "Instance | AzureCloudInstance",
            "Instance and AzureCloudInstance both specify the azure cloud instance and cannot be set at the same time",
        )?;
        AF::condition(
            !(value.tenant_id.is_some() && value.aad_authority_audience.is_some()),
            "TenantId | AadAuthorityAudience",
            "TenantId and AadAuthorityAudience both represent an authority audience and cannot be set at the same time",
        )?;

        Ok(PublicClientApplicationBuilder {
            app_config: AppConfig::try_from(value)?,
        })
    }
}

#[cfg(test)]
mod test {
    use http::header::AUTHORIZATION;
    use http::HeaderValue;
    use url::Url;
    use uuid::Uuid;

    use crate::identity::{AadAuthorityAudience, AzureCloudInstance, TokenCredentialExecutor};

    use super::*;

    #[test]
    #[should_panic]
    fn confidential_client_error_result_on_instance_and_aci() {
        ConfidentialClientApplicationBuilder::try_from(ApplicationOptions {
            client_id: Uuid::new_v4(),
            tenant_id: None,
            aad_authority_audience: None,
            instance: Some(Url::parse("https://login.microsoft.com").unwrap()),
            azure_cloud_instance: Some(AzureCloudInstance::AzurePublic),
            redirect_uri: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn confidential_client_error_result_on_tenant_id_and_aad_audience() {
        ConfidentialClientApplicationBuilder::try_from(ApplicationOptions {
            client_id: Uuid::new_v4(),
            tenant_id: Some("tenant_id".to_owned()),
            aad_authority_audience: Some(AadAuthorityAudience::AzureAdAndPersonalMicrosoftAccount),
            instance: None,
            azure_cloud_instance: None,
            redirect_uri: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn public_client_error_result_on_instance_and_aci() {
        PublicClientApplicationBuilder::try_from(ApplicationOptions {
            client_id: Uuid::new_v4(),
            tenant_id: None,
            aad_authority_audience: None,
            instance: Some(Url::parse("https://login.microsoft.com").unwrap()),
            azure_cloud_instance: Some(AzureCloudInstance::AzurePublic),
            redirect_uri: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn public_client_error_result_on_tenant_id_and_aad_audience() {
        PublicClientApplicationBuilder::try_from(ApplicationOptions {
            client_id: Uuid::new_v4(),
            tenant_id: Some("tenant_id".to_owned()),
            aad_authority_audience: Some(AadAuthorityAudience::AzureAdAndPersonalMicrosoftAccount),
            instance: None,
            azure_cloud_instance: None,
            redirect_uri: None,
        })
        .unwrap();
    }

    #[test]
    fn extra_parameters() {
        let mut confidential_client = ConfidentialClientApplicationBuilder::new("client-id");
        let mut map = HashMap::new();
        map.insert("name".to_owned(), "123".to_owned());
        confidential_client.with_extra_query_parameters(map);

        let mut header_map = HeaderMap::new();
        header_map.insert(AUTHORIZATION, HeaderValue::from_static("Bearer Token"));
        confidential_client.with_extra_header_parameters(header_map);

        assert_eq!(
            confidential_client
                .app_config
                .extra_header_parameters
                .get(AUTHORIZATION)
                .unwrap(),
            &HeaderValue::from_static("Bearer Token")
        );
        assert_eq!(
            confidential_client
                .app_config
                .extra_query_parameters
                .get("name")
                .unwrap(),
            &String::from("123")
        );
    }

    #[test]
    fn confidential_client_builder() {
        let client_id = Uuid::new_v4();
        let confidential_client = ConfidentialClientApplicationBuilder::new(client_id)
            .with_tenant("tenant-id")
            .with_client_secret("client-secret")
            .with_scope(vec!["scope"])
            .build();

        assert_eq!(
            confidential_client.client_id().to_string(),
            client_id.to_string()
        );
    }
}
