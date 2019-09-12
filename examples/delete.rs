use from_as::*;
use graph_rs::oauth::OAuth;
use graph_rs::prelude::*;
use std::convert::TryFrom;

// Delete items in OneDrive. This will move deleted items to the recycle bin.
// It is recommended to create a new file that can be used for demonstration purposes here.
// Deleting an item can be done in 2 different ways shown in the methods below.
fn main() {
    delete_id("DRIVE_ITEM_ID");
    delete_path("PATH_FROM_ROOT");
}

// Delte a drive item by id.
fn delete_id(item_id: &str) {
    // Create a new Drive instance.
    let oauth: OAuth = OAuth::from_file("./examples/example_files/web_oauth.json").unwrap();
    let drive = Graph::try_from(&oauth).unwrap();

    // Send the request.
    let mut response: GraphResponse<()> = drive
        .v1()
        .me()
        .drive()
        .delete()
        .by_id(item_id)
        .send()
        .unwrap();

    println!("{:#?}", response);
    println!("\nItem was deleted: {:#?}", response.success());
}

// Deleting an item by path.
pub fn delete_path(path: &str) {
    // Create a new Drive instance.
    let oauth: OAuth = OAuth::from_file("./examples/example_files/web_oauth.json").unwrap();
    let drive = Graph::try_from(&oauth).unwrap();

    // Send the request.
    let mut response: GraphResponse<()> = drive
        .v1()
        .me()
        .drive()
        .delete()
        .by_path(path)
        .send()
        .unwrap();

    println!("{:#?}", response);
    println!("\nItem was deleted: {:#?}", response.success());
}
