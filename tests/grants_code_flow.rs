use rust_onedrive::oauth::{AccessToken, GrantRequest, GrantType, OAuth};

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

    let u = oauth
        .encode_uri(GrantType::CodeFlow(GrantRequest::Authorization))
        .unwrap();
    let s = "https://example.com/oauth2/v2.0/authorize?client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&response_mode=query&response_type=code&scope=https%3A%2F%2Fgraph.microsoft.com%2F.default".to_string();
    assert_eq!(u, s);
}

#[test]
fn sign_in_code_url_with_state() {
    // Test the sign in url with a manually set response type.
    let mut oauth = OAuth::new();
    oauth
        .authorize_url("https://example.com/oauth2/v2.0/authorize")
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_url("http://localhost:8888/redirect")
        .response_type("code")
        .state("state");
    oauth.add_scope("https://graph.microsoft.com/.default");
    let u = oauth
        .encode_uri(GrantType::CodeFlow(GrantRequest::Authorization))
        .unwrap();
    let s = "https://example.com/oauth2/v2.0/authorize?client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&state=state&response_mode=query&response_type=code&scope=https%3A%2F%2Fgraph.microsoft.com%2F.default".to_string();
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

    let code_body = oauth
        .encode_uri(GrantType::CodeFlow(GrantRequest::AccessToken))
        .unwrap();
    assert_eq!(code_body, "client_id=bb301aaa-1201-4259-a230923fds32&client_secret=CLDIE3F&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&response_type=token&grant_type=authorization_code&code=ALDSKFJLKERLKJALSDKJF2209LAKJGFL".to_string());
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

    let mut access_token = AccessToken::new("access_token", 3600, "Read.Write", "asfasf");
    access_token.refresh_token(Some("32LKLASDKJ"));
    oauth.access_token(access_token);

    let body = oauth
        .encode_uri(GrantType::CodeFlow(GrantRequest::RefreshToken))
        .unwrap();
    assert_eq!(body, "refresh_token=32LKLASDKJ&client_id=bb301aaa-1201-4259-a230923fds32&client_secret=CLDIE3F&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&grant_type=refresh_token&code=ALDSKFJLKERLKJALSDKJF2209LAKJGFL".to_string());
}

#[test]
fn get_refresh_token() {
    let mut oauth = OAuth::new();
    oauth
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_url("http://localhost:8888/redirect")
        .client_secret("CLDIE3F")
        .access_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL")
        .refresh_token_url("https://www.example.com/token?")
        .authorize_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize?")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token?");

    let mut access_token = AccessToken::new("access_token", 3600, "Read.Write", "asfasf");
    access_token.refresh_token(Some("32LKLASDKJ"));
    oauth.access_token(access_token);

    assert_eq!("32LKLASDKJ", oauth.get_refresh_token().unwrap());
}

#[test]
fn multi_scope() {
    let mut oauth = OAuth::new();
    oauth
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .add_scope("Files.Read")
        .add_scope("Files.ReadWrite")
        .add_scope("Files.Read.All")
        .add_scope("Files.ReadWrite.All")
        .add_scope("wl.offline_access")
        .redirect_url("http://localhost:8000/redirect")
        .authorize_url("https://login.live.com/oauth20_authorize.srf?")
        .access_token_url("https://login.live.com/oauth20_token.srf")
        .refresh_token_url("https://login.live.com/oauth20_token.srf")
        .response_mode("query")
        .logout_url("https://login.live.com/oauth20_logout.srf?");

    let url = oauth
        .encode_uri(GrantType::CodeFlow(GrantRequest::Authorization))
        .unwrap();
    let test_url = "https://login.live.com/oauth20_authorize.srf?client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2Fredirect&response_mode=query&response_type=code&scope=Files.Read+Files.ReadWrite+Files.Read.All+Files.ReadWrite.All+wl.offline_access";
    assert_eq!(test_url, url.as_str())
}
