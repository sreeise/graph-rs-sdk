use rust_onedrive::flow::v1::AccountType;
use rust_onedrive::flow::v1::AuthFlow;
use rust_onedrive::flow::v1::AuthUrl;
use rust_onedrive::flow::v1::FlowType;
use rust_onedrive::process::jsonio::JsonFile;
use std::fs;
use rust_onedrive::flow::accesstoken::AccessToken;

fn access_token_struct() -> AccessToken {
    let access_token = AccessToken::new(
        "Bearer",
        3600,
        "Read.Write",
        "",
        Some("32LKLASDKJ".to_string()),
        None,
        None,
    );
    access_token
}

#[test]
fn create() {
    let mut auth_flow = AuthFlow::new(true);
    auth_flow
        .set_client_id("graph_client_id")
        .set_client_secret("A_client_secret")
        .set_auth_url("https://example.com/authorize")
        .set_token_url("https://example.com/token");

    let mut result = auth_flow.get_client_id();
    assert_eq!(result.is_none(), false);
    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap(), "graph_client_id");
    result = auth_flow.get_client_secret();
    assert_eq!(result.is_none(), false);
    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap(), "A_client_secret");
    result = auth_flow.get_auth_url();
    assert_eq!(result.is_none(), false);
    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap(), "https://example.com/authorize");
    result = auth_flow.get_token_url();
    assert_eq!(result.is_none(), false);
    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap(), "https://example.com/token");
}

#[test]
fn reset_config() {
    let mut auth_flow = AuthFlow::new(true);
    auth_flow.set_client_id("graph_client_id");
    assert_eq!(auth_flow.get_client_id().unwrap(), "graph_client_id");
    auth_flow.allow_reset(true);
    auth_flow.set_client_id("diff_client_id");
    assert_eq!(auth_flow.get_client_id().unwrap(), "diff_client_id");
}

#[test]
fn token_auth() {
    let mut auth_flow = AuthFlow::new(true);
    auth_flow
        .set_auth_url("https://example.com/oauth2/v2.0/authorize")
        .set_client_id("bb301aaa-1201-4259-a230923fds32")
        .set_redirect_uri("http://localhost:8888/redirect");

    let auth_url = auth_flow.build(FlowType::AuthorizeTokenFlow);
    assert_eq!(auth_url, Some("https://example.com/oauth2/v2.0/authorize?client_id=bb301aaa-1201-4259-a230923fds32&scope=https%3A%2F%2Fgraph.microsoft.com%2F.default&response_type=token&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect".to_string()));
}

#[test]
fn code_auth() {
    let mut auth_flow = AuthFlow::new(true);
    auth_flow
        .set_auth_url("https://example.com/oauth2/v2.0/authorize?")
        .set_client_id("bb301aaa-1201-4259-a230923fds32")
        .set_redirect_uri("http://localhost:8888/redirect");

    let auth_url = auth_flow.build(FlowType::AuthorizeCodeFlow);
    assert_eq!(auth_url, Some("https://example.com/oauth2/v2.0/authorize?client_id=bb301aaa-1201-4259-a230923fds32&scope=https%3A%2F%2Fgraph.microsoft.com%2F.default&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect".to_string()));
}

#[test]
fn access_token() {
    let mut code_flow = AuthFlow::new(true);
    code_flow
        .set_client_id("bb301aaa-1201-4259-a230923fds32")
        .set_redirect_uri("http://localhost:8888/redirect")
        .set_client_secret("CLDIE3F")
        .set_token_url("https://www.example.com/token")
        .set_access_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");

    let code_body = code_flow.build(FlowType::GrantTypeAuthCode);
    assert_eq!(code_body, Some("client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&client_secret=CLDIE3F&code=ALDSKFJLKERLKJALSDKJF2209LAKJGFL&grant_type=authorization_code".to_string()));
}

#[test]
fn refresh_token() {
    let mut refresh_flow = AuthFlow::new(true);
    refresh_flow
        .set_client_id("bb301aaa-1201-4259-a230923fds32")
        .set_redirect_uri("http://localhost:8888/redirect")
        .set_client_secret("CLDIE3F")
        .set_token_url("https://www.example.com/token")
        .set_access_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");

    refresh_flow.set_access_token(access_token_struct());
    let refresh_body = refresh_flow.build(FlowType::GrantTypeRefreshToken);
    assert_eq!(refresh_body, Some("client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&client_secret=CLDIE3F&refresh_token=32LKLASDKJ&grant_type=refresh_token".to_string()));
}

