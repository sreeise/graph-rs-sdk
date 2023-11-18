use std::fmt::Display;
use url::{ParseError, Url};

lazy_static! {
    pub static ref AZURE_PUBLIC_CLOUD_INSTANCE: Url = {
        Url::parse(AzureCloudInstance::AzurePublic.as_ref())
            .expect("Unable to create Azure Public Cloud Instance Url")
    };
    pub static ref AZURE_CHINA_CLOUD_INSTANCE: Url = {
        Url::parse(AzureCloudInstance::AzureChina.as_ref())
            .expect("Unable to create Azure China Cloud Instance Url")
    };
    pub static ref AZURE_GERMANY_CLOUD_INSTANCE: Url = {
        Url::parse(AzureCloudInstance::AzureGermany.as_ref())
            .expect("Unable to create Azure Germany Cloud Instance Url")
    };
    pub static ref AZURE_US_GOVERNMENT: Url = {
        Url::parse(AzureCloudInstance::AzureUsGovernment.as_ref())
            .expect("Unable to create Azure Us Government Cloud Instance Url")
    };
}

/// STS instance (for instance https://login.microsoftonline.com for the Azure public cloud).
/// Maps to the instance url string.
#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum AzureCloudInstance {
    /// Microsoft Azure public cloud. Maps to https://login.microsoftonline.com
    #[default]
    AzurePublic,
    /// Microsoft Chinese national cloud. Maps to https://login.chinacloudapi.cn
    AzureChina,
    /// Microsoft German national cloud ("Black Forest"). Maps to https://login.microsoftonline.de
    AzureGermany,
    /// US Government cloud. Maps to https://login.microsoftonline.us
    AzureUsGovernment,
}

impl AzureCloudInstance {
    pub fn get_open_id_configuration_url(&self, authority: Authority) -> String {
        format!("{}/v2.0/{}", self.as_ref(), authority.as_ref())
    }
}

impl AsRef<str> for AzureCloudInstance {
    fn as_ref(&self) -> &str {
        match self {
            AzureCloudInstance::AzurePublic => "https://login.microsoftonline.com",
            AzureCloudInstance::AzureChina => "https://login.chinacloudapi.cn",
            AzureCloudInstance::AzureGermany => "https://login.microsoftonline.de",
            AzureCloudInstance::AzureUsGovernment => "https://login.microsoftonline.us",
        }
    }
}

impl From<&AzureCloudInstance> for Url {
    fn from(value: &AzureCloudInstance) -> Self {
        match value {
            AzureCloudInstance::AzurePublic => AZURE_PUBLIC_CLOUD_INSTANCE.clone(),
            AzureCloudInstance::AzureChina => AZURE_CHINA_CLOUD_INSTANCE.clone(),
            AzureCloudInstance::AzureGermany => AZURE_GERMANY_CLOUD_INSTANCE.clone(),
            AzureCloudInstance::AzureUsGovernment => AZURE_US_GOVERNMENT.clone(),
        }
    }
}

impl From<AzureCloudInstance> for Url {
    fn from(value: AzureCloudInstance) -> Self {
        match value {
            AzureCloudInstance::AzurePublic => AZURE_PUBLIC_CLOUD_INSTANCE.clone(),
            AzureCloudInstance::AzureChina => AZURE_CHINA_CLOUD_INSTANCE.clone(),
            AzureCloudInstance::AzureGermany => AZURE_GERMANY_CLOUD_INSTANCE.clone(),
            AzureCloudInstance::AzureUsGovernment => AZURE_US_GOVERNMENT.clone(),
        }
    }
}

impl AzureCloudInstance {
    pub fn auth_uri(&self, authority: &Authority) -> Result<Url, ParseError> {
        Url::parse(&format!(
            "{}/{}/oauth2/v2.0/authorize",
            self.as_ref(),
            authority.as_ref()
        ))
    }

    pub fn token_uri(&self, authority: &Authority) -> Result<Url, ParseError> {
        Url::parse(&format!(
            "{}/{}/oauth2/v2.0/token",
            self.as_ref(),
            authority.as_ref()
        ))
    }

    pub fn admin_consent_uri(&self, authority: &Authority) -> Result<Url, ParseError> {
        Url::parse(&format!(
            "{}/{}/adminconsent",
            self.as_ref(),
            authority.as_ref()
        ))
    }

    pub fn device_code_uri(&self, authority: &Authority) -> Result<Url, ParseError> {
        Url::parse(&format!(
            "{}/{}/oauth2/v2.0/devicecode",
            self.as_ref(),
            authority.as_ref()
        ))
    }

    /*
    pub fn default_microsoft_graph_scope(&self) -> &'static str {
        "https://graph.microsoft.com/.default"
    }

    pub fn default_managed_identity_scope(&self) -> &'static str {
        match self {
            AzureCloudInstance::AzurePublic => "https://management.azure.com//.default",
            AzureCloudInstance::AzureChina => "https://management.chinacloudapi.cn/.default",
            AzureCloudInstance::AzureGermany => "https://management.microsoftazure.de/.default",
            AzureCloudInstance::AzureUsGovernment => {
                "https://management.usgovcloudapi.net/.default"
            }
        }
    }
    */
}

