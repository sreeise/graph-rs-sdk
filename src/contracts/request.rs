use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::{GraphResponse, IntoResponse};
use reqwest::Method;

register_client!(ContractsRequest,);

#[allow(dead_code)]
impl<'a, Client> ContractsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action getMemberGroups",
        name: get_member_groups,
        response: Collection<serde_json::Value>,
        path: "/contracts/{{id}}/microsoft.graph.getMemberGroups",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get entities from contracts",
        name: list_contract,
        response: Collection<serde_json::Value>,
        path: "/contracts",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Add new entity to contracts",
        name: create_contract,
        response: serde_json::Value,
        path: "/contracts",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action checkMemberGroups",
        name: check_member_groups,
        response: Collection<serde_json::Value>,
        path: "/contracts/{{id}}/microsoft.graph.checkMemberGroups",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action getByIds",
        name: get_by_ids,
        response: Collection<serde_json::Value>,
        path: "/contracts/microsoft.graph.getByIds",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get entity from contracts by key",
        name: get_contract,
        response: serde_json::Value,
        path: "/contracts/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update entity in contracts",
        name: update_contract,
        response: GraphResponse<Content>,
        path: "/contracts/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from contracts",
        name: delete_contract,
        response: GraphResponse<Content>,
        path: "/contracts/{{id}}",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action checkMemberObjects",
        name: check_member_objects,
        response: Collection<serde_json::Value>,
        path: "/contracts/{{id}}/microsoft.graph.checkMemberObjects",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action getMemberObjects",
        name: get_member_objects,
        response: Collection<serde_json::Value>,
        path: "/contracts/{{id}}/microsoft.graph.getMemberObjects",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action restore",
        name: restore,
        response: serde_json::Value,
        path: "/contracts/{{id}}/microsoft.graph.restore",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action validateProperties",
        name: validate_properties,
        response: GraphResponse<Content>,
        path: "/contracts/microsoft.graph.validateProperties",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        response: Collection<serde_json::Value>,
        path: "/contracts/microsoft.graph.getAvailableExtensionProperties",
        params: 0,
        has_body: true
    });
}
