use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(PlacesRequest,);

impl<'a, Client> PlacesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entity from places by key",
        name: get_place,
        response: serde_json::Value,
        path: "/places/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update entity in places",
        name: update_place,
        response: NoContent,
        path: "/places/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from places",
        name: delete_place,
        response: NoContent,
        path: "/places/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get entities from places",
        name: list_place,
        response: Collection<serde_json::Value>,
        path: "/places",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Add new entity to places",
        name: create_place,
        response: serde_json::Value,
        path: "/places",
        params: 0,
        has_body: true
    });
}
