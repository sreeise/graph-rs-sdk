use graph_oauth::oauth::wellknown::{Commons, WellKnown};
use graph_oauth::oauth::{OAuth, OAuthCredential};
use rust_onedrive::from_to::*;
use rust_onedrive::oauth::graphdiscovery::{
    GraphDiscovery, MicrosoftSigningKeysV1, MicrosoftSigningKeysV2,
};

#[test]
fn signing_keys() {
    let v1_keys: MicrosoftSigningKeysV1 = GraphDiscovery::V1.signing_keys().unwrap();
    v1_keys
        .to_json_file("./test_files/well_known_discovery/graphv1.json")
        .unwrap();
    let v2_keys: MicrosoftSigningKeysV2 = GraphDiscovery::V2.signing_keys().unwrap();
    v2_keys
        .to_json_file("./test_files/well_known_discovery/graphv2.json")
        .unwrap();
    signing_keys_v1();
    signing_keys_v2();
    graph_discovery_oauth_v1();
    graph_discovery_oauth_v2();
}

fn signing_keys_v1() {
    let t: MicrosoftSigningKeysV1 =
        Commons::signing_keys("https://login.live.com/.well-known/openid-configuration").unwrap();

    let u: MicrosoftSigningKeysV1 =
        MicrosoftSigningKeysV1::from_json_file("./test_files/well_known_discovery/graphv1.json")
            .unwrap();
    assert_eq!(t, u);
}

fn signing_keys_v2() {
    let t: MicrosoftSigningKeysV2 = Commons::signing_keys(
        "https://login.microsoftonline.com/common/.well-known/openid-configuration",
    )
    .unwrap();

    let u: MicrosoftSigningKeysV2 =
        MicrosoftSigningKeysV2::from_json_file("./test_files/well_known_discovery/graphv2.json")
            .unwrap();
    assert_eq!(t, u);
}

fn graph_discovery_oauth_v1() {
    let oauth: OAuth = GraphDiscovery::V1.oauth().unwrap();
    let keys: MicrosoftSigningKeysV1 =
        MicrosoftSigningKeysV1::from_json_file("./test_files/well_known_discovery/graphv1.json")
            .unwrap();
    assert_eq!(
        oauth.get(OAuthCredential::AuthorizeURL),
        Some(keys.authorization_endpoint.to_string())
    );
    assert_eq!(
        oauth.get(OAuthCredential::AccessTokenURL),
        Some(keys.token_endpoint.to_string())
    );
    assert_eq!(
        oauth.get(OAuthCredential::RefreshTokenURL),
        Some(keys.token_endpoint.to_string())
    );
    assert_eq!(
        oauth.get(OAuthCredential::LogoutURL),
        Some(keys.end_session_endpoint.to_string())
    );
}

fn graph_discovery_oauth_v2() {
    let oauth: OAuth = GraphDiscovery::V2.oauth().unwrap();
    let keys: MicrosoftSigningKeysV2 =
        MicrosoftSigningKeysV2::from_json_file("./test_files/well_known_discovery/graphv2.json")
            .unwrap();
    assert_eq!(
        oauth.get(OAuthCredential::AuthorizeURL),
        Some(keys.authorization_endpoint.to_string())
    );
    assert_eq!(
        oauth.get(OAuthCredential::AccessTokenURL),
        Some(keys.token_endpoint.to_string())
    );
    assert_eq!(
        oauth.get(OAuthCredential::RefreshTokenURL),
        Some(keys.token_endpoint.to_string())
    );
    assert_eq!(
        oauth.get(OAuthCredential::LogoutURL),
        Some(keys.end_session_endpoint.to_string())
    );
}
