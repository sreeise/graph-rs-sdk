// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    DeletedItemsApiClient,
    DeletedItemsIdApiClient,
    ResourceIdentity::DeletedItems
);

impl DeletedItemsApiClient {
    post!(
        doc: "Create new navigation property to deletedItems for directory",
        name: create_deleted_items,
        path: "/deletedItems",
        body: true
    );
    get!(
        doc: "Get deletedItems from directory",
        name: list_deleted_items,
        path: "/deletedItems"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_deleted_items_count,
        path: "/deletedItems/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/deletedItems/delta()"
    );
    post!(
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        path: "/deletedItems/getAvailableExtensionProperties",
        body: true
    );
    post!(
        doc: "Invoke action getByIds",
        name: get_by_ids,
        path: "/deletedItems/getByIds",
        body: true
    );
    get!(
        doc: "Get the items of type microsoft.graph.application in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_application_type,
        path: "/deletedItems/graph.application"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_application_count,
        path: "/deletedItems/graph.application/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.group in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_group_type,
        path: "/deletedItems/graph.group"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_group_count,
        path: "/deletedItems/graph.group/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.user in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_user_type,
        path: "/deletedItems/graph.user"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_user_count,
        path: "/deletedItems/graph.user/$count"
    );
    post!(
        doc: "Invoke action validateProperties",
        name: validate_properties,
        path: "/deletedItems/validateProperties",
        body: true
    );
}

impl DeletedItemsIdApiClient {
    delete!(
        doc: "Delete navigation property deletedItems for directory",
        name: delete_deleted_items,
        path: "/deletedItems/{{RID}}"
    );
    get!(
        doc: "Get deletedItems from directory",
        name: get_deleted_items,
        path: "/deletedItems/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property deletedItems in directory",
        name: update_deleted_items,
        path: "/deletedItems/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberGroups",
        name: check_member_groups,
        path: "/deletedItems/{{RID}}/checkMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberObjects",
        name: check_member_objects,
        path: "/deletedItems/{{RID}}/checkMemberObjects",
        body: true
    );
    post!(
        doc: "Invoke action getMemberGroups",
        name: get_member_groups,
        path: "/deletedItems/{{RID}}/getMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action getMemberObjects",
        name: get_member_objects,
        path: "/deletedItems/{{RID}}/getMemberObjects",
        body: true
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.application",
        name: get_directory_object_item_as_application_type,
        path: "/deletedItems/{{RID}}/graph.application"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.group",
        name: get_directory_object_item_as_group_type,
        path: "/deletedItems/{{RID}}/graph.group"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.user",
        name: get_directory_object_item_as_user_type,
        path: "/deletedItems/{{RID}}/graph.user"
    );
    post!(
        doc: "Invoke action restore",
        name: restore,
        path: "/deletedItems/{{RID}}/restore"
    );
}
