use graph_rs_sdk::prelude::*;

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
    let embeddable_url = serde_json::json!({
        // Properties
        "allow_edit": true
    });
    get_preview_with_properties("<ACCESS_TOKEN>", "<ITEM_ID>", embeddable_url);
}

fn get_preview(access_token: &str, item_id: &str) {
    let drive = Graph::new(access_token);

    let preview = drive
        .beta()
        .me()
        .drive()
        .preview::<&str, ()>(item_id, None)
        .send()
        .unwrap();
    println!("{:#?}", preview);
}

fn get_preview_with_properties(
    access_token: &str,
    item_id: &str,
    embeddable_url: serde_json::Value,
) {
    let drive = Graph::new(access_token);

    let preview = drive
        .beta()
        .me()
        .drive()
        .preview(item_id, Some(&embeddable_url))
        .send()
        .unwrap();
    println!("{:#?}", preview);
}
