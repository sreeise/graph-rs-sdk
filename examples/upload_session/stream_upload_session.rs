use bytes::{Buf, BytesMut};
use graph_rs_sdk::http::ResponseExt;
use graph_rs_sdk::*;

use futures::StreamExt;

// Stream bytes to a file in OneDrive
// See https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_createuploadsession?view=odsp-graph-online

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

// The path where you wan to place the file in OneDrive
// including the file name. For the root folder just
// put the file name here like so: :/file.ext:
static PATH_IN_ONE_DRIVE: &str = ":/Documents/file.ext:";

pub async fn stream(bytes: BytesMut) -> GraphResult<()> {
    let client = GraphClient::new(ACCESS_TOKEN);

    let upload = serde_json::json!({
        "@microsoft.graph.conflictBehavior": Some("fail".to_string())
    });

    let response = client
        .me()
        .drive()
        .item_by_path(PATH_IN_ONE_DRIVE)
        .create_upload_session(&upload)
        .send()
        .await?;

    let mut upload_session = response.into_upload_session(bytes.reader()).await?;
    let mut stream = upload_session.stream()?;

    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                println!("{response:#?}");

                let body: serde_json::Value = response.json().await?;
                println!("{body:#?}");
            }
            Err(err) => panic!("Error on upload session {:#?}", err),
        }
    }

    Ok(())
}
