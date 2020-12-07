use from_as::*;
use graph_oauth::oauth::AccessToken;
use graph_oauth::oauth::OAuth;
use graph_oauth::oauth::OAuthCredential;
use std::path::Path;
use std::time::Duration;
use std::{fs, thread};
use test_tools::support::cleanup::CleanUp;

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
fn oauth_json_file() {
    let file_location = "./test_files/test_file.json";
    let mut clean_up = CleanUp::new(|| {
        if Path::new(file_location).exists() {
            fs::remove_file(Path::new(file_location)).unwrap();
        }
    });

    clean_up.rm_files(file_location.into());

    let mut oauth = OAuth::new();
    oauth
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_uri("http://localhost:8888/redirect")
        .authorize_url("https://example.com/oauth2/v2.0/authorize");

    let mut builder = AccessToken::default();
    builder
        .set_token_type("token")
        .set_bearer_token("access_token")
        .set_expires_in(3600)
        .set_scope("scope");

    oauth.as_file(&file_location).unwrap();

    let metadata =
        fs::metadata(&file_location).expect("Could not get metadata for test_files/test_file.json");
    let file_type = metadata.file_type();
    assert_eq!(file_type.is_file(), true);

    let oauth_from_file: OAuth = match OAuth::from_file(&file_location) {
        Ok(t) => t,
        Err(e) => panic!("Could not get OAuth from file. Error: {:#?}", e),
    };

    assert_eq!(&oauth, &oauth_from_file);
    assert_eq!(
        oauth_from_file.get(OAuthCredential::ClientId),
        Some("bb301aaa-1201-4259-a230923fds32".into())
    );
    assert_eq!(
        oauth_from_file.get(OAuthCredential::RedirectURI),
        Some("http://localhost:8888/redirect".into())
    );
    assert_eq!(
        oauth_from_file.get(OAuthCredential::AuthorizeURL),
        Some("https://example.com/oauth2/v2.0/authorize".into())
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
