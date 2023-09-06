use crate::identity::credentials::app_config::AppConfig;
use crate::identity::credentials::client_assertion_credential::ClientAssertionCredentialBuilder;
use crate::identity::{
    application_options::ApplicationOptions, AuthCodeAuthorizationUrlParameterBuilder, Authority,
    AuthorizationCodeCertificateCredentialBuilder, AuthorizationCodeCredentialBuilder,
    AzureCloudInstance, ClientCredentialsAuthorizationUrlBuilder, ClientSecretCredentialBuilder,
    DeviceCodeCredentialBuilder, EnvironmentCredential, OpenIdCredentialBuilder,
    PublicClientApplication,
};
#[cfg(feature = "openssl")]
use crate::identity::{ClientCertificateCredentialBuilder, X509Certificate};
use graph_error::{AuthorizationResult, AF};
use http::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::env::VarError;
use url::Url;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AuthorityHost {
    /// STS instance (for instance https://login.microsoftonline.com for the Azure public cloud).
    /// Maps to the instance url string.
    AzureCloudInstance(AzureCloudInstance),
    Uri(Url),
}

impl From<AzureCloudInstance> for AuthorityHost {
    fn from(value: AzureCloudInstance) -> Self {
        AuthorityHost::AzureCloudInstance(value)
    }
}

impl From<Url> for AuthorityHost {
    fn from(value: Url) -> Self {
        AuthorityHost::Uri(value)
    }
}

impl Default for AuthorityHost {
    fn default() -> Self {
        AuthorityHost::AzureCloudInstance(AzureCloudInstance::default())
    }
}

pub struct ConfidentialClientApplicationBuilder {
    pub(crate) app_config: AppConfig,
}

impl ConfidentialClientApplicationBuilder {
    pub fn new(client_id: impl AsRef<str>) -> Self {
        ConfidentialClientApplicationBuilder {
            app_config: AppConfig::new_with_client_id(client_id),
        }
    }

    pub fn new_with_application_options(
        application_options: ApplicationOptions,
    ) -> AuthorizationResult<ConfidentialClientApplicationBuilder> {
        ConfidentialClientApplicationBuilder::try_from(application_options)
    }

    pub fn with_tenant_id(&mut self, tenant_id: impl AsRef<str>) -> &mut Self {
        let tenant = tenant_id.as_ref().to_string();
        self.app_config.tenant_id = Some(tenant.clone());
        self.app_config.authority = Authority::TenantId(tenant);
        self
    }

    /// Extends the query parameters of both the default query params and user defined params.
    /// Does not overwrite default params.
    pub fn with_extra_query_param(&mut self, query_param: (String, String)) -> &mut Self {
        self.app_config
            .extra_query_parameters
            .insert(query_param.0, query_param.1);
        self
    }

    /// Extends the query parameters of both the default query params and user defined params.
    /// Does not overwrite default params.
    pub fn with_extra_query_parameters(
        &mut self,
        query_parameters: HashMap<String, String>,
    ) -> &mut Self {
        self.app_config
            .extra_query_parameters
            .extend(query_parameters);
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
            .extra_header_parameters
            .insert(header_name.into(), header_value.into());
        self
    }

    /// Extends the header parameters of both the default header params and user defined params.
    /// Does not overwrite default params.
    pub fn with_extra_header_parameters(&mut self, header_parameters: HeaderMap) -> &mut Self {
        self.app_config
            .extra_header_parameters
            .extend(header_parameters);
        self
    }

    pub fn authorization_code_url_builder(&mut self) -> AuthCodeAuthorizationUrlParameterBuilder {
        AuthCodeAuthorizationUrlParameterBuilder::new_with_app_config(self.app_config.clone())
    }

    pub fn client_credentials_auth_url_builder(
        &mut self,
    ) -> ClientCredentialsAuthorizationUrlBuilder {
        ClientCredentialsAuthorizationUrlBuilder::new_with_app_config(self.app_config.clone())
    }

    pub fn openid_authorization_url_builder(&mut self) -> ClientCredentialsAuthorizationUrlBuilder {
        ClientCredentialsAuthorizationUrlBuilder::new_with_app_config(self.app_config.clone())
    }

