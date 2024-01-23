// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    WorkbookTablesColumnsApiClient,
    WorkbookTablesColumnsIdApiClient,
    ResourceIdentity::WorkbookTablesColumns
);

impl WorkbookTablesColumnsApiClient {
    post!(
        doc: "Create TableColumn",
        name: create_columns,
        path: "/columns",
        body: true
    );
    get!(
        doc: "List TableColumnCollection",
        name: list_columns,
        path: "/columns"
    );
    post!(
        doc: "Invoke action add",
        name: add,
        path: "/columns/add",
        body: true
    );
    get!(
        doc: "Invoke function count",
        name: count,
        path: "/columns/count()"
    );
    get!(
        doc: "Invoke function itemAt",
        name: item_at,
        path: "/columns/itemAt(index={{id}})",
        params: index
    );
}

impl WorkbookTablesColumnsIdApiClient {
    get!(
        doc: "Get TableColumn",
        name: get_columns,
        path: "/columns/{{RID}}"
    );
    delete!(
        doc: "TableColumn: delete",
        name: delete_columns,
        path: "/columns/{{RID}}"
    );
    patch!(
        doc: "Update tablecolumn",
        name: update_columns,
        path: "/columns/{{RID}}",
        body: true
    );
    get!(
        doc: "Invoke function dataBodyRange",
        name: data_body_range,
        path: "/columns/{{RID}}/dataBodyRange()"
    );
    delete!(
        doc: "Delete navigation property filter for drives",
        name: delete_filter,
        path: "/columns/{{RID}}/filter"
    );
    get!(
        doc: "Get filter from drives",
        name: get_filter,
        path: "/columns/{{RID}}/filter"
    );
    patch!(
        doc: "Update the navigation property filter in drives",
        name: update_filter,
        path: "/columns/{{RID}}/filter",
        body: true
    );
    post!(
        doc: "Invoke action apply",
        name: apply,
        path: "/columns/{{RID}}/filter/apply",
        body: true
    );
    post!(
        doc: "Invoke action applyBottomItemsFilter",
        name: apply_bottom_items_filter,
        path: "/columns/{{RID}}/filter/applyBottomItemsFilter",
        body: true
    );
    post!(
        doc: "Invoke action applyBottomPercentFilter",
        name: apply_bottom_percent_filter,
        path: "/columns/{{RID}}/filter/applyBottomPercentFilter",
        body: true
    );
    post!(
        doc: "Invoke action applyCellColorFilter",
        name: apply_cell_color_filter,
        path: "/columns/{{RID}}/filter/applyCellColorFilter",
        body: true
    );
    post!(
        doc: "Invoke action applyCustomFilter",
        name: apply_custom_filter,
        path: "/columns/{{RID}}/filter/applyCustomFilter",
        body: true
    );
    post!(
        doc: "Invoke action applyDynamicFilter",
        name: apply_dynamic_filter,
        path: "/columns/{{RID}}/filter/applyDynamicFilter",
        body: true
    );
    post!(
        doc: "Invoke action applyFontColorFilter",
        name: apply_font_color_filter,
        path: "/columns/{{RID}}/filter/applyFontColorFilter",
        body: true
    );
    post!(
        doc: "Invoke action applyIconFilter",
        name: apply_icon_filter,
        path: "/columns/{{RID}}/filter/applyIconFilter",
        body: true
    );
    post!(
        doc: "Invoke action applyTopItemsFilter",
        name: apply_top_items_filter,
        path: "/columns/{{RID}}/filter/applyTopItemsFilter",
        body: true
    );
    post!(
        doc: "Invoke action applyTopPercentFilter",
        name: apply_top_percent_filter,
        path: "/columns/{{RID}}/filter/applyTopPercentFilter",
        body: true
    );
    post!(
        doc: "Invoke action applyValuesFilter",
        name: apply_values_filter,
        path: "/columns/{{RID}}/filter/applyValuesFilter",
        body: true
    );
    post!(
        doc: "Invoke action clear",
        name: clear,
        path: "/columns/{{RID}}/filter/clear"
    );
    get!(
        doc: "Invoke function headerRowRange",
        name: header_row_range,
        path: "/columns/{{RID}}/headerRowRange()"
    );
    get!(
        doc: "Invoke function range",
        name: range,
        path: "/columns/{{RID}}/range()"
    );
    get!(
        doc: "Invoke function totalRowRange",
        name: total_row_range,
        path: "/columns/{{RID}}/totalRowRange()"
    );
}
