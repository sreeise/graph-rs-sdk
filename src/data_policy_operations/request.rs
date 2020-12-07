// GENERATED CODE

use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(DataPolicyOperationsRequest,);

impl<'a, Client> DataPolicyOperationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entities from dataPolicyOperations",
        name: list_data_policy_operation,
        response: Collection<serde_json::Value>,
        path: "/dataPolicyOperations",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Add new entity to dataPolicyOperations",
        name: create_data_policy_operation,
        response: serde_json::Value,
        path: "/dataPolicyOperations",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get entity from dataPolicyOperations by key",
        name: get_data_policy_operation,
        response: serde_json::Value,
        path: "/dataPolicyOperations/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update entity in dataPolicyOperations",
        name: update_data_policy_operation,
        response: GraphResponse<Content>,
        path: "/dataPolicyOperations/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from dataPolicyOperations",
        name: delete_data_policy_operation,
        response: GraphResponse<Content>,
        path: "/dataPolicyOperations/{{id}}",
        params: 1,
        has_body: false
    });
}
