use crate::oauth::wellknown::WellKnown;
use crate::oauth::{OAuth, OAuthError};
use from_as::*;
use std::convert::TryFrom;
use std::io::{Read, Write};

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

pub enum GraphDiscovery {
    V1,
    V2,
    Tenant(String),
}

impl GraphDiscovery {
    /// Get the URL for the public keys used by the Microsoft identity platform
    /// to sign security tokens.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::graphdiscovery::GraphDiscovery;
    /// let url = GraphDiscovery::V1.url();
    /// println!("{}", url);
    /// ```
    pub fn url(&self) -> String {
        match self {
            GraphDiscovery::V1 => format!("{}/{}", LOGIN_LIVE_HOST, OPEN_ID_PATH),
            GraphDiscovery::V2 => format!("{}/common/v2.0/{}", MICROSOFT_ONLINE_HOST, OPEN_ID_PATH),
            GraphDiscovery::Tenant(tenant) => format!(
                "{}/{}/v2.0/{}",
                MICROSOFT_ONLINE_HOST, &tenant, OPEN_ID_PATH
            ),
        }
    }

    /// Get the public keys used by the Microsoft identity platform
    /// to sign security tokens.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::graphdiscovery::GraphDiscovery;
    /// let keys: serde_json::Value = GraphDiscovery::V1.signing_keys().unwrap();
    /// println!("{:#?}", keys);
    /// ```
    pub fn signing_keys<T>(self) -> Result<T, OAuthError>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let t: T = WellKnown::signing_keys(self.url().as_str())?;
        Ok(t)
    }

    /// Get the public keys used by the Microsoft identity platform
    /// to sign security tokens.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use graph_oauth::oauth::graphdiscovery::GraphDiscovery;
    /// let keys: serde_json::Value = GraphDiscovery::V1.async_signing_keys().await.unwrap();
    /// println!("{:#?}", keys);
    /// ```
    pub async fn async_signing_keys<T>(self) -> Result<T, OAuthError>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let t: T = WellKnown::async_signing_keys(self.url().as_str()).await?;
        Ok(t)
    }

    /// Automatically convert the public keys used by the Microsoft identity platform
    /// to sign security tokens into an OAuth object. This will get the common urls
    /// for authorization and access tokens and insert them into OAuth.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::graphdiscovery::GraphDiscovery;
    /// let oauth = GraphDiscovery::V1.oauth().unwrap();
    /// println!("{:#?}", oauth);
    /// ```
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
            }
            GraphDiscovery::V2 | GraphDiscovery::Tenant(_) => {
                let k: MicrosoftSigningKeysV2 = self.signing_keys()?;
                oauth
                    .authorize_url(k.authorization_endpoint.as_str())
                    .access_token_url(k.token_endpoint.as_str())
                    .refresh_token_url(k.token_endpoint.as_str())
                    .logout_url(k.end_session_endpoint.as_str());
                Ok(oauth)
            }
        }
    }

    /// Automatically convert the public keys used by the Microsoft identity platform
    /// to sign security tokens into an OAuth object. This will get the common urls
    /// for authorization and access tokens and insert them into OAuth.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use graph_oauth::oauth::graphdiscovery::GraphDiscovery;
    /// let oauth = GraphDiscovery::V1.async_oauth().await.unwrap();
    /// println!("{:#?}", oauth);
    /// ```
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
            }
            GraphDiscovery::V2 | GraphDiscovery::Tenant(_) => {
                let k: MicrosoftSigningKeysV2 = self.async_signing_keys().await?;
                oauth
                    .authorize_url(k.authorization_endpoint.as_str())
                    .access_token_url(k.token_endpoint.as_str())
                    .refresh_token_url(k.token_endpoint.as_str())
                    .logout_url(k.end_session_endpoint.as_str());
                Ok(oauth)
            }
        }
    }
}
