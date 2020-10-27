use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::{GraphResponse, IntoResponse};
use reqwest::Method;

register_client!(GroupLifecyclePolicyRequest,);

#[allow(dead_code)]
impl<'a, Client> GroupLifecyclePolicyRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entity from groupLifecyclePolicies by key",
        name: get_group_lifecycle_policy,
        response: serde_json::Value,
        path: "/groupLifecyclePolicies/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update entity in groupLifecyclePolicies",
        name: update_group_lifecycle_policy,
        response: GraphResponse<Content>,
        path: "/groupLifecyclePolicies/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from groupLifecyclePolicies",
        name: delete_group_lifecycle_policy,
        response: GraphResponse<Content>,
        path: "/groupLifecyclePolicies/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get entities from groupLifecyclePolicies",
        name: list_group_lifecycle_policy,
        response: Collection<serde_json::Value>,
        path: "/groupLifecyclePolicies",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Add new entity to groupLifecyclePolicies",
        name: create_group_lifecycle_policy,
        response: serde_json::Value,
        path: "/groupLifecyclePolicies",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action addGroup",
        name: add_group,
        response: serde_json::Value,
        path: "/groupLifecyclePolicies/{{id}}/addGroup",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action removeGroup",
        name: remove_group,
        response: serde_json::Value,
        path: "/groupLifecyclePolicies/{{id}}/removeGroup",
        params: 1,
        has_body: true
    });
}
