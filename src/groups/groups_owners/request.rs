// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    GroupsOwnersApiClient,
    GroupsOwnersIdApiClient,
    ResourceIdentity::GroupsOwners
);

impl GroupsOwnersApiClient {
    get!(
        doc: "Get owners from groups",
        name: list_owners,
        path: "/owners"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_owners_count,
        path: "/owners/$count"
    );
    post!(
        doc: "Create new navigation property ref to owners for groups",
        name: create_ref_owners,
        path: "/owners/$ref",
        body: true
    );
    get!(
        doc: "Get ref of owners from groups",
        name: list_ref_owners,
        path: "/owners/$ref"
    );
    get!(
        doc: "Get the items of type microsoft.graph.application in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_application_type,
        path: "/owners/graph.application"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_application_count,
        path: "/owners/graph.application/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.device in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_device_type,
        path: "/owners/graph.device"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_count,
        path: "/owners/graph.device/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.group in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_group_type,
        path: "/owners/graph.group"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_group_count,
        path: "/owners/graph.group/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.orgContact in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_org_contact_type,
        path: "/owners/graph.orgContact"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_org_contact_count,
        path: "/owners/graph.orgContact/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.servicePrincipal in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_service_principal_type,
        path: "/owners/graph.servicePrincipal"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_service_principal_count,
        path: "/owners/graph.servicePrincipal/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.user in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_user_type,
        path: "/owners/graph.user"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_user_count,
        path: "/owners/graph.user/$count"
    );
}

impl GroupsOwnersIdApiClient {
    delete!(
        doc: "Delete ref of navigation property owners for groups",
        name: delete_ref_owners,
        path: "/owners/{{RID}}/$ref"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.application",
        name: get_directory_object_item_as_application_type,
        path: "/owners/{{RID}}/graph.application"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.device",
        name: get_directory_object_item_as_device_type,
        path: "/owners/{{RID}}/graph.device"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.group",
        name: get_directory_object_item_as_group_type,
        path: "/owners/{{RID}}/graph.group"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.orgContact",
        name: get_directory_object_item_as_org_contact_type,
        path: "/owners/{{RID}}/graph.orgContact"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.servicePrincipal",
        name: get_directory_object_item_as_service_principal_type,
        path: "/owners/{{RID}}/graph.servicePrincipal"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.user",
        name: get_directory_object_item_as_user_type,
        path: "/owners/{{RID}}/graph.user"
    );
}
