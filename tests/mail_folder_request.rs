use graph_http::api_impl::ODataQuery;
use test_tools::oauthrequest::THROTTLE_MUTEX;
use test_tools::oauthrequest::{Environment, OAuthTestClient};

#[tokio::test]
async fn get_drafts_mail_folder() {
    if Environment::is_appveyor() {
        return;
    }

    let _lock = THROTTLE_MUTEX.lock().unwrap();
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let result = client
            .v1()
            .user(id.as_str())
            .mail_folder("drafts")
            .get_mail_folders()
            .send()
            .await;

        if let Ok(response) = result {
            assert!(response.status().is_success());
            let body: serde_json::Value = response.json().await.unwrap();
            let display_name = body["displayName"].as_str().unwrap();
            assert_eq!("Drafts", display_name);
        } else if let Err(e) = result {
            panic!("Test get_mail_folders. Error:\n{e:#?}");
        }
    }
}

#[tokio::test]
async fn mail_folder_list_messages() {
    if Environment::is_appveyor() {
        return;
    }

    let _lock = THROTTLE_MUTEX.lock().unwrap();
    std::env::set_var("GRAPH_TEST_ENV", "true");
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let result = client
            .v1()
            .user(id.as_str())
            .mail_folder("inbox")
            .messages()
            .list_messages()
            .top("2")
            .send()
            .await;

        if let Ok(response) = result {
            assert!(response.status().is_success());
            let body: serde_json::Value = response.json().await.unwrap();
            let messages = body["value"].as_array().unwrap();
            assert_eq!(messages.len(), 2);
        } else if let Err(e) = result {
            panic!("Test get_mail_folders. Error:\n{e:#?}");
        }
    }
}
