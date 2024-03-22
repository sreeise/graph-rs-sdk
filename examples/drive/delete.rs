use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

// Delete items in OneDrive. This will move deleted items to the recycle bin.
// It is recommended to create a new file that can be used for demonstration purposes here.
// Deleting an item can be done in 2 different ways shown in the methods below.

// Delete a drive item by id.
pub async fn delete_by_id(item_id: &str) {
    let client = GraphClient::new(ACCESS_TOKEN);

    // Send the request.
    let response = client
        .me()
        .drive()
        .item(item_id)
        .delete_items()
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
    println!("\nItem deleted Status: {}", response.status());
}

// Deleting an item by path.
pub async fn delete_by_path(path: &str) {
    let client = GraphClient::new(ACCESS_TOKEN);

    // Send the request.
    let response = client
        .me()
        .drive()
        .item_by_path(path)
        .delete_items()
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
    println!("\nItem deleted Status: {}", response.status());
}
