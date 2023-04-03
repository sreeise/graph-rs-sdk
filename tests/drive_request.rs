use graph_rs_sdk::{
    error::GraphResult,
    header::{HeaderValue, CONTENT_LENGTH},
    http::FileConfig,
    prelude::Graph,
};

use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;
use test_tools::oauth_request::DRIVE_ASYNC_THROTTLE_MUTEX;
use test_tools::oauth_request::{Environment, OAuthTestClient};

#[tokio::test]
async fn list_versions_get_item() {
    if Environment::is_local() {
        let _lock = DRIVE_ASYNC_THROTTLE_MUTEX.lock().await;
        if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
            let get_item_res = client
                .user(id.as_str())
                .drive()
                .item_by_path(":/copy_folder:")
                .get_items()
                .send()
                .await;

            if let Ok(res) = get_item_res {
                let body: serde_json::Value = res.json().await.unwrap();
                assert!(body["id"].as_str().is_some());
                let item_id = body["id"].as_str().unwrap();

                let response = client
                    .user(id.as_str())
                    .drive()
                    .item(item_id)
                    .list_versions()
                    .send()
                    .await
                    .unwrap();

                assert!(response.status().is_success());
            } else if let Err(e) = get_item_res {
                panic!("Request Error. Method: drive get_item. Error: {e:#?}");
            }
        }
    }
}

#[tokio::test]
async fn drive_check_in_out() {
    let _lock = DRIVE_ASYNC_THROTTLE_MUTEX.lock().await;
    if Environment::is_local() {
        if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
            let result = client
                .drive(id.as_str())
                .item_by_path(":/test_check_out_document.docx:")
                .checkout()
                .header(CONTENT_LENGTH, HeaderValue::from(0))
                .send()
                .await;

            let response = result.unwrap();
            assert!(response.status().is_success());
            std::thread::sleep(Duration::from_secs(2));

            let response = client
                .drive(id.as_str())
                .item_by_path(":/test_check_out_document.docx:")
                .checkin(&serde_json::json!({
                    "comment": "test check in",
                }))
                .send()
                .await
                .unwrap();

            assert!(response.status().is_success());
        }
    }
}

#[tokio::test]
async fn drive_update() {
    if Environment::is_local() {
        let _lock = DRIVE_ASYNC_THROTTLE_MUTEX.lock().await;
        if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
            let req = client
                .drive(id.as_str())
                .item_by_path(":/update_test_document.docx:")
                .update_items(&serde_json::json!({
                    "name": "update_test.docx"
                }))
                .send()
                .await;

            if let Ok(response) = req {
                assert!(response.status().is_success());
                let body: serde_json::Value = response.json().await.unwrap();
                assert_eq!(body["name"].as_str(), Some("update_test.docx"));
                thread::sleep(Duration::from_secs(2));

                let req = client
                    .drive(id.as_str())
                    .item_by_path(":/update_test.docx:")
                    .update_items(&serde_json::json!({
                        "name": "update_test_document.docx"
                    }))
                    .send()
                    .await;

                if let Ok(response) = req {
                    assert!(response.status().is_success());
                    let body: serde_json::Value = response.json().await.unwrap();
                    assert_eq!(body["name"].as_str(), Some("update_test_document.docx"));
                } else if let Err(e) = req {
                    panic!("Request Error. Method: drive update. Error: {e:#?}");
                }
            } else if let Err(e) = req {
                panic!("Request Error. Method: drive check_out. Error: {e:#?}");
            }
        }
    }
}

async fn get_special_folder_id(user_id: &str, folder: &str, client: &Graph) -> GraphResult<String> {
    let response = client
        .user(user_id)
        .drive()
        .get_special(folder)
        .send()
        .await?;

    let body: serde_json::Value = response.json().await?;
    let parent_reference_id = body["id"].as_str().unwrap();
    Ok(parent_reference_id.into())
}

async fn upload_new_file(
    user_id: &str,
    parent_reference_id: &str,
    file_name: &str,
    local_file: &str,
    client: &Graph,
) -> GraphResult<reqwest::Response> {
    client
        .drive(user_id)
        .item(parent_reference_id)
        .upload_items_content(file_name, &FileConfig::new(local_file))
        .send()
        .await
}

async fn update_file(
    user_id: &str,
    onedrive_file_path: &str,
    local_file: &str,
    client: &Graph,
) -> GraphResult<reqwest::Response> {
    client
        .user(user_id)
        .drive()
        .item_by_path(onedrive_file_path)
        .update_items_content(&FileConfig::new(local_file))
        .send()
        .await
}

async fn delete_file(
    user_id: &str,
    item_id: &str,
    client: &Graph,
) -> GraphResult<reqwest::Response> {
    client
        .user(user_id)
        .drive()
        .item(item_id)
        .delete_items()
        .send()
        .await
}

#[tokio::test]
async fn drive_upload_item() {
    let _lock = DRIVE_ASYNC_THROTTLE_MUTEX.lock().await;
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let local_file = "./test_files/test_upload_file.txt";
        let file_name = ":/test_upload_file.txt:";
        let onedrive_file_path = ":/Documents/test_upload_file.txt:";

        let parent_reference_id = get_special_folder_id(id.as_str(), "Documents", &client)
            .await
            .unwrap();
        let upload_res = upload_new_file(
            id.as_str(),
            parent_reference_id.as_str(),
            file_name,
            local_file,
            &client,
        )
        .await;

        if let Ok(response) = upload_res {
            assert!(response.status().is_success());
            let body: serde_json::Value = response.json().await.unwrap();
            assert!(body["id"].as_str().is_some());
            let item_id = body["id"].as_str().unwrap();

            let mut file = OpenOptions::new().write(true).open(local_file).unwrap();
            file.write_all("Test Update File".as_bytes()).unwrap();
            file.sync_all().unwrap();

            thread::sleep(Duration::from_secs(2));

            let update_res =
                update_file(id.as_str(), onedrive_file_path, local_file, &client).await;

            if let Ok(response2) = update_res {
                assert!(response2.status().is_success());
                let body: serde_json::Value = response2.json().await.unwrap();
                assert!(body["id"].as_str().is_some());
                let item_id2 = body["id"].as_str().unwrap();
                assert_eq!(item_id, item_id2);
            } else if let Err(err) = update_res {
                panic!("Request Error. Method: update item. Error: {err:#?}");
            }

            thread::sleep(Duration::from_secs(2));

            let delete_res = delete_file(id.as_str(), item_id, &client).await;

            if let Ok(response) = delete_res {
                assert!(response.status().is_success());
            } else if let Err(err) = delete_res {
                panic!("Request Error. Method: drive delete. Error: {err:#?}");
            }
        } else if let Err(err) = upload_res {
            panic!("Request Error. Method: drive upload. Error: {err:#?}");
        }
    }
}
