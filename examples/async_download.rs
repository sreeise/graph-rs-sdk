use graph_error::{download::AsyncDownloadError, GraphFailure};
use graph_rs_sdk::prelude::*;
use std::ffi::OsString;
use std::path::PathBuf;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
static USER_ID: &str = "USER_ID";

#[tokio::main]
async fn main() -> Result<(), Error> {
    download().await?;
    download_with_format().await?;
    Ok(())
}

async fn download() -> Result<(), Error> {
    let client = Graph::new_async(ACCESS_TOKEN);

    let download_client = client
        .v1()
        .drive(USER_ID)
        .download(":/download.txt:", "./examples");

    let path_buf: PathBuf = download_client.send().await?;
    println!("{:#?}", path_buf);
    Ok(())
}

async fn download_with_format() -> Result<(), Error> {
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

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Graph(#[from] GraphFailure),

    #[error(transparent)]
    Download(#[from] AsyncDownloadError),
}
