// GENERATED CODE

use crate::administrative_units_members::{
    AdministrativeUnitsMembersIdRequest, AdministrativeUnitsMembersRequest,
};
use crate::api_default_imports::*;
use graph_http::types::DeltaPhantom;
use graph_http::types::NoContent;

register_client!(DirectoryRolesRequest,);
register_client!(DirectoryRolesIdRequest, ());

impl<'a, Client> DirectoryRolesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Activate directoryRole",
        name: create_directory_role,
        response: serde_json::Value,
        path: "/directoryRoles",
        has_body: true
    });
    get!({
        doc: "List directoryRoles",
        name: list_directory_role,
        response: serde_json::Value,
        path: "/directoryRoles",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_directory_roles_count,
        response: serde_json::Value,
        path: "/directoryRoles/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/directoryRoles/microsoft.graph.delta()",
        has_body: false
    });
    post!({
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        response: serde_json::Value,
        path: "/directoryRoles/microsoft.graph.getAvailableExtensionProperties",
        has_body: true
    });
    post!({
        doc: "Invoke action getByIds",
        name: get_by_ids,
        response: serde_json::Value,
        path: "/directoryRoles/microsoft.graph.getByIds",
        has_body: true
    });
    post!({
        doc: "Invoke action validateProperties",
        name: validate_properties,
        response: NoContent,
        path: "/directoryRoles/microsoft.graph.validateProperties",
        has_body: true
    });
}

impl<'a, Client> DirectoryRolesIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn members(&self) -> AdministrativeUnitsMembersRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client
            .set_ident(ResourceIdentity::AdministrativeUnitsMembers);
        AdministrativeUnitsMembersRequest::new(self.client)
    }

    pub fn member<ID: AsRef<str>>(
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
        doc: "Delete entity from directoryRoles",
        name: delete_directory_role,
        response: NoContent,
        path: "/directoryRoles/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get directoryRole",
        name: get_directory_role,
        response: serde_json::Value,
        path: "/directoryRoles/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update entity in directoryRoles",
        name: update_directory_role,
        response: NoContent,
        path: "/directoryRoles/{{RID}}",
        has_body: true
    });
    post!({
        doc: "Invoke action checkMemberGroups",
        name: check_member_groups,
        response: serde_json::Value,
        path: "/directoryRoles/{{RID}}/microsoft.graph.checkMemberGroups",
        has_body: true
    });
    post!({
        doc: "Invoke action checkMemberObjects",
        name: check_member_objects,
        response: serde_json::Value,
        path: "/directoryRoles/{{RID}}/microsoft.graph.checkMemberObjects",
        has_body: true
    });
    post!({
        doc: "Invoke action getMemberGroups",
        name: get_member_groups,
        response: serde_json::Value,
        path: "/directoryRoles/{{RID}}/microsoft.graph.getMemberGroups",
        has_body: true
    });
    post!({
        doc: "Invoke action getMemberObjects",
        name: get_member_objects,
        response: serde_json::Value,
        path: "/directoryRoles/{{RID}}/microsoft.graph.getMemberObjects",
        has_body: true
    });
    post!({
        doc: "Invoke action restore",
        name: restore,
        response: serde_json::Value,
        path: "/directoryRoles/{{RID}}/microsoft.graph.restore",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to scopedMembers for directoryRoles",
        name: create_scoped_members,
        response: serde_json::Value,
        path: "/directoryRoles/{{RID}}/scopedMembers",
        has_body: true
    });
    get!({
        doc: "Get scopedMembers from directoryRoles",
        name: list_scoped_members,
        response: serde_json::Value,
        path: "/directoryRoles/{{RID}}/scopedMembers",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_scoped_members_count,
        response: serde_json::Value,
        path: "/directoryRoles/{{RID}}/scopedMembers/$count",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property scopedMembers for directoryRoles",
        name: delete_scoped_members,
        response: NoContent,
        path: "/directoryRoles/{{RID}}/scopedMembers/{{id}}",
        params: [ scoped_role_membership_id ],
        has_body: false
    });
    get!({
        doc: "Get scopedMembers from directoryRoles",
        name: get_scoped_members,
        response: serde_json::Value,
        path: "/directoryRoles/{{RID}}/scopedMembers/{{id}}",
        params: [ scoped_role_membership_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property scopedMembers in directoryRoles",
        name: update_scoped_members,
        response: NoContent,
        path: "/directoryRoles/{{RID}}/scopedMembers/{{id}}",
        params: [ scoped_role_membership_id ],
        has_body: true
    });
}
