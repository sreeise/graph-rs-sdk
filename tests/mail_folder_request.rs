use graph_http::api_impl::ODataQuery;
use test_tools::oauth_request::ASYNC_THROTTLE_MUTEX;
use test_tools::oauth_request::{Environment, OAuthTestClient};

#[tokio::test]
async fn get_drafts_mail_folder() {
    if Environment::is_appveyor() {
        return;
    }

    let _ = ASYNC_THROTTLE_MUTEX.lock().await;
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let response = client
            .user(id.as_str())
            .mail_folder("drafts")
            .get_mail_folders()
            .send()
            .await
            .unwrap();

        assert!(response.status().is_success());
        let body: serde_json::Value = response.json().await.unwrap();
        let display_name = body["displayName"].as_str().unwrap();
        assert_eq!("Drafts", display_name);
    }
}

#[tokio::test]
async fn mail_folder_list_messages() {
    if Environment::is_appveyor() {
        return;
    }

    let _ = ASYNC_THROTTLE_MUTEX.lock().await;
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let response = client
            .user(id.as_str())
            .mail_folder("inbox")
            .messages()
            .list_messages()
            .top("2")
            .send()
            .await
            .unwrap();

        assert!(response.status().is_success());
        let body: serde_json::Value = response.json().await.unwrap();
        let messages = body["value"].as_array().unwrap();
        assert_eq!(messages.len(), 2);
    }
}
