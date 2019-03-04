#[macro_use]
extern crate serde_derive;
use graph_oauth::commons::Commons;
use graph_oauth::commons::WellKnown;
use jsonfile::JsonFile;

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

#[test]
fn graph_discovery_v1() {
    let t: MicrosoftSigningKeysV1 =
        Commons::signing_keys("https://login.live.com/.well-known/openid-configuration").unwrap();

    let u: MicrosoftSigningKeysV1 =
        JsonFile::from_file("../test_files/well_known_discovery/graphv1.json").unwrap();
    assert_eq!(t, u);
}

#[test]
fn graph_discovery_v2() {
    let t: MicrosoftSigningKeysV2 = Commons::signing_keys(
        "https://login.microsoftonline.com/common/.well-known/openid-configuration",
    )
    .unwrap();

    let u: MicrosoftSigningKeysV2 =
        JsonFile::from_file("../test_files/well_known_discovery/graphv2.json").unwrap();
    assert_eq!(t, u);
}
