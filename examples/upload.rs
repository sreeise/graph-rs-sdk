use rust_onedrive::prelude::*;

// Put the path to your file and the file name itself that
// you want to upload to one drive.
static LOCAL_FILE_PATH: &str = "/path/to/file/file.txt";

// Parent folder id of where to store this file.
static DRIVE_PARENT_ID: &str = "PARENT_ID";

// If you are using a sites, groups, drives, or users
// path.
static RESOURCE_ID: &str = "RESOURCE_ID";

fn main() {
    upload_file();
    // Or
    upload_new();
    // Using a drives, sites, groups, or users path.
    sites_upload_new();
}

// Uploading a file using the drive id and parent id.
fn upload_file() {
    let drive = Drive::new("ACCESS_TOKEN");
    let mut req = drive
        .v1()
        .me()
        .upload_replace(LOCAL_FILE_PATH)
        .by_id(DRIVE_PARENT_ID);
    // The update drive item metadata.
    let drive_item: DriveItem = req.send().unwrap();

    println!("{:#?}", drive_item);
}

fn upload_new() {
    let drive = Drive::new("ACCESS_TOKEN");

    let mut req = drive
        .v1()
        .me()
        .upload_new(LOCAL_FILE_PATH)
        .unwrap()
        .by_id(DRIVE_PARENT_ID);
    let drive_item: DriveItem = req.send().unwrap();
    println!("{:#?}", drive_item);
}

// Upload a file using a ParentReference.
// This example uses the root folder of a users one drive where
// other common folders such as Documents are stored.
fn sites_upload_new() {
    // Get the latest metadata for the root drive folder items.
    let drive = Drive::new("ACCESS_TOKEN");

    let mut req = drive
        .v1()
        .sites()
        .upload_new(LOCAL_FILE_PATH)
        .unwrap()
        .by_id(DRIVE_PARENT_ID, RESOURCE_ID);
    let drive_item: DriveItem = req.send().unwrap();
    println!("{:#?}", drive_item);
}
