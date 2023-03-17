use graph_rs_sdk::prelude::*;
use std::collections::HashMap;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
static FOLDER_NAME: &str = "NEW_FOLDER_NAME";
static PARENT_ID: &str = "PARENT_ID";

// For more info on creating a folder see:
// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_post_children?view=odsp-graph-online

pub fn create_new_folder() {
    let client = Graph::new(ACCESS_TOKEN);
    let folder: HashMap<String, serde_json::Value> = HashMap::new();

    let drive_item: GraphResponse<serde_json::Value> = client
        .me()
        .default_drive()
        .item(PARENT_ID)
        .create_children(&serde_json::json!({
            "name": FOLDER_NAME,
            "folder": folder,
            "@microsoft.graph.conflictBehavior": "fail"
        }))
        .send()
        .unwrap();
    println!("{:#?}", drive_item);
}
