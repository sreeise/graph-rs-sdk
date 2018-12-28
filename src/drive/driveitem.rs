/*
DRIVE ITEM

TODO: Change DriveItem in mod.rs as it's naming is off. This is the correct implementation
for drive item
*/

use crate::drive::driveresource::DriveResource;
use crate::drive::identity::IdentitySet;
use std::collections::HashMap;

pub struct DriveItemImage {
    height: u64,
    width: u64,
}

pub struct DriveFile {
    meme_type: String,
    hashes: HashMap<String, String>,
}

pub struct ParentReference {
    drive_id: String,
    drive_type: String,
    id: String,
    path: String,
}

pub struct FileSystemInfo {
    created_date_time: String,
    last_modified_date_time: String,
}

pub struct Shared {
    scope: String,
}

pub struct DriveItem {
    data_context: Option<String>, // Depends on the url end point.
    download_url: String,
    created_date_time: String,
    c_tag: String,
    e_tag: String,
    id: String,
    last_modified_date_time: IdentitySet,
    name: String,
    size: u64,
    web_url: String,
    created_by: Vec<IdentitySet>,
    last_modified_by: IdentitySet,
    parent_reference: ParentReference,
    file: DriveFile,
    file_system_info: FileSystemInfo,
    shared: Shared,
}
