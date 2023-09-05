use crate::identity::AzureCloudInstance;
use reqwest::header::HeaderMap;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TokenCredentialOptions {
    pub azure_authority_host: AzureCloudInstance,

    pub extra_query_parameters: HashMap<String, String>,

    pub extra_header_parameters: HeaderMap,

    /// Specifies if the token request will ignore the access token in the token cache
    /// and will attempt to acquire a new access token.
    pub force_refresh: bool,

    /// Enables to override the tenant/account for which to get a token.
    /// This is useful in multi-tenant apps in the cases where a given user account is a guest
    /// in other tenants, and you want to acquire tokens for a specific tenant.
    pub tenant: Option<String>,
}
