use rust_onedrive::oauth::OAuth;
use rust_onedrive::prelude::*;
use std::convert::TryFrom;

// Delete items in OneDrive. This will move deleted items to the recycle bin.
// It is recommended to create a new file that can be used for demonstration purposes here.
// Deleting an item can be done in 2 different ways shown in the methods below.
fn main() {
    delete_drive_item("YOUR DRIVE ITEM FILE NAME SUCH AS: my_item.txt");
    delete_with_item_id("DRIVE_ITEM_ID");
}

// Deleting an item by value is the easiest way to delete an item because
// it only requires a drive::value::Value.
// The method used here is: Drive::delete_by_value().
pub fn delete_drive_item(item_name: &str) {
    // Create a new Drive instance.
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json").unwrap();
    let drive: Drive = Drive::try_from(oauth).unwrap();

    // Call the API. drive_root_child is the files in the users main documents folder.
    let mut collection: DriveItemCollection = drive.v1().drive_root_child().send().unwrap();
    // Save the metadata of the files.
    collection
        .to_json_file("./examples/example_files/drive_root_child.json")
        .unwrap();

    // Find the file based on it's name.
    let drive_item = collection.find_by_name(item_name).unwrap();

    // Create the request to delete the drive item.
    let mut req = drive.v1().me().delete_drive_item(&drive_item);

    // Send the request.
    let mut response: ItemResponse = req.send().unwrap();
    println!("{:#?}", response);
    println!("\nItem was deleted: {:#?}", response.success());
}

fn delete_with_item_id(item_id: &str) {
    // Create a new Drive instance.
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json").unwrap();
    let drive: Drive = Drive::try_from(oauth).unwrap();

    // Create the request to delete the drive item.
    let mut req = drive.v1().me().delete(item_id);

    // Send the request.
    let mut response: ItemResponse = req.send().unwrap();
    println!("{:#?}", response);
    println!("\nItem was deleted: {:#?}", response.success());
}
