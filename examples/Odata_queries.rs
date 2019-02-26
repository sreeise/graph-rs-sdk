use graph_oauth::oauth::OAuth;
#[allow(unused_imports)]
use rust_onedrive::drive::base::driveitem::DriveItem;
use rust_onedrive::drive::endpoint::DriveEndPoint;
use rust_onedrive::drive::query_string::QueryString;
use rust_onedrive::drive::Drive;
use rust_onedrive::process::jsonio::JsonFile;

#[allow(dead_code)]
fn expand_example() {
    // See native_client.rs and web_client.rs for getting access_token/refresh_tokens and
    // using serde_json to save OAuth to a file.
    let oauth: OAuth = JsonFile::from_file("examples/oauth.json").unwrap();
    let mut drive = Drive::from(oauth);
    let vec = vec!["name", "size"];
    let base_item = drive.expand(DriveEndPoint::DriveRoot, "children", &vec);
    if base_item.is_some() {
        println!("{:#?}", &base_item);
    } else {
        println!("{:#?}", &base_item.error);
    }
}

#[allow(dead_code)]
fn search_example() {
    // See native_client.rs and web_client.rs for getting access_token/refresh_tokens and
    // using serde_json to save OAuth to a file.
    let oauth: OAuth = JsonFile::from_file("examples/oauth.json").unwrap();
    let mut drive = Drive::from(oauth);
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
