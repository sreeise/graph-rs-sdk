use graph_rs::prelude::*;
use test_tools::oauthrequest::OAuthRequest;

#[test]
fn standalone_request_json() {
    OAuthRequest::access_token_fn(|t| {
        if let Some((id, bearer)) = t {
            let client = Graph::new(bearer.as_str());
            let request = client.beta().users(id.as_str()).get().build();

            let response: serde_json::Value = request.json().unwrap();
            let enabled = response["accountEnabled"].as_bool().unwrap();
            assert!(enabled);
        }
    });
}
