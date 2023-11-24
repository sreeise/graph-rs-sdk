use graph_http::traits::ResponseExt;
use graph_rs_sdk::http::FileConfig;
use graph_rs_sdk::*;
use std::ffi::OsStr;
use test_tools::oauth_request::{Environment, OAuthTestClient, DRIVE_ASYNC_THROTTLE_MUTEX2};
use test_tools::support::cleanup::AsyncCleanUp;

#[tokio::test]
async fn drive_download() {
    let _lock = DRIVE_ASYNC_THROTTLE_MUTEX2.lock().await;
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let response = client
            .drive(id.as_str())
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
}

#[tokio::test]
async fn drive_download_format() {
    let _lock = DRIVE_ASYNC_THROTTLE_MUTEX2.lock().await;
    if Environment::is_local() {
        if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
            let response = client
                .drive(id.as_str())
                .item_by_path(":/test_document.docx:")
                .get_items_content()
                .format("pdf")
                .send()
                .await
                .unwrap();

            assert!(response.status().is_success());

            let response2 = response
                .download(
                    &FileConfig::new("./test_files").file_name(OsStr::new("test_document.pdf")),
                )
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
}
