use graph_http::traits::ResponseExt;
use graph_rs_sdk::http::FileConfig;
use graph_rs_sdk::*;
use std::ffi::OsStr;
use test_tools::oauth_request::{Environment, DEFAULT_CLIENT_CREDENTIALS_MUTEX4};
use test_tools::support::cleanup::AsyncCleanUp;

#[tokio::test]
async fn drive_download() {
    let test_client = DEFAULT_CLIENT_CREDENTIALS_MUTEX4.lock().await;
    let response = test_client
        .client
        .drive(test_client.user_id.as_str())
        .item_by_path(":/test_document.docx:")
        .get_items_content()
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());

    let response2 = response
        .download(&FileConfig::new("./test_files"))
        .await
        .unwrap();

    let path_buf = response2.into_body();
    assert!(path_buf.exists());

    let file_location = "./test_files/test_document.docx";
    let mut clean_up = AsyncCleanUp::new_remove_existing(file_location);
    clean_up.rm_files(file_location.into());
}

#[tokio::test]
async fn drive_download_format() {
    if Environment::is_local() {
        let test_client = DEFAULT_CLIENT_CREDENTIALS_MUTEX4.lock().await;
        let response = test_client
            .client
            .drive(test_client.user_id.as_str())
            .item_by_path(":/test_document.docx:")
            .get_items_content()
            .format("pdf")
            .send()
            .await
            .unwrap();

        assert!(response.status().is_success());

        let response2 = response
            .download(&FileConfig::new("./test_files").file_name(OsStr::new("test_document.pdf")))
            .await
            .unwrap();

        let path_buf = response2.into_body();
        assert!(path_buf.exists());
        assert_eq!(path_buf.extension(), Some(OsStr::new("pdf")));
        assert_eq!(path_buf.file_name(), Some(OsStr::new("test_document.pdf")));

        let file_location = "./test_files/test_document.pdf";
        let mut clean_up = AsyncCleanUp::new_remove_existing(file_location);
        clean_up.rm_files(file_location.into());
    }
}
