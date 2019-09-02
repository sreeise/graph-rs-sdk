use rust_onedrive::http::{NextSession, Session};
use rust_onedrive::prelude::*;

// This example shows creating an upload session to replace an existing file
// and iterating through the individual upload session values.
// See https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_createuploadsession?view=odsp-graph-online

static ITEM_ID: &str = "ITEM_ID";
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

    let mut upload = Session::default();
    upload.name = Some(FILE_NAME.into());
    upload.microsoft_graph_conflict_behavior = Some(CONFLICT_BEHAVIOR.into());

    let session = client
        .v1()
        .me()
        .drive()
        .upload_session(PATH_TO_FILE, upload)
        .by_id(ITEM_ID)
        .send();

    if let Ok(session) = session {
        let mut iter = session.into_iter();

        while let Some(next) = iter.next() {
            match next {
                Ok(NextSession::Next((session, response))) => {
                    println!("Response: {:#?}", response);
                    println!("Expiration date time: {:#?}", session.expiration_date_time);
                    println!("Next expected ranges: {:#?}", session.next_expected_ranges);
                },
                Ok(NextSession::Done((drive_item, _response))) => {
                    println!("Session finished. DriveItem: {:#?}", drive_item);
                },
                Err(e) => {
                    println!("Error: {:#?}", e);
                    break;
                },
            }
        }
    } else if let Err(e) = session {
        println!("Error: {:#?}", e);
    }
}
