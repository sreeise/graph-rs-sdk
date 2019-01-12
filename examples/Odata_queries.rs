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
    let value: Value = drive.expand(DriveEndPoint::DriveRootMe, "children", &vec);
    println!("{:#?}", &value);
}

fn search_example() {
    // See native_client.rs and web_client.rs for getting access_token/refresh_tokens and
    // using serde_json to save AuthFlow to a file.
    let mut auth_flow: AuthFlow = JsonFile::from_file("examples/auth_flow.json").unwrap();
    let mut drive = auth_flow.into_drive().unwrap();
    let drive_item: DriveItem = drive.search(DriveEndPoint::DriveRootMe, "Documents");
    println!("{:#?}", &drive_item);
}

fn main() {
    expand_example();
    search_example();
}
