use rust_onedrive::drive::event::EmbeddableUrl;
use rust_onedrive::prelude::*;

// You can get preview URLs for an item. Provide optional
// properties with an EmbeddableUrl.

// Note: The previews API currently only works for SharePoint and OneDrive business accounts.

// For more info on previews see:
// V1.0 API: https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_preview?view=odsp-graph-online
// Beta API: https://docs.microsoft.com/en-us/graph/api/driveitem-preview?view=graph-rest-beta

fn main() {
    // The following preview methods use the Graph Beta API.

    // Provide the access token to query the API and the item id of the drive item you
    // want to get a preview for.
    get_preview("<ACCESS_TOKEN>", "<ITEM_ID>");

    // Or add optional properties to the request.
    let mut embeddable_url = EmbeddableUrl::default();
    embeddable_url.set_allow_edit(Some(true));
    get_preview_with_properties("<ACCESS_TOKEN>", "<ITEM_ID>", embeddable_url);
}

fn get_preview(access_token: &str, item_id: &str) {
    let drive = Drive::new(access_token);

    let mut req = drive.v2().me().preview(item_id, None);

    match req.send() {
        Ok(t) => {
            println!("Preview {:#?}", t);
        },
        Err(e) => {
            println!("Error: {:#?}", e);
        },
    }
}

fn get_preview_with_properties(access_token: &str, item_id: &str, embeddable_url: EmbeddableUrl) {
    let drive = Drive::new(access_token);

    let mut req = drive.v2().me().preview(item_id, Some(embeddable_url));

    match req.send() {
        Ok(t) => {
            println!("Preview {:#?}", t);
        },
        Err(e) => {
            println!("Error: {:#?}", e);
        },
    }
}
