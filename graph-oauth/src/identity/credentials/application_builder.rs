use crate::identity::credentials::app_config::AppConfig;
use crate::identity::credentials::client_assertion_credential::ClientAssertionCredentialBuilder;
#[cfg(feature = "openssl")]
use crate::identity::X509Certificate;
use crate::identity::{
    application_options::ApplicationOptions, AuthCodeAuthorizationUrlParameterBuilder, Authority,
    AuthorizationCodeCertificateCredentialBuilder, AuthorizationCodeCredential,
    AuthorizationCodeCredentialBuilder, AzureCloudInstance, ClientCertificateCredential,
    ClientCertificateCredentialBuilder, ClientCredentialsAuthorizationUrlBuilder,
    ClientSecretCredentialBuilder,
};
use crate::oauth::ConfidentialClientApplication;
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use url::Url;

macro_rules! application_builder_impl {
    ($name:ident) => {
        impl $name {
            pub fn with_client_id(&mut self, client_id: impl AsRef<str>) -> &mut Self {
                self.client_id = client_id.as_ref().to_owned();
                self
            }

            pub fn with_tenant_id(&mut self, tenant_id: impl AsRef<str>) -> &mut Self {
                self.tenant_id = Some(tenant_id.as_ref().to_owned());
                self.authority = Authority::TenantId(tenant_id.as_ref().to_owned());
                self
            }

            pub fn with_authority<T: Into<AuthorityHost>, U: Into<Authority>>(
                &mut self,
                authority_host: T,
                authority: U,
            ) -> &mut Self {
                self.authority_url = authority_host.into();
                self.authority = authority.into();
                self
            }

            /// Adds a known Azure AD authority to the application to sign-in users specifying
            /// the full authority Uri. See https://aka.ms/msal-net-application-configuration.
            pub fn with_authority_uri(&mut self, authority_uri: Url) -> &mut Self {
                self.authority_url = AuthorityHost::Uri(authority_uri);
                self
            }

            pub fn with_azure_cloud_instance(
                &mut self,
                azure_cloud_instance: AzureCloudInstance,
            ) -> &mut Self {
                self.authority_url = AuthorityHost::AzureCloudInstance(azure_cloud_instance);
                self
            }

            pub fn with_redirect_uri(&mut self, redirect_uri: Url) -> &mut Self {
                self.redirect_uri = Some(redirect_uri);
                self.default_redirect_uri = false;
                self
            }

            pub fn with_default_redirect_uri(&mut self) -> &mut Self {
                self.default_redirect_uri = true;
                self
            }

            pub fn with_extra_query_parameters<F: Fn(&mut HashMap<String, String>)>(
                &mut self,
                f: F,
            ) -> &mut Self {
                f(&mut self.extra_query_parameters);
                self
            }

            pub fn with_extra_header_parameters<F: Fn(&mut HeaderMap)>(
                &mut self,
                f: F,
            ) -> &mut Self {
                f(&mut self.extra_header_parameters);
                self
            }
        }
    };
}

/*
pub fn with_extra_query_parameters(
                &mut self,
                query_parameters: HashMap<String, String>,
            ) -> &mut Self {
                self.extra_query_parameters = query_parameters;
                self
            }

            pub fn with_extra_header_parameters(&mut self, header_parameters: HeaderMap) -> &mut Self {
                self.extra_header_parameters = header_parameters;
                self
            }
 */

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

pub enum ClientCredentialParameter {
    #[cfg(feature = "openssl")]
    Certificate(X509Certificate),
    SecretString(String),
    SignedAssertion(String),
}

pub struct ConfidentialClientApplicationBuilder {
    app_config: AppConfig,
    default_redirect_uri: bool,
    redirect_uri: Option<Url>,
    client_credential_parameter: Option<ClientCredentialParameter>,
}

// application_builder_impl!(ConfidentialClientApplicationBuilder);

impl ConfidentialClientApplicationBuilder {
    pub fn new(client_id: impl AsRef<str>) -> ConfidentialClientApplicationBuilder {
        ConfidentialClientApplicationBuilder {
            app_config: AppConfig::new_with_client_id(client_id),
            default_redirect_uri: false,
            redirect_uri: None,
            client_credential_parameter: None,
        }
    }

