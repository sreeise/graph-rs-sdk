use graph_rs::prelude::*;
use graph_rs_types::entitytypes::DriveItem;
// This example shows choosing a file in the root of a drive (normally where
// folders such as Documents are), and changing the name.
static ITEM_ID: &str = "ITEM_ID";
static DRIVE_FILE_NEW_NAME: &str = "NEW_DRIVE_FILE_NAME.txt";

fn main() {
    update();
}

// Use the old and new drive item.
fn update() {
    // Get the latest metadata for the root drive folder items.
    let graph = Graph::new("ACCESS_TOKEN");

    // Create a new drive::value::Value that will be used for the
    // updated items.
    let mut updated_value: DriveItem = DriveItem::default();

    // Update the name of the file (or whatever you want to update).
    // Only include the fields that you want updated.
    // Fields that are not included will not be changed.
    updated_value.name = Some(DRIVE_FILE_NEW_NAME.into());

    let updated: GraphResponse<DriveItem> = graph
        .v1()
        .me()
        .drive()
        .update(&updated_value)
        .by_id(ITEM_ID)
        .send()
        .unwrap();

    println!("{:#?}", updated.value());
}
