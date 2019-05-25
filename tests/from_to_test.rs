use drive_test_tools::support::cleanup::CleanUp;
use rust_onedrive::drive::driveitem::DriveItem;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use transform_request::prelude::*;

#[test]
fn drive_item_from_to_json() {
    let drive_item = DriveItem::default();
    let file_location = "./test_files/test_from_to_drive_item.json";
    let mut clean_up = CleanUp::new(|| {
        if Path::new(file_location).exists() {
            fs::remove_file(Path::new(file_location)).unwrap();
        }
    });

    clean_up.rm_files(file_location.into());

    drive_item.to_file(file_location).unwrap();
    let path = Path::new(file_location);

    let parent = path.parent();
    assert_eq!(parent, Some(Path::new("./test_files")));

    let file_stem = path.file_stem();
    assert_eq!(file_stem, Some(OsStr::new("test_from_to_drive_item")));

    let extension = path.extension();
    assert_eq!(extension, Some(OsStr::new("json")));
}

#[test]
fn drive_item_from_to_yaml() {
    let drive_item = DriveItem::default();
    let file_location = "./test_files/test_from_to_drive_item_yaml.yaml";
    let mut clean_up = CleanUp::new(|| {
        if Path::new(file_location).exists() {
            fs::remove_file(Path::new(file_location)).unwrap();
        }
    });

    clean_up.rm_files(file_location.into());

    drive_item.to_file(file_location).unwrap();
    let path = Path::new(file_location);

    let parent = path.parent();
    assert_eq!(parent, Some(Path::new("./test_files")));

    let file_stem = path.file_stem();
    assert_eq!(file_stem, Some(OsStr::new("test_from_to_drive_item_yaml")));

    let extension = path.extension();
    assert_eq!(extension, Some(OsStr::new("yaml")));
}