    pub fn new_with_application_options(
        application_options: ApplicationOptions,
    ) -> anyhow::Result<ConfidentialClientApplicationBuilder> {
        ConfidentialClientApplicationBuilder::try_from(application_options)
    }

    pub fn get_authorization_request_url<T: ToString, I: IntoIterator<Item = T>>(
        &mut self,
        scopes: I,
    ) -> AuthCodeAuthorizationUrlParameterBuilder {
        let mut builder = AuthCodeAuthorizationUrlParameterBuilder::new();
        builder.with_scope(scopes);
        builder
    }

    pub fn get_client_credential_request_url(
        &mut self,
    ) -> ClientCredentialsAuthorizationUrlBuilder {
        ClientCredentialsAuthorizationUrlBuilder::new()
    }

    #[cfg(feature = "openssl")]
    pub fn with_client_certificate_credential(
        self,
        certificate: &X509Certificate,
    ) -> anyhow::Result<ClientCertificateCredentialBuilder> {
        ClientCertificateCredentialBuilder::new_with_certificate(certificate, self.app_config)
    }

    pub fn with_client_secret_credential(
        self,
        client_secret: impl AsRef<str>,
    ) -> ClientSecretCredentialBuilder {
        ClientSecretCredentialBuilder::new_with_client_secret(client_secret, self.app_config)
    }

    pub fn with_client_assertion_credential(
        self,
        signed_assertion: impl AsRef<str>,
    ) -> ClientAssertionCredentialBuilder {
        ClientAssertionCredentialBuilder::new_with_signed_assertion(
            signed_assertion.as_ref().to_string(),
            self.app_config,
        )
    }

    pub fn with_authorization_code_credential(
        self,
        authorization_code: impl AsRef<str>,
    ) -> AuthorizationCodeCredentialBuilder {
        AuthorizationCodeCredentialBuilder::new_with_auth_code(self.into(), authorization_code)
    }

