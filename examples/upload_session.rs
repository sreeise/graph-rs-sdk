#![allow(dead_code)]

use bytes::{Buf, Bytes};
use graph_rs_sdk::http::{AsyncIterator, ResponseExt};
use graph_rs_sdk::prelude::*;
use std::fs::OpenOptions;

// This example shows creating an upload session for a new file
// and iterating through the individual upload session values.
// See https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_createuploadsession?view=odsp-graph-online

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

// The file you want to upload.
// This could also be anything that implements AsRef<Path>
static PATH_TO_FILE: &str = "path/to/file/file.ext";

// The path where you wan to place the file in OneDrive
// including the file name. For the root folder just
// put the file name here.
static PATH_IN_ONE_DRIVE: &str = ":/Documents/file.ext:";

// The conflict behavior can be one of: fail, rename, or replace.
static CONFLICT_BEHAVIOR: &str = "rename";

// Use the into_upload_session method on a reqwest::Response to begin the upload session.
// The into_upload_session method takes any std::io::Reader so you can use things like
// Bytes and BytesMut from the bytes crate which has methods to get a reader, files, Vec<u8>, etc.

// Use the []() for tokio::io::AsyncReadExt

#[tokio::main]
async fn main() -> GraphResult<()> {
    upload_file().await?;
    upload_bytes().await?;
    Ok(())
}

async fn upload_file() -> GraphResult<()> {
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

    let file = OpenOptions::new().read(true).open(PATH_TO_FILE).unwrap();

    let mut iter = response.into_upload_session(file).await?;

    while let Some(result) = iter.next().await {
        let response = result?;
        println!("{response:#?}");
    }

    Ok(())
}

async fn upload_file_async_read() -> GraphResult<()> {
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

// Requires bytes crate
async fn upload_bytes() -> GraphResult<()> {
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

    let bytes = Bytes::new();

    let mut iter = response.into_upload_session(bytes.reader()).await?;

    while let Some(result) = iter.next().await {
        let response = result?;
        println!("{response:#?}");
    }

    Ok(())
}

async fn upload_vec_u8(bytes: &[u8]) -> GraphResult<()> {
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

    let mut iter = response.into_upload_session(bytes.reader()).await?;

    while let Some(result) = iter.next().await {
        let response = result?;
        println!("{response:#?}");
    }

    Ok(())
}

async fn cancel_upload_session(bytes: &[u8]) -> GraphResult<()> {
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

    let mut iter = response.into_upload_session(bytes.reader()).await?;

    while let Some(result) = iter.next().await {
        match result {
            Ok(response) => {
                println!("{response:#?}");
            }
            Err(err) => {
                println!("{err:#?}");
                let response = iter.cancel().await?;
                println!("{response:#?}");
            }
        }
    }

    Ok(())
}
