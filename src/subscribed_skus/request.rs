use crate::client::Graph;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(SubscribedSkusRequest,);

impl<'a, Client> SubscribedSkusRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entities from subscribedSkus",
        name: list_subscribed_sku,
        response: serde_json::Value,
        path: "/subscribedSkus",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Add new entity to subscribedSkus",
        name: create_subscribed_sku,
        response: serde_json::Value,
        path: "/subscribedSkus",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get entity from subscribedSkus by key",
        name: get_subscribed_sku,
        response: serde_json::Value,
        path: "/subscribedSkus/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update entity in subscribedSkus",
        name: update_subscribed_sku,
        response: NoContent,
        path: "/subscribedSkus/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from subscribedSkus",
        name: delete_subscribed_sku,
        response: NoContent,
        path: "/subscribedSkus/{{id}}",
        params: 1,
        has_body: false
    });
}
