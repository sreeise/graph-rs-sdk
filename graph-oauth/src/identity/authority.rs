use url::Url;

/// STS instance (for instance https://login.microsoftonline.com for the Azure public cloud).
/// Authentication libraries from Microsoft (this is not one) call this the
/// AzureCloudInstance enum or the Instance url string.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AzureAuthorityHost {
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

    OneDriveAndSharePoint,
}

impl AsRef<str> for AzureAuthorityHost {
    fn as_ref(&self) -> &str {
        match self {
            AzureAuthorityHost::AzurePublic => "https://login.microsoftonline.com",
            AzureAuthorityHost::AzureChina => "https://login.chinacloudapi.cn",
            AzureAuthorityHost::AzureGermany => "https://login.microsoftonline.de",
            AzureAuthorityHost::AzureUsGovernment => "https://login.microsoftonline.us",
            AzureAuthorityHost::OneDriveAndSharePoint => {
                "https://login.live.com/oauth20_desktop.srf"
            }
        }
    }
}

impl TryFrom<AzureAuthorityHost> for Url {
    type Error = url::ParseError;

    fn try_from(azure_cloud_instance: AzureAuthorityHost) -> Result<Self, Self::Error> {
        Url::parse(azure_cloud_instance.as_ref())
    }
}

impl AzureAuthorityHost {
    pub fn default_microsoft_graph_scope(&self) -> &'static str {
        "https://graph.microsoft.com/.default"
    }

    pub fn default_managed_identity_scope(&self) -> &'static str {
        match self {
            AzureAuthorityHost::AzurePublic => "https://management.azure.com//.default",
            AzureAuthorityHost::AzureChina => "https://management.chinacloudapi.cn/.default",
            AzureAuthorityHost::AzureGermany => "https://management.microsoftazure.de/.default",
            AzureAuthorityHost::AzureUsGovernment => {
                "https://management.usgovcloudapi.net/.default"
            }
            AzureAuthorityHost::OneDriveAndSharePoint => "",
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Authority {
    /// Users with both a personal Microsoft account and a work or school account
    /// from Azure AD can sign in to the application.
    ///
    /// [Authority::AzureActiveDirectory] is the same as [Authority::Common].
    /// [Authority::Common] is a convenience enum variant that may be more
    /// familiar with it from the Microsoft Identity Platform documentation.
    #[default]
    AzureActiveDirectory,
    AzureDirectoryFederatedServices,
    /// Users with both a personal Microsoft account and a work or school account
    /// from Azure AD can sign in to the application.
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
