// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(DirectoryRoleTemplatesRequest,);
register_client!(DirectoryRoleTemplatesIdRequest, ());

impl<'a, Client> DirectoryRoleTemplatesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Add new entity to directoryRoleTemplates",
        name: create_directory_role_template,
        response: serde_json::Value,
        path: "/directoryRoleTemplates",
        has_body: true
    });
    get!({
        doc: "List directoryRoleTemplates",
        name: list_directory_role_template,
        response: serde_json::Value,
        path: "/directoryRoleTemplates",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_directory_role_templates_count,
        response: serde_json::Value,
        path: "/directoryRoleTemplates/$count",
        has_body: false
    });
    post!({
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        response: serde_json::Value,
        path: "/directoryRoleTemplates/microsoft.graph.getAvailableExtensionProperties",
        has_body: true
    });
    post!({
        doc: "Invoke action getByIds",
        name: get_by_ids,
        response: serde_json::Value,
        path: "/directoryRoleTemplates/microsoft.graph.getByIds",
        has_body: true
    });
    post!({
        doc: "Invoke action validateProperties",
        name: validate_properties,
        response: NoContent,
        path: "/directoryRoleTemplates/microsoft.graph.validateProperties",
        has_body: true
    });
}

impl<'a, Client> DirectoryRoleTemplatesIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    delete!({
        doc: "Delete entity from directoryRoleTemplates",
        name: delete_directory_role_template,
        response: NoContent,
        path: "/directoryRoleTemplates/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get directoryRoleTemplate",
        name: get_directory_role_template,
        response: serde_json::Value,
        path: "/directoryRoleTemplates/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update entity in directoryRoleTemplates",
        name: update_directory_role_template,
        response: NoContent,
        path: "/directoryRoleTemplates/{{RID}}",
        has_body: true
    });
    post!({
        doc: "Invoke action checkMemberGroups",
        name: check_member_groups,
        response: serde_json::Value,
        path: "/directoryRoleTemplates/{{RID}}/microsoft.graph.checkMemberGroups",
        has_body: true
    });
    post!({
        doc: "Invoke action checkMemberObjects",
        name: check_member_objects,
        response: serde_json::Value,
        path: "/directoryRoleTemplates/{{RID}}/microsoft.graph.checkMemberObjects",
        has_body: true
    });
    post!({
        doc: "Invoke action getMemberGroups",
        name: get_member_groups,
        response: serde_json::Value,
        path: "/directoryRoleTemplates/{{RID}}/microsoft.graph.getMemberGroups",
        has_body: true
    });
    post!({
        doc: "Invoke action getMemberObjects",
        name: get_member_objects,
        response: serde_json::Value,
        path: "/directoryRoleTemplates/{{RID}}/microsoft.graph.getMemberObjects",
        has_body: true
    });
    post!({
        doc: "Invoke action restore",
        name: restore,
        response: serde_json::Value,
        path: "/directoryRoleTemplates/{{RID}}/microsoft.graph.restore",
        has_body: false
    });
}
