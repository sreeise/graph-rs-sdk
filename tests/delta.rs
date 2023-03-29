use test_tools::oauthrequest::ASYNC_THROTTLE_MUTEX;
use test_tools::oauthrequest::{Environment, OAuthTestClient};

#[tokio::test]
async fn delta_req() {
    if Environment::is_appveyor() {
        return;
    }

    let _lock = ASYNC_THROTTLE_MUTEX.lock().await;
    if let Some((_id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let mut vec = client
            .users()
            .delta()
            .paging()
            .json_deque::<serde_json::Value>()
            .await
            .unwrap();

        assert!(!vec.is_empty());
        for response in vec.iter() {
            assert!(response.status().is_success())
        }

        let response = vec.pop_back().unwrap();
        assert!(response.body()["@odata.deltaLink"].as_str().is_some())
    }
}
