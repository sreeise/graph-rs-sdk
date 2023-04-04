use graph_rs_sdk::error::ErrorMessage;
use reqwest::StatusCode;
use test_tools::oauth_request::OAuthTestClient;
use test_tools::oauth_request::ASYNC_THROTTLE_MUTEX;

#[tokio::test]
async fn drive_download_graph_error() {
    let _lock = ASYNC_THROTTLE_MUTEX.lock().await;
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let response = client
            .drive(id.as_str())
            .item_by_path(":/non_existent_file.docx:")
            .get_items_content()
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let error: ErrorMessage = response.json().await.unwrap();
        assert_eq!(Some("itemNotFound".into()), error.code_property());
    }
}
