use url::Url;

/// STS instance (for instance https://login.microsoftonline.com for the Azure public cloud).
/// Authentication libraries from Microsoft (this is not one) call this the
/// AzureCloudInstance enum or the Instance url string.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AzureAuthorityHost {
    /// Custom Value communicating that the AzureCloudInstance.
    Custom(String),
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

impl AsRef<str> for AzureAuthorityHost {
    fn as_ref(&self) -> &str {
        match self {
            AzureAuthorityHost::Custom(url) => url.as_str(),
            AzureAuthorityHost::AzurePublic => "https://login.microsoftonline.com",
            AzureAuthorityHost::AzureChina => "https://login.chinacloudapi.cn",
            AzureAuthorityHost::AzureGermany => "https://login.microsoftonline.de",
            AzureAuthorityHost::AzureUsGovernment => "https://login.microsoftonline.us",
        }
    }
}

impl TryFrom<AzureAuthorityHost> for Url {
    type Error = url::ParseError;

    fn try_from(azure_cloud_instance: AzureAuthorityHost) -> Result<Self, Self::Error> {
        Url::parse(azure_cloud_instance.as_ref())
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
            b"common" => Authority::Common,
            b"adfs" => Authority::AzureDirectoryFederatedServices,
            b"organizations" => Authority::Organizations,
            b"consumers" => Authority::Consumers,
            _ => Authority::TenantId(value.to_owned()),
        }
    }
}
