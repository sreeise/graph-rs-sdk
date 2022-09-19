// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::DeltaPhantom;
use graph_http::types::NoContent;

register_client!(AdministrativeUnitsRequest,);
register_client!(AdministrativeUnitsIdRequest, ());

impl<'a, Client> AdministrativeUnitsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Create new navigation property to administrativeUnits for directory",
        name: create_administrative_units,
        response: serde_json::Value,
        path: "/administrativeUnits",
        has_body: true
    });
    get!({
        doc: "Get administrativeUnits from directory",
        name: list_administrative_units,
        response: serde_json::Value,
        path: "/administrativeUnits",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/administrativeUnits/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/administrativeUnits/microsoft.graph.delta()",
        has_body: false
    });
}

impl<'a, Client> AdministrativeUnitsIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    delete!({
        doc: "Delete navigation property administrativeUnits for directory",
        name: delete_administrative_units,
        response: NoContent,
        path: "/administrativeUnits/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get administrativeUnits from directory",
        name: get_administrative_units,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property administrativeUnits in directory",
        name: update_administrative_units,
        response: NoContent,
        path: "/administrativeUnits/{{RID}}",
        has_body: true
    });
    post!({
        doc: "Create new navigation property to extensions for directory",
        name: create_extensions,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/extensions",
        has_body: true
    });
    get!({
        doc: "Get extensions from directory",
        name: list_extensions,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/extensions",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/extensions/$count",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property extensions for directory",
        name: delete_extensions,
        response: NoContent,
        path: "/administrativeUnits/{{RID}}/extensions/{{id}}",
        params: [ extension_id ],
        has_body: false
    });
    get!({
        doc: "Get extensions from directory",
        name: get_extensions,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/extensions/{{id}}",
        params: [ extension_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property extensions in directory",
        name: update_extensions,
        response: NoContent,
        path: "/administrativeUnits/{{RID}}/extensions/{{id}}",
        params: [ extension_id ],
        has_body: true
    });
    get!({
        doc: "Get members from directory",
        name: list_members,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/$count",
        has_body: false
    });
    post!({
        doc: "Create new navigation property ref to members for directory",
        name: create_ref_members,
        response: NoContent,
        path: "/administrativeUnits/{{RID}}/members/$ref",
        has_body: true
    });
    get!({
        doc: "Get ref of members from directory",
        name: list_ref_members,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/$ref",
        has_body: false
    });
    get!({
        doc: "Get the items of type microsoft.graph.application in the microsoft.graph.directoryObject collection",
        name: graph,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/microsoft.graph.application",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: graph,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/microsoft.graph.application/$count",
        has_body: false
    });
    get!({
        doc: "Get the items of type microsoft.graph.device in the microsoft.graph.directoryObject collection",
        name: graph,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/microsoft.graph.device",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: graph,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/microsoft.graph.device/$count",
        has_body: false
    });
    get!({
        doc: "Get the items of type microsoft.graph.group in the microsoft.graph.directoryObject collection",
        name: graph,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/microsoft.graph.group",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: graph,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/microsoft.graph.group/$count",
        has_body: false
    });
    get!({
        doc: "Get the items of type microsoft.graph.orgContact in the microsoft.graph.directoryObject collection",
        name: graph,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/microsoft.graph.orgContact",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: graph,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/microsoft.graph.orgContact/$count",
        has_body: false
    });
    get!({
        doc: "Get the items of type microsoft.graph.servicePrincipal in the microsoft.graph.directoryObject collection",
        name: graph,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/microsoft.graph.servicePrincipal",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: graph,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/microsoft.graph.servicePrincipal/$count",
        has_body: false
    });
    get!({
        doc: "Get the items of type microsoft.graph.user in the microsoft.graph.directoryObject collection",
        name: graph,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/microsoft.graph.user",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: graph,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/microsoft.graph.user/$count",
        has_body: false
    });
    delete!({
        doc: "Delete ref of navigation property members for directory",
        name: delete_ref_members,
        response: NoContent,
        path: "/administrativeUnits/{{RID}}/members/{{id}}/$ref",
        params: [ directory_object_id ],
        has_body: false
    });
    get!({
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.application",
        name: graph,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/{{id}}/microsoft.graph.application",
        params: [ directory_object_id ],
        has_body: false
    });
    get!({
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.device",
        name: graph,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/{{id}}/microsoft.graph.device",
        params: [ directory_object_id ],
        has_body: false
    });
    get!({
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.group",
        name: graph,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/{{id}}/microsoft.graph.group",
        params: [ directory_object_id ],
        has_body: false
    });
    get!({
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.orgContact",
        name: graph,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/{{id}}/microsoft.graph.orgContact",
        params: [ directory_object_id ],
        has_body: false
    });
    get!({
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.servicePrincipal",
        name: graph,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/{{id}}/microsoft.graph.servicePrincipal",
        params: [ directory_object_id ],
        has_body: false
    });
    get!({
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.user",
        name: graph,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/members/{{id}}/microsoft.graph.user",
        params: [ directory_object_id ],
        has_body: false
    });
    post!({
        doc: "Create new navigation property to scopedRoleMembers for directory",
        name: create_scoped_role_members,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/scopedRoleMembers",
        has_body: true
    });
    get!({
        doc: "Get scopedRoleMembers from directory",
        name: list_scoped_role_members,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/scopedRoleMembers",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/scopedRoleMembers/$count",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property scopedRoleMembers for directory",
        name: delete_scoped_role_members,
        response: NoContent,
        path: "/administrativeUnits/{{RID}}/scopedRoleMembers/{{id}}",
        params: [ scoped_role_membership_id ],
        has_body: false
    });
    get!({
        doc: "Get scopedRoleMembers from directory",
        name: get_scoped_role_members,
        response: serde_json::Value,
        path: "/administrativeUnits/{{RID}}/scopedRoleMembers/{{id}}",
        params: [ scoped_role_membership_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property scopedRoleMembers in directory",
        name: update_scoped_role_members,
        response: NoContent,
        path: "/administrativeUnits/{{RID}}/scopedRoleMembers/{{id}}",
        params: [ scoped_role_membership_id ],
        has_body: true
    });
}
