use rust_onedrive::drive::event::{ConflictBehavior, NewFolder};
use rust_onedrive::from_to::*;
use rust_onedrive::oauth::OAuth;
use rust_onedrive::prelude::*;
use std::convert::TryFrom;

static FOLDER_NAME: &str = "NEW_FOLDER_NAME";

fn main() {
    // For more info on creating a folder see:
    // https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_post_children?view=odsp-graph-online

    // A drive id, parent item id, and DriveResource is used to create a new
    // folder through id based addressing.
    // If you want to use the same location as a drive::value::Value,
    // the drive id and parent item id can be found in the ParentReference within
    // each drive::value::Value. The ParentReference contains a drive_id and an id
    // field which is the the parent id that can be used to create a new folder.
    create_new_folder("YOUR_DRIVE_ID", "YOUR_PARENT_ITEM_ID");

    // You can also set the location where the folder is created
    // by path.
    create_folder_by_given_path();
}

fn create_new_folder(drive_id: &str, parent_id: &str) {
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json").unwrap();
    let mut drive: Drive = Drive::try_from(oauth).unwrap();

    // A NewFolder struct specifies the new folders name and the conflict behavior
    // to use in case of a naming conflict. Can be one of rename, fail, or replace.
    let new_folder: NewFolder = NewFolder::new(FOLDER_NAME, ConflictBehavior::Rename);

    // Create the folder by referencing the drive id and parent id and the resource.
    // Returns a drive::value::Value which is the new drive item metadata.
    let value = drive
        .create_folder(new_folder, drive_id, parent_id, DriveResource::Drives)
        .unwrap();
    println!("{:#?}", value);
}

fn create_folder_by_given_path() {
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json").unwrap();
    let mut drive: Drive = Drive::try_from(oauth).unwrap();

    // A NewFolder struct specifies the new folders name and the conflict behavior
    // to use in case of a naming conflict. Can be one of rename, fail, or replace.
    let new_folder: NewFolder = NewFolder::new(FOLDER_NAME, ConflictBehavior::Rename);

    // Creates a new PathBuilder by passing a reference of a Drive. The PathBuilder
    // will use the drive version URL for the Drive as the host URL for the path.
    let mut path_builder = PathBuilder::from(&drive);

    // Use the main root drive location to create the folder in.
    path_builder.drive_endpoint(DriveEndPoint::DriveRootChild);

    // Create the folder by path.
    // Returns a drive::value::Value which is the new drive item metadata.
    let value = drive
        .create_folder_by_path(new_folder, &mut path_builder)
        .unwrap();
    println!("{:#?}", value);
}