/// Specifies which Microsoft accounts can be used for sign-in with a given application.
/// See https://aka.ms/msal-net-application-configuration
///
/// [AadAuthorityAudience] uses the application names selected in the Azure Portal and
/// maps to [Authority]
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AadAuthorityAudience {
    /// Users with a Microsoft work or school account in my organization’s Azure AD tenant (i.e. single tenant).
    /// Maps to https://[AzureCloudInstance]/[AadAuthorityAudience::AzureAdMyOrg(tenant_id)]
    /// or https://[instance]/[tenant_id]
    ///
    /// # Using Tenant Id
    /// ```rust
    /// # use graph_oauth::oauth::AadAuthorityAudience;
    /// let authority_audience = AadAuthorityAudience::AzureAdMyOrg("tenant_id".into());
    /// ```
    AzureAdMyOrg(String),

    /// Users with a personal Microsoft account, or a work or school account in any organization’s Azure AD tenant
    /// Maps to https://[AzureCloudInstance]/common/ or https://[instance]/[common]/\
    #[default]
    AzureAdAndPersonalMicrosoftAccount,

    /// Users with a Microsoft work or school account in any organization’s Azure AD tenant (i.e. multi-tenant).
    /// Maps to https://[AzureCloudInstance]/organizations/ or https://[instance]/organizations/
    AzureAdMultipleOrgs,

    /// Users with a personal Microsoft account. Maps to https://[AzureCloudInstance]/consumers/
    /// or https://[instance]/consumers/
    PersonalMicrosoftAccount,
}

impl AadAuthorityAudience {
    pub fn as_str(&self) -> &str {
        match self {
            AadAuthorityAudience::AzureAdMyOrg(tenant) => tenant.as_str(),
            AadAuthorityAudience::AzureAdAndPersonalMicrosoftAccount => "common",
            AadAuthorityAudience::AzureAdMultipleOrgs => "organizations",
            AadAuthorityAudience::PersonalMicrosoftAccount => "consumers",
        }
    }
}

impl AsRef<str> for AadAuthorityAudience {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for AadAuthorityAudience {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for AadAuthorityAudience {
    fn from(value: &str) -> Self {
        match value.as_bytes() {
            b"common" => AadAuthorityAudience::AzureAdAndPersonalMicrosoftAccount,
            b"organizations" => AadAuthorityAudience::AzureAdMultipleOrgs,
            b"consumers" => AadAuthorityAudience::PersonalMicrosoftAccount,
            _ => AadAuthorityAudience::AzureAdMyOrg(value.to_string()),
        }
    }
}

/// Specifies which Microsoft accounts can be used for sign-in with a given application.
/// See https://aka.ms/msal-net-application-configuration
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Authority {
    /// Users with both a personal Microsoft account and a work or school account
    /// from Azure AD can sign in to the application.
    /// /// Maps to https://[AzureCloudInstance]/common/
    ///
    /// [Authority::AzureActiveDirectory] is the same as [Authority::Common].
    /// [Authority::Common] is a convenience enum variant that may be more
    /// familiar with it from the Microsoft Identity Platform documentation.
    #[default]
    AzureActiveDirectory,
    AzureDirectoryFederatedServices,
    /// Users with both a personal Microsoft account and a work or school account
    /// from Azure AD can sign in to the application.
    /// Maps to https://[instance]/common/
    ///
    /// [Authority::Common] is the same as [Authority::AzureActiveDirectory].
    ///
    /// [Authority::Common] is a convenience enum variant that may be more
    /// familiar with it from the Microsoft Identity Platform documentation.
    Common,
    /// Only users with work or school accounts from Azure AD can sign in to the application.
    Organizations,
    /// Only users with a personal Microsoft account can sign in to the application.
    Consumers,
    /// The value can be the domain name of the Azure AD tenant or the tenant ID in GUID format.
    /// You can also use the consumer tenant GUID, 9188040d-6c67-4c5b-b112-36a304b66dad,
    /// in place of consumers.
    ///
    /// Only users from a specific Azure AD tenant (directory members with a work or
    /// school account or directory guests with a personal Microsoft account) can sign in
    /// to the application.
    TenantId(String),
}

impl Authority {
    pub fn tenant_id(&self) -> Option<&String> {
        match self {
            Authority::TenantId(tenant_id) => Some(tenant_id),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Authority::AzureActiveDirectory | Authority::Common => "common",
            Authority::AzureDirectoryFederatedServices => "adfs",
            Authority::Organizations => "organizations",
            Authority::Consumers => "consumers",
            Authority::TenantId(tenant_id) => tenant_id.as_str(),
        }
    }
}

impl From<&AadAuthorityAudience> for Authority {
    fn from(value: &AadAuthorityAudience) -> Self {
        match value {
            AadAuthorityAudience::AzureAdAndPersonalMicrosoftAccount => Authority::Common,
            AadAuthorityAudience::AzureAdMyOrg(tenant_id) => Authority::TenantId(tenant_id.clone()),
            AadAuthorityAudience::AzureAdMultipleOrgs => Authority::Organizations,
            AadAuthorityAudience::PersonalMicrosoftAccount => Authority::Consumers,
        }
    }
}

impl From<AadAuthorityAudience> for Authority {
    fn from(value: AadAuthorityAudience) -> Self {
        match value {
            AadAuthorityAudience::AzureAdAndPersonalMicrosoftAccount => Authority::Common,
            AadAuthorityAudience::AzureAdMyOrg(tenant_id) => Authority::TenantId(tenant_id),
            AadAuthorityAudience::AzureAdMultipleOrgs => Authority::Organizations,
            AadAuthorityAudience::PersonalMicrosoftAccount => Authority::Consumers,
        }
    }
}

impl AsRef<str> for Authority {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for Authority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for Authority {
    fn from(value: &str) -> Self {
        match value.as_bytes() {
            b"aad" => Authority::AzureActiveDirectory,
            b"common" => Authority::Common,
            b"adfs" => Authority::AzureDirectoryFederatedServices,
            b"organizations" => Authority::Organizations,
            b"consumers" => Authority::Consumers,
            _ => Authority::TenantId(value.to_owned()),
        }
    }
}
