// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    WorksheetsApiClient,
    WorksheetsIdApiClient,
    ResourceIdentity::Worksheets
);

impl WorksheetsApiClient {
    post!(
        doc: "Create new navigation property to worksheets for drives",
        name: create_worksheets,
        path: "/worksheets",
        body: true
    );
    get!(
        doc: "List WorksheetCollection",
        name: list_worksheets,
        path: "/worksheets"
    );
    get!(
        doc: "Get the number of the resource",
        name: worksheets,
        path: "/worksheets/$count"
    );
    post!(
        doc: "Invoke action add",
        name: add,
        path: "/worksheets/add",
        body: true
    );
}

impl WorksheetsIdApiClient {
    get!(
        doc: "Get Worksheet",
        name: get_worksheets,
        path: "/worksheets/{{RID}}"
    );
    patch!(
        doc: "Update worksheet",
        name: update_worksheets,
        path: "/worksheets/{{RID}}",
        body: true
    );
    delete!(
        doc: "Worksheet: delete",
        name: delete_worksheets,
        path: "/worksheets/{{RID}}"
    );
    get!(
        doc: "Invoke function cell",
        name: cell,
        path: "/worksheets/{{RID}}/cell(row={{id}},column={{id2}})",
        params: row, column
    );
    delete!(
        doc: "Delete navigation property protection for drives",
        name: delete_protection,
        path: "/worksheets/{{RID}}/protection"
    );
    get!(
        doc: "Get WorksheetProtection",
        name: get_protection,
        path: "/worksheets/{{RID}}/protection"
    );
    patch!(
        doc: "Update the navigation property protection in drives",
        name: update_protection,
        path: "/worksheets/{{RID}}/protection",
        body: true
    );
    post!(
        doc: "Invoke action protect",
        name: protect,
        path: "/worksheets/{{RID}}/protection/protect",
        body: true
    );
    post!(
        doc: "Invoke action unprotect",
        name: unprotect,
        path: "/worksheets/{{RID}}/protection/unprotect"
    );
    get!(
        doc: "Invoke function range",
        name: get_range_object,
        path: "/worksheets/{{RID}}/range()"
    );
    get!(
        doc: "Invoke function range",
        name: get_range_object_by_address,
        path: "/worksheets/{{RID}}/range(address='{{id}}')",
        params: address
    );
    get!(
        doc: "Invoke function usedRange",
        name: get_used_range_object,
        path: "/worksheets/{{RID}}/usedRange()"
    );
    get!(
        doc: "Invoke function usedRange",
        name: get_used_range_object_with_values_only,
        path: "/worksheets/{{RID}}/usedRange(valuesOnly={{id}})",
        params: values_only
    );
}
