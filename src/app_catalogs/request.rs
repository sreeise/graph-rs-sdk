// GENERATED CODE

use crate::client::Graph;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(AppCatalogsRequest,);
register_client!(AppDefinitionsRequest,);
register_client!(TeamsAppsRequest,);

impl<'a, Client> AppCatalogsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn teams_apps(&self) -> TeamsAppsRequest<'a, Client> {
        TeamsAppsRequest::new(self.client)
    }
    get!({
        doc: "# Get appCatalogs",
        name: get_app_catalogs,
        response: serde_json::Value,
        path: "/appCatalogs",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update appCatalogs",
        name: update_app_catalogs,
        response: NoContent,
        path: "/appCatalogs",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get teamsApps from appCatalogs",
        name: list_teams_apps,
        response: serde_json::Value,
        path: "/appCatalogs/teamsApps",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to teamsApps for appCatalogs",
        name: create_teams_apps,
        response: serde_json::Value,
        path: "/appCatalogs/teamsApps",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get teamsApps from appCatalogs",
        name: get_teams_apps,
        response: serde_json::Value,
        path: "/appCatalogs/teamsApps/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property teamsApps in appCatalogs",
        name: update_teams_apps,
        response: NoContent,
        path: "/appCatalogs/teamsApps/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property teamsApps for appCatalogs",
        name: delete_teams_apps,
        response: NoContent,
        path: "/appCatalogs/teamsApps/{{id}}",
        params: 1,
        has_body: false
    });
}

impl<'a, Client> AppDefinitionsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get bot from appCatalogs",
        name: get_bot,
        response: serde_json::Value,
        path: "/appCatalogs/teamsApps/{{id}}/appDefinitions/{{id2}}/bot",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property bot in appCatalogs",
        name: update_bot,
        response: NoContent,
        path: "/appCatalogs/teamsApps/{{id}}/appDefinitions/{{id2}}/bot",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property bot for appCatalogs",
        name: delete_bot,
        response: NoContent,
        path: "/appCatalogs/teamsApps/{{id}}/appDefinitions/{{id2}}/bot",
        params: 2,
        has_body: false
    });
}

impl<'a, Client> TeamsAppsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn app_definitions(&self) -> AppDefinitionsRequest<'a, Client> {
        AppDefinitionsRequest::new(self.client)
    }
    get!({
        doc: "# Get appDefinitions from appCatalogs",
        name: list_app_definitions,
        response: serde_json::Value,
        path: "/appCatalogs/teamsApps/{{id}}/appDefinitions",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to appDefinitions for appCatalogs",
        name: create_app_definitions,
        response: serde_json::Value,
        path: "/appCatalogs/teamsApps/{{id}}/appDefinitions",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get appDefinitions from appCatalogs",
        name: get_app_definitions,
        response: serde_json::Value,
        path: "/appCatalogs/teamsApps/{{id}}/appDefinitions/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property appDefinitions in appCatalogs",
        name: update_app_definitions,
        response: NoContent,
        path: "/appCatalogs/teamsApps/{{id}}/appDefinitions/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property appDefinitions for appCatalogs",
        name: delete_app_definitions,
        response: NoContent,
        path: "/appCatalogs/teamsApps/{{id}}/appDefinitions/{{id2}}",
        params: 2,
        has_body: false
    });
}
