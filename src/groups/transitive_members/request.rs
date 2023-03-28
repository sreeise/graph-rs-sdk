// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    TransitiveMembersApiClient,
    TransitiveMembersIdApiClient,
    ResourceIdentity::TransitiveMembers
);

impl TransitiveMembersApiClient {
    get!(
        doc: "Get transitiveMembers from groups",
        name: list_transitive_members,
        path: "/transitiveMembers"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_transitive_members_count,
        path: "/transitiveMembers/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.application in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_application_type,
        path: "/transitiveMembers/graph.application"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_application_count,
        path: "/transitiveMembers/graph.application/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.device in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_device_type,
        path: "/transitiveMembers/graph.device"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_count,
        path: "/transitiveMembers/graph.device/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.group in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_group_type,
        path: "/transitiveMembers/graph.group"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_group_count,
        path: "/transitiveMembers/graph.group/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.orgContact in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_org_contact_type,
        path: "/transitiveMembers/graph.orgContact"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_org_contact_count,
        path: "/transitiveMembers/graph.orgContact/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.servicePrincipal in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_service_principal_type,
        path: "/transitiveMembers/graph.servicePrincipal"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_service_principal_count,
        path: "/transitiveMembers/graph.servicePrincipal/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.user in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_user_type,
        path: "/transitiveMembers/graph.user"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_user_count,
        path: "/transitiveMembers/graph.user/$count"
    );
}

impl TransitiveMembersIdApiClient {
    get!(
        doc: "Get transitiveMembers from groups",
        name: get_transitive_members,
        path: "/transitiveMembers/{{RID}}"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.application",
        name: get_directory_object_item_as_application_type,
        path: "/transitiveMembers/{{RID}}/graph.application"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.device",
        name: get_directory_object_item_as_device_type,
        path: "/transitiveMembers/{{RID}}/graph.device"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.group",
        name: get_directory_object_item_as_group_type,
        path: "/transitiveMembers/{{RID}}/graph.group"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.orgContact",
        name: get_directory_object_item_as_org_contact_type,
        path: "/transitiveMembers/{{RID}}/graph.orgContact"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.servicePrincipal",
        name: get_directory_object_item_as_service_principal_type,
        path: "/transitiveMembers/{{RID}}/graph.servicePrincipal"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.user",
        name: get_directory_object_item_as_user_type,
        path: "/transitiveMembers/{{RID}}/graph.user"
    );
}
