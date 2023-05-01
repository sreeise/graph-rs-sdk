// GENERATED CODE

use crate::api_default_imports::*;

api_client!(GroupsTeamApiClient, ResourceIdentity::GroupsTeam);

impl GroupsTeamApiClient {
    patch!(
        doc: "Create team from group",
        name: update_team,
        path: "/team",
        body: true
    );
    delete!(
        doc: "Delete navigation property team for groups",
        name: delete_team,
        path: "/team"
    );
    get!(
        doc: "Get team from groups",
        name: get_team,
        path: "/team"
    );
    get!(
        doc: "List allChannels",
        name: list_all_channels,
        path: "/team/allChannels"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_all_channels_count,
        path: "/team/allChannels/$count"
    );
    get!(
        doc: "Get allChannels from groups",
        name: get_all_channels,
        path: "/team/allChannels/{{id}}",
        params: channel_id
    );
    post!(
        doc: "Invoke action archive",
        name: archive,
        path: "/team/archive",
        body: true
    );
    post!(
        doc: "Invoke action clone",
        name: clone,
        path: "/team/clone",
        body: true
    );
    post!(
        doc: "Invoke action completeMigration",
        name: complete_migration,
        path: "/team/completeMigration"
    );
    get!(
        doc: "Get group from groups",
        name: get_group,
        path: "/team/group"
    );
    get!(
        doc: "List incomingChannels",
        name: list_incoming_channels,
        path: "/team/incomingChannels"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_incoming_channels_count,
        path: "/team/incomingChannels/$count"
    );
    get!(
        doc: "Get incomingChannels from groups",
        name: get_incoming_channels,
        path: "/team/incomingChannels/{{id}}",
        params: channel_id
    );
    post!(
        doc: "Add app to team",
        name: create_installed_apps,
        path: "/team/installedApps",
        body: true
    );
    get!(
        doc: "List apps in team",
        name: list_installed_apps,
        path: "/team/installedApps"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_installed_apps_count,
        path: "/team/installedApps/$count"
    );
    delete!(
        doc: "Delete navigation property installedApps for groups",
        name: delete_installed_apps,
        path: "/team/installedApps/{{id}}",
        params: teams_app_installation_id
    );
    get!(
        doc: "Get installedApps from groups",
        name: get_installed_apps,
        path: "/team/installedApps/{{id}}",
        params: teams_app_installation_id
    );
    patch!(
        doc: "Update the navigation property installedApps in groups",
        name: update_installed_apps,
        path: "/team/installedApps/{{id}}",
        body: true,
        params: teams_app_installation_id
    );
    get!(
        doc: "Get teamsApp from groups",
        name: get_teams_app,
        path: "/team/installedApps/{{id}}/teamsApp",
        params: teams_app_installation_id
    );
    get!(
        doc: "Get teamsAppDefinition from groups",
        name: get_teams_app_definition,
        path: "/team/installedApps/{{id}}/teamsAppDefinition",
        params: teams_app_installation_id
    );
    post!(
        doc: "Invoke action upgrade",
        name: upgrade,
        path: "/team/installedApps/{{id}}/upgrade",
        params: teams_app_installation_id
    );
    post!(
        doc: "Create new navigation property to operations for groups",
        name: create_operations,
        path: "/team/operations",
        body: true
    );
    get!(
        doc: "Get operations from groups",
        name: list_operations,
        path: "/team/operations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_operations_count,
        path: "/team/operations/$count"
    );
    delete!(
        doc: "Delete navigation property operations for groups",
        name: delete_operations,
        path: "/team/operations/{{id}}",
        params: teams_async_operation_id
    );
    get!(
        doc: "Get operations from groups",
        name: get_operations,
        path: "/team/operations/{{id}}",
        params: teams_async_operation_id
    );
    patch!(
        doc: "Update the navigation property operations in groups",
        name: update_operations,
        path: "/team/operations/{{id}}",
        body: true,
        params: teams_async_operation_id
    );
    delete!(
        doc: "Delete navigation property photo for groups",
        name: delete_photo,
        path: "/team/photo"
    );
    get!(
        doc: "Get photo from groups",
        name: get_photo,
        path: "/team/photo"
    );
    patch!(
        doc: "Update the navigation property photo in groups",
        name: update_photo,
        path: "/team/photo",
        body: true
    );
    get!(
        doc: "Get media content for the navigation property photo from groups",
        name: get_photo_content,
        path: "/team/photo/$value"
    );
    put!(
        doc: "Update media content for the navigation property photo in groups",
        name: update_photo_content,
        path: "/team/photo/$value",
        body: true
    );
    post!(
        doc: "Invoke action sendActivityNotification",
        name: send_activity_notification,
        path: "/team/sendActivityNotification",
        body: true
    );
    get!(
        doc: "Get template from groups",
        name: get_template,
        path: "/team/template"
    );
    post!(
        doc: "Invoke action unarchive",
        name: unarchive,
        path: "/team/unarchive"
    );
}
