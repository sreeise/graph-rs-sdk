use graph_core::resource::ResourceIdentity;
use graph_http::api_impl::ODataQuery;

use test_tools::oauth_request::*;

#[tokio::test]
async fn list_health_issues_test() {
    if let Some((_id, client)) = OAuthTestClient::graph_by_rid_async(ResourceIdentity::Admin).await
    {
        let response = client
            .admin()
            .list_issues()
            .filter(&["startDateTime ge 2022-02-22 and startDateTime lt 2022-02-23"])
            .send()
            .await
            .unwrap();

        assert!(response.status().is_success());
    }
}
