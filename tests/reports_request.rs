use graph_core::resource::ResourceIdentity;
use graph_http::traits::ResponseExt;
use graph_rs_sdk::http::FileConfig;
use std::ffi::OsStr;
use test_tools::oauth_request::*;
use test_tools::support::cleanup::AsyncCleanUp;

#[tokio::test]
async fn async_download_office_365_user_counts_reports_test() {
    if Environment::is_local() || Environment::is_github() {
        return;
    }

    let _ = ASYNC_THROTTLE_MUTEX.lock().await;

    if let Some((_id, client)) =
        OAuthTestClient::graph_by_rid_async(ResourceIdentity::Reports).await
    {
        let response = client
            .reports()
            .get_office_365_active_user_counts_by_period("D90")
            .send()
            .await
            .expect("Request Error. API: Reports | Method: download_async get_office_365_active_user_counts.");

        assert!(response.status().is_success());

        let response2 = response
            .download(
                &FileConfig::new("./test_files")
                    .file_name(OsStr::new("async_user_count_report.csv")),
            )
            .await
            .unwrap();

        let path_buf = response2.into_body();
        assert!(path_buf.exists());

        let file_location = "./test_files/async_user_count_report.csv";
        let mut clean_up = AsyncCleanUp::new_remove_existing(file_location);
        clean_up.rm_files(file_location.into());
    }
}

#[tokio::test]
async fn get_office_365_user_counts_reports_text() {
    if Environment::is_local() {
        let _ = ASYNC_THROTTLE_MUTEX.lock().await;

        if let Some((_id, client)) =
            OAuthTestClient::graph_by_rid_async(ResourceIdentity::Reports).await
        {
            let response = client
                .reports()
                .get_office_365_active_user_counts_by_period("D90")
                .send()
                .await
                .unwrap();

            assert!(response.status().is_success());
            let text = response.text().await.unwrap();
            assert!(!text.is_empty());
        }
    }
}
