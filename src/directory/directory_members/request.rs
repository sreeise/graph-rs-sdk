// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    DirectoryMembersApiClient,
    DirectoryMembersIdApiClient,
    ResourceIdentity::DirectoryMembers
);

impl DirectoryMembersApiClient {
    get!(
        doc: "Get members from directory",
        name: list_members,
        path: "/members"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_members_count,
        path: "/members/$count"
    );
    post!(
        doc: "Add a member",
        name: create_ref_members,
        path: "/members/$ref",
        body: true
    );
    get!(
        doc: "Get ref of members from directory",
        name: list_ref_members,
        path: "/members/$ref"
    );
    get!(
        doc: "Get the items of type microsoft.graph.application in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_application_type,
        path: "/members/graph.application"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_application_count,
        path: "/members/graph.application/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.device in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_device_type,
        path: "/members/graph.device"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_count,
        path: "/members/graph.device/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.group in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_group_type,
        path: "/members/graph.group"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_group_count,
        path: "/members/graph.group/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.orgContact in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_org_contact_type,
        path: "/members/graph.orgContact"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_org_contact_count,
        path: "/members/graph.orgContact/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.servicePrincipal in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_service_principal_type,
        path: "/members/graph.servicePrincipal"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_service_principal_count,
        path: "/members/graph.servicePrincipal/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.user in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_user_type,
        path: "/members/graph.user"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_user_count,
        path: "/members/graph.user/$count"
    );
}

impl DirectoryMembersIdApiClient {
    delete!(
        doc: "Delete ref of navigation property members for directory",
        name: delete_ref_members,
        path: "/members/{{RID}}/$ref"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.application",
        name: get_directory_object_item_as_application_type,
        path: "/members/{{RID}}/graph.application"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.device",
        name: get_directory_object_item_as_device_type,
        path: "/members/{{RID}}/graph.device"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.group",
        name: get_directory_object_item_as_group_type,
        path: "/members/{{RID}}/graph.group"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.orgContact",
        name: get_directory_object_item_as_org_contact_type,
        path: "/members/{{RID}}/graph.orgContact"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.servicePrincipal",
        name: get_directory_object_item_as_service_principal_type,
        path: "/members/{{RID}}/graph.servicePrincipal"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.user",
        name: get_directory_object_item_as_user_type,
        path: "/members/{{RID}}/graph.user"
    );
}
