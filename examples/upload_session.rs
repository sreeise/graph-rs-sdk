use rust_onedrive::drive::pipelines::uploadsessionpipeline::SessionResult;
use rust_onedrive::prelude::*;
use std::ffi::OsString;

// This example shows creating an upload session and iterating through the
// individual upload session values.
// See https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_createuploadsession?view=odsp-graph-online
fn main() {
    let drive = Drive::new("<ACCESS_TOKEN>");

    // The file path in OneDrive
    let path = OsString::from("Documents/<YOUR_FILE.ext>");

    // Pass the OneDrive path and the local path of the file you want to upload.
    let mut req = drive
        .v1()
        .me()
        .upload_session_new("./examples/example_files/<YOUR_FILE.ext>", None)
        .by_path(path);
    let mut upload_session = req.send().unwrap();

    // Iterate through each API request. The next method returns a SessionResult enum
    // where SessionResult::Next(UploadSession) containing the next UploadSession.
    // When the upload session is finished a SessionResult::Done(driveItem)
    // will be returned with the new DriveItem.

    while !upload_session.is_empty() {
        // The next() method executes a PUT request on each call
        // to get the next upload session.
        match upload_session.next() {
            Ok(session) => match session {
                SessionResult::Next(s) => {
                    println!("Current upload session: {:#?}", s);
                },
                SessionResult::Done(drive_item) => {
                    println!("Uploaded DriveItem: {:#?}", drive_item);
                    break;
                },
            },
            Err(e) => {
                println!("Error: {:#?}", e);
                break;
            },
        }
    }
}
