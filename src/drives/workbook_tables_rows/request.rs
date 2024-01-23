// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    WorkbookTablesRowsApiClient,
    WorkbookTablesRowsIdApiClient,
    ResourceIdentity::WorkbookTablesRows
);

impl WorkbookTablesRowsApiClient {
    post!(
        doc: "Create TableRow",
        name: create_rows,
        path: "/rows",
        body: true
    );
    get!(
        doc: "List rows",
        name: list_rows,
        path: "/rows"
    );
    post!(
        doc: "Invoke action add",
        name: add,
        path: "/rows/add",
        body: true
    );
    get!(
        doc: "Invoke function count",
        name: count,
        path: "/rows/count()"
    );
    get!(
        doc: "Invoke function itemAt",
        name: item_at,
        path: "/rows/itemAt(index={{id}})",
        params: index
    );
}

impl WorkbookTablesRowsIdApiClient {
    get!(
        doc: "Get TableRow",
        name: get_rows,
        path: "/rows/{{RID}}"
    );
    delete!(
        doc: "TableRow: delete",
        name: delete_rows,
        path: "/rows/{{RID}}"
    );
    patch!(
        doc: "Update tablerow",
        name: update_rows,
        path: "/rows/{{RID}}",
        body: true
    );
    get!(
        doc: "Invoke function range",
        name: range,
        path: "/rows/{{RID}}/range()"
    );
}
