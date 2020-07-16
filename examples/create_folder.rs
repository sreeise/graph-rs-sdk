use from_as::*;
use graph_rs::oauth::OAuth;
use graph_rs::prelude::*;
use std::collections::HashMap;
use std::convert::TryFrom;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
static FOLDER_NAME: &str = "NEW_FOLDER_NAME";
static PARENT_ID: &str = "PARENT_ID";

fn main() {
    // For more info on creating a folder see:
    // https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_post_children?view=odsp-graph-online
    create_new_folder();
}

fn create_new_folder() {
    let client = Graph::new(ACCESS_TOKEN);
    let folder: HashMap<String, serde_json::Value> = HashMap::new();

    let drive_item: GraphResponse<serde_json::Value> = client
        .v1()
        .me()
        .drive()
        .create_folder(
            PARENT_ID,
            &serde_json::json!({
                "name": FOLDER_NAME,
                "folder": folder,
                "@microsoft.graph.conflictBehavior": "fail"
            }),
        )
        .send()
        .unwrap();
    println!("{:#?}", drive_item);
}
