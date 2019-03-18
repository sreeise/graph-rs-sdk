use crate::oauth::OAuthError;
use serde;
use transform_request::prelude::*;

pub trait WellKnown {
    fn signing_keys<T>(url: &str) -> Result<T, OAuthError>
    where
        T: serde::Serialize,
        for<'de> T: serde::Deserialize<'de>;
}

#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Commons;

impl WellKnown for Commons {
    fn signing_keys<T>(url: &str) -> Result<T, OAuthError>
    where
        T: serde::Serialize,
        for<'de> T: serde::Deserialize<'de>,
    {
        let client = reqwest::Client::builder().build()?;
        let response = client.get(url).send();

        match response {
            Ok(mut t) => {
                let keys: T = t.json()?;
                Ok(keys)
            },
            Err(e) => Err(OAuthError::from(e)),
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize, FromFile, ToFile)]
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

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize, FromFile, ToFile)]
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
