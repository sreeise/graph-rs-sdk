use graph_oauth::oauth::{AccessToken, Credential, OAuth};

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

    assert_eq!(body, "client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&client_secret=CLDIE3F&response_type=token&refresh_token=32LKLASDKJ&grant_type=refresh_token".to_string());
}

#[test]
fn refresh_token_test() {
    let mut oauth = get_oauth();
    oauth
        .authorize_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize?")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token?");

    let access_token = at();
    oauth.access_token(access_token);

    assert_eq!("32LKLASDKJ", oauth.get_refresh_token().unwrap());
}
