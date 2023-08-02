use url::Url;

/// STS instance (for instance https://login.microsoftonline.com for the Azure public cloud).
/// Maps to the instance url string.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AzureCloudInstance {
    // Custom Value communicating that the AzureCloudInstance.
    //Custom(String),
    /// Microsoft Azure public cloud. Maps to https://login.microsoftonline.com
    #[default]
    AzurePublic,
    /// Microsoft Chinese national cloud. Maps to https://login.chinacloudapi.cn
    AzureChina,
    /// Microsoft German national cloud ("Black Forest"). Maps to https://login.microsoftonline.de
    AzureGermany,
    /// US Government cloud. Maps to https://login.microsoftonline.us
    AzureUsGovernment,
    /// Legacy OneDrive and SharePoint. Maps to "https://login.live.com/oauth20_desktop.srf"
    OneDriveAndSharePoint,
}

impl AsRef<str> for AzureCloudInstance {
    fn as_ref(&self) -> &str {
        match self {
            AzureCloudInstance::AzurePublic => "https://login.microsoftonline.com",
            AzureCloudInstance::AzureChina => "https://login.chinacloudapi.cn",
            AzureCloudInstance::AzureGermany => "https://login.microsoftonline.de",
            AzureCloudInstance::AzureUsGovernment => "https://login.microsoftonline.us",
            AzureCloudInstance::OneDriveAndSharePoint => {
                "https://login.live.com/oauth20_desktop.srf"
            }
        }
    }
}

impl TryFrom<AzureCloudInstance> for Url {
    type Error = url::ParseError;

    fn try_from(azure_cloud_instance: AzureCloudInstance) -> Result<Self, Self::Error> {
        Url::parse(azure_cloud_instance.as_ref())
    }
}

impl AzureCloudInstance {
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
            AzureCloudInstance::OneDriveAndSharePoint => "",
        }
    }
}

/// Specifies which Microsoft accounts can be used for sign-in with a given application.
/// See https://aka.ms/msal-net-application-configuration
///
/// [AadAuthorityAudience] uses the application names selected in the Azure Portal and
/// maps to [Authority]
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AadAuthorityAudience {
    /// The sign-in audience was not specified
    #[default]
    None,

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
    /// Maps to https://[AzureCloudInstance]/common/ or https://[instance]/[common]/
    AzureAdAndPersonalMicrosoftAccount,

    /// Users with a Microsoft work or school account in any organization’s Azure AD tenant (i.e. multi-tenant).
    /// Maps to https://[AzureCloudInstance]/organizations/ or https://[instance]/organizations/
    AzureAdMultipleOrgs,

    /// Users with a personal Microsoft account. Maps to https://[AzureCloudInstance]/consumers/
    /// or https://[instance]/consumers/
    PersonalMicrosoftAccount
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
}

impl From<AadAuthorityAudience> for Authority {
    fn from(value: AadAuthorityAudience) -> Self {
        match value {
            AadAuthorityAudience::None => Authority::Common,
            AadAuthorityAudience::AzureAdMyOrg(tenant_id) => Authority::TenantId(tenant_id),
            AadAuthorityAudience::AzureAdAndPersonalMicrosoftAccount => Authority::Common,
            AadAuthorityAudience::AzureAdMultipleOrgs => Authority::Organizations,
            AadAuthorityAudience::PersonalMicrosoftAccount => Authority::Consumers,
        }
    }
}

impl AsRef<str> for Authority {
    fn as_ref(&self) -> &str {
        match self {
            Authority::AzureActiveDirectory | Authority::Common => "common",
            Authority::AzureDirectoryFederatedServices => "adfs",
            Authority::Organizations => "organizations",
            Authority::Consumers => "consumers",
            Authority::TenantId(tenant_id) => tenant_id.as_str(),
        }
    }
}

impl ToString for Authority {
    fn to_string(&self) -> String {
        String::from(self.as_ref())
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
