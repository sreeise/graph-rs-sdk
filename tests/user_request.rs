use graph_rs_sdk::http::ODataMetadataLink;
use test_tools::oauth_request::ASYNC_THROTTLE_MUTEX;
use test_tools::oauth_request::{Environment, OAuthTestClient};

#[tokio::test]
async fn list_users() {
    if Environment::is_local() {
        let _ = ASYNC_THROTTLE_MUTEX.lock();
        if let Some((_id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
            let result = client.users().list_user().send().await;

            if let Ok(response) = result {
                assert!(response.status().is_success());
                let value = response.json::<serde_json::Value>().await.unwrap();
                let metadata_link = value.odata_metadata_link().unwrap();
                assert_eq!(
                    "https://graph.microsoft.com/v1.0/$metadata#users",
                    metadata_link.as_str()
                );
            } else if let Err(e) = result {
                panic!("Request error. Method: users list. Error: {e:#?}");
            }
        }
    }
}

#[tokio::test]
async fn get_user() {
    if Environment::is_local() {
        let _ = ASYNC_THROTTLE_MUTEX.lock();
        if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
            let result = client.users().id(id).get_user().send().await;

            if let Ok(response) = result {
                assert!(response.status().is_success());
            } else if let Err(e) = result {
                panic!("Request error. Method: users list. Error: {e:#?}");
            }
        }
    }
}
