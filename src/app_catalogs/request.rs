// GENERATED CODE

use crate::api_default_imports::*;

api_client!(AppCatalogsApiClient, ResourceIdentity::AppCatalogs);

impl AppCatalogsApiClient {
    get!(
        doc: "Get appCatalogs",
        name: get_app_catalogs,
        path: "/appCatalogs"
    );
    patch!(
        doc: "Update appCatalogs",
        name: update_app_catalogs,
        path: "/appCatalogs",
        body: true
    );
    get!(
        doc: "List teamsApp",
        name: list_teams_apps,
        path: "/appCatalogs/teamsApps"
    );
    post!(
        doc: "Publish teamsapp",
        name: create_teams_apps,
        path: "/appCatalogs/teamsApps",
        body: true
    );
    get!(
        doc: "Get the number of the resource",
        name: get_teams_apps_count,
        path: "/appCatalogs/teamsApps/$count"
    );
    delete!(
        doc: "Delete navigation property teamsApps for appCatalogs",
        name: delete_teams_apps,
        path: "/appCatalogs/teamsApps/{{id}}",
        params: teams_app_id
    );
    get!(
        doc: "Get teamsApps from appCatalogs",
        name: get_teams_apps,
        path: "/appCatalogs/teamsApps/{{id}}",
        params: teams_app_id
    );
    patch!(
        doc: "Update the navigation property teamsApps in appCatalogs",
        name: update_teams_apps,
        path: "/appCatalogs/teamsApps/{{id}}",
        body: true,
        params: teams_app_id
    );
    get!(
        doc: "Get appDefinitions from appCatalogs",
        name: list_app_definitions,
        path: "/appCatalogs/teamsApps/{{id}}/appDefinitions",
        params: teams_app_id
    );
    post!(
        doc: "Update teamsApp",
        name: create_app_definitions,
        path: "/appCatalogs/teamsApps/{{id}}/appDefinitions",
        body: true,
        params: teams_app_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_app_definitions_count,
        path: "/appCatalogs/teamsApps/{{id}}/appDefinitions/$count",
        params: teams_app_id
    );
    delete!(
        doc: "Delete navigation property appDefinitions for appCatalogs",
        name: delete_app_definitions,
        path: "/appCatalogs/teamsApps/{{id}}/appDefinitions/{{id2}}",
        params: teams_app_id, teams_app_definition_id
    );
    get!(
        doc: "Get appDefinitions from appCatalogs",
        name: get_app_definitions,
        path: "/appCatalogs/teamsApps/{{id}}/appDefinitions/{{id2}}",
        params: teams_app_id, teams_app_definition_id
    );
    patch!(
        doc: "Update the navigation property appDefinitions in appCatalogs",
        name: update_app_definitions,
        path: "/appCatalogs/teamsApps/{{id}}/appDefinitions/{{id2}}",
        body: true,
        params: teams_app_id, teams_app_definition_id
    );
    delete!(
        doc: "Delete navigation property bot for appCatalogs",
        name: delete_bot,
        path: "/appCatalogs/teamsApps/{{id}}/appDefinitions/{{id2}}/bot",
        params: teams_app_id, teams_app_definition_id
    );
    get!(
        doc: "Get teamworkBot",
        name: get_bot,
        path: "/appCatalogs/teamsApps/{{id}}/appDefinitions/{{id2}}/bot",
        params: teams_app_id, teams_app_definition_id
    );
    patch!(
        doc: "Update the navigation property bot in appCatalogs",
        name: update_bot,
        path: "/appCatalogs/teamsApps/{{id}}/appDefinitions/{{id2}}/bot",
        body: true,
        params: teams_app_id, teams_app_definition_id
    );
}
