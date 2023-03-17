use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

// Delete items in OneDrive. This will move deleted items to the recycle bin.
// It is recommended to create a new file that can be used for demonstration purposes here.
// Deleting an item can be done in 2 different ways shown in the methods below.

// Delete a drive item by id.
pub fn delete_by_id(item_id: &str) {
    let client = Graph::new(ACCESS_TOKEN);

    // Send the request.
    let response = client
        .me()
        .default_drive()
        .item(item_id)
        .delete_items()
        .send()
        .unwrap();

    println!("{:#?}", response);
    println!("\nItem was deleted. Status: {}", response.status());
}

/*
// Deleting an item by path.
pub fn delete_by_path(path: &str) {
    let client = Graph::new(ACCESS_TOKEN);

    // Send the request.
    let response = client.v1().me().drive().delete_items(path).send().unwrap();

    println!("{:#?}", response);
    println!("\nItem was deleted. Status: {}", response.status());
}

 */
