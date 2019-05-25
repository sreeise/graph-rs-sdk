use rust_onedrive::drive::driveitem::DriveItem;
use rust_onedrive::drive::value::Value;
use rust_onedrive::from_to::*;

#[test]
fn drive_item_value_by_id() {
    let mut drive_item: DriveItem =
        DriveItem::from_json_file("./test_files/item_test/drive_recent.json").unwrap();
    let value: Value = drive_item
        .find_by_id("01BYE5RZ2ZF3LD4S4EHZFZUYWNC5TGWFAR")
        .unwrap();
    assert_eq!("Temperatures.xlsx", value.name().unwrap());
}

#[test]
fn drive_item_value_by_name() {
    let mut drive_item: DriveItem =
        DriveItem::from_json_file("./test_files/item_test/drive_recent.json").unwrap();
    let value: Value = drive_item.find_by_name("Temperatures.xlsx").unwrap();
    assert_eq!("01BYE5RZ2ZF3LD4S4EHZFZUYWNC5TGWFAR", value.id().unwrap());
}

#[test]
fn drive_item_file_names() {
    let mut drive_item: DriveItem =
        DriveItem::from_json_file("./test_files/item_test/drive_recent.json").unwrap();
    let file_names: Vec<Option<String>> = drive_item.file_names().unwrap();
    assert!(file_names.contains(&Some("Temperatures.xlsx".into())));
    assert!(file_names.contains(&Some("Business Card.pdf".into())));
}
