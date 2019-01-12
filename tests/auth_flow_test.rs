use rust_onedrive::flow::v1::AccountType;
use rust_onedrive::flow::v1::AuthFlow;
use rust_onedrive::flow::v1::AuthUrl;
use rust_onedrive::flow::v1::FlowType;
use rust_onedrive::process::jsonio::JsonFile;
use std::fs;

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
        .set_redirect_uri("http://localhost:8888/redirect")
        .set_response_type("token");
    let auth_url = auth_flow.build(FlowType::AuthorizeTokenFlow).unwrap();
    assert_eq!(auth_url, "https://example.com/oauth2/v2.0/authorizeclient_id=bb301aaa-1201-4259-a230923fds32&scope=https%3A%2F%2Fgraph.microsoft.com%2F.default&response_type=token&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect");
}

#[test]
fn code_auth() {
    let mut auth_flow = AuthFlow::new(true);
    auth_flow
        .set_auth_url("https://example.com/oauth2/v2.0/authorize?")
        .set_client_id("bb301aaa-1201-4259-a230923fds32")
        .set_redirect_uri("http://localhost:8888/redirect")
        .set_response_type("code");
    let auth_url = auth_flow.build(FlowType::AuthorizeCodeFlow).unwrap();
    assert_eq!(auth_url, "https://example.com/oauth2/v2.0/authorize?client_id=bb301aaa-1201-4259-a230923fds32&scope=https%3A%2F%2Fgraph.microsoft.com%2F.default&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect");
}

#[test]
fn access_token() {
    let mut code_flow = AuthFlow::new(true);
    code_flow
        .set_client_id("bb301aaa-1201-4259-a230923fds32")
        .set_redirect_uri("http://localhost:8888/redirect")
        .set_response_type("token")
        .set_client_secret("CLDIE3F")
        .set_token_url("https://www.example.com/token")
        .set_access_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");

    let code_body = code_flow.build(FlowType::GrantTypeAuthCode).unwrap();
    assert_eq!(code_body, "client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&client_secret=CLDIE3F&code=ALDSKFJLKERLKJALSDKJF2209LAKJGFL&grant_type=authorization_code");
}

#[test]
fn refresh_token() {
    let mut refresh_flow = AuthFlow::new(true);
    refresh_flow
        .set_client_id("bb301aaa-1201-4259-a230923fds32")
        .set_redirect_uri("http://localhost:8888/redirect")
        .set_response_type("token")
        .set_client_secret("CLDIE3F")
        .set_refresh("32LKLASDKJ")
        .set_token_url("https://www.example.com/token")
        .set_access_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");
    let refresh_body = refresh_flow.build(FlowType::GrantTypeRefreshToken).unwrap();
    assert_eq!(refresh_body, "client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&client_secret=CLDIE3F&refresh_token=32LKLASDKJ&grant_type=refresh_token");
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
        .set_response_type("token")
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
