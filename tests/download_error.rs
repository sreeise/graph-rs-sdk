use graph_rs_sdk::error::{download::AsyncDownloadError, GraphFailure};
use graph_rs_sdk::http::FileConfig;
use graph_rs_sdk::prelude::*;
use std::ffi::OsStr;
use test_tools::oauthrequest::OAuthTestClient;
use test_tools::oauthrequest::DRIVE_ASYNC_THROTTLE_MUTEX;

#[tokio::test]
#[should_panic]
async fn download_config_dir_no_exists() {
    let client = Graph::new("");

    let _ = client
        .me()
        .drive()
        .item("")
        .get_items_content()
        .download(&FileConfig::new("./test_files/download_dir"))
        .await
        .unwrap();
}

#[tokio::test]
async fn download_config_file_exists() {
    let _lock = DRIVE_ASYNC_THROTTLE_MUTEX.lock().await;
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let result = client
            .user(id.as_str())
            .drive()
            .item_by_path(":/downloadtestdoc.txt:")
            .get_items_content()
            .download(&FileConfig::new("./test_files").file_name(OsStr::new("downloadtestdoc.txt")))
            .await;

        match result {
			Ok(path) => panic!("Download request should have thrown AsyncDownloadError::FileExists. Instead got successful PathBuf: {path:#?}"),

			Err(GraphFailure::AsyncDownloadError(AsyncDownloadError::FileExists(name))) => {
				if cfg!(target_os = "windows") {
					assert_eq!(name, "./test_files\\downloadtestdoc.txt".to_string());
				} else {
					assert_eq!(name, "./test_files/downloadtestdoc.txt".to_string());
				}
			}

			Err(err) => panic!("Incorrect error thrown. Should have been AsyncDownloadError::FileExists. Got: {err:#?}"),
		}
    }
}

#[tokio::test]
async fn download_is_err_config_dir_no_exists() {
    let _lock = DRIVE_ASYNC_THROTTLE_MUTEX.lock().await;
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let response = client
            .user(id.as_str())
            .drive()
            .item_by_path(":/downloadtestdoc.txt:")
            .get_items_content()
            .download(&FileConfig::new("./test_files/download_dir"))
            .await;

        match response {
			Ok(path) => panic!("Download request should have thrown AsyncDownloadError::TargetDoesNotExist. Instead got successful PathBuf: {path:#?}"),
			Err(GraphFailure::AsyncDownloadError(AsyncDownloadError::TargetDoesNotExist(dir))) => assert_eq!("./test_files/download_dir".to_string(), dir),
			Err(err) => panic!("Incorrect error thrown. Should have been GraphRsError::DownloadDirNoExists. Got: {err:#?}"),
		}
    }
}
