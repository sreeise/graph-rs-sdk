// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(AdministrativeUnitsMembersRequest,);
register_client!(AdministrativeUnitsMembersIdRequest, ());

impl<'a, Client> AdministrativeUnitsMembersRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "Get members from directory",
        name: list_members,
        response: serde_json::Value,
        path: "/members",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_members_count,
        response: serde_json::Value,
        path: "/members/$count",
        has_body: false
    });
    post!({
        doc: "Create new navigation property ref to members for directory",
        name: create_ref_members,
        response: NoContent,
        path: "/members/$ref",
        has_body: true
    });
    get!({
        doc: "Get ref of members from directory",
        name: list_ref_members,
        response: serde_json::Value,
        path: "/members/$ref",
        has_body: false
    });
    get!({
        doc: "Get the items of type microsoft.graph.application in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_application_type,
        response: serde_json::Value,
        path: "/members/microsoft.graph.application",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_application_count,
        response: serde_json::Value,
        path: "/members/microsoft.graph.application/$count",
        has_body: false
    });
    get!({
        doc: "Get the items of type microsoft.graph.device in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_device_type,
        response: serde_json::Value,
        path: "/members/microsoft.graph.device",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_device_count,
        response: serde_json::Value,
        path: "/members/microsoft.graph.device/$count",
        has_body: false
    });
    get!({
        doc: "Get the items of type microsoft.graph.group in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_group_type,
        response: serde_json::Value,
        path: "/members/microsoft.graph.group",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_group_count,
        response: serde_json::Value,
        path: "/members/microsoft.graph.group/$count",
        has_body: false
    });
    get!({
        doc: "Get the items of type microsoft.graph.orgContact in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_org_contact_type,
        response: serde_json::Value,
        path: "/members/microsoft.graph.orgContact",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_org_contact_count,
        response: serde_json::Value,
        path: "/members/microsoft.graph.orgContact/$count",
        has_body: false
    });
    get!({
        doc: "Get the items of type microsoft.graph.servicePrincipal in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_service_principal_type,
        response: serde_json::Value,
        path: "/members/microsoft.graph.servicePrincipal",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_service_principal_count,
        response: serde_json::Value,
        path: "/members/microsoft.graph.servicePrincipal/$count",
        has_body: false
    });
    get!({
        doc: "Get the items of type microsoft.graph.user in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_user_type,
        response: serde_json::Value,
        path: "/members/microsoft.graph.user",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_user_count,
        response: serde_json::Value,
        path: "/members/microsoft.graph.user/$count",
        has_body: false
    });
}

impl<'a, Client> AdministrativeUnitsMembersIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    delete!({
        doc: "Delete ref of navigation property members for directory",
        name: delete_ref_members,
        response: NoContent,
        path: "/members/{{RID}}/$ref",
        has_body: false
    });
    get!({
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.application",
        name: get_directory_object_items_as_application_type,
        response: serde_json::Value,
        path: "/members/{{RID}}/microsoft.graph.application",
        has_body: false
    });
    get!({
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.device",
        name: get_directory_object_items_as_device_type,
        response: serde_json::Value,
        path: "/members/{{RID}}/microsoft.graph.device",
        has_body: false
    });
    get!({
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.group",
        name: get_directory_object_items_as_group_type,
        response: serde_json::Value,
        path: "/members/{{RID}}/microsoft.graph.group",
        has_body: false
    });
    get!({
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.orgContact",
        name: get_directory_object_items_as_org_contact_type,
        response: serde_json::Value,
        path: "/members/{{RID}}/microsoft.graph.orgContact",
        has_body: false
    });
    get!({
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.servicePrincipal",
        name: get_directory_object_items_as_service_principal_type,
        response: serde_json::Value,
        path: "/members/{{RID}}/microsoft.graph.servicePrincipal",
        has_body: false
    });
    get!({
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.user",
        name: get_directory_object_items_as_user_type,
        response: serde_json::Value,
        path: "/members/{{RID}}/microsoft.graph.user",
        has_body: false
    });
}
