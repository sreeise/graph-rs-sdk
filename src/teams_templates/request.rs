// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(TeamsTemplatesRequest,);
register_client!(TeamsTemplatesIdRequest, ());

impl<'a, Client> TeamsTemplatesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "Get entities from teamsTemplates",
        name: list_teams_template,
        response: serde_json::Value,
        path: "/teamsTemplates",
        has_body: false
    });
    post!({
        doc: "Add new entity to teamsTemplates",
        name: create_teams_template,
        response: serde_json::Value,
        path: "/teamsTemplates",
        has_body: true
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/teamsTemplates/$count",
        has_body: false
    });
}

impl<'a, Client> TeamsTemplatesIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "Get entity from teamsTemplates by key",
        name: get_teams_template,
        response: serde_json::Value,
        path: "/teamsTemplates/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update entity in teamsTemplates",
        name: update_teams_template,
        response: NoContent,
        path: "/teamsTemplates/{{RID}}",
        has_body: true
    });
    delete!({
        doc: "Delete entity from teamsTemplates",
        name: delete_teams_template,
        response: NoContent,
        path: "/teamsTemplates/{{RID}}",
        has_body: false
    });
}
