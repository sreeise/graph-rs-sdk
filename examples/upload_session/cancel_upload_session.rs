use bytes::Buf;
use graph_rs_sdk::http::{AsyncIterator, ResponseExt};
use graph_rs_sdk::*;

// This example shows creating an upload session for a new file
// and iterating through the individual upload session values
// along with canceling an upload session in the event of an error.
// See https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_createuploadsession?view=odsp-graph-online

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

// The path where you wan to place the file in OneDrive
// including the file name. For the root folder just
// put the file name here like so: :/file.ext:
static PATH_IN_ONE_DRIVE: &str = ":/Documents/file.ext:";

// The conflict behavior can be one of: fail, rename, or replace.
static CONFLICT_BEHAVIOR: &str = "rename";

pub async fn cancel_upload_session(bytes: &[u8]) -> GraphResult<()> {
    let client = GraphClient::new(ACCESS_TOKEN);

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

    // Get the request builder for canceling a request. Calling cancel() does not automatically
    // send the request.
    let cancel_request = iter.cancel();

    while let Some(result) = iter.next().await {
        match result {
            Ok(response) => {
                println!("{response:#?}");
            }
            Err(err) => {
                println!("{err:#?}");

                let response = cancel_request.send().await?;

                println!("{response:#?}");

                // Microsoft Graph may return error info in the request body
                // of a Response.
                if let Ok(body) = response.json::<serde_json::Value>().await {
                    println!("{body:#?}");
                }

                break;
            }
        }
    }

    Ok(())
}
