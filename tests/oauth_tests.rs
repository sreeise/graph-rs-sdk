use graph_oauth::oauth::{OAuth, OAuthCredential};
use strum::IntoEnumIterator;

#[test]
fn oauth_parameters_from_credential() {
    // Doesn't matter the flow here as this is for testing
    // that the credentials are entered/retrieved correctly.
    let mut oauth = OAuth::new();
    oauth
        .client_id("client_id")
        .client_secret("client_secret")
        .authorize_url("https://example.com/authorize?")
        .access_token_url("https://example.com/token?")
        .refresh_token_url("https://example.com/token?")
        .redirect_uri("https://example.com/redirect?")
        .access_code("ADSLFJL4L3")
        .response_mode("response_mode")
        .response_type("response_type")
        .state("state")
        .grant_type("grant_type")
        .nonce("nonce")
        .prompt("login")
        .session_state("session_state")
        .client_assertion("client_assertion")
        .client_assertion_type("client_assertion_type")
        .code_verifier("code_verifier")
        .login_hint("login_hint")
        .domain_hint("domain_hint")
        .resource("resource")
        .logout_url("https://example.com/logout?")
        .post_logout_redirect_uri("https://example.com/redirect?");

    OAuthCredential::iter().for_each(|credential| {
        if oauth.contains(credential) {
            match credential {
                OAuthCredential::ClientId => {
                    assert_eq!(oauth.get(credential), Some("client_id".into()))
                },
                OAuthCredential::ClientSecret => {
                    assert_eq!(oauth.get(credential), Some("client_secret".into()))
                },
                OAuthCredential::AuthorizeURL => assert_eq!(
                    oauth.get(credential),
                    Some("https://example.com/authorize?".into())
                ),
                OAuthCredential::AccessTokenURL => assert_eq!(
                    oauth.get(credential),
                    Some("https://example.com/token?".into())
                ),
                OAuthCredential::RefreshTokenURL => assert_eq!(
                    oauth.get(credential),
                    Some("https://example.com/token?".into())
                ),
                OAuthCredential::RedirectURI => assert_eq!(
                    oauth.get(credential),
                    Some("https://example.com/redirect?".into())
                ),
                OAuthCredential::AccessCode => {
                    assert_eq!(oauth.get(credential), Some("ADSLFJL4L3".into()))
                },
                OAuthCredential::ResponseMode => {
                    assert_eq!(oauth.get(credential), Some("response_mode".into()))
                },
                OAuthCredential::ResponseType => {
                    assert_eq!(oauth.get(credential), Some("response_type".into()))
                },
                OAuthCredential::State => assert_eq!(oauth.get(credential), Some("state".into())),
                OAuthCredential::GrantType => {
                    assert_eq!(oauth.get(credential), Some("grant_type".into()))
                },
                OAuthCredential::Nonce => assert_eq!(oauth.get(credential), Some("nonce".into())),
                OAuthCredential::LogoutURL => assert_eq!(
                    oauth.get(credential),
                    Some("https://example.com/logout?".into())
                ),
                OAuthCredential::PostLogoutRedirectURI => assert_eq!(
                    oauth.get(credential),
                    Some("https://example.com/redirect?".into())
                ),
                OAuthCredential::Prompt => assert_eq!(oauth.get(credential), Some("login".into())),
                OAuthCredential::SessionState => {
                    assert_eq!(oauth.get(credential), Some("session_state".into()))
                },
                OAuthCredential::ClientAssertion => {
                    assert_eq!(oauth.get(credential), Some("client_assertion".into()))
                },
                OAuthCredential::ClientAssertionType => {
                    assert_eq!(oauth.get(credential), Some("client_assertion_type".into()))
                },
                OAuthCredential::CodeVerifier => {
                    assert_eq!(oauth.get(credential), Some("code_verifier".into()))
                },
                OAuthCredential::Resource => {
                    assert_eq!(oauth.get(credential), Some("resource".into()))
                },
                _ => {},
            }
        }
    });
}

#[test]
fn remove_credential() {
    // Doesn't matter the flow here as this is for testing
    // that the credentials are entered/retrieved correctly.
    let mut oauth = OAuth::new();
    oauth
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_uri("http://localhost:8888/redirect")
        .client_secret("CLDIE3F")
        .authorize_url("https://www.example.com/authorize?")
        .refresh_token_url("https://www.example.com/token?")
        .access_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");
    assert_eq!(oauth.get(OAuthCredential::ClientId).is_some(), true);
    oauth.remove(OAuthCredential::ClientId);
    assert_eq!(oauth.get(OAuthCredential::ClientId).is_some(), false);
    oauth.client_id("client_id");
    assert_ne!(oauth.get(OAuthCredential::ClientId).is_some(), false);

    assert_eq!(oauth.get(OAuthCredential::RedirectURI).is_some(), true);
    oauth.remove(OAuthCredential::RedirectURI);
    assert_eq!(oauth.get(OAuthCredential::RedirectURI).is_some(), false);
}

#[test]
fn setters() {
    // Doesn't matter the flow here as this is for testing
    // that the credentials are entered/retrieved correctly.
    let mut oauth = OAuth::new();
    oauth
        .client_id("client_id")
        .client_secret("client_secret")
        .authorize_url("https://example.com/authorize")
        .refresh_token_url("https://example.com/token")
        .access_token_url("https://example.com/token")
        .redirect_uri("https://example.com/redirect")
        .access_code("access_code");

    let test_setter = |c: OAuthCredential, s: &str| {
        let result = oauth.get(c);
        assert_eq!(result.is_none(), false);
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap(), s);
    };

    test_setter(OAuthCredential::ClientId, "client_id");
    test_setter(OAuthCredential::ClientSecret, "client_secret");
    test_setter(
        OAuthCredential::AuthorizeURL,
        "https://example.com/authorize",
    );
    test_setter(
        OAuthCredential::RefreshTokenURL,
        "https://example.com/token",
    );
    test_setter(OAuthCredential::AccessTokenURL, "https://example.com/token");
    test_setter(OAuthCredential::RedirectURI, "https://example.com/redirect");
    test_setter(OAuthCredential::AccessCode, "access_code");
}
