use crate::client::Graph;
use graph_http::types::DeltaPhantom;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(DirectoryRequest,);
register_client!(AdministrativeUnitsRequest,);
register_client!(DirectoryObjectsRequest,);
register_client!(DirectoryRoleTemplatesRequest,);
register_client!(DirectoryRolesRequest,);

impl<'a, Client> DirectoryRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get administrativeUnits from directory",
        name: list_administrative_units,
        response: serde_json::Value,
        path: "/directory/administrativeUnits",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to administrativeUnits for directory",
        name: create_administrative_units,
        response: serde_json::Value,
        path: "/directory/administrativeUnits",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get deletedItems from directory",
        name: get_deleted_items,
        response: serde_json::Value,
        path: "/directory/deletedItems/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deletedItems in directory",
        name: update_deleted_items,
        response: NoContent,
        path: "/directory/deletedItems/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get directory",
        name: get_directory,
        response: serde_json::Value,
        path: "/directory",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update directory",
        name: update_directory,
        response: NoContent,
        path: "/directory",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get administrativeUnits from directory",
        name: get_administrative_units,
        response: serde_json::Value,
        path: "/directory/administrativeUnits/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property administrativeUnits in directory",
        name: update_administrative_units,
        response: NoContent,
        path: "/directory/administrativeUnits/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get deletedItems from directory",
        name: list_deleted_items,
        response: serde_json::Value,
        path: "/directory/deletedItems",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to deletedItems for directory",
        name: create_deleted_items,
        response: serde_json::Value,
        path: "/directory/deletedItems",
        params: 0,
        has_body: true
    });

    pub fn administrative_units(&self) -> AdministrativeUnitsRequest<'a, Client> {
        AdministrativeUnitsRequest::new(&self.client)
    }

    pub fn directory_roles(&self) -> DirectoryRolesRequest<'a, Client> {
        DirectoryRolesRequest::new(&self.client)
    }

    pub fn directory_objects(&self) -> DirectoryObjectsRequest<'a, Client> {
        DirectoryObjectsRequest::new(&self.client)
    }

    pub fn directory_role_templates(&self) -> DirectoryRoleTemplatesRequest<'a, Client> {
        DirectoryRoleTemplatesRequest::new(&self.client)
    }
}

impl<'a, Client> AdministrativeUnitsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get members from directory",
        name: list_members,
        response: serde_json::Value,
        path: "/directory/administrativeUnits/{{id}}/members",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get scopedRoleMembers from directory",
        name: get_scoped_role_members,
        response: serde_json::Value,
        path: "/directory/administrativeUnits/{{id}}/scopedRoleMembers/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property scopedRoleMembers in directory",
        name: update_scoped_role_members,
        response: NoContent,
        path: "/directory/administrativeUnits/{{id}}/scopedRoleMembers/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get extensions from directory",
        name: get_extensions,
        response: serde_json::Value,
        path: "/directory/administrativeUnits/{{id}}/extensions/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property extensions in directory",
        name: update_extensions,
        response: NoContent,
        path: "/directory/administrativeUnits/{{id}}/extensions/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get members from directory",
        name: get_members,
        response: serde_json::Value,
        path: "/directory/administrativeUnits/{{id}}/members/{{id2}}",
        params: 2,
        has_body: false
    });

    get!({
        doc: "# Get scopedRoleMembers from directory",
        name: list_scoped_role_members,
        response: serde_json::Value,
        path: "/directory/administrativeUnits/{{id}}/scopedRoleMembers",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to scopedRoleMembers for directory",
        name: create_scoped_role_members,
        response: serde_json::Value,
        path: "/directory/administrativeUnits/{{id}}/scopedRoleMembers",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get extensions from directory",
        name: list_extensions,
        response: serde_json::Value,
        path: "/directory/administrativeUnits/{{id}}/extensions",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to extensions for directory",
        name: create_extensions,
        response: serde_json::Value,
        path: "/directory/administrativeUnits/{{id}}/extensions",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> DirectoryObjectsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entities from directoryObjects",
        name: list_directory_object,
        response: serde_json::Value,
        path: "/directoryObjects",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Add new entity to directoryObjects",
        name: create_directory_object,
        response: serde_json::Value,
        path: "/directoryObjects",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get entity from directoryObjects by key",
        name: get_directory_object,
        response: serde_json::Value,
        path: "/directoryObjects/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update entity in directoryObjects",
        name: update_directory_object,
        response: NoContent,
        path: "/directoryObjects/{{id}}",
        params: 1,
        has_body: true
    });

    delete!({
        doc: "# Delete entity from directoryObjects",
        name: delete_directory_object,
        response: NoContent,
        path: "/directoryObjects/{{id}}",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action checkMemberGroups",
        name: check_member_groups,
        response: serde_json::Value,
        path: "/directoryObjects/{{id}}/checkMemberGroups",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action getMemberObjects",
        name: get_member_objects,
        response: serde_json::Value,
        path: "/directoryObjects/{{id}}/getMemberObjects",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action validateProperties",
        name: validate_properties,
        response: NoContent,
        path: "/directoryObjects/validateProperties",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action checkMemberObjects",
        name: check_member_objects,
        response: serde_json::Value,
        path: "/directoryObjects/{{id}}/checkMemberObjects",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action restore",
        name: restore,
        response: serde_json::Value,
        path: "/directoryObjects/{{id}}/restore",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action getByIds",
        name: get_by_ids,
        response: serde_json::Value,
        path: "/directoryObjects/getByIds",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action getMemberGroups",
        name: get_member_groups,
        response: serde_json::Value,
        path: "/directoryObjects/{{id}}/getMemberGroups",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        response: serde_json::Value,
        path: "/directoryObjects/getAvailableExtensionProperties",
        params: 0,
        has_body: true
    });

    pub fn administrative_units(&self) -> AdministrativeUnitsRequest<'a, Client> {
        AdministrativeUnitsRequest::new(&self.client)
    }
}

