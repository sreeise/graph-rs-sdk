// GENERATED CODE

use crate::api_default_imports::*;
use crate::sites::*;

resource_api_client!(
    SitesListsApiClient,
    SitesListsIdApiClient,
    ResourceIdentity::SitesLists
);

impl SitesListsApiClient {
    post!(
        doc: "Create a new list",
        name: create_lists,
        path: "/lists",
        body: true
    );
    get!(
        doc: "Enumerate lists in a site",
        name: list_lists,
        path: "/lists"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_lists_count,
        path: "/lists/$count"
    );
}

impl SitesListsIdApiClient {
    api_client_link_id!(item, SitesItemsIdApiClient);
    api_client_link!(items, SitesItemsApiClient);
    api_client_link_id!(content_type, SitesContentTypesIdApiClient);
    api_client_link!(content_types, SitesContentTypesApiClient);

    delete!(
        doc: "Delete navigation property lists for sites",
        name: delete_lists,
        path: "/lists/{{RID}}"
    );
    get!(
        doc: "Get lists from sites",
        name: get_lists,
        path: "/lists/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property lists in sites",
        name: update_lists,
        path: "/lists/{{RID}}",
        body: true
    );
    post!(
        doc: "Create a columnDefinition in a list",
        name: create_columns,
        path: "/lists/{{RID}}/columns",
        body: true
    );
    get!(
        doc: "List columnDefinitions in a list",
        name: list_columns,
        path: "/lists/{{RID}}/columns"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_columns_count,
        path: "/lists/{{RID}}/columns/$count"
    );
    delete!(
        doc: "Delete navigation property columns for sites",
        name: delete_columns,
        path: "/lists/{{RID}}/columns/{{id}}",
        params: column_definition_id
    );
    get!(
        doc: "Get columns from sites",
        name: get_columns,
        path: "/lists/{{RID}}/columns/{{id}}",
        params: column_definition_id
    );
    patch!(
        doc: "Update the navigation property columns in sites",
        name: update_columns,
        path: "/lists/{{RID}}/columns/{{id}}",
        body: true,
        params: column_definition_id
    );
    get!(
        doc: "Get sourceColumn from sites",
        name: get_source_column,
        path: "/lists/{{RID}}/columns/{{id}}/sourceColumn",
        params: column_definition_id
    );
    get!(
        doc: "Get drive from sites",
        name: get_drive,
        path: "/lists/{{RID}}/drive"
    );
    post!(
        doc: "Create new navigation property to operations for sites",
        name: create_operations,
        path: "/lists/{{RID}}/operations",
        body: true
    );
    get!(
        doc: "Get operations from sites",
        name: list_operations,
        path: "/lists/{{RID}}/operations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_operations_count,
        path: "/lists/{{RID}}/operations/$count"
    );
    delete!(
        doc: "Delete navigation property operations for sites",
        name: delete_operations,
        path: "/lists/{{RID}}/operations/{{id}}",
        params: rich_long_running_operation_id
    );
    get!(
        doc: "Get operations from sites",
        name: get_operations,
        path: "/lists/{{RID}}/operations/{{id}}",
        params: rich_long_running_operation_id
    );
    patch!(
        doc: "Update the navigation property operations in sites",
        name: update_operations,
        path: "/lists/{{RID}}/operations/{{id}}",
        body: true,
        params: rich_long_running_operation_id
    );
    post!(
        doc: "Create new navigation property to subscriptions for sites",
        name: create_subscriptions,
        path: "/lists/{{RID}}/subscriptions",
        body: true
    );
    get!(
        doc: "Get subscriptions from sites",
        name: list_subscriptions,
        path: "/lists/{{RID}}/subscriptions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_subscriptions_count,
        path: "/lists/{{RID}}/subscriptions/$count"
    );
    delete!(
        doc: "Delete navigation property subscriptions for sites",
        name: delete_subscriptions,
        path: "/lists/{{RID}}/subscriptions/{{id}}",
        params: subscription_id
    );
    get!(
        doc: "Get subscriptions from sites",
        name: get_subscriptions,
        path: "/lists/{{RID}}/subscriptions/{{id}}",
        params: subscription_id
    );
    patch!(
        doc: "Update the navigation property subscriptions in sites",
        name: update_subscriptions,
        path: "/lists/{{RID}}/subscriptions/{{id}}",
        body: true,
        params: subscription_id
    );
    post!(
        doc: "Invoke action reauthorize",
        name: reauthorize,
        path: "/lists/{{RID}}/subscriptions/{{id}}/reauthorize",
        params: subscription_id
    );
}
