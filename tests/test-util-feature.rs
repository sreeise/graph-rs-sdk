use graph_rs_sdk::{Graph, GraphClientConfiguration};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_util_feature() {
    // Start a background HTTP server on a random local port
    let mock_server = MockServer::start().await;

    // Arrange the behaviour of the MockServer adding a Mock:
    // when it receives a GET request on '/hello' it will respond with a 200.
    Mock::given(method("GET"))
        .and(path("/users"))
        .respond_with(ResponseTemplate::new(200))
        // Mounting the mock on the mock server - it's now effective!
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
