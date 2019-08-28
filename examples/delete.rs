use rust_onedrive::drive::statusresponse::StatusResponse;
use rust_onedrive::oauth::OAuth;
use rust_onedrive::prelude::*;
use std::convert::TryFrom;
use std::ffi::OsString;

// Delete items in OneDrive. This will move deleted items to the recycle bin.
// It is recommended to create a new file that can be used for demonstration purposes here.
// Deleting an item can be done in 2 different ways shown in the methods below.
fn main() {
    delete_id("DRIVE_ITEM_ID");
    delete_path(OsString::from("PATH_FROM_ROOT"));
}

// Delte a drive item by id.
fn delete_id(item_id: &str) {
    // Create a new Drive instance.
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json").unwrap();
    let drive: Drive = Drive::try_from(oauth).unwrap();

    // Create the request to delete the drive item.
    let mut req = drive.v1().me().delete().by_id(item_id);

    // Send the request.
    let mut response: StatusResponse = req.send().unwrap();
    println!("{:#?}", response);
    println!("\nItem was deleted: {:#?}", response.success());
}

// Deleting an item by path.
pub fn delete_path(path: OsString) {
    // Create a new Drive instance.
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json").unwrap();
    let drive: Drive = Drive::try_from(oauth).unwrap();

    // Create the request to delete the drive item.
    let mut req = drive.v1().me().delete().by_path(path);

    // Send the request.
    let mut response: StatusResponse = req.send().unwrap();
    println!("{:#?}", response);
    println!("\nItem was deleted: {:#?}", response.success());
}
