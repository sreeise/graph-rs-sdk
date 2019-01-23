#[allow(unused_imports)]
use rust_onedrive::drive::driveitem::{DriveItem, Value};
use rust_onedrive::drive::endpoint::DriveEndPoint;
use rust_onedrive::drive::QueryString;
use rust_onedrive::flow::v1::AuthFlow;
use rust_onedrive::process::jsonio::JsonFile;

fn expand_example() {
    // See native_client.rs and web_client.rs for getting access_token/refresh_tokens and
    // using serde_json to save AuthFlow to a file.
    let mut auth_flow: AuthFlow = JsonFile::from_file("examples/auth_flow.json").unwrap();
    let mut drive = auth_flow.into_drive().unwrap();
    let vec = vec!["name", "size"];
    let base_item = drive.expand(DriveEndPoint::DriveRoot, "children", &vec);
    if base_item.is_some() {
        println!("{:#?}", &base_item);
    } else {
        println!("{:#?}", &base_item.error);
    }
}

fn search_example() {
    // See native_client.rs and web_client.rs for getting access_token/refresh_tokens and
    // using serde_json to save AuthFlow to a file.
    let mut auth_flow: AuthFlow = JsonFile::from_file("examples/auth_flow.json").unwrap();
    let mut drive = auth_flow.into_drive().unwrap();
    let base_item = drive.search(DriveEndPoint::DriveRootMe, "Documents");
    if base_item.is_some() {
        println!("{:#?}", &base_item);
    } else {
        println!("{:#?}", &base_item.error);
    }
}

fn main() {
    expand_example();
    search_example();
}
