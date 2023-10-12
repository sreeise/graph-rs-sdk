use url::Url;
use uuid::Uuid;

use crate::identity::AadAuthorityAudience;
use crate::oauth::AzureCloudInstance;

/// Application Options typically stored as JSON file in .net applications.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct ApplicationOptions {
    /// Client ID (also known as App ID) of the application as registered in the
    /// application registration portal (https://aka.ms/msal-net-register-app)
    /// Required parameter for ApplicationOptions.
    #[serde(alias = "clientId", alias = "ClientId", alias = "client_id")]
    pub client_id: Uuid,
    /// Tenant from which the application will allow users to sign it. This can be:
    /// a domain associated with a tenant, a GUID (tenant id), or a meta-tenant (e.g. consumers).
    /// This property is mutually exclusive with [AadAuthorityAudience]. If both
    /// are provided, an error result will be returned when mapping to [crate::identity::ConfidentialClientApplication]
    #[serde(alias = "tenantId", alias = "TenantId", alias = "tenant_id")]
    pub tenant_id: Option<String>,
    #[serde(
        alias = "aadAuthorityAudience",
        alias = "AadAuthorityAudience",
        alias = "aad_authority_audience"
    )]
    pub aad_authority_audience: Option<AadAuthorityAudience>,
    #[serde(alias = "instance", alias = "Instance")]
    pub instance: Option<Url>,
    #[serde(
        alias = "azureCloudInstance",
        alias = "AzureCloudInstance",
        alias = "azure_cloud_instance"
    )]
    pub azure_cloud_instance: Option<AzureCloudInstance>,
    #[serde(alias = "redirectUri", alias = "RedirectUri", alias = "redirect_uri")]
    pub redirect_uri: Option<Url>,
}

impl ApplicationOptions {
    pub fn new(client_id: impl AsRef<str>) -> ApplicationOptions {
        ApplicationOptions {
            client_id: Uuid::try_parse(client_id.as_ref())
                .expect("Invalid Client Id - Must be a valid Uuid"),
            tenant_id: None,
            aad_authority_audience: None,
            instance: None,
            azure_cloud_instance: None,
            redirect_uri: None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;

    use super::*;

    #[test]
    fn application_options_from_file() {
        let file =
            File::open(r#"./src/identity/credentials/test/application_options/aad_options.json"#)
                .unwrap();
        let application_options: ApplicationOptions = serde_json::from_reader(file).unwrap();
        assert_eq!(
            application_options.aad_authority_audience,
            Some(AadAuthorityAudience::PersonalMicrosoftAccount)
        );
    }
}
