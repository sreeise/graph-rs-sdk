#![allow(dead_code, unused)]

mod check_in_out;
mod copy;
mod create_folder;
mod delete;
mod download;
mod get_item;
mod list_drive_items;
mod preview_item;
mod thumbnails;
mod update_item;
mod upload_and_update_file;
mod upload_file;

// For a complete example of uploading a new file, updating an existing file,
// and deleting a file see upload_and_update_file.rs

// For reference a DriveItem is the JSON metadata for an item in OneDrive
// whereas the Content of an item is the actual content or file itself.

#[tokio::main]
async fn main() {
    download::download_files().await;
    list_drive_items::list_drive_items().await;
    copy::copy_item().await;

    // Delete items in OneDrive. This will move deleted items to the recycle bin.
    // It is recommended to create a new file that can be used for demonstration purposes here.
    // Deleting an item can be done in 2 different ways shown in the methods below.
    delete::delete_by_id("DRIVE_ITEM_ID").await;
    delete::delete_by_path(":/PATH_FROM_ROOT:").await;
}
