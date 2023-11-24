use graph_http::api_impl::ODataQuery;
use test_tools::oauth_request::{DEFAULT_ONENOTE_CREDENTIALS_MUTEX};

#[ignore]
#[tokio::test]
async fn get_drafts_mail_folder() {
    let test_client = DEFAULT_ONENOTE_CREDENTIALS_MUTEX.lock().await;
    let response = test_client
        .client
        .user(test_client.user_id.as_str())
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

#[tokio::test]
async fn mail_folder_list_messages() {
    let test_client = DEFAULT_ONENOTE_CREDENTIALS_MUTEX.lock().await;
    let response = test_client
        .client
        .user(test_client.user_id.as_str())
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
