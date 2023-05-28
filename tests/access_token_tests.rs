use graph_oauth::oauth::AccessToken;
use std::thread;
use std::time::Duration;

#[test]
fn is_expired_test() {
    let mut access_token = AccessToken::default();
    access_token.set_expires_in(1);
    thread::sleep(Duration::from_secs(3));
    assert!(access_token.is_expired());
    let mut access_token = AccessToken::default();
    access_token.set_expires_in(10);
    thread::sleep(Duration::from_secs(4));
    assert!(!access_token.is_expired());
}

pub const ACCESS_TOKEN_INT: &str = r#"{
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

pub const ACCESS_TOKEN_STRING: &str = r#"{
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
