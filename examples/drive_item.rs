use rust_onedrive::drive::driveitem::DriveItem;
use rust_onedrive::drive::Drive;
use rust_onedrive::drive::EP;
use rust_onedrive::from_to::*;
use rust_onedrive::oauth::OAuth;
use std::convert::TryFrom;

fn main() {
    let oauth = OAuth::from_json_file("./examples/example_files/web_oauth.json").unwrap();
    let mut drive = Drive::try_from(oauth).unwrap();

    // You can pick a function below to query common OneDrive resources.
    // For more common OneDrive API queries see the EP trait.

    // This will run all the API requests below.
    drive_root(&mut drive);
    drive_root_children(&mut drive);
    special_docs(&mut drive);
    special_docs_child(&mut drive);
    special_folder_selection(&mut drive);
}

fn drive_root(drive: &mut Drive) {
    let drive_item: DriveItem = drive.drive_root().unwrap();
    println!("{:#?}", drive_item);
}

fn drive_root_children(drive: &mut Drive) {
    let drive_item: DriveItem = drive.drive_root_child().unwrap();
    println!("{:#?}", drive_item);
}

fn special_docs(drive: &mut Drive) {
    let drive_item: DriveItem = drive.special_documents().unwrap();
    println!("{:#?}", drive_item);
}

fn special_docs_child(drive: &mut Drive) {
    let drive_item: DriveItem = drive.special_documents_child().unwrap();
    println!("{:#?}", drive_item);
}

// Specify a special folder name.
fn special_folder_selection(drive: &mut Drive) {
    let drive_item: DriveItem = drive.special_folder("documents").unwrap();
    drive_item.into_iter().for_each(|value| {
        println!("{:#?}", value);
    });
}
