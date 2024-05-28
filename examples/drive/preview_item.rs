use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

static ONEDRIVE_FILE_PATH: &str = ":/Documents/file.txt:";

// Request Body (https://learn.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_preview?view=odsp-graph-online):
// viewer 	    string 	        Optional. Preview app to use. onedrive or office. If null, a suitable viewer will be chosen automatically.
// chromeless 	boolean 	    Optional. If true (default), the embedded view will not include any controls.
// allowEdit 	boolean 	    Optional. If true, the file can be edited from the embedded UI.
// page 	    string/number 	Optional. Page number of document to start at, if applicable. Specified as string for future use cases around file types such as ZIP.
// zoom 	    number 	        Optional. Zoom level to start at, if applicable.

pub async fn get_drive_item(item_id: &str) {
    let client = GraphClient::new(ACCESS_TOKEN);

    let body = serde_json::json!({
        "viewer": null,
        "chromeless": false
        // etc..
    });

    let response = client
        .me()
        .drive()
        .item_by_path(ONEDRIVE_FILE_PATH)
        .preview(&body)
        .send()
        .await
        .unwrap();

    println!("{response:#?}");

    let response_body: serde_json::Value = response.json().await.unwrap();
    println!("{response_body:#?}");
}
