use rust_onedrive::drive::drive_item::collection::Collection;
use rust_onedrive::drive::drive_item::driveitem::DriveItem;
use rust_onedrive::drive::endpoint::EP;
use rust_onedrive::drive::Drive;
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
    let drive_item: DriveItem = drive.v1().drive_root().send().unwrap();
    println!("{:#?}", drive_item);
}

fn drive_root_children(drive: &mut Drive) {
    let drive_item: Collection<DriveItem> = drive.v1().drive_root_child().send().unwrap();
    println!("{:#?}", drive_item);
}

fn special_docs(drive: &mut Drive) {
    let drive_item: Collection<DriveItem> = drive.v1().special_documents().send().unwrap();
    println!("{:#?}", drive_item);
}

fn special_docs_child(drive: &mut Drive) {
    let drive_item: Collection<DriveItem> = drive.v1().special_documents_child().send().unwrap();
    println!("{:#?}", drive_item);
}

// Specify a special folder name.
fn special_folder_selection(drive: &mut Drive) {
    let drive_item: Collection<DriveItem> = drive.v1().special_folder("documents").send().unwrap();
    drive_item.into_iter().for_each(|value| {
        println!("{:#?}", value);
    });
}
