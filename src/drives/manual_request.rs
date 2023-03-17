use crate::api_default_imports::*;
use crate::drives::{DrivesIdApiClient, DrivesItemsIdApiClient};

impl DrivesIdApiClient {
    get!(
        doc: "List children of a driveItem",
        name: list_children_by_file,
        path: "/root{{RID}}children"
    );
    get!(
        doc: "List children of a driveItem",
        name: get_items_content_by_file,
        path: "/items{{RID}}content"
    );
}

impl DrivesItemsIdApiClient {
    get!(
        doc: "List children of a driveItem",
        name: list_children_by_file,
        path: "/root{{RID}}children"
    );
    get!(
        doc: "List children of a driveItem",
        name: get_items_content_by_file,
        path: "/root{{RID}}content"
    );
}
