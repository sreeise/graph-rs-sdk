use rust_onedrive::drive::driveinfo::DriveInfo;
use rust_onedrive::oauth::OAuth;
use rust_onedrive::prelude::*;
use std::convert::TryFrom;

// Delete items in OneDrive. This will move deleted items to the recycle bin.
// It is recommended to create a new file that can be used for demonstration purposes here.
// Deleting an item can be done in 2 different ways shown in the methods below.
fn main() {
    delete_item_by_value("YOUR DRIVE ITEM FILE NAME SUCH AS: my_item.txt");
    delete_by_item_id_and_drive_id("YOUR DRIVE ITEM FILE NAME SUCH AS: my_item.txt");
}

// Deleting an item by value is the easiest way to delete an item because
// it only requires a drive::value::Value.
// The method used here is: Drive::delete_by_value().
pub fn delete_item_by_value(item_name: &str) {
    // Create a new Drive instance.
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json").unwrap();
    let mut drive: Drive = Drive::try_from(oauth).unwrap();

    // Call the API. drive_root_child is the files in the users main documents folder.
    let mut drive_item: DriveItemCollection = drive.drive_root_child().unwrap();

    // Find the file based on it's name.
    let value = drive_item.find_by_name(item_name).unwrap();

    // Delete the item by value. This method will use the Values's id and the
    // drive_id stored withing the Value's parent_reference to send a request
    // to delete the item.
    let mut response: ItemResponse = drive.delete_by_value(value).unwrap();
    println!("{:#?}", response);
    println!("\nItem was deleted: {:#?}", response.success());
}

// Deleting an item by a drive id and item id requires finding the id for the drive.
// This is done here by calling the Drive::drive() method which returns a DriveInfo.
// The method used here is: Drive::delete().
pub fn delete_by_item_id_and_drive_id(item_name: &str) {
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json").unwrap();
    let mut drive: Drive = Drive::try_from(oauth).unwrap();

    // Call the API. drive_root_child is the files in the users main documents folder.
    let mut drive_item: DriveItemCollection = drive.drive_root_child().unwrap();

    // Find the file based on it's name.
    let value = drive_item.find_by_name(item_name).unwrap();

    // Get the drive id from the DriveInfo metadata.
    let drive_info: DriveInfo = drive.drive().unwrap();
    let drive_id = drive_info.id().unwrap();

    // Get the item id from the files value metadata.
    let item_id = value.id().unwrap();

    // Delete the item based on the type of drive using DriveResource.
    // This can be one of:
    //    DriveResource::Drives,
    //    DriveResource::Groups,
    //    DriveResource::Sites,
    //    DriveResource::Users,
    //    DriveResource::Me,
    //
    // The DriveResource changes the URL being used to make the request.
    // For instance, given an item id of 1234 and a drive id of 1, the URL for
    // drives and users would look like:
    //
    // DriveResource::Drives => "https://graph.microsoft.com/v1.0/drives/1/items/1234/"
    // DriveResource::Users => https://graph.microsoft.com/v1.0/users/1/drive/items/1234/
    //
    // Note: DriveResource::Me does not use the drive_id, so while this is an option
    // it may be better to use the delete_by_value method which will make sure to use
    // the correct drive id. However, this method, always uses the DriveResource::Drives
    // URL. This may change in the future.
    let mut response: ItemResponse = drive
        .delete(drive_id.as_str(), item_id.as_str(), DriveResource::Drives)
        .unwrap();
    println!("{:#?}", response);
    println!("\nItem was deleted: {:#?}", response.success());
}