    #[cfg(feature = "openssl")]
    pub fn with_client_x509_certificate(
        self,
        certificate: &X509Certificate,
    ) -> anyhow::Result<ClientCertificateCredentialBuilder> {
        ClientCertificateCredentialBuilder::new_with_certificate(certificate, self.app_config)
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
            signed_assertion.as_ref().to_string(),
            self.app_config,
        )
    }

    pub fn with_authorization_code(
        self,
        authorization_code: impl AsRef<str>,
    ) -> AuthorizationCodeCredentialBuilder {
        AuthorizationCodeCredentialBuilder::new_with_auth_code(self.into(), authorization_code)
    }

    pub fn with_authorization_code_assertion(
        self,
        authorization_code: impl AsRef<str>,
        assertion: impl AsRef<str>,
    ) -> AuthorizationCodeCertificateCredentialBuilder {
        AuthorizationCodeCertificateCredentialBuilder::new_with_auth_code_and_assertion(
            self.into(),
            authorization_code,
            assertion,
        )
    }

    #[cfg(feature = "openssl")]
    pub fn with_authorization_code_x509_certificate(
        self,
        authorization_code: impl AsRef<str>,
        x509: &X509Certificate,
    ) -> anyhow::Result<AuthorizationCodeCertificateCredentialBuilder> {
        AuthorizationCodeCertificateCredentialBuilder::new_with_auth_code_and_x509(
            self.into(),
            authorization_code,
            x509,
        )
    }

    pub fn with_openid(
        self,
        authorization_code: impl AsRef<str>,
        client_secret: impl AsRef<str>,
    ) -> OpenIdCredentialBuilder {
        OpenIdCredentialBuilder::new_with_auth_code_and_secret(
            authorization_code,
            client_secret,
            self.app_config,
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
            app_config: AppConfig {
                tenant_id: value.tenant_id,
                client_id: Uuid::try_parse(&value.client_id.to_string()).unwrap_or_default(),
                authority: value
                    .aad_authority_audience
                    .map(Authority::from)
                    .unwrap_or_default(),
                azure_cloud_instance: value.azure_cloud_instance.unwrap_or_default(),
                extra_query_parameters: Default::default(),
                extra_header_parameters: Default::default(),
                redirect_uri: None,
            },
        })
    }
}

#[allow(dead_code)]
pub struct PublicClientApplicationBuilder {
    app_config: AppConfig,
}

impl PublicClientApplicationBuilder {
    #[allow(dead_code)]
    pub fn new(client_id: &str) -> PublicClientApplicationBuilder {
        PublicClientApplicationBuilder {
            app_config: AppConfig::new_with_client_id(client_id),
        }
    }

    #[allow(dead_code)]
    pub fn create_with_application_options(
        application_options: ApplicationOptions,
    ) -> AuthorizationResult<PublicClientApplicationBuilder> {
        PublicClientApplicationBuilder::try_from(application_options)
    }

    pub fn with_tenant_id(&mut self, tenant_id: impl AsRef<str>) -> &mut Self {
        let tenant = tenant_id.as_ref().to_string();
        self.app_config.tenant_id = Some(tenant.clone());
        self.app_config.authority = Authority::TenantId(tenant);
        self
    }

    /// Extends the query parameters of both the default query params and user defined params.
    /// Does not overwrite default params.
    pub fn with_extra_query_param(&mut self, query_param: (String, String)) -> &mut Self {
        self.app_config
            .extra_query_parameters
            .insert(query_param.0, query_param.1);
        self
    }

    /// Extends the query parameters of both the default query params and user defined params.
    /// Does not overwrite default params.
    pub fn with_extra_query_parameters(
        &mut self,
        query_parameters: HashMap<String, String>,
    ) -> &mut Self {
        self.app_config
            .extra_query_parameters
            .extend(query_parameters);
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
            .extra_header_parameters
            .insert(header_name.into(), header_value.into());
        self
    }

    /// Extends the header parameters of both the default header params and user defined params.
    /// Does not overwrite default params.
    pub fn with_extra_header_parameters(&mut self, header_parameters: HeaderMap) -> &mut Self {
        self.app_config
            .extra_header_parameters
            .extend(header_parameters);
        self
    }

    pub fn with_device_code_builder(self) -> DeviceCodeCredentialBuilder {
        DeviceCodeCredentialBuilder::new_with_app_config(self.app_config)
    }

    pub fn with_device_code(self, device_code: impl AsRef<str>) -> DeviceCodeCredentialBuilder {
        DeviceCodeCredentialBuilder::new_with_device_code(device_code.as_ref(), self.app_config)
    }

    /*
    pub fn interactive_authentication(self) -> DeviceCodeCredentialBuilder {

    }
     */

    pub fn try_from_environment() -> Result<PublicClientApplication, VarError> {
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
            app_config: AppConfig {
                tenant_id: value.tenant_id,
                client_id: value.client_id,
                authority: value
                    .aad_authority_audience
                    .map(Authority::from)
                    .unwrap_or_default(),
                azure_cloud_instance: value.azure_cloud_instance.unwrap_or_default(),
                extra_query_parameters: Default::default(),
                extra_header_parameters: Default::default(),
                redirect_uri: None,
            },
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::identity::AadAuthorityAudience;
    use http::header::AUTHORIZATION;
    use http::HeaderValue;

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
}
