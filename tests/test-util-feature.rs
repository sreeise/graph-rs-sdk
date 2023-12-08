use graph_rs_sdk::{Graph, GraphClientConfiguration};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Tests the test-util feature and setting https-only to false.
#[tokio::test]
async fn test_util_feature() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let graph_client_configuration = GraphClientConfiguration::new()
        .access_token("token")
        .https_only(false);

    let mut client = Graph::from(graph_client_configuration);
    client.use_endpoint(mock_server.uri().as_str());

    let response = client.users().list_user().send().await.unwrap();
    let status = response.status();
    assert_eq!(status.as_u16(), 200);
}
