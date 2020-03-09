use graph_rs::prelude::*;

// This example shows choosing a file in the root of a drive (normally where
// folders such as Documents are), and changing the name.

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
static ITEM_ID: &str = "ITEM_ID";
static DRIVE_FILE_NEW_NAME: &str = "NEW_DRIVE_FILE_NAME.txt";

fn main() {
    update();
}

// Use the old and new drive item.
fn update() {
    // Get the latest metadata for the root drive folder items.
    let graph = Graph::new(ACCESS_TOKEN);

    // Update the name of the file (or whatever you want to update).
    // Only include the fields that you want updated.
    // Fields that are not included will not be changed.
    let value = serde_json::json!({ "name": DRIVE_FILE_NEW_NAME });

    let updated: GraphResponse<serde_json::Value> = graph
        .v1()
        .me()
        .drive()
        .update(ITEM_ID, &value)
        .send()
        .unwrap();

    println!("{:#?}", updated);
}
