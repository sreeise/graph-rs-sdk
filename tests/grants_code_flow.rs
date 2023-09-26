use graph_oauth::oauth::GrantType;
use graph_rs_sdk::oauth::{GrantRequest, MsalToken, OAuthSerializer};

#[test]
fn sign_in_code_url() {
    // Test the sign in url with a manually set response type.
    let mut oauth = OAuthSerializer::new();
    oauth
        .authorization_url("https://login.live.com/oauth20_authorize.srf?")
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_uri("http://localhost:8888/redirect")
        .response_type("code")
        .add_scope("https://graph.microsoft.com/.default");
    let u = oauth
        .encode_uri(GrantType::CodeFlow, GrantRequest::Authorization)
        .unwrap();

    let s =
		"https://login.live.com/oauth20_authorize.srf?client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&response_type=code&scope=https%3A%2F%2Fgraph.microsoft.com%2F.default".to_string();
    assert_eq!(u, s);
}

#[test]
fn sign_in_code_url_with_state() {
    // Test the sign in url with a manually set response type.
    let mut oauth = OAuthSerializer::new();
    oauth
        .authorization_url("https://example.com/oauth2/v2.0/authorize")
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_uri("http://localhost:8888/redirect")
        .response_type("code")
        .state("state");
    oauth.add_scope("https://graph.microsoft.com/.default");
    let u = oauth
        .encode_uri(GrantType::CodeFlow, GrantRequest::Authorization)
        .unwrap();
    let s =
		"https://example.com/oauth2/v2.0/authorize?client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&state=state&response_type=code&scope=https%3A%2F%2Fgraph.microsoft.com%2F.default".to_string();
    assert_eq!(u, s);
}

#[test]
fn access_token() {
    let mut oauth = OAuthSerializer::new();
    oauth
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_uri("http://localhost:8888/redirect")
        .client_secret("CLDIE3F")
        .authorization_url("https://www.example.com/token")
        .authorization_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");

    let mut builder = MsalToken::default();
    builder
        .with_token_type("token")
        .with_access_token("access_token")
        .with_expires_in(3600)
        .with_scope(vec!["scope"]);

    let code_body = oauth
        .encode_uri(GrantType::CodeFlow, GrantRequest::AccessToken)
        .unwrap();
    assert_eq!(
		code_body,
		"client_id=bb301aaa-1201-4259-a230923fds32&client_secret=CLDIE3F&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&response_type=token&grant_type=authorization_code&code=ALDSKFJLKERLKJALSDKJF2209LAKJGFL".to_string()
	);
}

#[test]
fn refresh_token() {
    let mut oauth = OAuthSerializer::new();
    oauth
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_uri("http://localhost:8888/redirect")
        .client_secret("CLDIE3F")
        .authorization_url("https://www.example.com/token")
        .authorization_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");

    let mut access_token = MsalToken::new("access_token", 3600, "asfasf", vec!["Read.Write"]);
    access_token.with_refresh_token("32LKLASDKJ");
    oauth.access_token(access_token);

    let body = oauth
        .encode_uri(GrantType::CodeFlow, GrantRequest::RefreshToken)
        .unwrap();
    assert_eq!(
		body,
		"client_id=bb301aaa-1201-4259-a230923fds32&client_secret=CLDIE3F&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&grant_type=refresh_token&code=ALDSKFJLKERLKJALSDKJF2209LAKJGFL&refresh_token=32LKLASDKJ".to_string()
	);
}

#[test]
fn get_refresh_token() {
    let mut oauth = OAuthSerializer::new();
    oauth
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_uri("http://localhost:8888/redirect")
        .client_secret("CLDIE3F")
        .authorization_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL")
        .refresh_token_url("https://www.example.com/token?")
        .authorization_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize?")
        .token_uri("https://login.microsoftonline.com/common/oauth2/v2.0/token?");

    let mut access_token = MsalToken::new("Bearer", 3600, "token", vec!["User.Read"]);
    access_token.with_refresh_token("32LKLASDKJ");
    oauth.access_token(access_token);

    assert_eq!("32LKLASDKJ", oauth.get_refresh_token().unwrap());
}

#[test]
fn multi_scope() {
    let mut oauth = OAuthSerializer::new();
    oauth
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .add_scope("Files.Read")
        .add_scope("Files.ReadWrite")
        .add_scope("Files.Read.All")
        .add_scope("Files.ReadWrite.All")
        .add_scope("wl.offline_access")
        .redirect_uri("http://localhost:8000/redirect")
        .authorization_url("https://login.live.com/oauth20_authorize.srf?")
        .token_uri("https://login.live.com/oauth20_token.srf")
        .refresh_token_url("https://login.live.com/oauth20_token.srf")
        .response_type("code")
        .logout_url("https://login.live.com/oauth20_logout.srf?");

    let url = oauth
        .encode_uri(GrantType::CodeFlow, GrantRequest::Authorization)
        .unwrap();
    let test_url =
		"https://login.live.com/oauth20_authorize.srf?client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8000%2Fredirect&response_type=code&scope=Files.Read+Files.Read.All+Files.ReadWrite+Files.ReadWrite.All+wl.offline_access";
    assert_eq!(test_url, url.as_str())
}
