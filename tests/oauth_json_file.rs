use graph_oauth::oauth::AccessTokenBuilder;
use graph_oauth::oauth::OAuth;
use graph_oauth::oauth::OAuthParam;
use rust_onedrive::process::jsonio::JsonFile;
use std::fs;

#[test]
fn flow_as_json_file() {
    let file_location = "test_files/test_file.json";

    let mut oauth = OAuth::new();
    oauth
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_url("http://localhost:8888/redirect")
        .sign_in_url("https://example.com/oauth2/v2.0/authorize");

    let mut builder = AccessTokenBuilder::default();
    builder
        .token_type("token".to_string())
        .access_token("access_token".to_string())
        .expires_in(3600)
        .scope("scope")
        .refresh_token(None)
        .user_id(None)
        .id_token(None);

    match builder.build() {
        Ok(t) => oauth.access_token(t),
        Err(e) => panic!("{:#?}", e),
    };

    JsonFile::json_file(&file_location, &oauth).unwrap();

    let metadata = fs::metadata(&file_location)
        .expect("Could not get metadata for auth_configs/test_file.json");
    let file_type = metadata.file_type();
    assert_eq!(file_type.is_file(), true);

    let oauth_from_file: OAuth = match JsonFile::from_file(&file_location) {
        Ok(t) => t,
        Err(e) => panic!("Could not get OAuth from file. Error: {:#?}", e),
    };

    assert_eq!(&oauth, &oauth_from_file);
    assert_eq!(
        oauth_from_file.get(OAuthParam::ClientId),
        Some("bb301aaa-1201-4259-a230923fds32")
    );
    assert_eq!(
        oauth_from_file.get(OAuthParam::RedirectUri),
        Some("http://localhost:8888/redirect")
    );
    assert_eq!(
        oauth_from_file.get(OAuthParam::SignInUrl),
        Some("https://example.com/oauth2/v2.0/authorize")
    );

    fs::remove_file(&file_location).unwrap();
}
