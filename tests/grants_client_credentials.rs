#![feature(vec_remove_item)]
use drive_test_tools::oauth::OAuthTestTool;
use rust_onedrive::oauth::{GrantRequest, GrantType, OAuth, OAuthCredential};

#[test]
pub fn authorization_url() {
    let client_id = "6731de76-14a6-49ae-97bc-6eba6914391e";
    let redirect_uri = "http://localhost:8000/redirect?";
    let authorize_url = "https://login.microsoftonline.com/common/adminconsent?";
    let state = "1234";

    let mut oauth: OAuth = OAuth::client_credentials_grant();
    oauth
        .authorize_url(authorize_url)
        .client_id(client_id)
        .state(state)
        .redirect_uri(redirect_uri)
        .admin_consent(true);
    let uri = oauth.encode_uri(GrantRequest::Authorization).unwrap();
    assert_eq!("https://login.microsoftonline.com/common/adminconsent?client_id=6731de76-14a6-49ae-97bc-6eba6914391e&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2Fredirect%3F&state=1234&admin_consent=True", uri);
}

fn get_oauth() -> OAuth {
    let mut oauth = OAuth::client_credentials_grant();
    oauth
        .authorize_url("https://login.microsoftonline.com/common/adminconsent?")
        .access_token_url("https://login.microsoftonline.com/common/authorize?")
        .client_id("6731de76-14a6-49ae-97bc-6eba6914391e")
        .client_secret("320930238-283923")
        .redirect_uri("http://localhost:8080")
        .admin_consent(true)
        .add_scope("Read.Write")
        .response_mode("query")
        .response_type("code")
        .nonce("54321")
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
        .resource("resource")
        .logout_url("https://login.live.com/oauth20_logout.srf?")
        .post_logout_redirect_uri("http://localhost:8000/redirect");
    oauth
}

#[test]
pub fn authorize_url_included_credentials() {
    let mut oauth = get_oauth();
    let vec_included =
        GrantType::ClientCredentials.available_credentials(GrantRequest::Authorization);

    OAuthTestTool::oauth_contains_credentials(&mut oauth, &vec_included);
    OAuthTestTool::oauth_query_uri_test(&mut oauth, GrantRequest::Authorization, vec_included);
}

#[test]
pub fn access_token_url_included_credentials() {
    let mut oauth = get_oauth();
    let vec_included =
        GrantType::ClientCredentials.available_credentials(GrantRequest::AccessToken);
    OAuthTestTool::oauth_query_uri_test(&mut oauth, GrantRequest::AccessToken, vec_included);
}

#[test]
pub fn access_token_shared_secret() {
    let mut oauth = get_oauth();
    oauth.grant_type("client_credentials");
    let uri = oauth.encode_uri(GrantRequest::AccessToken).unwrap();
    assert_eq!("client_id=6731de76-14a6-49ae-97bc-6eba6914391e&scope=Read.Write&grant_type=client_credentials&client_secret=320930238-283923", uri);
    assert_eq!(
        false,
        uri.contains(OAuthCredential::ClientAssertion.alias())
    );
    assert_eq!(
        false,
        uri.contains(OAuthCredential::ClientAssertionType.alias())
    );
}

#[test]
pub fn access_token_certificate() {
    let mut oauth = get_oauth();
    oauth.grant_type("client_credentials");
    oauth.remove(OAuthCredential::ClientSecret);
    let uri = oauth.encode_uri(GrantRequest::AccessToken).unwrap();
    assert_eq!("client_id=6731de76-14a6-49ae-97bc-6eba6914391e&scope=Read.Write&grant_type=client_credentials&client_assertion=client_assertion&client_assertion_type=client_assertion_type", uri);
    assert_eq!(false, uri.contains("client_secret"));
}
