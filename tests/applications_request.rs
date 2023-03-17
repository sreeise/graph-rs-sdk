use graph_rs_sdk::core::ResourceIdentity;
use test_tools::common::TestTools;
use test_tools::oauthrequest::OAuthTestClient;

#[tokio::test]
async fn list_applications() {
    if let Some((_id, client)) =
        OAuthTestClient::graph_by_rid_async(ResourceIdentity::Applications).await
    {
        let response = client
            .v1()
            .applications()
            .list_application()
            .send()
            .await
            .unwrap();
        assert!(response.status().is_success());
    }
}
