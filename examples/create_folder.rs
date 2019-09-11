use from_as::*;
use graph_rs::oauth::OAuth;
use graph_rs::prelude::*;
use graph_rs_types::entitytypes::DriveItem;
use std::convert::TryFrom;

static FOLDER_NAME: &str = "NEW_FOLDER_NAME";
static PARENT_ID: &str = "PARENT_ID";

fn main() {
    // For more info on creating a folder see:
    // https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_post_children?view=odsp-graph-online
    create_new_folder();
}

fn create_new_folder() {
    let oauth: OAuth = OAuth::from_file("./examples/example_files/web_oauth.json").unwrap();
    let drive = Graph::try_from(&oauth).unwrap();

    let drive_item: DriveItem = drive
        .v1()
        .me()
        .drive()
        .create_folder(FOLDER_NAME, None)
        .by_id(PARENT_ID)
        .send()
        .unwrap();
    println!("{:#?}", drive_item);
}