#[test]
fn flow_as_json_file() {
    let mut auth_flow = AuthFlow::new(true);
    auth_flow
        .set_client_id("bb301aaa-1201-4259-a230923fds32")
        .set_redirect_uri("http://localhost:8888/redirect")
        .set_auth_url("https://example.com/oauth2/v2.0/authorize");
    JsonFile::json_file("graph_configs/test_file.json", &auth_flow).unwrap();

    let metadata = fs::metadata("graph_configs/test_file.json")
        .expect("Could not get metadata for auth_configs/test_file.json");
    let file_type = metadata.file_type();
    assert_eq!(file_type.is_file(), true);

    let auth_flow_from_file: AuthFlow = JsonFile::from_file("graph_configs/test_file.json")
        .expect("Could not create AuthFlow from graph_configs/test_file.json");
    assert_eq!(
        auth_flow_from_file.get_client_id().unwrap(),
        "bb301aaa-1201-4259-a230923fds32"
    );
    assert_eq!(
        auth_flow_from_file.get_redirect_uri().unwrap(),
        "http://localhost:8888/redirect"
    );
    assert_eq!(
        auth_flow_from_file.get_auth_url().unwrap(),
        "https://example.com/oauth2/v2.0/authorize"
    );

    fs::remove_file("graph_configs/test_file.json")
        .expect("graph_configs/test_file.json could not be removed. It must be removed manually.");
}

#[test]
fn use_default_auth_test() {
    let mut auth_flow = AuthFlow::new(true);
    auth_flow
        .set_client_id("bb301aaa-1201-4259-a230923fds32")
        .set_redirect_uri("http://localhost:8888/redirect")
        .set_auth_url("https://www.example.com/token") // Shouldn't be used
        .set_client_secret("CLDIE3F")
        .set_token_url("https://www.example.com/token"); // Should'nt be used

    auth_flow.use_default_auth_url(AccountType::Account);
    assert_eq!(
        auth_flow.get_auth_url().unwrap(),
        AuthUrl::AccountAuth.as_str()
    );
    assert_eq!(
        auth_flow.get_token_url().unwrap(),
        AuthUrl::AccountToken.as_str()
    );

    let auth_url = auth_flow.build_auth(FlowType::AuthorizeCodeFlow);
    assert_eq!(auth_url, "https://login.live.com/oauth20_authorize.srf?client_id=bb301aaa-1201-4259-a230923fds32&scope=https%3A%2F%2Fgraph.microsoft.com%2F.default&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect");

    auth_flow.use_default_auth_url(AccountType::Graph);
    assert_eq!(
        auth_flow.get_auth_url().unwrap(),
        AuthUrl::GraphAuth.as_str()
    );
    assert_eq!(
        auth_flow.get_token_url().unwrap(),
        AuthUrl::GraphToken.as_str()
    );

    let auth_url2 = auth_flow.build_auth(FlowType::AuthorizeCodeFlow);
    assert_eq!(auth_url2, "https://login.microsoftonline.com/common/oauth2/v2.0/authorize?client_id=bb301aaa-1201-4259-a230923fds32&scope=https%3A%2F%2Fgraph.microsoft.com%2F.default&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect");
}

#[test]
fn default_encode_set() {
    let mut auth_flow = AuthFlow::native_client();
    auth_flow
        .set_client_id("CLIENT_ID")
        .set_redirect_uri("https://login.microsoftonline.com/common/oauth2/nativeclient")
        .add_scope("Files.Read");

    auth_flow.use_default_auth_url(AccountType::Account);

    let code_flow_url = auth_flow.build_auth(FlowType::AuthorizeCodeFlow);
    assert_eq!(code_flow_url, "https://login.live.com/oauth20_authorize.srf?client_id=CLIENT_ID&scope=Files.Read&response_type=code&redirect_uri=https%3A%2F%2Flogin.microsoftonline.com%2Fcommon%2Foauth2%2Fnativeclient");

    let token_flow_url = auth_flow.build_auth(FlowType::AuthorizeTokenFlow);
    assert_eq!(token_flow_url, "https://login.live.com/oauth20_authorize.srf?client_id=CLIENT_ID&scope=Files.Read&response_type=token&redirect_uri=https%3A%2F%2Flogin.microsoftonline.com%2Fcommon%2Foauth2%2Fnativeclient");

    let grant_refresh_token_url = auth_flow.build_auth(FlowType::GrantTypeRefreshToken);
    assert_eq!(grant_refresh_token_url, "client_id=CLIENT_ID&scope=Files.Read&response_type=refresh_token&redirect_uri=https%3A%2F%2Flogin.microsoftonline.com%2Fcommon%2Foauth2%2Fnativeclient");

    let grant_auth_code_url = auth_flow.build_auth(FlowType::GrantTypeAuthCode);
    assert_eq!(grant_auth_code_url, "client_id=CLIENT_ID&scope=Files.Read&response_type=authorization_code&redirect_uri=https%3A%2F%2Flogin.microsoftonline.com%2Fcommon%2Foauth2%2Fnativeclient");
}

