use graph_error::GraphFailure;
use graph_rs::prelude::*;
use std::path::PathBuf;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
static USER_ID: &str = "USER_ID";

#[tokio::main]
async fn main() -> Result<(), GraphFailure> {
    let client = Graph::new_async(ACCESS_TOKEN);

    let download_client = client
        .v1()
        .drives(USER_ID)
        .drive()
        .download(":/download.txt:", "./test_files");

    let path_buf: PathBuf = download_client.send().await?;
    println!("{:#?}", path_buf);
    Ok(())
}
