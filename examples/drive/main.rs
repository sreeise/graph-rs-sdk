#![allow(dead_code, unused, unused_imports)]

mod check_in_out;
mod copy;
mod create_folder;
mod delete;
mod download;
mod get_item;
mod list_drive_items;
mod thumbnails;
mod update_item;

mod upload_item;
use crate::copy::copy_item;
use crate::delete::*;
use crate::download::*;
use crate::list_drive_items::*;

fn main() {
    delete();
    download_files();
    list_drive_items();
    copy_drive_items();
}

pub fn delete() {
    // Delete items in OneDrive. This will move deleted items to the recycle bin.
    // It is recommended to create a new file that can be used for demonstration purposes here.
    // Deleting an item can be done in 2 different ways shown in the methods below.
    delete_by_id("DRIVE_ITEM_ID");
    delete_by_path(":/PATH_FROM_ROOT:");
}

pub fn copy_drive_items() {
    copy_item();
}
