#![allow(dead_code, unused, unused_imports)]

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

use crate::copy::copy_item;
use crate::delete::*;
use crate::download::*;
use crate::list_drive_items::*;

// For a complete example of uploading a new file, updating an existing file,
// and deleting a file see upload_and_update_file.rs

#[tokio::main]
async fn main() {
    download_files().await;
    list_drive_items().await;
    copy_item().await;

    // Delete items in OneDrive. This will move deleted items to the recycle bin.
    // It is recommended to create a new file that can be used for demonstration purposes here.
    // Deleting an item can be done in 2 different ways shown in the methods below.
    delete_by_id("DRIVE_ITEM_ID").await;
    delete_by_path(":/PATH_FROM_ROOT:").await;
}
