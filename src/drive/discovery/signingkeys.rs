use graph_oauth::commons::Commons;
use graph_oauth::commons::WellKnown;
use graph_oauth::oauth::{OAuth, OAuthError};

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftSigningKeysV1 {
    issuer: String,
    authorization_endpoint: String,
    token_endpoint: String,
    token_endpoint_auth_methods_supported: Vec<String>,
    jwks_uri: String,
    response_types_supported: Vec<String>,
    response_modes_supported: Vec<String>,
    subject_types_supported: Vec<String>,
    scopes_supported: Vec<String>,
    id_token_signing_alg_values_supported: Vec<String>,
    claims_supported: Vec<String>,
    request_uri_parameter_supported: bool,
    end_session_endpoint: String,
    frontchannel_logout_supported: bool,
    http_logout_supported: bool,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct MicrosoftSigningKeysV2 {
    authorization_endpoint: String,
    token_endpoint: String,
    token_endpoint_auth_methods_supported: Vec<String>,
    jwks_uri: String,
    response_modes_supported: Vec<String>,
    subject_types_supported: Vec<String>,
    id_token_signing_alg_values_supported: Vec<String>,
    http_logout_supported: bool,
    frontchannel_logout_supported: bool,
    end_session_endpoint: String,
    response_types_supported: Vec<String>,
    scopes_supported: Vec<String>,
    issuer: String,
    claims_supported: Vec<String>,
    microsoft_multi_refresh_token: bool,
    check_session_iframe: String,
    userinfo_endpoint: String,
    #[serde(skip)]
    tenant_region_scope: String,
    cloud_instance_name: String,
    cloud_graph_host_name: String,
    msgraph_host: String,
    rbac_url: String,
}

pub trait Discovery {
    type Keys;
}

impl Discovery for MicrosoftSigningKeysV1 {
    type Keys = MicrosoftSigningKeysV1;
}

impl Discovery for MicrosoftSigningKeysV2 {
    type Keys = MicrosoftSigningKeysV2;
}

pub enum GraphDiscovery {
    V1,
    V2,
    Tenant(String),
}

impl GraphDiscovery {
    pub fn signing_keys<T>(self) -> Result<T, OAuthError>
    where
        T: Discovery,
        for<'de> T: serde::Deserialize<'de>,
        T: serde::Serialize,
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
                let t: T = Commons::signing_keys(tenant.as_str())?;
                Ok(t)
            },
        }
    }

    pub fn oauth(self) -> Result<OAuth, OAuthError> {
        let mut oauth = OAuth::default();
        match self {
            GraphDiscovery::V1 => {
                let k: MicrosoftSigningKeysV1 = Commons::signing_keys(
                    "https://login.live.com/.well-known/openid-configuration",
                )?;

                oauth
                    .authorize_url(k.authorization_endpoint.as_str())
                    .access_token_url(k.token_endpoint.as_str())
                    .refresh_token_url(k.token_endpoint.as_str());

                Ok(oauth)
            },
            GraphDiscovery::V2 => {
                let k: MicrosoftSigningKeysV2 = Commons::signing_keys(
                    "https://login.microsoftonline.com/common/.well-known/openid-configuration",
                )?;

                oauth
                    .authorize_url(k.authorization_endpoint.as_str())
                    .access_token_url(k.token_endpoint.as_str())
                    .refresh_token_url(k.token_endpoint.as_str());

                Ok(oauth)
            },
            GraphDiscovery::Tenant(tenant) => {
                let k: MicrosoftSigningKeysV2 = Commons::signing_keys(tenant.as_str())?;
                oauth
                    .authorize_url(k.authorization_endpoint.as_str())
                    .access_token_url(k.token_endpoint.as_str())
                    .refresh_token_url(k.token_endpoint.as_str());

                Ok(oauth)
            },
        }
    }
}
