use graph_rs_sdk::header::{HeaderValue, CONTENT_LENGTH};
use graph_rs_sdk::prelude::*;

// This example shows checking a drive item in and out.

// API Access Token
static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

// ID of the drive item to check in and out.
static ITEM_ID: &str = "ITEM_ID";

// For more information on checking out a drive item see:
// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_checkout?view=odsp-graph-online
async fn check_out_item() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .default_drive()
        .item(ITEM_ID)
        .checkout()
        .header(CONTENT_LENGTH, HeaderValue::from(0)) // Required for checkout. When there is no body the checkin content length will just be 0
        .send()
        .await
        .unwrap();

    // Should be 204 for a successful check out.
    println!("{:#?}", response.status());
}

// For more information on checking in a drive item see:
// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_checkin?view=odsp-graph-online
async fn check_in_item() {
    let client = Graph::new(ACCESS_TOKEN);

    // checkInAs: Optional. The desired status of the document after the check-in
    // operation is complete. Can be 'published' or 'unspecified'.
    let check_in_as = "CHECK_IN_AS";

    // comment: A check-in comment that is associated with the version.
    let comment = "COMMENT";

    let response = client
        .me()
        .default_drive()
        .item(ITEM_ID)
        .checkin(&serde_json::json!({
            "comment": comment,
            "checkInAs": check_in_as
        }))
        .send()
        .await
        .unwrap();

    // Should be 204 for a successful check in.
    println!("{:#?}", response.status());
}
