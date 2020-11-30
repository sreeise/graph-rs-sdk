use test_tools::oauthrequest::OAuthTestClient;

#[test]
fn standalone_request_json() {
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let request = client.beta().user(id.as_str()).get_user().build();

        let response: serde_json::Value = request.json().unwrap();
        let enabled = response["accountEnabled"].as_bool().unwrap();
        assert!(enabled);
    }
}
