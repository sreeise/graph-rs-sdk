// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(TabsRequest,);
register_client!(TabsIdRequest, ());

impl<'a, Client> TabsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "Get tabs from teams",
        name: list_tabs,
        response: serde_json::Value,
        path: "/tabs",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to tabs for teams",
        name: create_tabs,
        response: serde_json::Value,
        path: "/tabs",
        has_body: true
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/tabs/$count",
        has_body: false
    });
}

impl<'a, Client> TabsIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "Get tabs from teams",
        name: get_tabs,
        response: serde_json::Value,
        path: "/tabs/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property tabs in teams",
        name: update_tabs,
        response: NoContent,
        path: "/tabs/{{RID}}",
        has_body: true
    });
    delete!({
        doc: "Delete navigation property tabs for teams",
        name: delete_tabs,
        response: NoContent,
        path: "/tabs/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get teamsApp from teams",
        name: get_teams_app,
        response: serde_json::Value,
        path: "/tabs/{{RID}}/teamsApp",
        has_body: false
    });
}
