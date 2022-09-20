// GENERATED CODE

use crate::administrative_units_members::{
    AdministrativeUnitsMembersIdRequest, AdministrativeUnitsMembersRequest,
};
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
        name: get_administrative_units_count,
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
    pub fn administrative_units_members(&self) -> AdministrativeUnitsMembersRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client
            .set_ident(ResourceIdentity::AdministrativeUnitsMembers);
        AdministrativeUnitsMembersRequest::new(self.client)
    }

    pub fn administrative_units_member<ID: AsRef<str>>(
        &self,
        id: ID,
    ) -> AdministrativeUnitsMembersIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client
            .set_ident(ResourceIdentity::AdministrativeUnitsMembers);
        AdministrativeUnitsMembersIdRequest::new(id.as_ref(), self.client)
    }

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
        name: get_extensions_count,
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
        name: get_scoped_role_members_count,
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
