use graph_rs_sdk::core::ResourceIdentity;
use test_tools::{common::TestTools, oauthrequest::OAuthTestClient};

#[test]
fn list_applications() {
    if let Some((_id, client)) = OAuthTestClient::graph_by_rid(ResourceIdentity::Applications) {
        let response = client.v1().applications().list_application().send();
        TestTools::assert_success(
            &response,
            "List applications Resource Identity: Applications",
        );
    }
}
