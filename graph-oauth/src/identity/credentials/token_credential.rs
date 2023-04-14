use crate::identity::AzureAuthorityHost;

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TokenCredentialOptions {
    pub(crate) azure_cloud_endpoint: AzureAuthorityHost,
}
