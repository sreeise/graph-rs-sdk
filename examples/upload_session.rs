use graph_http::traits::{AsyncIterator, ResponseExt};
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

#[tokio::main]
async fn main() {
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

    //
    let mut iter = response.into_upload_session(file).await.unwrap();

    while let Some(Ok(response)) = iter.next().await {
        println!("{response:#?}");
    }
}
