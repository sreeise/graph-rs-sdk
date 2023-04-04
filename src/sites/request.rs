// GENERATED CODE

use crate::api_default_imports::*;
use crate::default_drive::*;
use crate::sites::*;
use crate::users::*;

resource_api_client!(SitesApiClient, SitesIdApiClient, ResourceIdentity::Sites);

impl SitesApiClient {
    get!(
        doc: "Search for sites",
        name: list_site,
        path: "/sites"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_sites_count,
        path: "/sites/$count"
    );
    post!(
        doc: "Invoke action add",
        name: add,
        path: "/sites/add",
        body: true
    );
    post!(
        doc: "Invoke action remove",
        name: remove,
        path: "/sites/remove",
        body: true
    );
}

impl SitesIdApiClient {
    api_client_link!(onenote, OnenoteApiClient);
    api_client_link!(lists, SitesListsApiClient);
    api_client_link!(items, SitesItemsApiClient);
    api_client_link_id!(list, SitesListsIdApiClient);
    api_client_link_id!(content_type, SitesContentTypesIdApiClient);
    api_client_link!(drive, DefaultDriveApiClient);
    api_client_link_id!(term_stores_id, TermStoresIdApiClient);
    api_client_link!(content_types, SitesContentTypesApiClient);
    api_client_link!(term_stores, TermStoresApiClient);
    api_client_link!(term_store, TermStoreApiClient);

    get!(
        doc: "Get a site resource",
        name: get_site,
        path: "/sites/{{RID}}"
    );
    patch!(
        doc: "Update entity in sites",
        name: update_site,
        path: "/sites/{{RID}}",
        body: true
    );
    get!(
        doc: "Get analytics from sites",
        name: get_analytics,
        path: "/sites/{{RID}}/analytics"
    );
    post!(
        doc: "Create a columnDefinition in a site",
        name: create_columns,
        path: "/sites/{{RID}}/columns",
        body: true
    );
    get!(
        doc: "List columns in a site",
        name: list_columns,
        path: "/sites/{{RID}}/columns"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_columns_count,
        path: "/sites/{{RID}}/columns/$count"
    );
    delete!(
        doc: "Delete navigation property columns for sites",
        name: delete_columns,
        path: "/sites/{{RID}}/columns/{{id}}",
        params: column_definition_id
    );
    get!(
        doc: "Get columns from sites",
        name: get_columns,
        path: "/sites/{{RID}}/columns/{{id}}",
        params: column_definition_id
    );
    patch!(
        doc: "Update the navigation property columns in sites",
        name: update_columns,
        path: "/sites/{{RID}}/columns/{{id}}",
        body: true,
        params: column_definition_id
    );
    get!(
        doc: "Get sourceColumn from sites",
        name: get_source_column,
        path: "/sites/{{RID}}/columns/{{id}}/sourceColumn",
        params: column_definition_id
    );
    get!(
        doc: "Get Drive",
        name: get_drive,
        path: "/sites/{{RID}}/drive"
    );
    get!(
        doc: "List available drives",
        name: list_drives,
        path: "/sites/{{RID}}/drives"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_drives_count,
        path: "/sites/{{RID}}/drives/$count"
    );
    get!(
        doc: "Get drives from sites",
        name: get_drives,
        path: "/sites/{{RID}}/drives/{{id}}",
        params: drive_id
    );
    get!(
        doc: "Get externalColumns from sites",
        name: list_external_columns,
        path: "/sites/{{RID}}/externalColumns"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_external_columns_count,
        path: "/sites/{{RID}}/externalColumns/$count"
    );
    get!(
        doc: "Get externalColumns from sites",
        name: get_external_columns,
        path: "/sites/{{RID}}/externalColumns/{{id}}",
        params: column_definition_id
    );
    get!(
        doc: "Invoke function getActivitiesByInterval",
        name: site,
        path: "/sites/{{RID}}/getActivitiesByInterval(startDateTime='{{id}}',endDateTime='{{id2}}',interval='{{id3}}')",
        params: start_date_time, end_date_time, interval
    );
    get!(
        doc: "Invoke function getApplicableContentTypesForList",
        name: get_applicable_content_types_for_list,
        path: "/sites/{{RID}}/getApplicableContentTypesForList(listId='{{id}}')",
        params: list_id
    );
    get!(
        doc: "Invoke function getByPath",
        name: get_by_path,
        path: "/sites/{{RID}}/getByPath(path='{{id}}')",
        params: path
    );
    get!(
        doc: "Get items from sites",
        name: list_items,
        path: "/sites/{{RID}}/items"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_items_count,
        path: "/sites/{{RID}}/items/$count"
    );
    get!(
        doc: "Get items from sites",
        name: get_items,
        path: "/sites/{{RID}}/items/{{id}}",
        params: base_item_id
    );
    post!(
        doc: "Create new navigation property to operations for sites",
        name: create_operations,
        path: "/sites/{{RID}}/operations",
        body: true
    );
    get!(
        doc: "List operations on a site",
        name: list_operations,
        path: "/sites/{{RID}}/operations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_operations_count,
        path: "/sites/{{RID}}/operations/$count"
    );
    delete!(
        doc: "Delete navigation property operations for sites",
        name: delete_operations,
        path: "/sites/{{RID}}/operations/{{id}}",
        params: rich_long_running_operation_id
    );
    get!(
        doc: "Get operations from sites",
        name: get_operations,
        path: "/sites/{{RID}}/operations/{{id}}",
        params: rich_long_running_operation_id
    );
    patch!(
        doc: "Update the navigation property operations in sites",
        name: update_operations,
        path: "/sites/{{RID}}/operations/{{id}}",
        body: true,
        params: rich_long_running_operation_id
    );
    post!(
        doc: "Create permission",
        name: create_permissions,
        path: "/sites/{{RID}}/permissions",
        body: true
    );
    get!(
        doc: "List permissions",
        name: list_permissions,
        path: "/sites/{{RID}}/permissions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_permissions_count,
        path: "/sites/{{RID}}/permissions/$count"
    );
    delete!(
        doc: "Delete navigation property permissions for sites",
        name: delete_permissions,
        path: "/sites/{{RID}}/permissions/{{id}}",
        params: permission_id
    );
    get!(
        doc: "Get permissions from sites",
        name: get_permissions,
        path: "/sites/{{RID}}/permissions/{{id}}",
        params: permission_id
    );
    patch!(
        doc: "Update the navigation property permissions in sites",
        name: update_permissions,
        path: "/sites/{{RID}}/permissions/{{id}}",
        body: true,
        params: permission_id
    );
    post!(
        doc: "Invoke action grant",
        name: grant,
        path: "/sites/{{RID}}/permissions/{{id}}/grant",
        body: true,
        params: permission_id
    );
    get!(
        doc: "Enumerate subsites",
        name: list_sites,
        path: "/sites/{{RID}}/sites"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_sites_count,
        path: "/sites/{{RID}}/sites/$count"
    );
    get!(
        doc: "Get sites from sites",
        name: get_sites,
        path: "/sites/{{RID}}/sites/{{id}}",
        params: site_id_1
    );
}
