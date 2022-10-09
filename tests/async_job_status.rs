use std::time::Duration;
use test_tools::common::TestTools;
use test_tools::oauthrequest::ASYNC_THROTTLE_MUTEX;
use test_tools::oauthrequest::{Environment, OAuthTestClient};

async fn async_delete_item(item_id: &str) {
    if let Some((drive_id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let _result = client
            .v1()
            .drive(drive_id.as_str())
            .delete_items(item_id)
            .send()
            .await;
    }
}

#[tokio::test]
async fn async_job_status() {
    if Environment::is_appveyor() {
        return;
    }

    let _lock = ASYNC_THROTTLE_MUTEX.lock().await;

    let original_file = ":/test_async_job_status.txt:";
    let copy_name = "test_async_job_status_copy.txt";
    let copy_drive_path = ":/test_async_job_status_copy.txt:";

    // Delete file if its still there from last test run.
    async_delete_item(copy_drive_path).await;
    tokio::time::sleep(Duration::from_secs(2)).await;
    if let Some((drive_id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let result = client
            .v1()
            .drive(drive_id.as_str())
            .get_items(original_file)
            .send()
            .await;

        let response = result.expect("Async get file: Drive");
        let id_option = response.body().unwrap()["id"].as_str();
        assert!(id_option.is_some());

        tokio::time::sleep(Duration::from_secs(2)).await;

        let copy_result = client
            .v1()
            .drive(drive_id.as_str())
            .copy_item(original_file, &serde_json::json!({ "name": copy_name }))
            .send()
            .await;

        if let Err(e) = copy_result {
            panic!("Async job status for drive copy. Error: {:#?}", e);
        }

        TestTools::assert_success(&copy_result, "Async copy file: Drive");

        let copy_response = copy_result.expect("Async copy file: Drive");
        let job_status = copy_response.async_job_status().await.unwrap();

        if let Ok(response) = job_status {
            let status_option = response.body().unwrap()["status"].as_str();
            assert!(status_option.eq(&Some("inProgress")) | status_option.eq(&Some("completed")));

            if status_option.eq(&Some("completed")) {
                async_delete_item(copy_drive_path).await;
            }
        } else if let Err(e) = job_status {
            panic!("Async job status for drive copy. Error: {:#?}", e);
        }
    }
}
