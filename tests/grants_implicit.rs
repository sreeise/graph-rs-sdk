use rust_onedrive::oauth::{Grant, GrantRequest, OAuth};

#[test]
pub fn implicit_grant_url() {
    let mut oauth = OAuth::implicit_grant();
    oauth
        .authorize_url("https://login.live.com/oauth20_authorize.srf?")
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .add_scope("Read")
        .add_scope("Read.Write")
        .redirect_uri("http://localhost:8888/redirect")
        .response_type("code");
    let url = oauth.encode_uri(GrantRequest::Authorization).unwrap();
    let test_url = "https://login.live.com/oauth20_authorize.srf?client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&scope=Read+Read.Write&response_type=code";
    assert_eq!(test_url, url);
}

#[test]
#[should_panic]
pub fn request_type_access_token_panics() {
    let mut oauth = OAuth::implicit_grant();
    oauth
        .authorize_url("https://login.live.com/oauth20_authorize.srf?")
        .client_id("bb301aaa-1201-4259-a230923fds32");
    oauth.request_access_token().unwrap();
}

#[test]
#[should_panic]
pub fn request_type_refresh_token_panics() {
    let mut oauth = OAuth::implicit_grant();
    oauth
        .authorize_url("https://login.live.com/oauth20_authorize.srf?")
        .client_id("bb301aaa-1201-4259-a230923fds32");
    oauth.request_refresh_token().unwrap();
}
