use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::types::DeltaPhantom;
use graph_http::{GraphResponse, IntoResponse};
use reqwest::Method;

register_client!(DirectoryRequest,);
register_client!(DirectoryRoleTemplatesRequest,);
register_client!(DirectoryObjectsRequest,);
register_client!(DirectoryRolesRequest,);
register_client!(AdministrativeUnitRequest,);

#[allow(dead_code)]
impl<'a, Client> DirectoryObjectsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn administrative_unit(&self) -> AdministrativeUnitRequest<'a, Client> {
        AdministrativeUnitRequest::new(&self.client)
    }
    post!({
        doc: "# Invoke action checkMemberGroups",
        name: check_member_groups,
        response: Collection<serde_json::Value>,
        path: "/directoryObjects/{{id}}/microsoft.graph.checkMemberGroups",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action getMemberGroups",
        name: get_member_groups,
        response: Collection<serde_json::Value>,
        path: "/directoryObjects/{{id}}/microsoft.graph.getMemberGroups",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action restore",
        name: restore,
        response: serde_json::Value,
        path: "/directoryObjects/{{id}}/microsoft.graph.restore",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get entities from directoryObjects",
        name: list_directory_object,
        response: Collection<serde_json::Value>,
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
        response: GraphResponse<Content>,
        path: "/directoryObjects/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from directoryObjects",
        name: delete_directory_object,
        response: GraphResponse<Content>,
        path: "/directoryObjects/{{id}}",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        response: Collection<serde_json::Value>,
        path: "/directoryObjects/microsoft.graph.getAvailableExtensionProperties",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action checkMemberObjects",
        name: check_member_objects,
        response: Collection<serde_json::Value>,
        path: "/directoryObjects/{{id}}/microsoft.graph.checkMemberObjects",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action getMemberObjects",
        name: get_member_objects,
        response: Collection<serde_json::Value>,
        path: "/directoryObjects/{{id}}/microsoft.graph.getMemberObjects",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action validateProperties",
        name: validate_properties,
        response: GraphResponse<Content>,
        path: "/directoryObjects/microsoft.graph.validateProperties",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action getByIds",
        name: get_by_ids,
        response: Collection<serde_json::Value>,
        path: "/directoryObjects/microsoft.graph.getByIds",
        params: 0,
        has_body: true
    });
}

#[allow(dead_code)]
impl<'a, Client> DirectoryRoleTemplatesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action getMemberObjects",
        name: get_member_objects,
        response: Collection<serde_json::Value>,
        path: "/directoryRoleTemplates/{{id}}/microsoft.graph.getMemberObjects",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action checkMemberGroups",
        name: check_member_groups,
        response: Collection<serde_json::Value>,
        path: "/directoryRoleTemplates/{{id}}/microsoft.graph.checkMemberGroups",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action getMemberGroups",
        name: get_member_groups,
        response: Collection<serde_json::Value>,
        path: "/directoryRoleTemplates/{{id}}/microsoft.graph.getMemberGroups",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        response: Collection<serde_json::Value>,
        path: "/directoryRoleTemplates/microsoft.graph.getAvailableExtensionProperties",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action getByIds",
        name: get_by_ids,
        response: Collection<serde_json::Value>,
        path: "/directoryRoleTemplates/microsoft.graph.getByIds",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action restore",
        name: restore,
        response: serde_json::Value,
        path: "/directoryRoleTemplates/{{id}}/microsoft.graph.restore",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action validateProperties",
        name: validate_properties,
        response: GraphResponse<Content>,
        path: "/directoryRoleTemplates/microsoft.graph.validateProperties",
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
        response: GraphResponse<Content>,
        path: "/directoryRoleTemplates/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from directoryRoleTemplates",
        name: delete_directory_role_template,
        response: GraphResponse<Content>,
        path: "/directoryRoleTemplates/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get entities from directoryRoleTemplates",
        name: list_directory_role_template,
        response: Collection<serde_json::Value>,
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
    post!({
        doc: "# Invoke action checkMemberObjects",
        name: check_member_objects,
        response: Collection<serde_json::Value>,
        path: "/directoryRoleTemplates/{{id}}/microsoft.graph.checkMemberObjects",
        params: 1,
        has_body: true
    });
}

