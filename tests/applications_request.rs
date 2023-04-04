use graph_core::resource::ResourceIdentity;

use test_tools::oauth_request::OAuthTestClient;

#[tokio::test]
async fn list_applications() {
    if let Some((_id, client)) =
        OAuthTestClient::graph_by_rid_async(ResourceIdentity::Applications).await
    {
        let response = client
            .applications()
            .list_application()
            .send()
            .await
            .unwrap();
        assert!(response.status().is_success());
    }
}
