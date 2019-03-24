use graph_oauth::oauth::{AccessToken, Credential, OAuth};
use strum::IntoEnumIterator;

fn at() -> AccessToken {
    let mut access_token = AccessToken::new("access_token", 3600, "Read.Write", "asfasf");
    access_token.refresh_token(Some("32LKLASDKJ"));
    access_token
}

fn get_oauth() -> OAuth {
    let mut oauth = OAuth::new();
    oauth
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_url("http://localhost:8888/redirect")
        .client_secret("CLDIE3F")
        .authorize_url("https://www.example.com/authorize?")
        .refresh_token_url("https://www.example.com/token?")
        .access_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");
    oauth
}

#[test]
fn oauth_parameters_from_credential() {
    let mut oauth = OAuth::new();
    oauth
        .client_id("client_id")
        .client_secret("client_secret")
        .authorize_url("https://example.com/authorize?")
        .access_token_url("https://example.com/token?")
        .refresh_token_url("https://example.com/token?")
        .redirect_url("https://example.com/redirect?")
        .access_code("ADSLFJL4L3")
        .response_mode("response_mode")
        .response_type("response_type")
        .state("state")
        .grant_type("grant_type")
        .nonce("nonce")
        .logout_url("https://example.com/logout?")
        .post_logout_redirect_uri("https://example.com/redirect?");

    Credential::iter().for_each(|credential| {
        if oauth.contains(credential) {
            match credential {
                Credential::ClientId => assert_eq!(oauth.get(credential), Some("client_id".into())),
                Credential::ClientSecret => {
                    assert_eq!(oauth.get(credential), Some("client_secret".into()))
                },
                Credential::AuthorizeURL => assert_eq!(
                    oauth.get(credential),
                    Some("https://example.com/authorize?".into())
                ),
                Credential::AccessTokenURL => assert_eq!(
                    oauth.get(credential),
                    Some("https://example.com/token?".into())
                ),
                Credential::RefreshTokenURL => assert_eq!(
                    oauth.get(credential),
                    Some("https://example.com/token?".into())
                ),
                Credential::RedirectURI => assert_eq!(
                    oauth.get(credential),
                    Some("https://example.com/redirect?".into())
                ),
                Credential::AccessCode => {
                    assert_eq!(oauth.get(credential), Some("ADSLFJL4L3".into()))
                },
                Credential::ResponseMode => {
                    assert_eq!(oauth.get(credential), Some("response_mode".into()))
                },
                Credential::ResponseType => {
                    assert_eq!(oauth.get(credential), Some("response_type".into()))
                },
                Credential::State => assert_eq!(oauth.get(credential), Some("state".into())),
                Credential::GrantType => {
                    assert_eq!(oauth.get(credential), Some("grant_type".into()))
                },
                Credential::Nonce => assert_eq!(oauth.get(credential), Some("nonce".into())),
                Credential::LogoutURL => assert_eq!(
                    oauth.get(credential),
                    Some("https://example.com/logout?".into())
                ),
                Credential::PostLogoutRedirectURI => assert_eq!(
                    oauth.get(credential),
                    Some("https://example.com/redirect?".into())
                ),
                _ => {},
            }
        }
    });
}

#[test]
fn remove_credential() {
    let mut oauth = get_oauth();
    assert_eq!(oauth.get(Credential::ClientId).is_some(), true);
    oauth.remove(Credential::ClientId);
    assert_eq!(oauth.get(Credential::ClientId).is_some(), false);
    oauth.client_id("client_id");
    assert_ne!(oauth.get(Credential::ClientId).is_some(), false);

    assert_eq!(oauth.get(Credential::RedirectURI).is_some(), true);
    oauth.remove(Credential::RedirectURI);
    assert_eq!(oauth.get(Credential::RedirectURI).is_some(), false);
}

#[test]
fn setters() {
    let mut oauth = OAuth::new();
    oauth
        .client_id("client_id")
        .client_secret("client_secret")
        .authorize_url("https://example.com/authorize")
        .refresh_token_url("https://example.com/token")
        .access_token_url("https://example.com/token")
        .redirect_url("https://example.com/redirect")
        .access_code("access_code");

    let test_setter = |c: Credential, s: &str| {
        let result = oauth.get(c);
        assert_eq!(result.is_none(), false);
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap(), s);
    };

    test_setter(Credential::ClientId, "client_id");
    test_setter(Credential::ClientSecret, "client_secret");
    test_setter(Credential::AuthorizeURL, "https://example.com/authorize");
    test_setter(Credential::RefreshTokenURL, "https://example.com/token");
    test_setter(Credential::AccessTokenURL, "https://example.com/token");
    test_setter(Credential::RedirectURI, "https://example.com/redirect");
    test_setter(Credential::AccessCode, "access_code");
}

#[test]
fn sign_in_code_url() {
    // Test the sign in url with a manually set response type.
    let mut oauth = OAuth::new();
    oauth
        .authorize_url("https://example.com/oauth2/v2.0/authorize")
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_url("http://localhost:8888/redirect")
        .response_type("code");
    oauth.add_scope("https://graph.microsoft.com/.default");

    let u = oauth.encoded_sign_in_url().unwrap();
    let s = "https://example.com/oauth2/v2.0/authorize?client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&scope=https%3A%2F%2Fgraph.microsoft.com%2F.default&response_type=code";
    assert_eq!(u, s);
}

#[test]
fn access_token() {
    let mut oauth = OAuth::new();
    oauth
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_url("http://localhost:8888/redirect")
        .client_secret("CLDIE3F")
        .authorize_url("https://www.example.com/token")
        .access_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");

    let mut builder = AccessToken::default();
    builder
        .token_type("token")
        .access_token("access_token")
        .expires_in(3600)
        .scope("scope")
        .refresh_token(None)
        .user_id(None)
        .id_token(None);
    let code_body = oauth.encoded_access_token_uri().unwrap();

    assert_eq!(code_body, "client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&client_secret=CLDIE3F&response_type=token&code=ALDSKFJLKERLKJALSDKJF2209LAKJGFL&grant_type=authorization_code".to_string());
}

#[test]
fn refresh_token() {
    let mut oauth = OAuth::new();
    oauth
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_url("http://localhost:8888/redirect")
        .client_secret("CLDIE3F")
        .authorize_url("https://www.example.com/token")
        .access_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");

    let access_token = at();
    oauth.access_token(access_token);
    let body = oauth.encoded_refresh_token_uri().unwrap();

    assert_eq!(body, "client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&client_secret=CLDIE3F&refresh_token=32LKLASDKJ&grant_type=refresh_token".to_string());
}

#[test]
fn get_refresh_token() {
    let mut oauth = get_oauth();
    oauth
        .authorize_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize?")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token?");

    let access_token = at();
    oauth.access_token(access_token);

    assert_eq!("32LKLASDKJ", oauth.get_refresh_token().unwrap());
}