#[allow(dead_code)]
impl<'a, Client> AdministrativeUnitRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<Collection<serde_json::Value>>,
        path: "/directoryObjects/microsoft.graph.administrativeUnit/microsoft.graph.delta()",
        params: 0,
        has_body: false
    });
}

#[allow(dead_code)]
impl<'a, Client> DirectoryRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn directory_roles(&self) -> DirectoryRolesRequest<'a, Client> {
        DirectoryRolesRequest::new(&self.client)
    }
    pub fn directory_objects(&self) -> DirectoryObjectsRequest<'a, Client> {
        DirectoryObjectsRequest::new(&self.client)
    }
    pub fn directory_role_templates(&self) -> DirectoryRoleTemplatesRequest<'a, Client> {
        DirectoryRoleTemplatesRequest::new(&self.client)
    }
    get!({
        doc: "# Get deletedItems from directory",
        name: list_deleted_items,
        response: Collection<serde_json::Value>,
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
        response: GraphResponse<Content>,
        path: "/directory",
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
        response: GraphResponse<Content>,
        path: "/directory/deletedItems/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property deletedItems for directory",
        name: delete_deleted_items,
        response: GraphResponse<Content>,
        path: "/directory/deletedItems/{{id}}",
        params: 1,
        has_body: false
    });
}

#[allow(dead_code)]
impl<'a, Client> DirectoryRolesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
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
        response: GraphResponse<Content>,
        path: "/directoryRoles/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from directoryRoles",
        name: delete_directory_role,
        response: GraphResponse<Content>,
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
    post!({
        doc: "# Invoke action checkMemberObjects",
        name: check_member_objects,
        response: Collection<serde_json::Value>,
        path: "/directoryRoles/{{id}}/microsoft.graph.checkMemberObjects",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action getMemberGroups",
        name: get_member_groups,
        response: Collection<serde_json::Value>,
        path: "/directoryRoles/{{id}}/microsoft.graph.getMemberGroups",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        response: Collection<serde_json::Value>,
        path: "/directoryRoles/microsoft.graph.getAvailableExtensionProperties",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action getByIds",
        name: get_by_ids,
        response: Collection<serde_json::Value>,
        path: "/directoryRoles/microsoft.graph.getByIds",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get ref of members from directoryRoles",
        name: list_ref_members,
        response: Collection<serde_json::Value>,
        path: "/directoryRoles/{{id}}/members/$ref",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property ref to members for directoryRoles",
        name: create_ref_members,
        response: serde_json::Value,
        path: "/directoryRoles/{{id}}/members/$ref",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete ref of navigation property members for directoryRoles",
        name: delete_ref_members,
        response: GraphResponse<Content>,
        path: "/directoryRoles/{{id}}/members/$ref",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<Collection<serde_json::Value>>,
        path: "/directoryRoles/microsoft.graph.delta()",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action restore",
        name: restore,
        response: serde_json::Value,
        path: "/directoryRoles/{{id}}/microsoft.graph.restore",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action checkMemberGroups",
        name: check_member_groups,
        response: Collection<serde_json::Value>,
        path: "/directoryRoles/{{id}}/microsoft.graph.checkMemberGroups",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action validateProperties",
        name: validate_properties,
        response: GraphResponse<Content>,
        path: "/directoryRoles/microsoft.graph.validateProperties",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get entities from directoryRoles",
        name: list_directory_role,
        response: Collection<serde_json::Value>,
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
        doc: "# Get members from directoryRoles",
        name: list_members,
        response: Collection<serde_json::Value>,
        path: "/directoryRoles/{{id}}/members",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action getMemberObjects",
        name: get_member_objects,
        response: Collection<serde_json::Value>,
        path: "/directoryRoles/{{id}}/microsoft.graph.getMemberObjects",
        params: 1,
        has_body: true
    });
}
