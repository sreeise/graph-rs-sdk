// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(SharedWithTeamsRequest,);
register_client!(SharedWithTeamsIdRequest, ());

impl<'a, Client> SharedWithTeamsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "Get sharedWithTeams from teams",
        name: list_shared_with_teams,
        response: serde_json::Value,
        path: "/sharedWithTeams",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to sharedWithTeams for teams",
        name: create_shared_with_teams,
        response: serde_json::Value,
        path: "/sharedWithTeams",
        has_body: true
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/sharedWithTeams/$count",
        has_body: false
    });
}

impl<'a, Client> SharedWithTeamsIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    delete!({
        doc: "Delete navigation property sharedWithTeams for teams",
        name: delete_shared_with_teams,
        response: NoContent,
        path: "/sharedWithTeams/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get sharedWithTeams from teams",
        name: get_shared_with_teams,
        response: serde_json::Value,
        path: "/sharedWithTeams/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property sharedWithTeams in teams",
        name: update_shared_with_teams,
        response: NoContent,
        path: "/sharedWithTeams/{{RID}}",
        has_body: true
    });
    get!({
        doc: "Get allowedMembers from teams",
        name: list_allowed_members,
        response: serde_json::Value,
        path: "/sharedWithTeams/{{RID}}/allowedMembers",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_allowed_members_count,
        response: serde_json::Value,
        path: "/sharedWithTeams/{{RID}}/allowedMembers/$count",
        has_body: false
    });
    get!({
        doc: "Get allowedMembers from teams",
        name: get_allowed_members,
        response: serde_json::Value,
        path: "/sharedWithTeams/{{RID}}/allowedMembers/{{id}}",
        params: [ conversation_member_id ],
        has_body: false
    });
}
