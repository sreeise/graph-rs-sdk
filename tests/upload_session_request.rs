use futures::StreamExt;
use graph_error::{GraphFailure, GraphResult};
use graph_http::api_impl::UploadSession;
use graph_http::traits::ResponseExt;
use graph_rs_sdk::Graph;
use std::thread;
use std::time::Duration;
use test_tools::oauth_request::{OAuthTestClient, DRIVE_ASYNC_THROTTLE_MUTEX};

async fn delete_item(
    drive_id: &str,
    item_id: &str,
    client: &Graph,
) -> GraphResult<reqwest::Response> {
    client
        .drive(drive_id)
        .item(item_id)
        .delete_items()
        .send()
        .await
}

async fn stream_upload_session(mut upload_session: UploadSession) -> GraphResult<Option<String>> {
    let cancel_request = upload_session.cancel();
    let mut stream = upload_session.stream()?;
    let mut item_id: Option<String> = None;
    let mut counter = 0;

    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                assert!(response.status().is_success());
                counter += 1;

                let body: serde_json::Value = response.json().await?;
                let expiration_date_time = body["expirationDateTime"].as_str();

                if expiration_date_time.is_none() {
                    assert!(body["@content.downloadUrl"].as_str().is_some());

                    let upload_item_id = body["id"].as_str().unwrap();
                    item_id = Some(upload_item_id.to_string());

                    // We should get at the very least 8 or more responses.
                    // The file is 5.14 MB (5,392,116 bytes).
                    assert!(counter >= 8);

                    return Ok(item_id);
                } else {
                    assert!(expiration_date_time.is_some());
                }
            }
            Err(err) => {
                cancel_request.send().await?;
                return Err(err);
            }
        }
    }

    // We should get at the very least 8 or more responses.
    // The file is 5.14 MB (5,392,116 bytes).
    assert!(counter >= 8);

    Ok(item_id)
}

async fn channel_upload_session(mut upload_session: UploadSession) -> GraphResult<Option<String>> {
    let cancel_request = upload_session.cancel();
    let mut receiver = upload_session.channel()?;
    let mut item_id: Option<String> = None;
    let mut counter = 0;

    while let Some(result) = receiver.recv().await {
        match result {
            Ok(response) => {
                assert!(response.status().is_success());
                counter += 1;

                let body: serde_json::Value = response.json().await?;
                let expiration_date_time = body["expirationDateTime"].as_str();

                if expiration_date_time.is_none() {
                    assert!(body["@content.downloadUrl"].as_str().is_some());

                    let upload_item_id = body["id"].as_str().unwrap();
                    item_id = Some(upload_item_id.to_string());

                    // We should get at the very least 8 or more responses.
                    // The file is 5.14 MB (5,392,116 bytes).
                    assert!(counter >= 8);

                    return Ok(item_id);
                } else {
                    assert!(expiration_date_time.is_some());
                }
            }
            Err(err) => {
                cancel_request.send().await?;
                return Err(err).map_err(GraphFailure::from);
            }
        }
    }

    // We should get at the very least 8 or more responses.
    // The file is 5.14 MB (5,392,116 bytes).
    assert!(counter >= 8);

    Ok(item_id)
}

async fn file_upload_session_stream(
    drive_id: &str,
    item_by_path: &str,
    local_file: &str,
    client: &Graph,
) -> GraphResult<String> {
    let upload = serde_json::json!({
        "@microsoft.graph.conflictBehavior": Some("fail".to_string())
    });

    let response = client
        .drive(drive_id)
        .item_by_path(item_by_path)
        .create_upload_session(&upload)
        .send()
        .await?;

    assert!(response.status().is_success());

    let file = std::fs::File::open(local_file)?;

    let upload_session_task = response.into_upload_session(file).await?;
    let item_id = stream_upload_session(upload_session_task).await?.unwrap();
    Ok(item_id)
}

async fn file_upload_session_channel(
    drive_id: &str,
    item_by_path: &str,
    local_file: &str,
    client: &Graph,
) -> GraphResult<String> {
    let upload = serde_json::json!({
        "@microsoft.graph.conflictBehavior": Some("fail".to_string())
    });

    let response = client
        .drive(drive_id)
        .item_by_path(item_by_path)
        .create_upload_session(&upload)
        .send()
        .await?;

    assert!(response.status().is_success());

    let file = tokio::fs::File::open(local_file).await?;
    let upload_session_task = response.into_upload_session_async_read(file).await?;
    let item_id = channel_upload_session(upload_session_task).await?.unwrap();
    Ok(item_id)
}

// This is a long running test. 20 - 30 seconds.
#[tokio::test]
async fn test_upload_session() {
    let _lock = DRIVE_ASYNC_THROTTLE_MUTEX.lock().await;
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let item_by_path = ":/upload_session_file.txt:";
        let local_file = "./test_files/upload_session_file.txt";

        // Stream Upload Session
        let stream_item_id =
            file_upload_session_stream(id.as_str(), item_by_path, local_file, &client)
                .await
                .unwrap();
        let response = delete_item(id.as_str(), stream_item_id.as_str(), &client)
            .await
            .unwrap();
        assert!(response.status().is_success());

        thread::sleep(Duration::from_secs(2));

        // Channel Upload Session
        let channel_item_id =
            file_upload_session_channel(id.as_str(), item_by_path, local_file, &client)
                .await
                .unwrap();
        let response = delete_item(id.as_str(), channel_item_id.as_str(), &client)
            .await
            .unwrap();
        assert!(response.status().is_success());
    }
}
