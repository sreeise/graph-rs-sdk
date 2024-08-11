#![allow(dead_code)]

// The BodyRead object is used to provide easy use of many different types.

/*
In request methods that require a body you can use the following values:

- Any thing that implements serde serialize
- Anything implementing Read or AsyncRead using BodyRead::from_reader and
   BodyRead::from_async_read
- reqwest::Body (Async requests only)
- reqwest::blocking::Body (Blocking requests only)
- FileConfig which is a helper object for downloading files but can also be used
  for uploading files.


BodyRead can also take serializable objects, reqwest::Body, and reqwest::blocking::Body but
you do not need to use BodyRead for those. You can pass these objects directly to the
body parameter for those api methods that need one.
 */

use graph_rs_sdk::http::BodyRead;
use graph_rs_sdk::GraphClient;
use std::fs::File;

fn main() {}

// When using reqwest::Body and reqwest::blocking::Body you should only use
// reqwest::Body if your using async and reqwest::blocking::Body when using
// blocking requests.

// If you use a reqwest::blocking::Body for an async method the tokio runtime
// will error and exit.

fn use_reqwest_blocking_body() {
    let body = reqwest::blocking::Body::from(String::new());

    let client = GraphClient::new("token");
    client
        .user("id")
        .get_mail_tips(body)
        .into_blocking()
        .send()
        .unwrap();
}

async fn use_reqwest_async_body() {
    let body = reqwest::Body::from(String::new());

    let client = GraphClient::new("token");
    client.user("id").get_mail_tips(body).send().await.unwrap();
}

// You can pass file types directly to the API method:

fn use_file_directly(file: File) -> anyhow::Result<()> {
    let client = GraphClient::new("token");
    let _ = client
        .drive("drive-id")
        .item_by_path("/drive/path")
        .update_items_content(file)
        .into_blocking()
        .send()?;

    Ok(())
}

async fn use_async_file_directly(file: tokio::fs::File) -> anyhow::Result<()> {
    let client = GraphClient::new("token");
    let _ = client
        .drive("drive-id")
        .item_by_path("/drive/path")
        .update_items_content(file)
        .send()
        .await?;

    Ok(())
}

// Or use reqwest::Body and reqwest::blocking::Body

fn use_reqwest_for_files(file: File) -> anyhow::Result<()> {
    let client = GraphClient::new("token");
    let _ = client
        .drive("drive-id")
        .item_by_path("/drive/path")
        .update_items_content(reqwest::blocking::Body::from(file))
        .into_blocking()
        .send()?;

    Ok(())
}

async fn use_reqwest_for_tokio_files(file: tokio::fs::File) -> anyhow::Result<()> {
    let client = GraphClient::new("token");
    let _ = client
        .drive("drive-id")
        .item_by_path("/drive/path")
        .update_items_content(reqwest::Body::from(file))
        .send()
        .await?;

    Ok(())
}

// Using BodyRead for types that implement Read or AsyncRead

// BodyRead is a helper struct for using many different types
// as the body of a request.

fn use_read(reader: impl std::io::Read) -> anyhow::Result<()> {
    let client = GraphClient::new("token");
    let _ = client
        .drive("drive-id")
        .item_by_path(":/drive/path:")
        .update_items_content(BodyRead::from_read(reader)?)
        .into_blocking()
        .send()?;

    Ok(())
}

async fn use_async_read(async_reader: impl tokio::io::AsyncReadExt + Unpin) -> anyhow::Result<()> {
    let client = GraphClient::new("token");
    let _ = client
        .drive("drive-id")
        .item_by_path(":/drive/path:")
        .update_items_content(BodyRead::from_async_read(async_reader).await?)
        .send()
        .await?;

    Ok(())
}