#[test]
fn form_url_encoded() {
    let mut native_flow = AuthFlow::native_client();
    native_flow
        .set_client_id("bb301aaa-1201-4259-a230923fds32")
        .set_redirect_uri("https://login.microsoftonline.com/common/oauth2/nativeclient");
    native_flow.set_access_code("asfasdf").add_scope("Read.Write")
        .add_scope("wl.offline_access")
        .add_scope("Read.Write.All");
    native_flow.use_default_auth_url(AccountType::Account);

    let authorize_code_flow_form_encoded = native_flow.build_auth_using_form_urlencoded(FlowType::AuthorizeCodeFlow);
    assert_eq!(authorize_code_flow_form_encoded, "https://login.live.com/oauth20_authorize.srf?client_id=bb301aaa-1201-4259-a230923fds32&scope=Read.Write%2Fwl.offline_access%2FRead.Write.All&response_type=code&redirect_uri=https%3A%2F%2Flogin.microsoftonline.com%2Fcommon%2Foauth2%2Fnativeclient");

    let authorize_token_flow_form_encoded = native_flow.build_auth_using_form_urlencoded(FlowType::AuthorizeTokenFlow);
    assert_eq!(authorize_token_flow_form_encoded, "https://login.live.com/oauth20_authorize.srf?client_id=bb301aaa-1201-4259-a230923fds32&scope=Read.Write%2Fwl.offline_access%2FRead.Write.All&response_type=token&redirect_uri=https%3A%2F%2Flogin.microsoftonline.com%2Fcommon%2Foauth2%2Fnativeclient");

    let grant_code_body_form_encoded = native_flow.build_auth_using_form_urlencoded(FlowType::GrantTypeAuthCode);
    assert_eq!(grant_code_body_form_encoded, "client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=https%3A%2F%2Flogin.microsoftonline.com%2Fcommon%2Foauth2%2Fnativeclient&code=asfasdf&grant_type=authorization_code");

    native_flow.set_access_token(access_token_struct());
    let grant_refresh_flow_form_encoded = native_flow.build_auth_using_form_urlencoded(FlowType::GrantTypeRefreshToken);
    assert_eq!(grant_refresh_flow_form_encoded, "client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=https%3A%2F%2Flogin.microsoftonline.com%2Fcommon%2Foauth2%2Fnativeclient&refresh_token=32LKLASDKJ&grant_type=refresh_token");
}

#[test]
fn form_url_encoded_default_scopes() {
    let mut web_flow = AuthFlow::web_client(true);
    web_flow
        .set_client_id("bb301aaa-1201-4259-a230923fds32")
        .set_client_secret("CLDIE3F")
        .set_access_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL")
        .set_redirect_uri("http://localhost:8888/redirect");
    web_flow.use_default_auth_url(AccountType::Account);

    let web_code_flow_default_scopes = web_flow.build_auth_using_form_urlencoded(FlowType::AuthorizeCodeFlow);
    assert_eq!(web_code_flow_default_scopes, "https://login.live.com/oauth20_authorize.srf?client_id=bb301aaa-1201-4259-a230923fds32&scope=https%3A%2F%2Fgraph.microsoft.com%2F.default&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect");

    let web_token_flow_default_scopes = web_flow.build_auth_using_form_urlencoded(FlowType::AuthorizeTokenFlow);
    assert_eq!(web_token_flow_default_scopes, "https://login.live.com/oauth20_authorize.srf?client_id=bb301aaa-1201-4259-a230923fds32&scope=https%3A%2F%2Fgraph.microsoft.com%2F.default&response_type=token&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect");
}

#[test]
fn form_url_encoded_manual_scopes() {
    let mut web_flow = AuthFlow::web_client(false);
    web_flow
        .set_client_id("bb301aaa-1201-4259-a230923fds32")
        .set_client_secret("CLDIE3F")
        .set_access_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL")
        .set_redirect_uri("http://localhost:8888/redirect")
        .add_scope("Read.Write")
        .add_scope("wl.offline_access")
        .add_scope("Read.Write.All");
    web_flow.use_default_auth_url(AccountType::Account);

    let web_code_flow_manual_scopes = web_flow.build_auth_using_form_urlencoded(FlowType::AuthorizeCodeFlow);
    assert_eq!(web_code_flow_manual_scopes, "https://login.live.com/oauth20_authorize.srf?client_id=bb301aaa-1201-4259-a230923fds32&scope=Read.Write%2Fwl.offline_access%2FRead.Write.All&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect");

    let web_token_flow_manual_scopes = web_flow.build_auth_using_form_urlencoded(FlowType::AuthorizeTokenFlow);
    assert_eq!(web_token_flow_manual_scopes, "https://login.live.com/oauth20_authorize.srf?client_id=bb301aaa-1201-4259-a230923fds32&scope=Read.Write%2Fwl.offline_access%2FRead.Write.All&response_type=token&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect");
}