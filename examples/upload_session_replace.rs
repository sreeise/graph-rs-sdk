use graph_http::{NextSession, Session};
use graph_rs_sdk::prelude::*;

// This example shows creating an upload session to replace an existing file
// and iterating through the individual upload session values.
// See https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_createuploadsession?view=odsp-graph-online

static PARENT_ITEM_ID: &str = "ITEM_ID";
static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

// The file you want to upload.
// This could also be anything that implements AsRef<Path>
static PATH_TO_FILE: &str = "path/to/file/file.ext";

// The name of the file for the request body.
// Should be the the same as the actual file
// you are uploading.
static FILE_NAME: &str = "file.ext";

// The conflict behavior can be one of: fail, rename, or replace.
static CONFLICT_BEHAVIOR: &str = "replace";

fn main() {
    upload_session_replace();
}

fn upload_session_replace() {
    let client = Graph::new(ACCESS_TOKEN);

    let upload = Session {
        name: Some(FILE_NAME.into()),
        microsoft_graph_conflict_behavior: Some(CONFLICT_BEHAVIOR.into()),
        ..Default::default()
    };

    let session = client
        .v1()
        .me()
        .drive()
        .create_upload_session(PARENT_ITEM_ID, PATH_TO_FILE, &upload)
        .send();

    if let Ok(session) = session {
        for next in session {
            match next {
                Ok(NextSession::Next(response)) => {
                    println!("Response: {:#?}", response);
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
                    println!("Session finished. DriveItem: {:#?}", response.body());
                    break;
                }
                Err(e) => {
                    println!("Error: {:#?}", e);
                    break;
                }
            }
        }
    } else if let Err(e) = session {
        println!("Error: {:#?}", e);
    }
}
