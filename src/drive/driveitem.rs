use crate::drive::identity::IdentitySet;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct DriveItemImage {
    height: u64,
    width: u64,
}

#[allow(dead_code)]
pub struct DriveFile {
    meme_type: String,
    hashes: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct ParentReference {
    drive_id: String,
    drive_type: String,
    id: String,
    path: String,
}

#[allow(dead_code)]
pub struct FileSystemInfo {
    created_date_time: String,
    last_modified_date_time: String,
}

#[allow(dead_code)]
pub struct Shared {
    scope: String,
}

#[allow(dead_code)]
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
