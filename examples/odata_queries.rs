use graph_oauth::oauth::OAuth;
#[allow(unused_imports)]
use rust_onedrive::drive::driveitem::DriveItem;
use rust_onedrive::drive::query_string::QueryString;
use rust_onedrive::drive::Drive;
use rust_onedrive::drive::DriveEndPoint;
use transform_request::prelude::*;

#[allow(dead_code)]
fn expand_example() {
    // See rocket_example.rs, native_client.rs, or web_client.rs for getting
    // access_token/refresh_tokens
    let oauth: OAuth = OAuth::from_file("./examples/examples/web_oauth.json").unwrap();
    let mut drive = Drive::from(oauth);
    let drive_item_result = drive.expand(DriveEndPoint::DriveRoot, "children");
    println!("{:#?}", &drive_item_result); // -> Result<DriveItem, RequestError>
}

#[allow(dead_code)]
fn search_example() {
    // See rocket_example.rs, native_client.rs, or web_client.rs for getting
    // access_token/refresh_tokens
    let oauth: OAuth = OAuth::from_file("./examples/examples/web_oauth.json").unwrap();
    let mut drive = Drive::from(oauth);
    let drive_item_result = drive.search(DriveEndPoint::DriveRootMe, "Documents");
    println!("{:#?}", &drive_item_result); // -> Result<DriveItem, RequestError>
}

fn main() {
    expand_example();
    search_example();
}
