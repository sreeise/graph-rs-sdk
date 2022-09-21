// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(DirectoryDeletedItemsRequest,);
register_client!(DirectoryDeletedItemsIdRequest, ());

impl<'a, Client> DirectoryDeletedItemsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Create new navigation property to deletedItems for directory",
        name: create_deleted_items,
        response: serde_json::Value,
        path: "/deletedItems",
        has_body: true
    });
    get!({
        doc: "Get deletedItems from directory",
        name: list_deleted_items,
        response: serde_json::Value,
        path: "/deletedItems",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_deleted_items_count,
        response: serde_json::Value,
        path: "/deletedItems/$count",
        has_body: false
    });
    get!({
        doc: "Get the items of type microsoft.graph.application in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_application_type,
        response: serde_json::Value,
        path: "/deletedItems/microsoft.graph.application",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_application_count,
        response: serde_json::Value,
        path: "/deletedItems/microsoft.graph.application/$count",
        has_body: false
    });
    post!({
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        response: serde_json::Value,
        path: "/deletedItems/microsoft.graph.getAvailableExtensionProperties",
        has_body: true
    });
    post!({
        doc: "Invoke action getByIds",
        name: get_by_ids,
        response: serde_json::Value,
        path: "/deletedItems/microsoft.graph.getByIds",
        has_body: true
    });
    get!({
        doc: "Get the items of type microsoft.graph.group in the microsoft.graph.directoryObject collection",
        name: graph,
        response: serde_json::Value,
        path: "/deletedItems/microsoft.graph.group",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_group_count,
        response: serde_json::Value,
        path: "/deletedItems/microsoft.graph.group/$count",
        has_body: false
    });
    get!({
        doc: "Get the items of type microsoft.graph.user in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_user_type,
        response: serde_json::Value,
        path: "/deletedItems/microsoft.graph.user",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_user_count,
        response: serde_json::Value,
        path: "/deletedItems/microsoft.graph.user/$count",
        has_body: false
    });
    post!({
        doc: "Invoke action validateProperties",
        name: validate_properties,
        response: NoContent,
        path: "/deletedItems/microsoft.graph.validateProperties",
        has_body: true
    });
}

impl<'a, Client> DirectoryDeletedItemsIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    delete!({
        doc: "Delete navigation property deletedItems for directory",
        name: delete_deleted_items,
        response: NoContent,
        path: "/deletedItems/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get deletedItems from directory",
        name: get_deleted_items,
        response: serde_json::Value,
        path: "/deletedItems/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property deletedItems in directory",
        name: update_deleted_items,
        response: NoContent,
        path: "/deletedItems/{{RID}}",
        has_body: true
    });
    get!({
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.application",
        name: get_directory_object_item_as_application_type,
        response: serde_json::Value,
        path: "/deletedItems/{{RID}}/microsoft.graph.application",
        has_body: false
    });
    post!({
        doc: "Invoke action checkMemberGroups",
        name: check_member_groups,
        response: serde_json::Value,
        path: "/deletedItems/{{RID}}/microsoft.graph.checkMemberGroups",
        has_body: true
    });
    post!({
        doc: "Invoke action checkMemberObjects",
        name: check_member_objects,
        response: serde_json::Value,
        path: "/deletedItems/{{RID}}/microsoft.graph.checkMemberObjects",
        has_body: true
    });
    post!({
        doc: "Invoke action getMemberGroups",
        name: get_member_groups,
        response: serde_json::Value,
        path: "/deletedItems/{{RID}}/microsoft.graph.getMemberGroups",
        has_body: true
    });
    post!({
        doc: "Invoke action getMemberObjects",
        name: get_member_objects,
        response: serde_json::Value,
        path: "/deletedItems/{{RID}}/microsoft.graph.getMemberObjects",
        has_body: true
    });
    get!({
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.group",
        name: get_directory_object_item_as_group_type,
        response: serde_json::Value,
        path: "/deletedItems/{{RID}}/microsoft.graph.group",
        has_body: false
    });
    post!({
        doc: "Invoke action restore",
        name: restore,
        response: serde_json::Value,
        path: "/deletedItems/{{RID}}/microsoft.graph.restore",
        has_body: false
    });
    get!({
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.user",
        name: get_directory_object_item_as_user_type,
        response: serde_json::Value,
        path: "/deletedItems/{{RID}}/microsoft.graph.user",
        has_body: false
    });
}
