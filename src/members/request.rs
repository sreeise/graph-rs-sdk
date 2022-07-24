// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(MembersRequest,);
register_client!(MembersIdRequest, ());

impl<'a, Client> MembersRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Create new navigation property to members for teams",
        name: create_members,
        response: serde_json::Value,
        path: "/members",
        has_body: true
    });
    get!({
        doc: "Get members from teams",
        name: list_members,
        response: serde_json::Value,
        path: "/members",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/members/$count",
        has_body: false
    });
    post!({
        doc: "Invoke action add",
        name: add,
        response: serde_json::Value,
        path: "/members/microsoft.graph.add",
        has_body: true
    });
}

impl<'a, Client> MembersIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    patch!({
        doc: "Update the navigation property members in teams",
        name: update_members,
        response: NoContent,
        path: "/members/{{RID}}",
        has_body: true
    });
    delete!({
        doc: "Delete navigation property members for teams",
        name: delete_members,
        response: NoContent,
        path: "/members/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get members from teams",
        name: get_members,
        response: serde_json::Value,
        path: "/members/{{RID}}",
        has_body: false
    });
}
