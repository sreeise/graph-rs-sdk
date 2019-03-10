use graph_oauth::oauth::OAuth;
use jsonfile::JsonFile;
#[allow(unused_imports)]
use rust_onedrive::drive::driveitem::DriveItem;
use rust_onedrive::drive::query_string::QueryString;
use rust_onedrive::drive::Drive;
use rust_onedrive::drive::DriveEndPoint;

#[allow(dead_code)]
fn expand_example() {
    // See native_client.rs and web_client.rs for getting access_token/refresh_tokens and
    // using serde_json to save OAuth to a file.
    let oauth: OAuth = JsonFile::from_file("examples/oauth.json").unwrap();
    let mut drive = Drive::from(oauth);
    let vec = vec!["name", "size"];
    let drive_item_result = drive.expand(DriveEndPoint::DriveRoot, "children", &vec);
    println!("{:#?}", &drive_item_result); // -> Result<DriveItem, RequestError>
}

#[allow(dead_code)]
fn search_example() {
    // See native_client.rs and web_client.rs for getting access_token/refresh_tokens and
    // using serde_json to save OAuth to a file.
    let oauth: OAuth = JsonFile::from_file("examples/oauth.json").unwrap();
    let mut drive = Drive::from(oauth);
    let drive_item_result = drive.search(DriveEndPoint::DriveRootMe, "Documents");
    println!("{:#?}", &drive_item_result); // -> Result<DriveItem, RequestError>
}

fn main() {
    expand_example();
    search_example();
}
