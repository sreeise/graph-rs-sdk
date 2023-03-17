use graph_http::traits::ODataMetadataLink;
use std::env;
use test_tools::oauthrequest::THROTTLE_MUTEX;
use test_tools::oauthrequest::{Environment, OAuthTestClient};

#[tokio::test]
async fn user_request_test() {
    if Environment::is_appveyor() {
        return;
    }
    env::set_var("GRAPH_TEST_ENV", "true");
    let _lock = THROTTLE_MUTEX.lock().unwrap();
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_v2().await {
        let users = client.users().list_user().send().await;

        if let Ok(response) = users {
            assert!(
                response.status() == 200 || response.status() == 201 || response.status() == 204
            );
            let value = response.json::<serde_json::Value>().await.unwrap();
            let metadata_link = value.odata_metadata_link().unwrap();
            assert_eq!(
                "https://graph.microsoft.com/v1.0/$metadata#users",
                metadata_link.as_str()
            );
        } else if let Err(e) = users {
            panic!("Request error. Method: users list. Error: {:#?}", e);
        }

        let user_res = client.users().id(id).get_user().send().await;

        if let Ok(response) = user_res {
            assert!(
                response.status() == 200 || response.status() == 201 || response.status() == 204
            );
        } else if let Err(e) = user_res {
            panic!("Request error. Method: users list. Error: {:#?}", e);
        }
    }
}
