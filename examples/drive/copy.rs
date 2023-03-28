use graph_http::traits::ResponseExt;
use graph_rs_sdk::prelude::*;
use std::thread;
use std::time::Duration;

// Set the name of the file you want to copy
// and a name for the copy of the file.
static DRIVE_FILE_COPY_NAME: &str = "FILE_NAME_OF_COPY";

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

// This can either be the id of the file or a path to the file in OneDrive
// that is wrapped by : Example ":/documents/my_file.txt:"
static ITEM_ID: &str = "ITEM_ID";

pub async fn copy_item() {
    let graph = Graph::new(ACCESS_TOKEN);

    // The DriveItem copy request uses a ItemReference (parent reference) which contains
    // the metadata for the drive id and path specifying where the new copy should be placed.
    // The path below in the ItemReference is typically the same path as the drive item
    // requested above so the copy of the item will be placed in the same folder. This can
    // be changed to wherever you would like the copy placed.

    let response = graph
        .me()
        .drive()
        .item(ITEM_ID)
        .copy(&serde_json::json!({
            "name": DRIVE_FILE_COPY_NAME,
            "parent_reference": {
                "path": "/drive/root:/Documents"
            }
        }))
        .send()
        .await
        .unwrap();

    // When an item is copied the response returns a URL in the location header
    // that can be used to monitor the progress. For events that may take longer to finish
    // such as copying an item, the job status method can be used to get the metadata returned
    // from the monitor URL. A copy request may return a successful response but this does not
    // mean the copy request has finished. The job status method should be used to check if the
    // event has finished instead of the success method.

    // Wait a few seconds before checking the progress (assuming the file or
    // folder size is small here).
    thread::sleep(Duration::from_secs(5));

    let job_status_response = response.job_status().await;
    println!("{:#?}", job_status_response);
}
