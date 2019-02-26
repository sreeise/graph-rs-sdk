use rust_onedrive::drive::base::driveinfo::DriveInfo;
use rust_onedrive::drive::base::driveitem::DriveItem;
use rust_onedrive::drive::base::filesysteminfo::FileSystemInfo;
use rust_onedrive::drive::base::parentreference::ParentReference;
use rust_onedrive::drive::base::quota::Quota;
use rust_onedrive::drive::base::value::Value;
use rust_onedrive::process::jsonio::JsonFile;

#[test]
fn drive() {
    let drive_info: DriveInfo = JsonFile::from_file("test_files/drive_ep/drive.json").unwrap();
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
    let drive_info: DriveInfo = JsonFile::from_file("test_files/drive_ep/drive_me.json").unwrap();
    let quota = Quota::new(
        0,
        1099292078173,
        "normal".to_string(),
        1099511627776,
        106330475,
    );
    assert_eq!(drive_info.quota().unwrap(), quota);
}

#[test]
fn drive_root() {
    let drive_info: Value = JsonFile::from_file("test_files/drive_ep/drive_root.json").unwrap();
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
    let drive_info: DriveItem =
        JsonFile::from_file("test_files/drive_ep/drive_root_child.json").unwrap();
    let drive_value = drive_info.value().unwrap();
    assert_eq!(drive_value[1].folder().unwrap().child_count(), Some(12));
}

#[test]
fn drive_root_changes() {
    let drive_info: DriveItem =
        JsonFile::from_file("test_files/drive_ep/drive_root_changes.json").unwrap();
    let drive_value = drive_info.value().unwrap();
    assert_eq!(drive_value.len(), 3);
}

#[test]
fn shared_with_me() {
    let drive_info: DriveItem =
        JsonFile::from_file("test_files/drive_ep/shared_with_me.json").unwrap();

    let drive_value = drive_info.value().unwrap();
    assert_eq!(drive_value.len(), 2);

    let parent_ref = drive_value[0].remote_item().unwrap().parent_reference();
    let parent_ref2 = ParentReference::new(
        Some(String::from(
            "b!bUbEy-Q7O0yQlf5IKmlRJE8XkS_I8MdFlUCq1BlcjgmhRfAj3-Z8RY2VpuvV_tpd",
        )),
        Some(String::from("business")),
        Some(String::from("01OMQY4ZN6Y2GOVW7725BZO354PWSELRRZ")),
        None,
    );
    assert_eq!(parent_ref, Some(parent_ref2));

    let file_system_info = drive_value[0].file_system_info();
    let file_system_info2 = FileSystemInfo::new(
        Some(String::from("2017-09-02T03:05:02Z")),
        Some(String::from("2017-09-02T03:05:02Z")),
    );
    assert_eq!(file_system_info, Some(file_system_info2));
    let value2 = drive_info.value_idx(1);
    assert_eq!(
        value2
            .remote_item()
            .unwrap()
            .shared()
            .unwrap()
            .shared_by()
            .unwrap()
            .user()
            .unwrap()
            .email(),
        Some("MiriamG@M365x214355.onmicrosoft.com".to_string())
    );
}

#[test]
fn drive_item_value_idx_method() {
    let drive_info: DriveItem =
        JsonFile::from_file("test_files/drive_ep/drive_root_changes.json").unwrap();
    let drive_value2 = drive_info.value_idx(2);
    assert_eq!(
        drive_value2.data_type().unwrap().as_str(),
        "#microsoft.graph.driveItem"
    );
}
