use graph_oauth::oauth::GrantType;
use graph_rs_sdk::oauth::{GrantRequest, MsalTokenResponse, OAuthSerializer};
use test_tools::oauth::OAuthTestTool;
use url::{Host, Url};

#[test]
pub fn authorization_url() {
    let mut oauth = OAuthSerializer::new();
    oauth
        .authorization_url("https://login.microsoftonline.com/common/oauth2/authorize")
        .client_id("6731de76-14a6-49ae-97bc-6eba6914391e")
        .response_type("code")
        .redirect_uri("http://localhost:8080")
        .response_mode("query")
        .response_type("code")
        .add_scope("Read.Write")
        .state("12345")
        .prompt("login")
        .code_challenge_method("plain")
        .code_challenge("code_challenge")
        .domain_hint("consumers");

    let url = oauth
        .encode_uri(GrantType::AuthorizationCode, GrantRequest::Authorization)
        .unwrap();
    let test_url =
		"https://login.microsoftonline.com/common/oauth2/authorize?client_id=6731de76-14a6-49ae-97bc-6eba6914391e&redirect_uri=http%3A%2F%2Flocalhost%3A8080&state=12345&response_mode=query&response_type=code&scope=Read.Write&prompt=login&domain_hint=consumers&code_challenge=code_challenge&code_challenge_method=plain";
    let parsed_url = Url::parse(url.as_str()).unwrap();

    assert_eq!(parsed_url.scheme(), "https");
    assert_eq!(
        parsed_url.host(),
        Some(Host::Domain("login.microsoftonline.com"))
    );
    assert_eq!(test_url, url);
}

#[test]
fn access_token_uri() {
    let mut oauth = OAuthSerializer::new();
    oauth
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .client_secret("CLDIE3F")
        .redirect_uri("http://localhost:8888/redirect")
        .grant_type("authorization_code")
        .add_scope("Read.Write")
        .add_scope("Fall.Down")
        .authorization_code("11201a230923f-4259-a230011201a230923f")
        .authorization_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL")
        .code_verifier("bb301aaab3011201a230923f-4259-a230923fds32");
    let test_url =
		"client_id=bb301aaa-1201-4259-a230923fds32&client_secret=CLDIE3F&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&code=ALDSKFJLKERLKJALSDKJF2209LAKJGFL&scope=Fall.Down+Read.Write&grant_type=authorization_code&code_verifier=bb301aaab3011201a230923f-4259-a230923fds32";
    let url = oauth
        .encode_uri(GrantType::AuthorizationCode, GrantRequest::AccessToken)
        .unwrap();
    assert_eq!(test_url, url);
}

#[test]
fn refresh_token_uri() {
    let mut oauth = OAuthSerializer::new();
    oauth
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .client_secret("CLDIE3F")
        .redirect_uri("http://localhost:8888/redirect")
        .grant_type("refresh_token")
        .add_scope("Read.Write")
        .add_scope("Fall.Down")
        .authorization_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");

    let mut access_token =
        MsalTokenResponse::new("access_token", 3600, "Read.Write Fall.Down", "asfasf");
    access_token.set_refresh_token("32LKLASDKJ");
    oauth.access_token(access_token);

    let body = oauth
        .encode_uri(GrantType::AuthorizationCode, GrantRequest::RefreshToken)
        .unwrap();
    let test_url =
        "client_id=bb301aaa-1201-4259-a230923fds32&client_secret=CLDIE3F&refresh_token=32LKLASDKJ&grant_type=refresh_token&scope=Fall.Down+Read.Write";
    assert_eq!(test_url, body);
}

#[test]
pub fn access_token_body_contains() {
    let mut oauth = OAuthSerializer::new();
    oauth
        .authorization_url("https://login.microsoftonline.com/common/oauth2/authorize")
        .client_id("6731de76-14a6-49ae-97bc-6eba6914391e")
        .redirect_uri("http://localhost:8080")
        .add_scope("Read.Write")
        .response_mode("query")
        .response_type("code")
        .state("12345")
        .prompt("login")
        .login_hint("value")
        .domain_hint("consumers")
        .code_challenge_method("plain")
        .code_challenge("code_challenge")
        .code_verifier("code_verifier")
        .client_assertion("client_assertion")
        .client_assertion_type("client_assertion_type")
        .session_state("session_state")
        .logout_url("https://login.live.com/oauth20_logout.srf?")
        .post_logout_redirect_uri("http://localhost:8000/redirect");

    let vec_included =
        GrantType::AuthorizationCode.available_credentials(GrantRequest::Authorization);
    OAuthTestTool::oauth_contains_credentials(&mut oauth, &vec_included);
    OAuthTestTool::oauth_query_uri_test(
        &mut oauth,
        GrantType::AuthorizationCode,
        GrantRequest::Authorization,
        vec_included,
    );
}
