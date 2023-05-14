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
    #[default]
    AzureActiveDirectory,
    AzureDirectoryFederatedServices,
    /// Same as Aad. This is here since `common` is more familiar some times
    Common,
    Organizations,
    Consumers,
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
