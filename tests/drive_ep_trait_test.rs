use from_to_file::*;
use rust_onedrive::drive::driveinfo::DriveInfo;
use rust_onedrive::drive::driveitem::DriveItem;
use rust_onedrive::drive::filesysteminfo::FileSystemInfo;
use rust_onedrive::drive::itemreference::ItemReference;
use rust_onedrive::drive::quota::Quota;
use rust_onedrive::prelude::Collection;

#[test]
fn drive() {
    let drive_info: DriveInfo =
        DriveInfo::from_json_file("test_files/drive_ep/drive.json").unwrap();
    assert_eq!(
        drive_info.data_context().unwrap().as_str(),
        "https://graph.microsoft.com/v1.0/$metadata#drives/$entity"
    );
    assert_eq!(
        drive_info.created_date_time().unwrap().as_str(),
        "2017-07-27T02:32:33Z"
    );
}

#[test]
fn drive_me() {
    let drive_info: DriveInfo =
        DriveInfo::from_json_file("test_files/drive_ep/drive_me.json").unwrap();
    let quota = Quota::new(
        0,
        1_099_292_078_173,
        "normal".to_string(),
        1_099_511_627_776,
        106_330_475,
    );
    assert_eq!(drive_info.quota().unwrap(), quota);
}

#[test]
fn drive_root() {
    let drive_info: DriveItem =
        DriveItem::from_json_file("test_files/drive_ep/drive_root.json").unwrap();
    assert_eq!(drive_info.name(), Some(String::from("root")));
    assert_eq!(
        drive_info.web_url(),
        Some(String::from(
            "https://m365x214355.sharepoint.com/Shared%20Documents"
        ))
    );
}

#[test]
fn drive_root_child() {
    let drive_info: Collection<DriveItem> =
        Collection::from_json_file("test_files/drive_ep/drive_root_child.json").unwrap();
    let drive_value = drive_info.value().as_ref().unwrap();
    assert_eq!(drive_value[1].folder().unwrap().child_count(), Some(12));
}

#[test]
fn drive_root_changes() {
    let drive_info: Collection<DriveItem> =
        Collection::from_json_file("test_files/drive_ep/drive_root_changes.json").unwrap();
    let drive_value = drive_info.value().as_ref().unwrap();
    assert_eq!(drive_value.len(), 3);
}

#[test]
fn shared_with_me() {
    let drive_info: Collection<DriveItem> =
        Collection::from_json_file("test_files/drive_ep/shared_with_me.json").unwrap();

    let drive_value = drive_info.value().as_ref().unwrap();
    assert_eq!(drive_value.len(), 2);

    let parent_ref = drive_value[0].remote_item().unwrap().parent_reference();
    let mut pr2 = ItemReference::default();
    pr2.set_drive_id(Some(
        "b!bUbEy-Q7O0yQlf5IKmlRJE8XkS_I8MdFlUCq1BlcjgmhRfAj3-Z8RY2VpuvV_tpd".into(),
    ));
    pr2.set_drive_type(Some("business".into()));
    pr2.set_id(Some("01OMQY4ZN6Y2GOVW7725BZO354PWSELRRZ".into()));
    assert_eq!(parent_ref, Some(pr2));

    let file_system_info = drive_value[0].file_system_info();
    let file_system_info2 = FileSystemInfo::new(
        Some(String::from("2017-09-02T03:05:02Z")),
        Some(String::from("2017-09-02T03:05:02Z")),
    );
    assert_eq!(file_system_info, Some(file_system_info2));
    let value2 = drive_info.index(1).unwrap();
    assert_eq!(
        value2
            .remote_item()
            .unwrap()
            .shared()
            .unwrap()
            .shared_by()
            .clone()
            .unwrap()
            .user()
            .clone()
            .unwrap()
            .email(),
        &Some("MiriamG@M365x214355.onmicrosoft.com".to_string())
    );
}

#[test]
fn drive_item_index_method() {
    let drive_info: Collection<DriveItem> =
        Collection::from_json_file("test_files/drive_ep/drive_root_changes.json").unwrap();
    let drive_value2 = drive_info.index(2).unwrap();
    assert_eq!(
        drive_value2.data_type().unwrap().as_str(),
        "#microsoft.graph.driveItem"
    );
}
