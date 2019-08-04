use graph_oauth::oauth::GrantType;
use rust_onedrive::oauth::{GrantRequest, OAuth};
use url::{Host, Url};

pub fn oauth() -> OAuth {
    let mut oauth = OAuth::new();
    oauth
        .authorize_url("https://login.microsoftonline.com/common/oauth2/authorize")
        .client_id("6731de76-14a6-49ae-97bc-6eba6914391e")
        .response_type("id_token")
        .redirect_uri("http://localhost:8080")
        .response_mode("form_post")
        .add_scope("openid")
        .state("12345")
        .nonce("7362CAEA-9CA5-4B43-9BA3-34D7C303EBA7");

    oauth
}

#[test]
pub fn test_open_id_url() {
    let mut oauth = oauth();

    let url = oauth
        .encode_uri(GrantType::OpenId, GrantRequest::Authorization)
        .unwrap();
    let test_url = "https://login.microsoftonline.com/common/oauth2/authorize?client_id=6731de76-14a6-49ae-97bc-6eba6914391e&response_type=id_token&redirect_uri=http%3A%2F%2Flocalhost%3A8080&response_mode=form_post&scope=openid&state=12345&nonce=7362CAEA-9CA5-4B43-9BA3-34D7C303EBA7";
    let parsed_url = Url::parse(url.as_str()).unwrap();

    assert!(parsed_url.scheme() == "https");
    assert_eq!(
        parsed_url.host(),
        Some(Host::Domain("login.microsoftonline.com"))
    );
    assert_eq!(test_url, url);
}

#[test]
pub fn test_access_token_uri() {
    let mut oauth = oauth();
    oauth.response_type("id_token code");
    let url_access_token = oauth
        .encode_uri(GrantType::OpenId, GrantRequest::AccessToken)
        .unwrap();
    let test_url_access_token = "client_id=6731de76-14a6-49ae-97bc-6eba6914391e&redirect_uri=http%3A%2F%2Flocalhost%3A8080&grant_type=authorization_code&scope=openid";
    assert_eq!(test_url_access_token, url_access_token);
}
