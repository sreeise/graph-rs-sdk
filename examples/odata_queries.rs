use from_to_file::*;
#[allow(unused_imports)]
use rust_onedrive::drive::driveitemcollection::DriveItemCollection;
use rust_onedrive::drive::query_string::QueryString;
use rust_onedrive::drive::Drive;
use rust_onedrive::drive::DriveEndPoint;
use rust_onedrive::oauth::OAuth;
use std::convert::TryFrom;

// Choose what to return in a response using query parameters. Query parameters
// are a work in progress and may not work exactly as expected.

// Note: A serde_json::Value may be returned due to the difference in returned items
// when using query parameters. Additionally, not all drive end points will
// can be used and may return an error.

// For more info on query parameters see: https://docs.microsoft.com/en-us/onedrive/developer/rest-api/concepts/optional-query-parameters?view=odsp-graph-online#expanding-collections

// See rocket_example.rs, native_client.rs, or web_client.rs for getting
// access_token/refresh_tokens

#[allow(dead_code)]
fn expand_example() {
    let oauth: OAuth = OAuth::from_json_file("./examples/examples/web_oauth.json").unwrap();
    let mut drive: Drive = Drive::try_from(oauth).unwrap();
    let drive_item_result = drive.expand(DriveEndPoint::DriveRootChild, "children");
    println!("{:#?}", &drive_item_result); // -> Result<serde_json::Value, GraphFailure>
}

fn automatic_expand_children() {
    let oauth: OAuth = OAuth::from_json_file("./examples/examples/web_oauth.json").unwrap();
    let mut drive: Drive = Drive::try_from(oauth).unwrap();
    let drive_item_result = drive.expand_children(DriveEndPoint::DriveRootChild);
    println!("{:#?}", &drive_item_result); // -> Result<ExpandChildren, GraphFailure>
}

#[allow(dead_code)]
fn search_example() {
    let oauth: OAuth = OAuth::from_json_file("./examples/examples/web_oauth.json").unwrap();
    let mut drive: Drive = Drive::try_from(oauth).unwrap();
    let drive_item_result = drive.search(DriveEndPoint::DriveRootChild, "Documents");
    println!("{:#?}", &drive_item_result); // -> Result<DriveItem, GraphFailure>
}

fn main() {
    expand_example();
    automatic_expand_children();
    search_example();
}
