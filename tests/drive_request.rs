use graph_rs_sdk::{
    error::GraphResult,
    header::{HeaderValue, CONTENT_LENGTH},
    http::FileConfig,
    Graph,
};
use std::fs::OpenOptions;
use std::io::{BufReader, Write};

use graph_http::api_impl::BodyRead;
use log::debug;
use std::time::Duration;
use test_tools::oauth_request::{
    Environment, GraphTestClient, DEFAULT_CLIENT_CREDENTIALS_MUTEX,
    DEFAULT_CLIENT_CREDENTIALS_MUTEX3, DEFAULT_CLIENT_CREDENTIALS_MUTEX4,
};
use tokio::sync::Mutex;

#[tokio::test]
async fn list_versions_get_item() {
    if Environment::is_local() {
        let test_client = DEFAULT_CLIENT_CREDENTIALS_MUTEX4.lock().await;
        let get_item_res = test_client
            .client
            .user(test_client.user_id.as_str())
            .drive()
            .item_by_path(":/copy_folder:")
            .get_items()
            .send()
            .await;

        if let Ok(res) = get_item_res {
            let body: serde_json::Value = res.json().await.unwrap();
            assert!(body["id"].as_str().is_some());
            let item_id = body["id"].as_str().unwrap();

            let response = test_client
                .client
                .user(test_client.user_id.as_str())
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

#[tokio::test]
async fn drive_check_in_out() {
    if Environment::is_local() {
        let test_client = DEFAULT_CLIENT_CREDENTIALS_MUTEX3.lock().await;
        let result = test_client
            .client
            .drive(test_client.user_id.as_str())
            .item_by_path(":/test_check_out_document.docx:")
            .checkout()
            .header(CONTENT_LENGTH, HeaderValue::from(0))
            .send()
            .await;

        let response = result.unwrap();
        assert!(response.status().is_success());
        tokio::time::sleep(Duration::from_millis(500)).await;

        let response = test_client
            .client
            .drive(test_client.user_id.as_str())
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

async fn update_item_by_path(
    drive_id: &str,
    path: &str,
    item: &serde_json::Value,
    client: &Graph,
) -> GraphResult<reqwest::Response> {
    client
        .drive(drive_id)
        .item_by_path(path)
        .update_items(item)
        .send()
        .await
}

#[ignore]
#[tokio::test]
async fn drive_update() {
    if Environment::is_local() {
        let test_client = DEFAULT_CLIENT_CREDENTIALS_MUTEX4.lock().await;
        let req = update_item_by_path(
            test_client.user_id.as_str(),
            ":/update_test_document.docx:",
            &serde_json::json!({
                "name": "update_test.docx"
            }),
            &test_client.client,
        )
        .await;

        if let Ok(response) = req {
            assert!(response.status().is_success());
            let body: serde_json::Value = response.json().await.unwrap();
            assert_eq!(body["name"].as_str(), Some("update_test.docx"));

            let req = update_item_by_path(
                test_client.user_id.as_str(),
                ":/update_test.docx:",
                &serde_json::json!({
                    "name": "update_test_document.docx"
                }),
                &test_client.client,
            )
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

async fn update_file_using_file_config(
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

async fn update_file_using_async_reader(
    user_id: &str,
    onedrive_file_path: &str,
    local_file: &str,
    client: &Graph,
) -> GraphResult<reqwest::Response> {
    let file = tokio::fs::File::open(local_file).await.unwrap();
    client
        .user(user_id)
        .drive()
        .item_by_path(onedrive_file_path)
        .update_items_content(BodyRead::from_async_read(file).await.unwrap())
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
    let test_client = DEFAULT_CLIENT_CREDENTIALS_MUTEX.lock().await;
    let local_file = "./test_files/test_upload_file.txt";
    let file_name = ":/test_upload_file.txt:";
    let onedrive_file_path = ":/Documents/test_upload_file.txt:";

    let parent_reference_id = get_special_folder_id(
        test_client.user_id.as_str(),
        "Documents",
        &test_client.client,
    )
    .await
    .unwrap();
    let upload_res = upload_new_file(
        test_client.user_id.as_str(),
        parent_reference_id.as_str(),
        file_name,
        local_file,
        &test_client.client,
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

        tokio::time::sleep(Duration::from_millis(500)).await;

        let update_res = update_file_using_file_config(
            test_client.user_id.as_str(),
            onedrive_file_path,
            local_file,
            &test_client.client,
        )
        .await;

        if let Ok(response2) = update_res {
            assert!(response2.status().is_success());
            let body: serde_json::Value = response2.json().await.unwrap();
            let item_id2 = body["id"].as_str().unwrap();
            assert_eq!(item_id, item_id2);
        } else if let Err(err) = update_res {
            panic!("Request Error. Method: update item. Error: {err:#?}");
        }

        tokio::time::sleep(Duration::from_millis(500)).await;

        let delete_res =
            delete_file(test_client.user_id.as_str(), item_id, &test_client.client).await;

        if let Ok(response) = delete_res {
            assert!(response.status().is_success());
        } else if let Err(err) = delete_res {
            panic!("Request Error. Method: drive delete. Error: {err:#?}");
        }
    } else if let Err(err) = upload_res {
        panic!("Request Error. Method: drive upload. Error: {err:#?}");
    }
}

#[tokio::test]
async fn drive_upload_item_using_async_reader() {
    let test_client = DEFAULT_CLIENT_CREDENTIALS_MUTEX.lock().await;
    let local_file = "./test_files/upload_file_by_async_reader.html";
    let file_name = ":/upload_file_by_async_reader.html:";
    let onedrive_file_path = ":/Documents/upload_file_by_async_reader.html:";

    let parent_reference_id = get_special_folder_id(
        test_client.user_id.as_str(),
        "Documents",
        &test_client.client,
    )
    .await
    .unwrap();
    let upload_res = upload_new_file(
        test_client.user_id.as_str(),
        parent_reference_id.as_str(),
        file_name,
        local_file,
        &test_client.client,
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

        tokio::time::sleep(Duration::from_millis(500)).await;

        let update_res = update_file_using_async_reader(
            test_client.user_id.as_str(),
            onedrive_file_path,
            local_file,
            &test_client.client,
        )
        .await;

        if let Ok(response2) = update_res {
            assert!(response2.status().is_success());
            let body: serde_json::Value = response2.json().await.unwrap();
            let item_id2 = body["id"].as_str().unwrap();
            assert_eq!(item_id, item_id2);
        } else if let Err(err) = update_res {
            panic!("Request Error. Method: update item. Error: {err:#?}");
        }

        tokio::time::sleep(Duration::from_millis(500)).await;

        let delete_res =
            delete_file(test_client.user_id.as_str(), item_id, &test_client.client).await;

        if let Ok(response) = delete_res {
            assert!(response.status().is_success());
        } else if let Err(err) = delete_res {
            panic!("Request Error. Method: drive delete. Error: {err:#?}");
        }
    } else if let Err(err) = upload_res {
        panic!("Request Error. Method: drive upload. Error: {err:#?}");
    }
}
