use graph_error::GraphFailure;
use graph_rs_sdk::prelude::*;
use std::{ffi::OsString, path::PathBuf};

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
static USER_ID: &str = "USER_ID";

#[tokio::main]
async fn main() -> Result<(), GraphFailure> {
    download().await?;
    download_with_format().await?;
    Ok(())
}

async fn download() -> Result<(), GraphFailure> {
    let client = Graph::new_async(ACCESS_TOKEN);

    let download_client = client
        .v1()
        .drive(USER_ID)
        .download(":/download.txt:", "./examples");

    let path_buf: PathBuf = download_client.send().await?;
    println!("{:#?}", path_buf);
    Ok(())
}

async fn download_with_format() -> Result<(), GraphFailure> {
    let client = Graph::new_async(ACCESS_TOKEN);

    let download_client = client
        .v1()
        .drive(USER_ID)
        .download(":/download.docx:", "./examples");

    download_client.format("pdf").await;
    download_client.rename(OsString::from("download.pdf")).await;
    let path_buf: PathBuf = download_client.send().await?;
    println!("{:#?}", path_buf);
    Ok(())
}
