use graph_oauth::oauth::AccessToken;
use graph_oauth::oauth::OAuth;
use graph_oauth::oauth::OAuthCredential;
use rust_onedrive::transform::*;
use std::path::Path;
use std::{fs, thread};

mod support;

pub use crate::support::cleanup::CleanUp;
use std::time::Duration;

#[test]
fn get_method() {
    let mut access_token = AccessToken::default();
    access_token
        .expires_in(3600)
        .token_type("bearer")
        .access_token("ASODFIUJ34KJ;LADSK")
        .scope("offline")
        .refresh_token(Some("eyJh...9323"));
    assert_eq!(access_token.get_expires_in(), 3600);
    assert_eq!(access_token.get_token_type(), "bearer");
    assert_eq!(access_token.get_access_token(), "ASODFIUJ34KJ;LADSK");
    assert_eq!(access_token.get_scopes(), "offline");
    assert_eq!(access_token.get_refresh_token(), Some("eyJh...9323".into()));
}

#[test]
fn access_token_field_encoding() {
    // Internally this is base64.
    let mut access_token = AccessToken::default();
    access_token.access_token("ASDFJ;34LIUASDOFI NASDOFIUY OP");
    assert_eq!(
        "ASDFJ;34LIUASDOFI NASDOFIUY OP",
        access_token.get_access_token()
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
        .redirect_url("http://localhost:8888/redirect")
        .authorize_url("https://example.com/oauth2/v2.0/authorize");

    let mut builder = AccessToken::default();
    builder
        .token_type("token")
        .access_token("access_token")
        .expires_in(3600)
        .scope("scope")
        .refresh_token(None)
        .user_id(None)
        .id_token(None);

    oauth.to_file(&file_location).unwrap();

    let metadata = fs::metadata(&file_location)
        .expect("Could not get metadata for auth_configs/test_file.json");
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
    access_token.expires_in(1);
    thread::sleep(Duration::from_secs(3));
    assert_eq!(access_token.is_expired(), true);
    let mut access_token = AccessToken::default();
    access_token.expires_in(10);
    thread::sleep(Duration::from_secs(4));
    assert_eq!(access_token.is_expired(), false);
}
