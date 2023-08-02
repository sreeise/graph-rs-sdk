use std::collections::HashMap;
use anyhow::{anyhow, ensure};
use reqwest::header::HeaderMap;
use url::Url;
#[cfg(feature = "openssl")]
use crate::identity::X509Certificate;
use crate::identity::{Authority};
use crate::identity::application_options::ApplicationOptions;
use crate::oauth::{AzureCloudInstance, ConfidentialClientApplication};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AuthorityHost {
    /// STS instance (for instance https://login.microsoftonline.com for the Azure public cloud).
    /// Maps to the instance url string.
    AzureCloudInstance(AzureCloudInstance),
    Uri(Url)
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
    CertificateClientCredential(X509Certificate),
    SecretStringClientCredential(String),
    SignedAssertionClientCredential(String),
}

pub struct ConfidentialClientApplicationBuilder {
    client_id: String,
    tenant_id: Option<String>,
    authority: Authority,
    authority_url: AuthorityHost,
    redirect_uri: Option<Url>,
    default_redirect_uri: bool,
    client_credential_parameter: Option<ClientCredentialParameter>,
    extra_query_parameters: HashMap<String, String>,
    extra_header_parameters: HeaderMap,
}

impl ConfidentialClientApplicationBuilder {
    pub fn create(client_id: &str) -> ConfidentialClientApplicationBuilder {
        ConfidentialClientApplicationBuilder {
            client_id: client_id.to_owned(),
            tenant_id: None,
            authority: Default::default(),
            authority_url: Default::default(),
            redirect_uri: None,
            default_redirect_uri: false,
            client_credential_parameter: None,
            extra_query_parameters: Default::default(),
            extra_header_parameters: Default::default(),
        }
    }

    pub fn create_with_application_options(application_options: ApplicationOptions) -> anyhow::Result<ConfidentialClientApplicationBuilder> {
        ConfidentialClientApplicationBuilder::try_from(application_options)
    }

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

    pub fn with_azure_cloud_instance(&mut self, azure_cloud_instance: AzureCloudInstance) -> &mut Self {
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

    #[cfg(feature = "openssl")]
    pub fn with_certificate(&mut self, certificate: X509Certificate) -> &mut self {
        self.client_credential_parameter = Some(ClientCredentialParameter::CertificateClientCredential(certificate));
        self
    }

    pub fn with_client_secret(&mut self, client_secret: impl AsRef<str>) -> &mut Self {
        self.client_credential_parameter = Some(ClientCredentialParameter::SecretStringClientCredential(client_secret.as_ref().to_owned()));
        self
    }

    pub fn with_signed_assertion(&mut self, signed_assertion: impl AsRef<str>) -> &mut Self {
        self.client_credential_parameter = Some(ClientCredentialParameter::SignedAssertionClientCredential(signed_assertion.as_ref().to_owned()));
        self
    }

    pub fn with_extra_query_parameters(&mut self, query_parameters: HashMap<String, String>) -> &mut Self {
        self.extra_query_parameters = query_parameters;
        self
    }

    pub fn with_extra_header_parameters(&mut self, header_parameters: HeaderMap) -> &mut Self {
        self.extra_header_parameters = header_parameters;
        self
    }
}

impl TryFrom<ApplicationOptions> for ConfidentialClientApplicationBuilder {
    type Error = anyhow::Error;

    fn try_from(value: ApplicationOptions) -> Result<Self, Self::Error> {
        anyhow::ensure!(value.client_id.is_empty(), "Client id cannot be empty");
        anyhow::ensure!(!(value.instance.is_some() && value.azure_cloud_instance.is_some()), "Instance and AzureCloudInstance cannot both be set");
        anyhow::ensure!(!(value.tenant_id.is_some() && value.aad_authority_audience.is_some()), "TenantId and AadAuthorityAudience cannot both be set");
        let default_redirect_uri = value.redirect_uri.is_none();

        Ok(ConfidentialClientApplicationBuilder {
            client_id: value.client_id,
            tenant_id: value.tenant_id,
            authority: value.aad_authority_audience.map(|aud| Authority::from(aud))
                .unwrap_or_default(),
            authority_url: value.azure_cloud_instance.map(|aci| AuthorityHost::AzureCloudInstance(aci))
                .unwrap_or_default(),
            redirect_uri: value.redirect_uri,
            default_redirect_uri,
            client_credential_parameter: None,
            extra_query_parameters: Default::default(),
            extra_header_parameters: Default::default(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn error_result_on_instance_and_aci() {
        ConfidentialClientApplicationBuilder::try_from(ApplicationOptions {
            client_id: "client-id".to_string(),
            tenant_id: None,
            aad_authority_audience: None,
            instance: Some(Url::parse("https://login.microsoft.com").unwrap()),
            azure_cloud_instance: Some(AzureCloudInstance::AzurePublic),
            redirect_uri: None,
        }).unwrap();
    }
}
