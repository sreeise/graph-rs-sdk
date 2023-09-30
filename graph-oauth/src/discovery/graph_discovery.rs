static LOGIN_LIVE_HOST: &str = "https://login.live.com";
static MICROSOFT_ONLINE_HOST: &str = "https://login.microsoftonline.com";
static OPEN_ID_PATH: &str = ".well-known/openid-configuration";

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftSigningKeysV1 {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub token_endpoint_auth_methods_supported: Vec<String>,
    pub jwks_uri: String,
    pub response_types_supported: Vec<String>,
    pub response_modes_supported: Vec<String>,
    pub subject_types_supported: Vec<String>,
    pub scopes_supported: Vec<String>,
    pub id_token_signing_alg_values_supported: Vec<String>,
    pub claims_supported: Vec<String>,
    pub request_uri_parameter_supported: bool,
    pub end_session_endpoint: String,
    pub frontchannel_logout_supported: bool,
    pub http_logout_supported: bool,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftSigningKeysV2 {
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub token_endpoint_auth_methods_supported: Vec<String>,
    pub jwks_uri: String,
    pub response_modes_supported: Vec<String>,
    pub subject_types_supported: Vec<String>,
    pub id_token_signing_alg_values_supported: Vec<String>,
    pub http_logout_supported: bool,
    pub frontchannel_logout_supported: bool,
    pub end_session_endpoint: String,
    pub response_types_supported: Vec<String>,
    pub scopes_supported: Vec<String>,
    pub issuer: String,
    pub claims_supported: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_multi_refresh_token: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_session_iframe: Option<String>,
    pub userinfo_endpoint: String,
    pub tenant_region_scope: Option<String>,
    pub cloud_instance_name: String,
    pub cloud_graph_host_name: String,
    pub msgraph_host: String,
    pub rbac_url: String,
}

pub enum SigningKeys {
    V1,
    V2,
    Tenant(String),
}

impl SigningKeys {
    /// Get the URL for the public keys used by the Microsoft identity platform
    /// to sign security tokens.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::graph_discovery::SigningKeys;
    /// let url = SigningKeys::V1.url();
    /// println!("{}", url);
    /// ```
    pub fn url(&self) -> String {
        match self {
            SigningKeys::V1 => format!("{LOGIN_LIVE_HOST}/{OPEN_ID_PATH}"),
            SigningKeys::V2 => format!("{MICROSOFT_ONLINE_HOST}/common/v2.0/{OPEN_ID_PATH}"),
            SigningKeys::Tenant(tenant) => {
                format!("{MICROSOFT_ONLINE_HOST}/{tenant}/v2.0/{OPEN_ID_PATH}")
            }
        }
    }
}
