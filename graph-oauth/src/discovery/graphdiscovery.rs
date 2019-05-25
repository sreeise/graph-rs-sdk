use crate::grants::GrantType;
use crate::oauth::wellknown::{Commons, WellKnown};
use crate::oauth::{OAuth, OAuthError};
use from_to_file::*;

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize, FromToFile)]
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

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize, FromToFile)]
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
    pub microsoft_multi_refresh_token: bool,
    pub check_session_iframe: String,
    pub userinfo_endpoint: String,
    pub tenant_region_scope: Option<String>,
    pub cloud_instance_name: String,
    pub cloud_graph_host_name: String,
    pub msgraph_host: String,
    pub rbac_url: String,
}

pub enum GraphDiscovery {
    V1,
    V2,
    Tenant(String),
}

impl GraphDiscovery {
    pub fn signing_keys<T>(self) -> Result<T, OAuthError>
    where
        T: serde::Serialize,
        for<'de> T: serde::Deserialize<'de>,
    {
        match self {
            GraphDiscovery::V1 => {
                let t: T = Commons::signing_keys(
                    "https://login.live.com/.well-known/openid-configuration",
                )?;
                Ok(t)
            },
            GraphDiscovery::V2 => {
                let t: T = Commons::signing_keys(
                    "https://login.microsoftonline.com/common/.well-known/openid-configuration",
                )?;
                Ok(t)
            },
            GraphDiscovery::Tenant(tenant) => {
                let url = vec![
                    "https://login.microsoftonline.com/",
                    &tenant,
                    "/.well-known/openid-configuration",
                ];
                let t: T = Commons::signing_keys(url.join("").as_str())?;
                Ok(t)
            },
        }
    }

    pub fn oauth(self, grant_type: GrantType) -> Result<OAuth, OAuthError> {
        let mut oauth = OAuth::new(grant_type);
        match self {
            GraphDiscovery::V1 => {
                let k: MicrosoftSigningKeysV1 = Commons::signing_keys(
                    "https://login.live.com/.well-known/openid-configuration",
                )?;

                oauth
                    .authorize_url(k.authorization_endpoint.as_str())
                    .access_token_url(k.token_endpoint.as_str())
                    .refresh_token_url(k.token_endpoint.as_str())
                    .logout_url(k.end_session_endpoint.as_str());

                Ok(oauth)
            },
            GraphDiscovery::V2 => {
                let k: MicrosoftSigningKeysV2 = Commons::signing_keys(
                    "https://login.microsoftonline.com/common/.well-known/openid-configuration",
                )?;

                oauth
                    .authorize_url(k.authorization_endpoint.as_str())
                    .access_token_url(k.token_endpoint.as_str())
                    .refresh_token_url(k.token_endpoint.as_str())
                    .logout_url(k.end_session_endpoint.as_str());

                Ok(oauth)
            },
            GraphDiscovery::Tenant(tenant) => {
                let url = vec![
                    "https://login.microsoftonline.com/",
                    &tenant,
                    "/.well-known/openid-configuration",
                ];
                let k: MicrosoftSigningKeysV2 = Commons::signing_keys(url.join("").as_str())?;
                oauth
                    .authorize_url(k.authorization_endpoint.as_str())
                    .access_token_url(k.token_endpoint.as_str())
                    .refresh_token_url(k.token_endpoint.as_str())
                    .logout_url(k.end_session_endpoint.as_str());

                Ok(oauth)
            },
        }
    }
}
