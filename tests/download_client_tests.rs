use graph_rs_types::entitytypes::DriveItem;
use rust_onedrive::drive::GRAPH_URL;
use rust_onedrive::prelude::*;
use std::ffi::OsString;

#[test]
fn set_file_name_by_drive_item() {
    let client = Graph::from("");
    let mut drive_item = DriveItem::default();
    drive_item.download_url = Some(GRAPH_URL.to_string());
    drive_item.name = Some("resume.docx".to_string());

    let download_client = client
        .beta()
        .me()
        .drive()
        .download("./examples/example_files")
        .by_drive_item(&drive_item)
        .unwrap();

    assert_eq!(
        Some(&OsString::from("resume.docx")),
        download_client.file_name()
    );
}
