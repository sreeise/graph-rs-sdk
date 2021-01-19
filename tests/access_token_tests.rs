use graph_oauth::oauth::AccessToken;
use std::thread;
use std::time::Duration;

#[test]
fn get_method() {
    let mut access_token = AccessToken::default();
    access_token
        .set_expires_in(3600)
        .set_token_type("bearer")
        .set_bearer_token("ASODFIUJ34KJ;LADSK")
        .set_scope("offline")
        .set_refresh_token("eyJh...9323");
    assert_eq!(access_token.expires_in(), 3600);
    assert_eq!(access_token.token_type(), "bearer");
    assert_eq!(access_token.bearer_token(), "ASODFIUJ34KJ;LADSK");
    assert_eq!(access_token.scopes(), Some(&"offline".into()));
    assert_eq!(
        access_token.refresh_token(),
        Some("eyJh...9323".to_string())
    );
}

#[test]
fn access_token_field_encoding() {
    // Internally this is base64.
    let mut access_token = AccessToken::default();
    access_token.set_bearer_token("ASDFJ;34LIUASDOFI NASDOFIUY OP");
    assert_eq!(
        "ASDFJ;34LIUASDOFI NASDOFIUY OP",
        access_token.bearer_token()
    );
}

#[test]
fn is_expired_test() {
    let mut access_token = AccessToken::default();
    access_token.set_expires_in(1);
    thread::sleep(Duration::from_secs(3));
    assert_eq!(access_token.is_expired(), true);
    let mut access_token = AccessToken::default();
    access_token.set_expires_in(10);
    thread::sleep(Duration::from_secs(4));
    assert_eq!(access_token.is_expired(), false);
}

pub const ACCESS_TOKEN_INT: &'static str = r#"{
    "access_token": "fasdfasdfasfdasdfasfsdf",
    "token_type": "Bearer",
    "expires_in": 65874,
    "scope": null,
    "refresh_token": null,
    "user_id": "santa@north.pole.com",
    "id_token": "789aasdf-asdf",
    "state": null,
    "timestamp": "2020-10-27T16:31:38.788098400Z"
}"#;

pub const ACCESS_TOKEN_STRING: &'static str = r#"{
    "access_token": "fasdfasdfasfdasdfasfsdf",
    "token_type": "Bearer",
    "expires_in": "65874",
    "scope": null,
    "refresh_token": null,
    "user_id": "helpers@north.pole.com",
    "id_token": "789aasdf-asdf",
    "state": null,
    "timestamp": "2020-10-27T16:31:38.788098400Z"
}"#;

#[test]
pub fn test_deserialize() {
    let _token: AccessToken = serde_json::from_str(ACCESS_TOKEN_INT).unwrap();
    let _token: AccessToken = serde_json::from_str(ACCESS_TOKEN_STRING).unwrap();
}
