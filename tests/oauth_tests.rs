use graph_oauth::oauth::IntoEnumIterator;
use graph_oauth::oauth::{OAuthParameter, OAuthSerializer};

#[test]
fn oauth_parameters_from_credential() {
    // Doesn't matter the flow here as this is for testing
    // that the credentials are entered/retrieved correctly.
    let mut oauth = OAuthSerializer::new();
    oauth
        .client_id("client_id")
        .client_secret("client_secret")
        .authorization_url("https://example.com/authorize?")
        .token_uri("https://example.com/token?")
        .refresh_token_url("https://example.com/token?")
        .redirect_uri("https://example.com/redirect?")
        .authorization_code("ADSLFJL4L3")
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

    OAuthParameter::iter().for_each(|credential| {
        if oauth.contains(credential) {
            match credential {
                OAuthParameter::ClientId => {
                    assert_eq!(oauth.get(credential), Some("client_id".into()))
                }
                OAuthParameter::ClientSecret => {
                    assert_eq!(oauth.get(credential), Some("client_secret".into()))
                }
                OAuthParameter::AuthorizationUrl => assert_eq!(
                    oauth.get(credential),
                    Some("https://example.com/authorize?".into())
                ),
                OAuthParameter::TokenUrl => assert_eq!(
                    oauth.get(credential),
                    Some("https://example.com/token?".into())
                ),
                OAuthParameter::RefreshTokenUrl => assert_eq!(
                    oauth.get(credential),
                    Some("https://example.com/token?".into())
                ),
                OAuthParameter::RedirectUri => assert_eq!(
                    oauth.get(credential),
                    Some("https://example.com/redirect?".into())
                ),
                OAuthParameter::AuthorizationCode => {
                    assert_eq!(oauth.get(credential), Some("ADSLFJL4L3".into()))
                }
                OAuthParameter::ResponseMode => {
                    assert_eq!(oauth.get(credential), Some("response_mode".into()))
                }
                OAuthParameter::ResponseType => {
                    assert_eq!(oauth.get(credential), Some("response_type".into()))
                }
                OAuthParameter::State => assert_eq!(oauth.get(credential), Some("state".into())),
                OAuthParameter::GrantType => {
                    assert_eq!(oauth.get(credential), Some("grant_type".into()))
                }
                OAuthParameter::Nonce => assert_eq!(oauth.get(credential), Some("nonce".into())),
                OAuthParameter::LogoutURL => assert_eq!(
                    oauth.get(credential),
                    Some("https://example.com/logout?".into())
                ),
                OAuthParameter::PostLogoutRedirectURI => assert_eq!(
                    oauth.get(credential),
                    Some("https://example.com/redirect?".into())
                ),
                OAuthParameter::Prompt => assert_eq!(oauth.get(credential), Some("login".into())),
                OAuthParameter::SessionState => {
                    assert_eq!(oauth.get(credential), Some("session_state".into()))
                }
                OAuthParameter::ClientAssertion => {
                    assert_eq!(oauth.get(credential), Some("client_assertion".into()))
                }
                OAuthParameter::ClientAssertionType => {
                    assert_eq!(oauth.get(credential), Some("client_assertion_type".into()))
                }
                OAuthParameter::CodeVerifier => {
                    assert_eq!(oauth.get(credential), Some("code_verifier".into()))
                }
                OAuthParameter::Resource => {
                    assert_eq!(oauth.get(credential), Some("resource".into()))
                }
                _ => {}
            }
        }
    });
}

#[test]
fn remove_credential() {
    // Doesn't matter the flow here as this is for testing
    // that the credentials are entered/retrieved correctly.
    let mut oauth = OAuthSerializer::new();
    oauth
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_uri("http://localhost:8888/redirect")
        .client_secret("CLDIE3F")
        .authorization_url("https://www.example.com/authorize?")
        .refresh_token_url("https://www.example.com/token?")
        .authorization_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");
    assert!(oauth.get(OAuthParameter::ClientId).is_some());
    oauth.remove(OAuthParameter::ClientId);
    assert!(oauth.get(OAuthParameter::ClientId).is_none());
    oauth.client_id("client_id");
    assert!(oauth.get(OAuthParameter::ClientId).is_some());

    assert!(oauth.get(OAuthParameter::RedirectUri).is_some());
    oauth.remove(OAuthParameter::RedirectUri);
    assert!(oauth.get(OAuthParameter::RedirectUri).is_none());
}

#[test]
fn setters() {
    // Doesn't matter the flow here as this is for testing
    // that the credentials are entered/retrieved correctly.
    let mut oauth = OAuthSerializer::new();
    oauth
        .client_id("client_id")
        .client_secret("client_secret")
        .authorization_url("https://example.com/authorize")
        .refresh_token_url("https://example.com/token")
        .token_uri("https://example.com/token")
        .redirect_uri("https://example.com/redirect")
        .authorization_code("access_code");

    let test_setter = |c: OAuthParameter, s: &str| {
        let result = oauth.get(c);
        assert!(result.is_some());
        assert!(result.is_some());
        assert_eq!(result.unwrap(), s);
    };

    test_setter(OAuthParameter::ClientId, "client_id");
    test_setter(OAuthParameter::ClientSecret, "client_secret");
    test_setter(
        OAuthParameter::AuthorizationUrl,
        "https://example.com/authorize",
    );
    test_setter(OAuthParameter::RefreshTokenUrl, "https://example.com/token");
    test_setter(OAuthParameter::TokenUrl, "https://example.com/token");
    test_setter(OAuthParameter::RedirectUri, "https://example.com/redirect");
    test_setter(OAuthParameter::AuthorizationCode, "access_code");
}
