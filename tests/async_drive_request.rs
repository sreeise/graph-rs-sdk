use graph_error::GraphError;
use graph_rs::http::AsyncIterator;
use graph_rs::http::NextSession;
use graph_rs::prelude::*;
use std::path::PathBuf;
use std::time::Duration;
use test_tools::oauthrequest::OAuthRequest;
use test_tools::oauthrequest::ASYNC_THROTTLE_MUTEX;
use test_tools::support::cleanup::AsyncCleanUp;
use test_tools::FileUtils;

#[tokio::test]
async fn async_download() {
    let _lock = ASYNC_THROTTLE_MUTEX.lock().await;

    if let Some((id, token)) = OAuthRequest::request_access_token_async().await {
        let file_location = "./test_files/download_async.txt";
        let mut clean_up = AsyncCleanUp::new_remove_existing(file_location);
        clean_up.rm_files(file_location.into());

        let bearer = token.bearer_token();
        let client = Graph::new_async(&bearer);

        let download = client
            .v1()
            .users(id.as_str())
            .drive()
            .download(":/download_async.txt:", "./test_files");

        let path_buf: PathBuf = download.send().await.unwrap();
        FileUtils::verify_contents_async(path_buf.as_path(), "ONEDRIVE ASYNC DOWNLOAD TEST".into())
            .await;
    }
}

#[tokio::test]
async fn async_upload_session() {
    let _lock = ASYNC_THROTTLE_MUTEX.lock().await;

    if let Some((id, token)) = OAuthRequest::request_access_token_async().await {
        let client = Graph::new_async(token.bearer_token());

        let upload = serde_json::json!({
            "@microsoft.graph.conflictBehavior": Some("fail".to_string())
        });

        let response = client
            .v1()
            .users(id.as_str())
            .drive()
            .upload_session(
                ":/async_upload_session.txt:",
                "./test_files/async_upload_session.txt",
                &upload,
            )
            .send()
            .await;

        if let Err(e) = response {
            panic!(
                "Request error. Upload session new on initial request. Error: {:#?}",
                e
            );
        } else if let Ok(mut session) = response {
            let cancel_request = session.cancel().await;

            while let Some(next) = session.next().await {
                match next {
                    Ok(NextSession::Next(response)) => {
                        assert!(!GraphError::is_error(response.status()));
                    },
                    Ok(NextSession::Done(response)) => {
                        assert!(!GraphError::is_error(response.status()));
                        let drive_item = response.body();
                        let drive_item_id =
                            drive_item["id"].as_str().unwrap_or_default().to_string();
                        tokio::time::delay_for(Duration::from_secs(3)).await;

                        let delete_res = client
                            .v1()
                            .users(id.as_str())
                            .drive()
                            .delete(drive_item_id.as_str())
                            .send()
                            .await;

                        if let Ok(res) = delete_res {
                            assert!(res.error().is_none());
                            assert_eq!(res.status(), 204);
                        } else if let Err(e) = delete_res {
                            panic!("Request error. Upload session new. Error: {:#?}", e);
                        }
                        break;
                    },
                    Err(e) => {
                        let _ = cancel_request.send().await.unwrap();
                        panic!("Request error. Upload session new. Error: {:#?}", e);
                    },
                }
            }
        }
    }
}
