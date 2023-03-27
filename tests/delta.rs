use graph_rs_sdk::prelude::*;
use test_tools::oauthrequest::ASYNC_THROTTLE_MUTEX;
use test_tools::oauthrequest::{Environment, OAuthTestClient};

#[tokio::test]
async fn delta_req() {
    if Environment::is_appveyor() {
        return;
    }

    let _lock = ASYNC_THROTTLE_MUTEX.lock().await;
    if let Some((_id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let mut delta_recv = client
            .users()
            .delta()
            .top("2")
            .channel_next_links::<serde_json::Value>()
            .await
            .unwrap();

        let mut is_done = false;

        while let Some(next_link_response) = delta_recv.recv().await {
            if let Some(err) = next_link_response.err() {
                panic!("Error {err:#?}");
            }

            assert!(next_link_response.is_success());
            is_done = true;
        }

        assert!(is_done);
    }
}
