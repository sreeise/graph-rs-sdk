// GENERATED CODE

use crate::api_default_imports::*;
use crate::drives::{
    CreatedByUserApiClient, DrivesItemsApiClient, DrivesItemsIdApiClient,
    DrivesListContentTypesApiClient, DrivesListContentTypesIdApiClient,
    LastModifiedByUserApiClient,
};

resource_api_client!(DrivesListApiClient, ResourceIdentity::DrivesList);

impl DrivesListApiClient {
    api_client_link_id!(item, DrivesItemsIdApiClient);
    api_client_link!(content_types, DrivesListContentTypesApiClient);
    api_client_link!(items, DrivesItemsApiClient);
    api_client_link!(last_modified_by_user, LastModifiedByUserApiClient);
    api_client_link!(created_by_user, CreatedByUserApiClient);
    api_client_link_id!(content_type, DrivesListContentTypesIdApiClient);

    delete!(
        doc: "Delete navigation property list for drives",
        name: delete_list,
        path: "/list"
    );
    get!(
        doc: "Get list from drives",
        name: get_list,
        path: "/list"
    );
    patch!(
        doc: "Update the navigation property list in drives",
        name: update_list,
        path: "/list",
        body: true
    );
    post!(
        doc: "Create a columnDefinition in a list",
        name: create_columns,
        path: "/list/columns",
        body: true
    );
    get!(
        doc: "List columnDefinitions in a list",
        name: list_columns,
        path: "/list/columns"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_columns_count,
        path: "/list/columns/$count"
    );
    delete!(
        doc: "Delete navigation property columns for drives",
        name: delete_columns,
        path: "/list/columns/{{id}}",
        params: column_definition_id
    );
    get!(
        doc: "Get columns from drives",
        name: get_columns,
        path: "/list/columns/{{id}}",
        params: column_definition_id
    );
    patch!(
        doc: "Update the navigation property columns in drives",
        name: update_columns,
        path: "/list/columns/{{id}}",
        body: true,
        params: column_definition_id
    );
    get!(
        doc: "Get sourceColumn from drives",
        name: get_source_column,
        path: "/list/columns/{{id}}/sourceColumn",
        params: column_definition_id
    );
    get!(
        doc: "Get drive from drives",
        name: get_drive,
        path: "/list/drive"
    );
    post!(
        doc: "Create new navigation property to operations for drives",
        name: create_operations,
        path: "/list/operations",
        body: true
    );
    get!(
        doc: "Get operations from drives",
        name: list_operations,
        path: "/list/operations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_operations_count,
        path: "/list/operations/$count"
    );
    delete!(
        doc: "Delete navigation property operations for drives",
        name: delete_operations,
        path: "/list/operations/{{id}}",
        params: rich_long_running_operation_id
    );
    get!(
        doc: "Get operations from drives",
        name: get_operations,
        path: "/list/operations/{{id}}",
        params: rich_long_running_operation_id
    );
    patch!(
        doc: "Update the navigation property operations in drives",
        name: update_operations,
        path: "/list/operations/{{id}}",
        body: true,
        params: rich_long_running_operation_id
    );
    post!(
        doc: "Create new navigation property to subscriptions for drives",
        name: create_subscriptions,
        path: "/list/subscriptions",
        body: true
    );
    get!(
        doc: "Get subscriptions from drives",
        name: list_subscriptions,
        path: "/list/subscriptions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_subscriptions_count,
        path: "/list/subscriptions/$count"
    );
    delete!(
        doc: "Delete navigation property subscriptions for drives",
        name: delete_subscriptions,
        path: "/list/subscriptions/{{id}}",
        params: subscription_id
    );
    get!(
        doc: "Get subscriptions from drives",
        name: get_subscriptions,
        path: "/list/subscriptions/{{id}}",
        params: subscription_id
    );
    patch!(
        doc: "Update the navigation property subscriptions in drives",
        name: update_subscriptions,
        path: "/list/subscriptions/{{id}}",
        body: true,
        params: subscription_id
    );
    post!(
        doc: "Invoke action reauthorize",
        name: reauthorize,
        path: "/list/subscriptions/{{id}}/reauthorize",
        params: subscription_id
    );
}
