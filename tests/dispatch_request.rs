use graph_http::GraphResponse;
use test_tools::oauthrequest::OAuthTestClient;

// Tests DispatchBlocking and DispatchAsync.

#[test]
fn dispatch_blocking_json() {
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let request = client.beta().user(id.as_str()).get_user().build();

        let response: GraphResponse<serde_json::Value> = request.json().unwrap();
        let body = response.body();
        let enabled = body["accountEnabled"].as_bool().unwrap();
        assert!(enabled);
    }
}

#[test]
fn dispatch_blocking_text() {
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let request = client.beta().user(id.as_str()).get_user().build();

        let response: GraphResponse<String> = request.text().unwrap();
        let body = response.body();
        let json: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
        let enabled = json["accountEnabled"].as_bool().unwrap();
        assert!(enabled);
    }
}

#[tokio::test]
async fn dispatch_async_json() {
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let request = client.beta().user(id.as_str()).get_user().build().await;

        let response: GraphResponse<serde_json::Value> = request.json().await.unwrap();
        let body = response.body();
        let enabled = body["accountEnabled"].as_bool().unwrap();
        assert!(enabled);
    }
}

#[tokio::test]
async fn dispatch_async_text() {
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let request = client.beta().user(id.as_str()).get_user().build().await;

        let response: GraphResponse<String> = request.text().await.unwrap();
        let body = response.body();
        let json: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
        let enabled = json["accountEnabled"].as_bool().unwrap();
        assert!(enabled);
    }
}
