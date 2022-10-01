// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(DirectoryObjectsRequest,);
register_client!(DirectoryObjectsIdRequest, ());

impl<'a, Client> DirectoryObjectsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Add new entity to directoryObjects",
        name: create_directory_object,
        response: serde_json::Value,
        path: "/directoryObjects",
        has_body: true
    });
    get!({
        doc: "Get entities from directoryObjects",
        name: list_directory_object,
        response: serde_json::Value,
        path: "/directoryObjects",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_directory_objects_count,
        response: serde_json::Value,
        path: "/directoryObjects/$count",
        has_body: false
    });
    post!({
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        response: serde_json::Value,
        path: "/directoryObjects/microsoft.graph.getAvailableExtensionProperties",
        has_body: true
    });
    post!({
        doc: "Invoke action getByIds",
        name: get_by_ids,
        response: serde_json::Value,
        path: "/directoryObjects/microsoft.graph.getByIds",
        has_body: true
    });
    post!({
        doc: "Invoke action validateProperties",
        name: validate_properties,
        response: NoContent,
        path: "/directoryObjects/microsoft.graph.validateProperties",
        has_body: true
    });
}

impl<'a, Client> DirectoryObjectsIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    delete!({
        doc: "Delete directoryObject",
        name: delete_directory_object,
        response: NoContent,
        path: "/directoryObjects/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get directoryObject",
        name: get_directory_object,
        response: serde_json::Value,
        path: "/directoryObjects/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update entity in directoryObjects",
        name: update_directory_object,
        response: NoContent,
        path: "/directoryObjects/{{RID}}",
        has_body: true
    });
    post!({
        doc: "Invoke action checkMemberGroups",
        name: check_member_groups,
        response: serde_json::Value,
        path: "/directoryObjects/{{RID}}/microsoft.graph.checkMemberGroups",
        has_body: true
    });
    post!({
        doc: "Invoke action checkMemberObjects",
        name: check_member_objects,
        response: serde_json::Value,
        path: "/directoryObjects/{{RID}}/microsoft.graph.checkMemberObjects",
        has_body: true
    });
    post!({
        doc: "Invoke action getMemberGroups",
        name: get_member_groups,
        response: serde_json::Value,
        path: "/directoryObjects/{{RID}}/microsoft.graph.getMemberGroups",
        has_body: true
    });
    post!({
        doc: "Invoke action getMemberObjects",
        name: get_member_objects,
        response: serde_json::Value,
        path: "/directoryObjects/{{RID}}/microsoft.graph.getMemberObjects",
        has_body: true
    });
    post!({
        doc: "Invoke action restore",
        name: restore,
        response: serde_json::Value,
        path: "/directoryObjects/{{RID}}/microsoft.graph.restore",
        has_body: false
    });
}
