use rust_onedrive::drive::thumbnail::ThumbnailCollection;
use rust_onedrive::oauth::OAuth;
use rust_onedrive::prelude::*;
use std::convert::TryFrom;

static DRIVE_FILE: &str = "YOUR_DRIVE_FILE_NAME";

pub fn main() {
    // Replace the default DriveItem with your own.
    let mut drive_item = DriveItemCollection::default();
    get_thumbnails(&mut drive_item);
}

pub fn get_drive() -> Drive {
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json").unwrap();
    let drive: Drive = Drive::try_from(oauth).unwrap();
    drive
}

pub fn get_thumbnails(drive_item: &mut DriveItemCollection) {
    let mut drive = get_drive();
    let value: DriveItem = drive_item.find_by_name(DRIVE_FILE).unwrap();
    let collection: ThumbnailCollection =
        drive.thumbnails_by_value(value, DriveResource::Me).unwrap();
    println!("{:#?}", collection.thumbnails());
}
