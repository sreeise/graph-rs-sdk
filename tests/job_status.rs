use std::thread;
use std::time::Duration;
use test_tools::common::TestTools;
use test_tools::oauthrequest::{Environment, OAuthTestClient, THROTTLE_MUTEX};

fn delete_item(item_id: &str) {
    if let Some((drive_id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let _result = client
            .v1()
            .drive(drive_id.as_str())
            .delete_items(item_id)
            .send();
    }
}

#[test]
fn job_status() {
    if Environment::is_appveyor() {
        return;
    }

    let _lock = THROTTLE_MUTEX.lock().unwrap();

    let original_file = ":/test_job_status.txt:";
    let copy_name = "test_job_status_copy.txt";
    let copy_drive_path = ":/test_job_status_copy.txt:";

    // Delete file if its still there from last test run.
    delete_item(copy_drive_path);
    thread::sleep(Duration::from_secs(2));
    if let Some((drive_id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let result = client
            .v1()
            .drive(drive_id.as_str())
            .get_items(original_file)
            .send();

        let response = result.expect("Async get file: Drive");
        let id_option = response.body().unwrap()["id"].as_str();
        assert!(id_option.is_some());

        thread::sleep(Duration::from_secs(2));

        let copy_result = client
            .v1()
            .drive(drive_id.as_str())
            .copy_item(original_file, &serde_json::json!({ "name": copy_name }))
            .send();

        if let Err(e) = copy_result {
            panic!("Async job status for drive copy. Error: {:#?}", e);
        }

        TestTools::assert_success(&copy_result, "Async copy file: Drive");

        let copy_response = copy_result.expect("Async copy file: Drive");
        let job_status = copy_response.job_status().unwrap();

        if let Ok(response) = job_status {
            let status_option = response.body().unwrap()["status"].as_str();
            assert!(status_option.eq(&Some("inProgress")) | status_option.eq(&Some("completed")));

            if status_option.eq(&Some("completed")) {
                delete_item(copy_drive_path);
            }
        } else if let Err(e) = job_status {
            panic!("Async job status for drive copy. Error: {:#?}", e);
        }
    }
}
