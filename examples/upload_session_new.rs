use graph_rs::http::{NextSession, Session};
use graph_rs::prelude::*;

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
static PATH_IN_ONE_DRIVE: &str = "Documents/file.ext";

// The conflict behavior can be one of: fail, rename, or replace.
static CONFLICT_BEHAVIOR: &str = "rename";

fn main() {
    upload_session_new();
}

fn upload_session_new() {
    let client = Graph::new(ACCESS_TOKEN);

    let mut upload = Session::default();
    upload.microsoft_graph_conflict_behavior = Some(CONFLICT_BEHAVIOR.into());

    let session = client
        .v1()
        .me()
        .drive()
        .upload_session(PATH_TO_FILE, upload)
        .by_path(PATH_IN_ONE_DRIVE)
        .send();

    if let Ok(mut session) = session {
        let cancel_request = session.cancel();
        let mut iter = session.into_iter();

        while let Some(next) = iter.next() {
            match next {
                Ok(NextSession::Next((session, response))) => {
                    println!("\nResponse: {:#?}\n", response);
                    println!("Expiration date time: {:#?}", session.expiration_date_time);
                    println!("Next expected ranges: {:#?}", session.next_expected_ranges);
                },
                Ok(NextSession::Done((drive_item, response))) => {
                    // When the upload session is done the drive item metadata
                    // for the uploaded file and the last response is returned.
                    println!("\nResponse: {:#?}\n", response);
                    println!("Session finished. DriveItem: {:#?}", drive_item);
                    break;
                },
                Err(e) => {
                    println!("Error: {:#?}", e);
                    let response = cancel_request.send().unwrap();
                    // A successful response for canceling an upload session
                    // should be 204 No Content
                    println!("Response from canceled upload session: {:#?}", response);
                    break;
                },
            }
        }
    } else if let Err(e) = session {
        println!("Error: {:#?}", e);
    }
}
