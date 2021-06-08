use crate::client::Graph;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(InsightsRequest,);
register_client!(SharedRequest,);
register_client!(TrendingRequest,);
register_client!(UsedRequest,);

impl<'a, Client> InsightsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get shared from me",
        name: list_shared,
        response: serde_json::Value,
        path: "/insights/shared",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to shared for me",
        name: create_shared,
        response: serde_json::Value,
        path: "/insights/shared",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get shared from me",
        name: get_shared,
        response: serde_json::Value,
        path: "/insights/shared/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property shared in me",
        name: update_shared,
        response: NoContent,
        path: "/insights/shared/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get trending from me",
        name: list_trending,
        response: serde_json::Value,
        path: "/insights/trending",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to trending for me",
        name: create_trending,
        response: serde_json::Value,
        path: "/insights/trending",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get trending from me",
        name: get_trending,
        response: serde_json::Value,
        path: "/insights/trending/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property trending in me",
        name: update_trending,
        response: NoContent,
        path: "/insights/trending/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get used from me",
        name: list_used,
        response: serde_json::Value,
        path: "/insights/used",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to used for me",
        name: create_used,
        response: serde_json::Value,
        path: "/insights/used",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get used from me",
        name: get_used,
        response: serde_json::Value,
        path: "/insights/used/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property used in me",
        name: update_used,
        response: NoContent,
        path: "/insights/used/{{id}}",
        params: 1,
        has_body: true
    });

    pub fn shared(&self) -> SharedRequest<'a, Client> {
        SharedRequest::new(self.client)
    }

    pub fn trending(&self) -> TrendingRequest<'a, Client> {
        TrendingRequest::new(self.client)
    }

    pub fn used(&self) -> UsedRequest<'a, Client> {
        UsedRequest::new(self.client)
    }
}

impl<'a, Client> SharedRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get lastSharedMethod from me",
        name: get_last_shared_method,
        response: serde_json::Value,
        path: "/insights/shared/{{id}}/lastSharedMethod",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get resource from me",
        name: get_resource,
        response: serde_json::Value,
        path: "/insights/shared/{{id}}/resource",
        params: 1,
        has_body: false
    });
}

impl<'a, Client> TrendingRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get resource from me",
        name: get_resource,
        response: serde_json::Value,
        path: "/insights/trending/{{id}}/resource",
        params: 1,
        has_body: false
    });
}

impl<'a, Client> UsedRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get resource from me",
        name: get_resource,
        response: serde_json::Value,
        path: "/insights/used/{{id}}/resource",
        params: 1,
        has_body: false
    });
}
