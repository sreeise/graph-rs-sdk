use graph_core::resource::ResourceIdentity;
use test_tools::oauthrequest::OAuthTestClient;

#[tokio::test]
async fn list_teams() {
    std::env::set_var("GRAPH_TEST_ENV", "true");
    if let Some((_id, mut client)) =
        OAuthTestClient::graph_by_rid_async(ResourceIdentity::Teams).await
    {
        let response = client.beta().teams().list_team().send().await.unwrap();

        assert!(response.status().is_success());
    }
}
