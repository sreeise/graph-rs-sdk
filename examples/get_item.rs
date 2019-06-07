use rust_onedrive::oauth::OAuth;
use rust_onedrive::prelude::*;
use std::convert::TryFrom;

fn main() {
    // Get the item based on it's id and resource id (drive id or group id or users id, etc.)
    // Change the DriveResource below if using a resource id other then Drives.
    get_drive_item("ITEM_ID", "RESOURCE_ID");

    // Get the item by its path and name. Do not include the 'root:' part of the path
    // that the API docs require. This is done automatically.
    get_drive_item_path("RESOURCE_ID", "path/to/drive_item.txt");
}

fn get_drive() -> ItemResult<Drive> {
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json")?;
    let drive: Drive = Drive::try_from(oauth)?;
    Ok(drive)
}

fn get_drive_item(item_id: &str, resource_id: &str) {
    let mut drive = get_drive().unwrap();
    let drive_item: Value = drive
        .get_item(item_id, resource_id, DriveResource::Drives)
        .unwrap();
    println!("{:#?}", drive_item);
}

fn get_drive_item_path(resource_id: &str, path_to_item: &str) {
    let mut drive = get_drive().unwrap();
    let drive_item: Value = drive
        .get_item_by_path(resource_id, path_to_item, DriveResource::Drives)
        .unwrap();
    println!("{:#?}", drive_item);
}
