use graph_rs::prelude::*;
use graph_rs::types::content::Content;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

// Delete items in OneDrive. This will move deleted items to the recycle bin.
// It is recommended to create a new file that can be used for demonstration purposes here.
// Deleting an item can be done in 2 different ways shown in the methods below.
fn main() {
    delete_id("DRIVE_ITEM_ID");
    delete_path(":/PATH_FROM_ROOT:");
}

// Delte a drive item by id.
fn delete_id(item_id: &str) {
    let client = Graph::new(ACCESS_TOKEN);

    // Send the request.
    let response: GraphResponse<Content> = client.v1().me().drive().delete(item_id).send().unwrap();

    println!("{:#?}", response);
    println!("\nItem was deleted: {:#?}", response.success());
}

// Deleting an item by path.
pub fn delete_path(path: &str) {
    let client = Graph::new(ACCESS_TOKEN);

    // Send the request.
    let response: GraphResponse<Content> = client.v1().me().drive().delete(path).send().unwrap();

    println!("{:#?}", response);
    println!("\nItem was deleted: {:#?}", response.success());
}
