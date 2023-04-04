use graph_rs_sdk::*;

// This example shows choosing a file in the root of a drive (normally where
// folders such as Documents are), and changing the name.

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
static ITEM_ID: &str = "ITEM_ID";
static DRIVE_FILE_NEW_NAME: &str = "NEW_DRIVE_FILE_NAME.txt";

// Use the old and new drive item.
async fn update() {
    // Update the name of the file (or whatever you want to update).
    // Only include the fields that you want updated.
    // Fields that are not included will not be changed.
    let value = serde_json::json!({ "name": DRIVE_FILE_NEW_NAME });

    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .drive()
        .item(ITEM_ID)
        .update_items(&value)
        .send()
        .await
        .unwrap();

    println!("{response:#?}");

    let drive_item: serde_json::Value = response.json().await.unwrap();
    println!("{drive_item:#?}");
}
