use graph_rs_sdk::components::http::FileConfig;
use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

// Put the path to your file and the file name itself that
// you want to upload to one drive.
static LOCAL_FILE_PATH: &str = "/path/to/file/file.txt";

// Parent folder id of where to store this file.
static DRIVE_PARENT_ID: &str = "PARENT_ID";

// OR by path

static DRIVE_UPLOAD_PATH: &str = ":/Documents:";

// If you are using a sites, groups, drives, or users
// path.
static RESOURCE_ID: &str = "RESOURCE_ID";

// Use upload_items_content() to upload a new file ore update
// an existing file.

// Uploading a file using the drive id and parent id.
async fn upload_file() {
    let graph = Graph::new(ACCESS_TOKEN);
    let response = graph
        .me()
        .drive()
        .item(DRIVE_PARENT_ID)
        .update_items_content(&FileConfig::new(LOCAL_FILE_PATH))
        .send()
        .await
        .unwrap();

    println!("{response:#?}");

    let drive_item: serde_json::Value = response.json().await.unwrap();
    println!("{drive_item:#?}");
}

// Upload a file using a ParentReference.
// This example uses the Documents folder of a users OneDrive.
async fn drive_upload() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .drive(RESOURCE_ID)
        .item_by_path(DRIVE_UPLOAD_PATH)
        .update_items_content(&FileConfig::new(LOCAL_FILE_PATH))
        .send()
        .await
        .unwrap();

    println!("{response:#?}");

    let drive_item: serde_json::Value = response.json().await.unwrap();
    println!("{drive_item:#?}");
}

// Upload a file using a ParentReference.
// This example uses the Documents folder of a users OneDrive.
async fn user_upload() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .user(RESOURCE_ID)
        .drive()
        .item_by_path(DRIVE_UPLOAD_PATH)
        .update_items_content(&FileConfig::new(LOCAL_FILE_PATH))
        .send()
        .await
        .unwrap();

    println!("{response:#?}");

    let drive_item: serde_json::Value = response.json().await.unwrap();
    println!("{drive_item:#?}");
}

// Upload a file using a ParentReference.
// This example uses the Documents folder of a users OneDrive.
async fn sites_upload() {
    // Get the latest metadata for the root drive folder items.
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .site(RESOURCE_ID)
        .drive()
        .item_by_path(DRIVE_UPLOAD_PATH)
        .update_items_content(&FileConfig::new(LOCAL_FILE_PATH))
        .send()
        .await
        .unwrap();

    println!("{response:#?}");

    let drive_item: serde_json::Value = response.json().await.unwrap();
    println!("{drive_item:#?}");
}