impl<'a, Client> AdministrativeUnitsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/directory/administrativeUnits/delta()",
        params: 0,
        has_body: false
    });
}

impl<'a, Client> DirectoryRoleTemplatesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entities from directoryRoleTemplates",
        name: list_directory_role_template,
        response: serde_json::Value,
        path: "/directoryRoleTemplates",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Add new entity to directoryRoleTemplates",
        name: create_directory_role_template,
        response: serde_json::Value,
        path: "/directoryRoleTemplates",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get entity from directoryRoleTemplates by key",
        name: get_directory_role_template,
        response: serde_json::Value,
        path: "/directoryRoleTemplates/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update entity in directoryRoleTemplates",
        name: update_directory_role_template,
        response: NoContent,
        path: "/directoryRoleTemplates/{{id}}",
        params: 1,
        has_body: true
    });

    delete!({
        doc: "# Delete entity from directoryRoleTemplates",
        name: delete_directory_role_template,
        response: NoContent,
        path: "/directoryRoleTemplates/{{id}}",
        params: 1,
        has_body: false
    });
}

impl<'a, Client> DirectoryRolesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entities from directoryRoles",
        name: list_directory_role,
        response: serde_json::Value,
        path: "/directoryRoles",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Add new entity to directoryRoles",
        name: create_directory_role,
        response: serde_json::Value,
        path: "/directoryRoles",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get scopedMembers from directoryRoles",
        name: get_scoped_members,
        response: serde_json::Value,
        path: "/directoryRoles/{{id}}/scopedMembers/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property scopedMembers in directoryRoles",
        name: update_scoped_members,
        response: NoContent,
        path: "/directoryRoles/{{id}}/scopedMembers/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/directoryRoles/delta()",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get members from directoryRoles",
        name: list_members,
        response: serde_json::Value,
        path: "/directoryRoles/{{id}}/members",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get scopedMembers from directoryRoles",
        name: list_scoped_members,
        response: serde_json::Value,
        path: "/directoryRoles/{{id}}/scopedMembers",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to scopedMembers for directoryRoles",
        name: create_scoped_members,
        response: serde_json::Value,
        path: "/directoryRoles/{{id}}/scopedMembers",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get entity from directoryRoles by key",
        name: get_directory_role,
        response: serde_json::Value,
        path: "/directoryRoles/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update entity in directoryRoles",
        name: update_directory_role,
        response: NoContent,
        path: "/directoryRoles/{{id}}",
        params: 1,
        has_body: true
    });

    delete!({
        doc: "# Delete entity from directoryRoles",
        name: delete_directory_role,
        response: NoContent,
        path: "/directoryRoles/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get members from directoryRoles",
        name: get_members,
        response: serde_json::Value,
        path: "/directoryRoles/{{id}}/members/{{id2}}",
        params: 2,
        has_body: false
    });
}
