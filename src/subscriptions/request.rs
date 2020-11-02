use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(SubscriptionsRequest,);

impl<'a, Client> SubscriptionsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entity from subscriptions by key",
        name: get_subscription,
        response: serde_json::Value,
        path: "/subscriptions/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update entity in subscriptions",
        name: update_subscription,
        response: GraphResponse<Content>,
        path: "/subscriptions/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from subscriptions",
        name: delete_subscription,
        response: GraphResponse<Content>,
        path: "/subscriptions/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get entities from subscriptions",
        name: list_subscription,
        response: Collection<serde_json::Value>,
        path: "/subscriptions",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Add new entity to subscriptions",
        name: create_subscription,
        response: serde_json::Value,
        path: "/subscriptions",
        params: 0,
        has_body: true
    });
}
