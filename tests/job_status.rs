use graph_error::GraphResult;
use graph_http::traits::ResponseExt;
use graph_rs_sdk::Graph;
use std::thread;
use std::time::Duration;

use test_tools::oauth_request::{Environment, OAuthTestClient, ASYNC_THROTTLE_MUTEX};

async fn delete_item(
    drive_id: &str,
    item_id: &str,
    client: &Graph,
) -> GraphResult<reqwest::Response> {
    client
        .drive(drive_id)
        .item_by_path(item_id)
        .delete_items()
        .send()
        .await
}

#[tokio::test]
async fn job_status() {
    if Environment::is_local() {
        let _lock = ASYNC_THROTTLE_MUTEX.lock();

        let original_file = ":/test_job_status.txt:";
        let copy_name = "test_job_status_copy.txt";
        let copy_drive_path = ":/test_job_status_copy.txt:";

        if let Some((drive_id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
            // Delete file if its still there from last test run.
            let _ = delete_item(drive_id.as_str(), copy_drive_path, &client).await;
            thread::sleep(Duration::from_secs(2));

            let result = client
                .drive(drive_id.as_str())
                .item_by_path(original_file)
                .get_items()
                .send()
                .await;

            let response = result.expect("Async get file: Drive");
            let body: serde_json::Value = response.json().await.unwrap();
            //let id_option = body["id"].as_str();
            assert!(body["id"].as_str().is_some());

            thread::sleep(Duration::from_secs(2));

            let copy_result = client
                .drive(drive_id.as_str())
                .item_by_path(original_file)
                .copy(&serde_json::json!({ "name": copy_name }))
                .send()
                .await;

            if let Err(e) = copy_result {
                panic!("Async job status for drive copy. Error: {e:#?}");
            } else if let Ok(response) = copy_result {
                assert!(response.status().is_success());

                let job_status = response.job_status().await.unwrap();

                if let Ok(response) = job_status {
                    let body: serde_json::Value = response.json().await.unwrap();
                    let status_option = body["status"].as_str();
                    assert!(
                        status_option.eq(&Some("inProgress"))
                            | status_option.eq(&Some("completed"))
                    );

                    if status_option.eq(&Some("completed")) {
                        let _ = delete_item(drive_id.as_str(), copy_drive_path, &client).await;
                    }
                } else if let Err(e) = job_status {
                    panic!("Async job status for drive copy. Error: {e:#?}");
                }
            }
        }
    }
}
