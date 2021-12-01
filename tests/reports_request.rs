use graph_core::resource::ResourceIdentity;
use graph_rs_sdk::prelude::*;
use std::ffi::OsString;
use std::path::Path;
use test_tools::common::TestTools;
use test_tools::oauthrequest::*;
use test_tools::support::cleanup::{AsyncCleanUp, CleanUp};

#[test]
fn download_office_365_user_counts_reports_test() {
    let _lock = THROTTLE_MUTEX.lock().unwrap();

    if let Some((_id, client)) = OAuthTestClient::graph_by_rid(ResourceIdentity::Reports) {
        let file_location = "./test_files/user_count_report.csv";
        let mut clean_up = CleanUp::new(|| {
            let path = Path::new(file_location);
            if path.exists() {
                std::fs::remove_file(path).unwrap();
            }
        });
        clean_up.rm_files(file_location.into());

        let download_client = client
            .v1()
            .reports()
            .get_office_365_active_user_counts("D90");

        download_client.rename(OsString::from("user_count_report.csv"));
        download_client.set_dir("./test_files");
        let path_buf = download_client.send().expect(
            "Request Error. API: Reports | Method: download get_office_365_active_user_counts.",
        );

        assert!(path_buf.exists());
    }
}

#[tokio::test]
async fn async_download_office_365_user_counts_reports_test() {
    let _lock = THROTTLE_MUTEX.lock().unwrap();

    if let Some((_id, client)) =
    OAuthTestClient::graph_by_rid_async(ResourceIdentity::Reports).await
    {
        let file_location = "./test_files/async_user_count_report.csv";
        let mut clean_up = AsyncCleanUp::new_remove_existing(file_location);
        clean_up.rm_files(file_location.into());

        let download_client = client
            .v1()
            .reports()
            .get_office_365_active_user_counts("D90")
            .await;

        download_client
            .rename(OsString::from("async_user_count_report.csv"))
            .await;
        download_client
            .set_dir("./test_files")
            .await;

        let path_buf = download_client.send()
            .await
            .expect("Request Error. API: Reports | Method: download_async get_office_365_active_user_counts.");

        assert!(path_buf.exists());
    }
}
