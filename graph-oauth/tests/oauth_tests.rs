use graph_oauth::oauth::AccessToken;
use graph_oauth::oauth::AccessTokenBuilder;
use graph_oauth::oauth::OAuth;
use graph_oauth::oauth::OAuthParam;

fn get_oauth() -> OAuth {
    let mut oauth = OAuth::new();
    oauth
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_url("http://localhost:8888/redirect")
        .client_secret("CLDIE3F")
        .sign_in_url("https://www.example.com/authorize?")
        .refresh_token_url("https://www.example.com/token?")
        .access_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");
    oauth
}

#[test]
fn setters() {
    let mut oauth = OAuth::new();
    oauth
        .client_id("graph_client_id")
        .client_secret("A_client_secret")
        .sign_in_url("https://example.com/authorize")
        .refresh_token_url("https://example.com/token");

    let result = oauth.get(OAuthParam::ClientId);
    assert_eq!(result.is_none(), false);
    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap(), "graph_client_id");

    let result = oauth.get(OAuthParam::ClientSecret);
    assert_eq!(result.is_none(), false);
    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap(), "A_client_secret");

    let result = oauth.get(OAuthParam::SignInUrl);
    assert_eq!(result.is_none(), false);
    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap(), "https://example.com/authorize");

    let result = oauth.get(OAuthParam::RefreshTokenURL);
    assert_eq!(result.is_none(), false);
    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap(), "https://example.com/token");
}

#[test]
fn sign_in_code_url() {
    let mut oauth = OAuth::new();
    oauth
        .sign_in_url("https://example.com/oauth2/v2.0/authorize")
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_url("http://localhost:8888/redirect")
        .v1_default_scope(true);

    let u = oauth.encoded_sign_in_url().unwrap();
    let s = "https://example.com/oauth2/v2.0/authorize?client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&scope=https%3A%2F%2Fgraph.microsoft.com%2F.default&response_type=code";
    assert_eq!(u, s);

    // Test the sign in url with a manually set response type.
    let mut oauth = OAuth::new();
    oauth
        .sign_in_url("https://example.com/oauth2/v2.0/authorize")
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_url("http://localhost:8888/redirect")
        .response_type("code")
        .v1_default_scope(true);

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
        .sign_in_url("https://www.example.com/token")
        .access_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");

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

    let code_body = oauth.encoded_access_token_uri().unwrap();

    assert_eq!(code_body, "client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&client_secret=CLDIE3F&code=ALDSKFJLKERLKJALSDKJF2209LAKJGFL&grant_type=authorization_code".to_string());
}

#[test]
fn refresh_token() {
    let mut oauth = OAuth::new();
    oauth
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_url("http://localhost:8888/redirect")
        .client_secret("CLDIE3F")
        .sign_in_url("https://www.example.com/token")
        .access_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");

    let access_token = AccessToken::new("", 0, "", "", Some("32LKLASDKJ"), None, None);
    oauth.access_token(access_token);
    let body = oauth.encoded_refresh_token_uri().unwrap();

    assert_eq!(body, "client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&client_secret=CLDIE3F&refresh_token=32LKLASDKJ&grant_type=refresh_token".to_string());
}

#[test]
fn refresh_token_test() {
    let mut oauth = get_oauth();
    oauth.for_common_native_accounts();

    let access_token = AccessToken::new("", 0, "", "", Some("32LKLASDKJ"), None, None);
    oauth.access_token(access_token);

    assert_eq!("32LKLASDKJ", oauth.get_refresh_token().unwrap());
}
