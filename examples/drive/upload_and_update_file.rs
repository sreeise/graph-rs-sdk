use graph_rs_sdk::http::FileConfig;
use graph_rs_sdk::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

static USER_ID: &str = "USER_ID";

static FILE_NAME: &str = ":/file.txt:";

static LOCAL_FILE: &str = "./examples/file.txt";

static ONEDRIVE_FILE_PATH: &str = ":/Documents/file.txt:";

// Used to get the id of the Documents folder in OneDrive to use as a parent reference id when
// uploading a new file.
static SPECIAL_DOCUMENT_FOLDER: &str = "Documents";

async fn get_special_folder_id(user_id: &str, folder: &str, client: &Graph) -> GraphResult<String> {
    let response = client
        .user(user_id)
        .drive()
        .get_special(folder)
        .send()
        .await?;

    let body: serde_json::Value = response.json().await?;
    let parent_reference_id = body["id"].as_str().unwrap();
    Ok(parent_reference_id.into())
}

async fn upload_new_file(
    user_id: &str,
    parent_reference_id: &str,
    file_name: &str,
    local_file: &str,
    client: &Graph,
) -> GraphResult<reqwest::Response> {
    client
        .drive(user_id)
        .item(parent_reference_id)
        .upload_items_content(file_name, &FileConfig::new(local_file))
        .send()
        .await
}

async fn update_file(
    user_id: &str,
    onedrive_file_path: &str,
    local_file: &str,
    client: &Graph,
) -> GraphResult<reqwest::Response> {
    client
        .user(user_id)
        .drive()
        .item_by_path(onedrive_file_path)
        .update_items_content(&FileConfig::new(local_file))
        .send()
        .await
}

async fn delete_file(
    user_id: &str,
    item_id: &str,
    client: &Graph,
) -> GraphResult<reqwest::Response> {
    client
        .user(user_id)
        .drive()
        .item(item_id)
        .delete_items()
        .send()
        .await
}

async fn upload_and_update_item() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    // Get the id for the Documents folder where the file will be uploaded.
    let parent_reference_id =
        get_special_folder_id(USER_ID, SPECIAL_DOCUMENT_FOLDER, &client).await?;

    // Upload the new file to the Documents folder in OneDrive
    let response = upload_new_file(
        USER_ID,
        parent_reference_id.as_str(),
        FILE_NAME,
        LOCAL_FILE,
        &client,
    )
    .await?;

    let body: serde_json::Value = response.json().await?;
    let item_id = body["id"].as_str().unwrap();

    // Update the file.
    let mut file = OpenOptions::new().write(true).open(LOCAL_FILE).unwrap();
    file.write_all("Test Update File".as_bytes()).unwrap();
    file.sync_all()?;

    // Update the file in OneDrive
    let update_response = update_file(USER_ID, ONEDRIVE_FILE_PATH, LOCAL_FILE, &client).await?;
    println!("{update_response:#?}");

    // Delete the file.
    let _ = delete_file(USER_ID, item_id, &client).await?;

    Ok(())
}
