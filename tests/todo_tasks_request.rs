use graph_core::resource::ResourceIdentity;
use serde::{Deserialize, Serialize};
use test_tools::oauth_request::OAuthTestClient;
use test_tools::oauth_request::ASYNC_THROTTLE_MUTEX;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    #[serde(alias = "displayName")]
    display_name: String,
    id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TodoListsTasks {
    #[serde(alias = "@odata.context")]
    odata_context: String,
    value: Vec<Task>,
}

#[tokio::test]
async fn list_users() {
    let _ = ASYNC_THROTTLE_MUTEX.lock().await;
    if let Some((id, client)) =
        OAuthTestClient::graph_by_rid_async(ResourceIdentity::TodoListsTasks).await
    {
        let response = client
            .user(id)
            .todo()
            .lists()
            .list_lists()
            .send()
            .await
            .unwrap();
        println!("{:#?}\n", response);
        assert!(response.status().is_success());
        let body: TodoListsTasks = response.json().await.unwrap();
        assert!(body.value.len() >= 2);
    }
}
