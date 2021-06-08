// GENERATED CODE

use crate::client::Graph;
use crate::core::ResourceIdentity;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(SessionRequest,);
register_client!(SessionsRequest, ());

impl<'a, Client> SessionRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get sessions from communications",
        name: list_sessions,
        response: serde_json::Value,
        path: "/sessions",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to sessions for communications",
        name: create_sessions,
        response: serde_json::Value,
        path: "/sessions",
        params: 0,
        has_body: true
    });

    pub fn id<ID: AsRef<str>>(&self, id: ID) -> SessionsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Sessions);
        SessionsRequest::new(id.as_ref(), self.client)
    }
}

impl<'a, Client> SessionsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get sessions from communications",
        name: get_sessions,
        response: serde_json::Value,
        path: "/sessions/{{RID}}",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property sessions in communications",
        name: update_sessions,
        response: NoContent,
        path: "/sessions/{{RID}}",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get segments from communications",
        name: list_segments,
        response: serde_json::Value,
        path: "/sessions/{{RID}}/segments",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to segments for communications",
        name: create_segments,
        response: serde_json::Value,
        path: "/sessions/{{RID}}/segments",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get segments from communications",
        name: get_segments,
        response: serde_json::Value,
        path: "/sessions/{{RID}}/segments/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property segments in communications",
        name: update_segments,
        response: NoContent,
        path: "/sessions/{{RID}}/segments/{{id}}",
        params: 1,
        has_body: true
    });
}
