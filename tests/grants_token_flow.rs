use graph_oauth::oauth::GrantType;
use graph_rs_sdk::oauth::{GrantRequest, OAuth};

#[test]
pub fn token_flow_url() {
    let mut oauth = OAuth::new();
    oauth
        .authorize_url("https://login.live.com/oauth20_authorize.srf?")
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .add_scope("Read")
        .add_scope("Read.Write")
        .redirect_uri("http://localhost:8888/redirect")
        .response_type("token");
    let url = oauth
        .encode_uri(GrantType::TokenFlow, GrantRequest::Authorization)
        .unwrap();
    let test_url = "https://login.live.com/oauth20_authorize.srf?client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&response_type=token&scope=Read+Read.Write";
    assert_eq!(test_url, url);
}