    pub fn with_authorization_code_assertion_credential(
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
    pub fn with_authorization_code_certificate_credential(
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
}

impl From<ConfidentialClientApplicationBuilder> for AppConfig {
    fn from(value: ConfidentialClientApplicationBuilder) -> Self {
        value.app_config
    }
}

impl TryFrom<ApplicationOptions> for ConfidentialClientApplicationBuilder {
    type Error = anyhow::Error;

    fn try_from(value: ApplicationOptions) -> Result<Self, Self::Error> {
        anyhow::ensure!(!value.client_id.is_empty(), "Client id cannot be empty");
        anyhow::ensure!(
            !(value.instance.is_some() && value.azure_cloud_instance.is_some()),
            "Instance and AzureCloudInstance both specify the azure cloud instance and cannot be set at the same time"
        );
        anyhow::ensure!(
            !(value.tenant_id.is_some() && value.aad_authority_audience.is_some()),
            "TenantId and AadAuthorityAudience both represent an authority audience and cannot be set at the same time"
        );

        /*
        client_id: value.client_id,
            tenant_id: value.tenant_id,
            authority: value
                .aad_authority_audience
                .map(Authority::from)
                .unwrap_or_default(),
            authority_url: value
                .azure_cloud_instance
                .map(AuthorityHost::AzureCloudInstance)
                .unwrap_or_default(),
         */

        Ok(ConfidentialClientApplicationBuilder {
            app_config: AppConfig {
                tenant_id: value.tenant_id,
                client_id: value.client_id,
                authority: value
                    .aad_authority_audience
                    .map(Authority::from)
                    .unwrap_or_default(),
                authority_url: value
                    .azure_cloud_instance
                    .map(AuthorityHost::AzureCloudInstance)
                    .unwrap_or_default(),
                extra_query_parameters: Default::default(),
                extra_header_parameters: Default::default(),
                redirect_uri: None,
            },
            default_redirect_uri: value.redirect_uri.is_none(),
            redirect_uri: value.redirect_uri,
            client_credential_parameter: None,
        })
    }
}

pub struct ConfidentialClientAppSelectionBuilder {
    builder: ConfidentialClientApplicationBuilder,
}

impl ConfidentialClientAppSelectionBuilder {}

pub struct PublicClientApplicationBuilder {
    client_id: String,
    tenant_id: Option<String>,
    authority: Authority,
    authority_url: AuthorityHost,
    redirect_uri: Option<Url>,
    default_redirect_uri: bool,
    extra_query_parameters: HashMap<String, String>,
    extra_header_parameters: HeaderMap,
}

application_builder_impl!(PublicClientApplicationBuilder);

impl PublicClientApplicationBuilder {
    pub fn new(client_id: &str) -> PublicClientApplicationBuilder {
        PublicClientApplicationBuilder {
            client_id: client_id.to_owned(),
            tenant_id: None,
            authority: Default::default(),
            authority_url: Default::default(),
            default_redirect_uri: false,
            redirect_uri: None,
            extra_query_parameters: Default::default(),
            extra_header_parameters: Default::default(),
        }
    }

    pub fn create_with_application_options(
        application_options: ApplicationOptions,
    ) -> anyhow::Result<PublicClientApplicationBuilder> {
        PublicClientApplicationBuilder::try_from(application_options)
    }
}

impl TryFrom<ApplicationOptions> for PublicClientApplicationBuilder {
    type Error = anyhow::Error;

    fn try_from(value: ApplicationOptions) -> Result<Self, Self::Error> {
        anyhow::ensure!(!value.client_id.is_empty(), "Client id cannot be empty");
        anyhow::ensure!(
            !(value.instance.is_some() && value.azure_cloud_instance.is_some()),
            "Instance and AzureCloudInstance both specify the azure cloud instance and cannot be set at the same time"
        );
        anyhow::ensure!(
            !(value.tenant_id.is_some() && value.aad_authority_audience.is_some()),
            "TenantId and AadAuthorityAudience both represent an authority audience and cannot be set at the same time"
        );

        Ok(PublicClientApplicationBuilder {
            client_id: value.client_id,
            tenant_id: value.tenant_id,
            authority: value
                .aad_authority_audience
                .map(Authority::from)
                .unwrap_or_default(),
            authority_url: value
                .azure_cloud_instance
                .map(AuthorityHost::AzureCloudInstance)
                .unwrap_or_default(),
            default_redirect_uri: value.redirect_uri.is_none(),
            redirect_uri: value.redirect_uri,
            extra_query_parameters: Default::default(),
            extra_header_parameters: Default::default(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::oauth::AadAuthorityAudience;
    use reqwest::header::AUTHORIZATION;
    use wry::http::HeaderValue;

    #[test]
    #[should_panic]
    fn confidential_client_error_result_on_instance_and_aci() {
        ConfidentialClientApplicationBuilder::try_from(ApplicationOptions {
            client_id: "client-id".to_string(),
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
            client_id: "client-id".to_owned(),
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
            client_id: "client-id".to_string(),
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
            client_id: "client-id".to_owned(),
            tenant_id: Some("tenant_id".to_owned()),
            aad_authority_audience: Some(AadAuthorityAudience::AzureAdAndPersonalMicrosoftAccount),
            instance: None,
            azure_cloud_instance: None,
            redirect_uri: None,
        })
        .unwrap();
    }

    /*
       #[test]
       fn extra_parameters() {
           let mut confidential_client = ConfidentialClientApplicationBuilder::new("client-id");
           confidential_client.with_extra_query_parameters(|query| {
               query.insert("name".into(), "123".into());
           })
               .with_extra_header_parameters(|map| {
                   map.insert(AUTHORIZATION, HeaderValue::from_static("Bearer Token"));
               });
           assert_eq!(confidential_client.extra_header_parameters.get(AUTHORIZATION).unwrap(), &HeaderValue::from_static("Bearer Token"));
           assert_eq!(confidential_client.extra_query_parameters.get("name").unwrap(), &String::from("123"));
       }
    */
}
