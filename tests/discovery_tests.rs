use graph_oauth::oauth::jwtkeys::JWTKeys;
use graph_oauth::oauth::{OAuth, OAuthCredential};
use graph_rs::oauth::graphdiscovery::{
    GraphDiscovery, MicrosoftSigningKeysV1, MicrosoftSigningKeysV2,
};

#[test]
fn graph_discovery_oauth_v1() {
    let oauth: OAuth = GraphDiscovery::V1.oauth().unwrap();
    let keys: MicrosoftSigningKeysV1 = GraphDiscovery::V1.signing_keys().unwrap();
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

#[test]
fn graph_discovery_oauth_v2() {
    let oauth: OAuth = GraphDiscovery::V2.oauth().unwrap();
    let keys: MicrosoftSigningKeysV2 = GraphDiscovery::V2.signing_keys().unwrap();
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

#[tokio::test]
async fn async_graph_discovery_oauth_v2() {
    let oauth: OAuth = GraphDiscovery::V2.async_oauth().await.unwrap();
    let keys: MicrosoftSigningKeysV2 = GraphDiscovery::V2.async_signing_keys().await.unwrap();
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

#[test]
fn jwt_keys() {
    let keys = JWTKeys::discovery().unwrap();
    assert!(keys.keys().len() > 0);

    for key in keys.into_iter() {
        assert!(key.kty.is_some());
    }
}

#[tokio::test]
async fn async_jwt_keys() {
    let keys = JWTKeys::async_discovery().await.unwrap();
    assert!(keys.keys().len() > 0);

    for key in keys.into_iter() {
        assert!(key.kty.is_some());
    }
}

#[test]
fn tenant_signing_keys() {
    if let Ok(tenant) = std::env::var("TEST_APP_TENANT") {
        let keys: MicrosoftSigningKeysV2 = GraphDiscovery::Tenant(tenant.to_string())
            .signing_keys()
            .unwrap();
        assert_eq!(
            keys.authorization_endpoint,
            format!(
                "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize",
                tenant
            )
        );
    }
}
