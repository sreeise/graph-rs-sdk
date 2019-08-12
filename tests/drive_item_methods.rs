use rust_onedrive::drive::collection::Collection;
use rust_onedrive::drive::driveitem::DriveItem;
use rust_onedrive::from_to::*;

#[test]
fn drive_item_value_by_id() {
    let mut drive_item: Collection<DriveItem> =
        Collection::from_json_file("./test_files/item_test/drive_recent.json").unwrap();
    let value: DriveItem = drive_item
        .find_by_id("01BYE5RZ2ZF3LD4S4EHZFZUYWNC5TGWFAR")
        .unwrap();
    assert_eq!("Temperatures.xlsx", value.name().unwrap());
}

#[test]
fn drive_item_value_by_name() {
    let mut drive_item: Collection<DriveItem> =
        Collection::from_json_file("./test_files/item_test/drive_recent.json").unwrap();
    let value: DriveItem = drive_item.find_by_name("Temperatures.xlsx").unwrap();
    assert_eq!("01BYE5RZ2ZF3LD4S4EHZFZUYWNC5TGWFAR", value.id().unwrap());
}

#[test]
fn drive_item_file_names() {
    let mut drive_item: Collection<DriveItem> =
        Collection::from_json_file("./test_files/item_test/drive_recent.json").unwrap();
    let file_names: Vec<String> = drive_item.file_names().unwrap();
    assert!(file_names.contains(&"Temperatures.xlsx".to_string()));
    assert!(file_names.contains(&"Business Card.pdf".to_string()));
}
