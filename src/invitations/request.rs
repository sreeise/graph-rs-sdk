use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(InvitationsRequest,);

impl<'a, Client> InvitationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entity from invitations by key",
        name: get_invitation,
        response: serde_json::Value,
        path: "/invitations/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update entity in invitations",
        name: update_invitation,
        response: GraphResponse<Content>,
        path: "/invitations/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from invitations",
        name: delete_invitation,
        response: GraphResponse<Content>,
        path: "/invitations/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get entities from invitations",
        name: list_invitation,
        response: Collection<serde_json::Value>,
        path: "/invitations",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Add new entity to invitations",
        name: create_invitation,
        response: serde_json::Value,
        path: "/invitations",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get invitedUser from invitations",
        name: get_invited_user,
        response: serde_json::Value,
        path: "/invitations/{{id}}/invitedUser",
        params: 1,
        has_body: false
    });
}
