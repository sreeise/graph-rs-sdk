// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(TeamworkApiClient, ResourceIdentity::Teamwork);

impl TeamworkApiClient {
    delete!(
        doc: "Delete navigation property teamwork for users",
        name: delete_teamwork,
        path: "/teamwork"
    );
    get!(
        doc: "Get teamwork from users",
        name: get_teamwork,
        path: "/teamwork"
    );
    patch!(
        doc: "Update the navigation property teamwork in users",
        name: update_teamwork,
        path: "/teamwork",
        body: true
    );
    post!(
        doc: "Create new navigation property to associatedTeams for users",
        name: create_associated_teams,
        path: "/teamwork/associatedTeams",
        body: true
    );
    get!(
        doc: "List associatedTeamInfo",
        name: list_associated_teams,
        path: "/teamwork/associatedTeams"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_associated_teams_count,
        path: "/teamwork/associatedTeams/$count"
    );
    delete!(
        doc: "Delete navigation property associatedTeams for users",
        name: delete_associated_teams,
        path: "/teamwork/associatedTeams/{{id}}",
        params: associated_team_info_id
    );
    get!(
        doc: "Get associatedTeams from users",
        name: get_associated_teams,
        path: "/teamwork/associatedTeams/{{id}}",
        params: associated_team_info_id
    );
    patch!(
        doc: "Update the navigation property associatedTeams in users",
        name: update_associated_teams,
        path: "/teamwork/associatedTeams/{{id}}",
        body: true,
        params: associated_team_info_id
    );
    post!(
        doc: "Install app for user",
        name: create_installed_apps,
        path: "/teamwork/installedApps",
        body: true
    );
    get!(
        doc: "List apps installed for user",
        name: list_installed_apps,
        path: "/teamwork/installedApps"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_installed_apps_count,
        path: "/teamwork/installedApps/$count"
    );
    delete!(
        doc: "Delete navigation property installedApps for users",
        name: delete_installed_apps,
        path: "/teamwork/installedApps/{{id}}",
        params: user_scope_teams_app_installation_id
    );
    get!(
        doc: "Get installedApps from users",
        name: get_installed_apps,
        path: "/teamwork/installedApps/{{id}}",
        params: user_scope_teams_app_installation_id
    );
    patch!(
        doc: "Update the navigation property installedApps in users",
        name: update_installed_apps,
        path: "/teamwork/installedApps/{{id}}",
        body: true,
        params: user_scope_teams_app_installation_id
    );
    get!(
        doc: "Get chat between user and teamsApp",
        name: get_chat,
        path: "/teamwork/installedApps/{{id}}/chat",
        params: user_scope_teams_app_installation_id
    );
    post!(
        doc: "Invoke action sendActivityNotification",
        name: send_activity_notification,
        path: "/teamwork/sendActivityNotification",
        body: true
    );
}
