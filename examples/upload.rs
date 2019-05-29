use graph_oauth::oauth::OAuth;
use rust_onedrive::drive::parentreference::ParentReference;
use rust_onedrive::prelude::*;
use std::convert::TryFrom;

// Put the path to your file and the file name itself that
// you want to upload to one drive.
static LOCAL_FILE_PATH: &str = "/path/to/file/file.txt";

// The drive id and parent id for a users drive.
// You can get these from a parent_reference which are also
// stored in the drive::value::Value's that represent drive items
// such as files and folders. This is shown in the second example below.
// If you want to use the DriveResource::Me endpoint, which represents
// the current signed in user, then just set the DRIVE_FILE_ID as en
// empty string.
static DRIVE_FILE_ID: &str = "DRIVE_ID";
static DRIVE_PARENT_ID: &str = "PARENT_ID";

fn main() {
    upload_file();
    // Or
    upload_using_parent_reference("FILE_NAME").unwrap();
}

fn get_drive() -> ItemResult<Drive> {
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json")?;
    let drive: Drive = Drive::try_from(oauth)?;
    Ok(drive)
}

// Uploading a file using the drive id and parent id.
fn upload_file() {
    let mut drive = get_drive().unwrap();

    // The API response returns a drive::value::Value representing the uploaded item.
    let value: Value = drive
        .upload(
            LOCAL_FILE_PATH,
            DRIVE_FILE_ID,
            DRIVE_PARENT_ID,
            DriveResource::Drives,
        )
        .unwrap();
    println!("{:#?}", value);
}

// Upload a file using a ParentReference.
// This example uses the root folder of a users one drive where
// other common folders such as Documents are stored.
fn upload_using_parent_reference(drive_file_name: &str) -> ItemResult<()> {
    // Get the latest metadata for the root drive folder items.
    let mut drive: Drive = get_drive()?;
    let mut drive_item: DriveItem = drive.drive_root_child()?;

    // Get a drive item Value which stores a parent_reference for the drive id and parent id.
    // The metadata for a drive_root_child is those files that are stored in the drives root
    // folder so choose a file name for a file that resides there.
    let value: Value = drive_item.find_by_name(drive_file_name)?;
    let parent_reference: ParentReference = value.parent_reference().unwrap();

    // The API response returns a drive::value::Value representing the uploaded item.
    let value: Value =
        drive.upload_by_parent_ref(LOCAL_FILE_PATH, &parent_reference, DriveResource::Drives)?;
    println!("{:#?}", value);
    Ok(())
}
