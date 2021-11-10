use graph_rs_sdk::prelude::*;
use test_tools::common::TestTools;
use test_tools::oauthrequest::*;

#[test]
fn office_365() {
    let _lock = THROTTLE_MUTEX.lock().unwrap();

    if let Some((_id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let result = client.v1().reports().get_report_root().send();

        TestTools::assert_success(&result, "Reports");
    }
}
