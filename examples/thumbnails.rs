use rust_onedrive::drive::drive_item::thumbnail::ThumbnailSet;
use rust_onedrive::oauth::OAuth;
use rust_onedrive::prelude::*;
use std::convert::TryFrom;

static DRIVE_FILE: &str = "YOUR_DRIVE_FILE_NAME";

pub fn main() {
    // Replace the default DriveItem with your own.
    let mut drive_item: Collection<DriveItem> = Collection::default();
    get_thumbnails(&mut drive_item);
}

pub fn get_drive() -> Drive {
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json").unwrap();
    let drive: Drive = Drive::try_from(oauth).unwrap();
    drive
}

pub fn get_thumbnails(drive_item: &mut Collection<DriveItem>) {
    let drive = get_drive();
    let drive_item: DriveItem = drive_item.find_by_name(DRIVE_FILE).unwrap();
    let item_id = drive_item.id().as_ref().unwrap();
    let collection: Collection<ThumbnailSet> =
        drive.v1().me().thumbnails(item_id.as_str()).send().unwrap();
    println!("{:#?}", collection.value());
}
