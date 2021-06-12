use graph_error::GraphFailure;
use graph_http::traits::AsyncIterator;
use graph_http::NextSession;
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
// put the file name in the format: ":/file.ext:".
static PATH_IN_ONE_DRIVE: &str = ":/Documents/file.ext:";

// The conflict behavior can be one of: fail, rename, or replace.
static CONFLICT_BEHAVIOR: &str = "rename";

#[tokio::main]
async fn main() -> Result<(), GraphFailure> {
    let client = Graph::new_async(ACCESS_TOKEN);

    let upload = serde_json::json!({
        "@microsoft.graph.conflictBehavior": Some(CONFLICT_BEHAVIOR.to_string())
    });

    let response = client
        .v1()
        .me()
        .drive()
        .create_upload_session(PATH_IN_ONE_DRIVE, PATH_TO_FILE, &upload)
        .send()
        .await;

    if let Err(e) = response {
        println!("Error: {:#?}", e);
    } else if let Ok(mut session) = response {
        let cancel_request = session.cancel().await;

        while let Some(next) = session.next().await {
            match next {
                Ok(NextSession::Next(response)) => {
                    println!("\nResponse: {:#?}\n", response);
                    println!(
                        "Expiration date time: {:#?}",
                        response.body()["expirationDateTime"]
                    );
                    println!(
                        "Next expected ranges: {:#?}",
                        response.body()["nextExpectedRanges"]
                    );
                }
                Ok(NextSession::Done(response)) => {
                    // When the upload session is done the drive item metadata
                    // for the uploaded file and the last response is returned.
                    println!("\nResponse: {:#?}\n", response);
                    println!("Session finished. Body: {:#?}", response.body());
                    break;
                }
                Err(e) => {
                    println!("Error: {:#?}", e);
                    let response = cancel_request.send().await?;
                    // A successful response for canceling an upload session
                    // should be 204 No Content
                    println!("Response from canceled upload session: {:#?}", response);
                    break;
                }
            }
        }
    }
    Ok(())
}
