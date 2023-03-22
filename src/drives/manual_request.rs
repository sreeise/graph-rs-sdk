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
    post!(
        doc: "Create drive item in root of drive",
        name: create_root_folder,
        path: "/drives/{{RID}}/root/children",
        body: true
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
    post!(
        name: create_folder,
        path: "/items/{{RID}}/children",
        body: true
    );
}
