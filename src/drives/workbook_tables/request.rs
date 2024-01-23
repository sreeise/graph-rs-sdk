// GENERATED CODE

use crate::api_default_imports::*;
use crate::drives::*;

resource_api_client!(
    WorkbookTablesApiClient,
    WorkbookTablesIdApiClient,
    ResourceIdentity::WorkbookTables
);

impl WorkbookTablesApiClient {
    post!(
        doc: "Create new navigation property to tables for drives",
        name: create_tables,
        path: "/tables",
        body: true
    );
    get!(
        doc: "List tables",
        name: list_tables,
        path: "/tables"
    );
    post!(
        doc: "Invoke action add",
        name: add,
        path: "/tables/add",
        body: true
    );
    get!(
        doc: "Invoke function count",
        name: count,
        path: "/tables/count()"
    );
    get!(
        doc: "Invoke function itemAt",
        name: item_at,
        path: "/tables/itemAt(index={{id}})",
        params: index
    );
}

impl WorkbookTablesIdApiClient {
    api_client_link_id!(row, WorkbookTablesRowsIdApiClient);
    api_client_link_id!(column, WorkbookTablesColumnsIdApiClient);
    api_client_link!(rows, WorkbookTablesRowsApiClient);
    api_client_link!(columns, WorkbookTablesColumnsApiClient);

    get!(
        doc: "Get Table",
        name: get_tables,
        path: "/tables/{{RID}}"
    );
    delete!(
        doc: "Table: delete",
        name: delete_tables,
        path: "/tables/{{RID}}"
    );
    patch!(
        doc: "Update table",
        name: update_tables,
        path: "/tables/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action clearFilters",
        name: clear_filters,
        path: "/tables/{{RID}}/clearFilters"
    );
    post!(
        doc: "Invoke action convertToRange",
        name: convert_to_range,
        path: "/tables/{{RID}}/convertToRange"
    );
    get!(
        doc: "Invoke function dataBodyRange",
        name: data_body_range,
        path: "/tables/{{RID}}/dataBodyRange()"
    );
    get!(
        doc: "Invoke function headerRowRange",
        name: header_row_range,
        path: "/tables/{{RID}}/headerRowRange()"
    );
    get!(
        doc: "Invoke function range",
        name: range,
        path: "/tables/{{RID}}/range()"
    );
    post!(
        doc: "Invoke action reapplyFilters",
        name: reapply_filters,
        path: "/tables/{{RID}}/reapplyFilters"
    );
    delete!(
        doc: "Delete navigation property sort for drives",
        name: delete_sort,
        path: "/tables/{{RID}}/sort"
    );
    get!(
        doc: "Get TableSort",
        name: get_sort,
        path: "/tables/{{RID}}/sort"
    );
    patch!(
        doc: "Update the navigation property sort in drives",
        name: update_sort,
        path: "/tables/{{RID}}/sort",
        body: true
    );
    post!(
        doc: "Invoke action apply",
        name: apply,
        path: "/tables/{{RID}}/sort/apply",
        body: true
    );
    post!(
        doc: "Invoke action clear",
        name: clear,
        path: "/tables/{{RID}}/sort/clear"
    );
    post!(
        doc: "Invoke action reapply",
        name: reapply,
        path: "/tables/{{RID}}/sort/reapply"
    );
    get!(
        doc: "Invoke function totalRowRange",
        name: total_row_range,
        path: "/tables/{{RID}}/totalRowRange()"
    );
    get!(
        doc: "Get worksheet from drives",
        name: get_worksheet,
        path: "/tables/{{RID}}/worksheet"
    );
}
