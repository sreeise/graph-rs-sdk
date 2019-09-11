use from_as::*;
use graph_rs::types::collection::Collection;
use graph_rs_types::entitytypes::DriveItem;

#[test]
fn drive_item_value_by_id() {
    let mut drive_item: Collection<DriveItem> =
        Collection::from_file("./test_files/item_test/drive_recent.json").unwrap();
    let value: DriveItem = drive_item
        .find_by_id("01BYE5RZ2ZF3LD4S4EHZFZUYWNC5TGWFAR")
        .unwrap();
    assert_eq!("Temperatures.xlsx", value.name.as_ref().unwrap());
}

#[test]
fn drive_item_value_by_name() {
    let mut drive_item: Collection<DriveItem> =
        Collection::from_file("./test_files/item_test/drive_recent.json").unwrap();
    let value: DriveItem = drive_item.find_by_name("Temperatures.xlsx").unwrap();
    assert_eq!(
        "01BYE5RZ2ZF3LD4S4EHZFZUYWNC5TGWFAR",
        value.id.as_ref().unwrap()
    );
}

#[test]
fn drive_item_file_names() {
    let mut collection: Collection<DriveItem> =
        Collection::from_file("./test_files/item_test/drive_recent.json").unwrap();
    let file_names: Vec<String> = collection.file_names().unwrap();
    assert!(file_names.contains(&"Temperatures.xlsx".to_string()));
    assert!(file_names.contains(&"Business Card.pdf".to_string()));
}
