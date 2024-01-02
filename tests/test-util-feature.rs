use graph_rs_sdk::{http::Url, Graph, GraphClientConfiguration, ODataQuery};
use wiremock::matchers::{bearer_token, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Tests the test-util feature and setting https-only to false.
#[tokio::test]
async fn test_util_feature() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users"))
        .and(query_param("$top", "10"))
        .and(bearer_token("token"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let graph_client_configuration = GraphClientConfiguration::new()
        .access_token("token")
        .https_only(false);

    let mut client = Graph::from(graph_client_configuration);
    let uri = mock_server.uri();
    client.use_endpoint(&Url::parse(uri.as_str()).unwrap());

    let response = client.users().list_user().top("10").send().await.unwrap();
    let status = response.status();
    assert_eq!(status.as_u16(), 200);
}
