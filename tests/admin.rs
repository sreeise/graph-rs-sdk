use graph_core::resource::ResourceIdentity;
use test_tools::common::TestTools;
use test_tools::oauthrequest::*;

#[tokio::test]
async fn list_health_issues_test() {
    if let Some((_id, client)) = OAuthTestClient::graph_by_rid_async(ResourceIdentity::Admin).await {
        let result = client
            .v1()
            .admin()
            .service_announcement()
            .list_issues()
            .filter(&[
                "startDateTime ge 2022-02-22 and startDateTime lt 2022-02-23",
            ])
            .text()
            .await;

        TestTools::assert_success(
            &result,
            "service_announcements.list_issues by text",
        );
    }
}

