use rust_onedrive::prelude::*;

// This example shows choosing a file in the root of a drive (normally where
// folders such as Documents are), and changing the name.
static DRIVE_FILE: &str = "DRIVE_FILE_NAME.txt";
static DRIVE_FILE_NEW_NAME: &str = "NEW_DRIVE_FILE_NAME.txt";

fn main() {
    update_item();
    // or
    update_by_drive_item();
}

fn get_drive_recent() -> Collection<DriveItem> {
    let drive = Drive::new("ACCESS_TOKEN");
    let mut req = drive.v1().drive_recent();
    req.send().unwrap()
}

fn update_item() {
    // Get the latest metadata for the root drive folder items.
    let drive = Drive::new("ACCESS_TOKEN");
    let mut collection = get_drive_recent();

    // Get the value you want to update. The drive::value::Value struct
    // stores metadata about a drive item such as a folder or file.
    let value: DriveItem = collection.find_by_name(DRIVE_FILE).unwrap();

    // Get the item id of the item that needs updating and the
    // drive id of the drive that houses the item.
    let item_id = value.id().as_ref().unwrap();

    // Create a new drive::value::Value that will be used for the
    // updated items.
    let mut updated_value: DriveItem = DriveItem::default();

    // Update the name of the file (or whatever you want to update).
    // Only include the fields that you want updated.
    // Fields that are not included will not be changed.
    updated_value.set_name(Some(DRIVE_FILE_NEW_NAME.into()));

    // Make the request to the API. This returns the item
    // with the updated values.
    let mut req = drive.v1().me().update(item_id.as_str(), &updated_value);

    let updated: DriveItem = req.send().unwrap();

    println!("{:#?}", updated);
}

// Use the old and new drive item.
fn update_by_drive_item() {
    // Get the latest metadata for the root drive folder items.
    let drive = Drive::new("ACCESS_TOKEN");
    let mut collection = get_drive_recent();

    // Get the value you want to update. The drive::value::Value struct
    // stores metadata about a drive item such as a folder or file.
    let current_value: DriveItem = collection.find_by_name(DRIVE_FILE).unwrap();

    // Create a new drive::value::Value that will be used for the
    // updated items.
    let mut updated_value: DriveItem = DriveItem::default();

    // Update the name of the file (or whatever you want to update).
    // Only include the fields that you want updated.
    // Fields that are not included will not be changed.
    updated_value.set_name(Some(DRIVE_FILE_NEW_NAME.into()));

    let mut req = drive
        .v1()
        .me()
        .update_drive_item(&current_value, &updated_value)
        .unwrap();

    let updated: DriveItem = req.send().unwrap();

    println!("{:#?}", updated);
}
