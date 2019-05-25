use rust_onedrive::drive::parentreference::ParentReference;
use rust_onedrive::oauth::OAuth;
use rust_onedrive::prelude::*;
use std::convert::TryFrom;
use std::thread;
use std::time::Duration;

// Set the name of the file you want to copy
// and a name for the copy of the file.
static DRIVE_FILE: &str = "YOUR_DRIVE_FILE_NAME";
static DRIVE_FILE_COPY_NAME: &str = "FILE_NAME_OF_COPY";

fn main() {
    copy_item();
}

fn copy_item() {
    let oauth: OAuth = OAuth::from_file("./examples/example_files/web_oauth.json").unwrap();
    let mut drive: Drive = Drive::try_from(oauth).unwrap();

    let mut drive_item: DriveItem = drive.drive_root_child().unwrap();

    // The file or folder that you want to copy.
    let value: Value = drive_item.find_by_name(DRIVE_FILE).unwrap();

    // The DriveItem copy request uses a ParentReference which contains the metadata
    // for the drive id and path specifying where the new copy should be placed.
    // The path below in the ParentReference is typically the same path as the drive item
    // requested above so the copy of the item will be placed in the same folder. This can
    // be changed to wherever you would like the copy placed.
    let parent_ref = ParentReference::new(None, None, None, Some("/drive/root:/Documents".into()));

    // A DriveItemCopy contains takes the parent reference, an optional name for the file copy,
    // and the drive resource that specifies the url to use for requesting the copy.
    let prc = DriveItemCopy::new(
        parent_ref,
        Some(DRIVE_FILE_COPY_NAME.into()),
        DriveResource::Drives,
    );

    let mut item_response: ItemResponse = drive.copy(value, prc).unwrap();
    println!("{:#?}", &item_response);

    // When an item is copied the response returns a URL in the location header
    // that can be used to monitor the progress. For events that may take longer to finish
    // such as copying an item, the ItemResponse event_progress() method can be used
    // to get the metadata returned from the monitor URL. This request returns an
    // EventProgress struct. Note, it is important to remember that EventProgress
    // is only used for specific API requests.
    //
    // The ItemResponse success() method will return true if the status of the
    // request returns 202 which means the request for copying an item is approved.
    // However, this does not mean that the copy event has finished. The
    // ItemResponse event_progress() should be used to check if the event
    // has finished instead of the success method.

    // Wait a few seconds before checking the progress (assuming the file or
    // folder size is small here).
    thread::sleep(Duration::from_secs(5));
    println!("{:#?}", &item_response.event_progress());
}
