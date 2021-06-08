use crate::client::Graph;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(ContractsRequest,);

impl<'a, Client> ContractsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
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
        response: NoContent,
        path: "/contracts/{{id}}",
        params: 1,
        has_body: true
    });

    delete!({
        doc: "# Delete entity from contracts",
        name: delete_contract,
        response: NoContent,
        path: "/contracts/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get entities from contracts",
        name: list_contract,
        response: serde_json::Value,
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
}
