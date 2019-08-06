use rust_onedrive::drive::event::{ConflictBehavior, NewFolder};
use rust_onedrive::from_to::*;
use rust_onedrive::oauth::OAuth;
use rust_onedrive::prelude::*;
use std::convert::TryFrom;

static FOLDER_NAME: &str = "NEW_FOLDER_NAME";
static PARENT_ID: &str = "PARENT_ID";

fn main() {
    // For more info on creating a folder see:
    // https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_post_children?view=odsp-graph-online
    create_new_folder();
}

fn create_new_folder() {
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json").unwrap();
    let drive: Drive = Drive::try_from(oauth).unwrap();

    let new_folder: NewFolder = NewFolder::new(FOLDER_NAME, ConflictBehavior::Rename);
    let mut request = drive.v1().me().create_folder(PARENT_ID, new_folder);

    let drive_item: DriveItem = request.send().unwrap();
    println!("{:#?}", drive_item);
}
