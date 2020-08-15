use crate::oauth::wellknown::WellKnown;
use crate::oauth::{OAuth, OAuthError};
use from_as::*;

static LOGIN_LIVE_HOST: &str = "https://login.live.com";
static MICROSOFT_ONLINE_HOST: &str = "https://login.microsoftonline.com";
static OPEN_ID_PATH: &str = ".well-known/openid-configuration";

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize, AsFile, FromFile)]
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

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize, AsFile, FromFile)]
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
    pub fn url(&self) -> String {
        match self {
            GraphDiscovery::V1 => format!("{}/{}", LOGIN_LIVE_HOST, OPEN_ID_PATH),
            GraphDiscovery::V2 => format!("{}/common/{}", MICROSOFT_ONLINE_HOST, OPEN_ID_PATH),
            GraphDiscovery::Tenant(tenant) => {
                format!("{}/{}/{}", MICROSOFT_ONLINE_HOST, &tenant, OPEN_ID_PATH)
            },
        }
    }

    pub fn signing_keys<T>(self) -> Result<T, OAuthError>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let t: T = WellKnown::signing_keys(self.url().as_str())?;
        Ok(t)
    }

    pub async fn async_signing_keys<T>(self) -> Result<T, OAuthError>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let t: T = WellKnown::async_signing_keys(self.url().as_str()).await?;
        Ok(t)
    }

    pub fn oauth(self) -> Result<OAuth, OAuthError> {
        let mut oauth = OAuth::new();
        match self {
            GraphDiscovery::V1 => {
                let k: MicrosoftSigningKeysV1 = self.signing_keys()?;
                oauth
                    .authorize_url(k.authorization_endpoint.as_str())
                    .access_token_url(k.token_endpoint.as_str())
                    .refresh_token_url(k.token_endpoint.as_str())
                    .logout_url(k.end_session_endpoint.as_str());
                Ok(oauth)
            },
            GraphDiscovery::V2 | GraphDiscovery::Tenant(_) => {
                let k: MicrosoftSigningKeysV2 = self.signing_keys()?;
                oauth
                    .authorize_url(k.authorization_endpoint.as_str())
                    .access_token_url(k.token_endpoint.as_str())
                    .refresh_token_url(k.token_endpoint.as_str())
                    .logout_url(k.end_session_endpoint.as_str());
                Ok(oauth)
            },
        }
    }

    pub async fn async_oauth(self) -> Result<OAuth, OAuthError> {
        let mut oauth = OAuth::new();
        match self {
            GraphDiscovery::V1 => {
                let k: MicrosoftSigningKeysV1 = self.async_signing_keys().await?;
                oauth
                    .authorize_url(k.authorization_endpoint.as_str())
                    .access_token_url(k.token_endpoint.as_str())
                    .refresh_token_url(k.token_endpoint.as_str())
                    .logout_url(k.end_session_endpoint.as_str());
                Ok(oauth)
            },
            GraphDiscovery::V2 | GraphDiscovery::Tenant(_) => {
                let k: MicrosoftSigningKeysV2 = self.async_signing_keys().await?;
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
