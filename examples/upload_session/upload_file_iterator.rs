#![allow(dead_code)]

use graph_rs_sdk::http::{AsyncIterator, ResponseExt};
use graph_rs_sdk::prelude::*;

// This example shows creating an upload session for a new file
// and iterating through the individual upload session values.
// See https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_createuploadsession?view=odsp-graph-online

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

// The file you want to upload.
// This could also be anything that implements AsRef<Path>
static PATH_TO_FILE: &str = "path/to/file/file.ext";

// The path where you wan to place the file in OneDrive
// including the file name. For the root folder just
// put the file name here like so: :/file.ext:
static PATH_IN_ONE_DRIVE: &str = ":/Documents/file.ext:";

// The conflict behavior can be one of: fail, rename, or replace.
static CONFLICT_BEHAVIOR: &str = "rename";

// Use the into_upload_session method on a reqwest::Response to begin the upload session.
// The into_upload_session method takes any std::io::Reader so you can use things like
// Bytes and BytesMut from the bytes crate which has methods to get a reader, files, Vec<u8>, etc.

// Use into_upload_session_async_read() for readers implementing tokio::io::AsyncReadExt

pub async fn upload_file(file: std::fs::File) -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let upload = serde_json::json!({
        "@microsoft.graph.conflictBehavior": Some(CONFLICT_BEHAVIOR.to_string())
    });

    let response = client
        .me()
        .drive()
        .item_by_path(PATH_IN_ONE_DRIVE)
        .create_upload_session(&upload)
        .send()
        .await
        .unwrap();

    let mut iter = response.into_upload_session(file).await?;

    while let Some(result) = iter.next().await {
        let response = result?;
        println!("{response:#?}");
    }

    Ok(())
}

pub async fn upload_file_async_read() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let upload = serde_json::json!({
        "@microsoft.graph.conflictBehavior": Some(CONFLICT_BEHAVIOR.to_string())
    });

    let response = client
        .me()
        .drive()
        .item_by_path(PATH_IN_ONE_DRIVE)
        .create_upload_session(&upload)
        .send()
        .await
        .unwrap();

    let file = tokio::fs::File::open(PATH_IN_ONE_DRIVE).await?;

    let mut iter = response.into_upload_session_async_read(file).await?;

    while let Some(result) = iter.next().await {
        let response = result?;
        println!("{response:#?}");
    }

    Ok(())
}
