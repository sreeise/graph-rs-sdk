use graph_rs_sdk::error::download::AsyncDownloadError;
use graph_rs_sdk::http::FileConfig;

use graph_http::traits::ResponseExt;
use std::ffi::OsStr;
use test_tools::oauth_request::Environment;
use test_tools::oauth_request::DEFAULT_CLIENT_CREDENTIALS_MUTEX2;

#[tokio::test]
async fn download_config_file_exists() {
    if Environment::is_local() {
        let test_client = DEFAULT_CLIENT_CREDENTIALS_MUTEX2.lock().await;
        let response = test_client
            .client
            .user(test_client.user_id.as_str())
            .drive()
            .item_by_path(":/downloadtestdoc.txt:")
            .get_items_content()
            .send()
            .await
            .unwrap();

        let result = response
            .download(&FileConfig::new("./test_files").file_name(OsStr::new("downloadtestdoc.txt")))
            .await;

        match result {
            Ok(response2) => panic!("Download request should have thrown AsyncDownloadError::FileExists. Instead got successful Response: {:#?}", response2),

            Err(AsyncDownloadError::FileExists(name)) => {
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
    if Environment::is_local() {
        let test_client = DEFAULT_CLIENT_CREDENTIALS_MUTEX2.lock().await;
        let response = test_client
            .client
            .user(test_client.user_id.as_str())
            .drive()
            .item_by_path(":/downloadtestdoc.txt:")
            .get_items_content()
            .send()
            .await
            .unwrap();

        let result = response
            .download(&FileConfig::new("./test_files/download_dir").create_directories(false))
            .await;

        match result {
            Ok(response) => panic!("Download request should have thrown AsyncDownloadError::TargetDoesNotExist. Instead got successful PathBuf: {response:#?}"),
            Err(AsyncDownloadError::TargetDoesNotExist(dir)) => assert_eq!("./test_files/download_dir".to_string(), dir),
            Err(err) => panic!("Incorrect error thrown. Should have been AsyncDownloadError::TargetDoesNotExist. Got: {err:#?}"),
        }
    }
}
