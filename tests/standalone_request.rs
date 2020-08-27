use test_tools::oauthrequest::OAuthTestClient;

#[test]
fn standalone_request_json() {
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let request = client.beta().users(id.as_str()).get().build();

        let response: serde_json::Value = request.json().unwrap();
        let enabled = response["accountEnabled"].as_bool().unwrap();
        assert!(enabled);
    }
}
