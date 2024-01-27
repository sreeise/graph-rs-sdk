use crate::api_default_imports::*;
use crate::drives::*;

impl DrivesIdApiClient {
    post!(
        doc: "Create drive item in root of drive",
        name: create_root_folder,
        path: "/drives/{{RID}}/root/children",
        body: true
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/drives/{{RID}}/root/delta()"
    );
    get!(
        doc: "Invoke function delta",
        name: delta_token,
        path: "/drives/{{RID}}/root/delta(token='{{id}}')",
        params: token
    );
}

impl DrivesItemsIdApiClient {
    post!(
        name: create_folder,
        path: "/items/{{RID}}/children",
        body: true
    );
    put!(
        name: upload_items_content,
        path: "items/{{RID}}{{id}}/content",
        body: true,
        params: file_name
    );
}

impl WorksheetsIdApiClient {
    patch!(
        doc: "Invoke function range",
        name: update_range_object_by_address,
        path: "/worksheets/{{RID}}/range(address='{{id}}')",
        body: true,
        params: address
    );
}
impl WorkbookTablesIdApiClient {
    patch!(
        doc: "Update range",
        name: update_range,
        path: "/tables/{{RID}}/range",
        body: true
    );
}

impl WorkbookTablesColumnsIdApiClient {
    patch!(
        doc: "Update range of column",
        name: update_range,
        path: "/columns/{{RID}}/range",
        body: true
    );
}
