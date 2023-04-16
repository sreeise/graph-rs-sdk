use graph_rs_sdk::oauth::{GrantRequest, GrantType, OAuth};

#[test]
pub fn implicit_grant_url() {
    let mut oauth = OAuth::new();
    oauth
        .authorization_url("https://login.live.com/oauth20_authorize.srf?")
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .add_scope("Read")
        .add_scope("Read.Write")
        .redirect_uri("http://localhost:8888/redirect")
        .response_type("code");
    let url = oauth
        .encode_uri(GrantType::Implicit, GrantRequest::Authorization)
        .unwrap();
    let test_url =
		"https://login.live.com/oauth20_authorize.srf?client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&scope=Read+Read.Write&response_type=code";
    assert_eq!(test_url, url);
}
