use rust_onedrive::oauth::*;
use rust_onedrive::prelude::*;
use std::convert::TryFrom;

// This example shows choosing a file in the root of a drive (normally where
// folders such as Documents are), and changing the name.
static DRIVE_FILE: &str = "DRIVE_FILE_NAME.txt";
static DRIVE_FILE_NEW_NAME: &str = "NEW_DRIVE_FILE_NAME.txt";

fn main() {
    update_item().unwrap();
    // or
    update_by_value().unwrap();
}

fn get_drive() -> ItemResult<Drive> {
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json")?;
    let drive: Drive = Drive::try_from(oauth)?;
    Ok(drive)
}

fn update_item() -> ItemResult<()> {
    // Get the latest metadata for the root drive folder items.
    let mut drive = get_drive()?;
    let mut drive_item = drive.drive_root_child()?;

    // Get the value you want to update. The drive::value::Value struct
    // stores metadata about a drive item such as a folder or file.
    let value: Value = drive_item.find_by_name(DRIVE_FILE)?;

    // Get the item id of the item that needs updating and the
    // drive id of the drive that houses the item.
    let item_id = value.id().unwrap();
    let drive_id = value.parent_reference().unwrap().drive_id().unwrap();

    // Create a new drive::value::Value that will be used for the
    // updated items.
    let mut updated_value: Value = Value::default();

    // Update the name of the file (or whatever you want to update).
    // Only include the fields that you want updated.
    // Fields that are not included will not be changed.
    updated_value.set_name(Some(DRIVE_FILE_NEW_NAME.into()));

    // Make the request to the API. This returns the item
    // with the updated values.
    let updated: Value = drive.update(
        item_id.as_str(),
        drive_id.as_str(),
        updated_value,
        DriveResource::Me,
    )?;

    println!("{:#?}", updated);
    Ok(())
}

// Pass the old drive::value::Value and the new drive::value::Value
// to update the item.
fn update_by_value() -> ItemResult<()> {
    // Get the latest metadata for the root drive folder items.
    let mut drive = get_drive()?;
    let mut drive_item = drive.drive_root_child()?;

    // Get the value you want to update. The drive::value::Value struct
    // stores metadata about a drive item such as a folder or file.
    let current_value: Value = drive_item.find_by_name(DRIVE_FILE)?;

    // Create a new drive::value::Value that will be used for the
    // updated items.
    let mut updated_value: Value = Value::default();

    // Update the name of the file (or whatever you want to update).
    // Only include the fields that you want updated.
    // Fields that are not included will not be changed.
    updated_value.set_name(Some(DRIVE_FILE_NEW_NAME.into()));

    let updated: Value = drive.update_by_value(updated_value, current_value, DriveResource::Me)?;

    println!("{:#?}", updated);
    Ok(())
}
