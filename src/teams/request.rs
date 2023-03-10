// GENERATED CODE

use crate::api_default_imports::*;
use crate::teams::*;
use crate::users::*;

resource_api_client!(TeamsApiClient, TeamsIdApiClient, ResourceIdentity::Teams);

impl TeamsApiClient {
    post!(
        doc: "Create team",
        name: create_team,
        path: "/teams",
        body: true
    );
    get!(
        doc: "Get team",
        name: list_team,
        path: "/teams"
    );
    get!(
        doc: "Get the number of the resource",
        name: count,
        path: "/teams/$count"
    );
    get!(
        doc: "Invoke function getAllMessages",
        name: get_all_messages,
        path: "/teams/getAllMessages()"
    );
}

impl TeamsIdApiClient {
    api_client_link!(channels, ChannelsApiClient);
    api_client_link!(primary_channel, TeamsPrimaryChannelApiClient);
    api_client_link_id!(channel, ChannelsIdApiClient);

    delete!(
        doc: "Delete entity from teams",
        name: delete_team,
        path: "/teams/{{RID}}"
    );
    get!(
        doc: "Get team",
        name: get_team,
        path: "/teams/{{RID}}"
    );
    patch!(
        doc: "Update team",
        name: update_team,
        path: "/teams/{{RID}}",
        body: true
    );
    get!(
        doc: "List allChannels",
        name: list_all_channels,
        path: "/teams/{{RID}}/allChannels"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_all_channels_count,
        path: "/teams/{{RID}}/allChannels/$count"
    );
    get!(
        doc: "Get allChannels from teams",
        name: get_all_channels,
        path: "/teams/{{RID}}/allChannels/{{id}}",
        params: channel_id
    );
    post!(
        doc: "Invoke action archive",
        name: archive,
        path: "/teams/{{RID}}/archive",
        body: true
    );
    post!(
        doc: "Invoke action clone",
        name: clone,
        path: "/teams/{{RID}}/clone",
        body: true
    );
    post!(
        doc: "Invoke action completeMigration",
        name: complete_migration,
        path: "/teams/{{RID}}/completeMigration"
    );
    get!(
        doc: "Get group from teams",
        name: get_group,
        path: "/teams/{{RID}}/group"
    );
    get!(
        doc: "List incomingChannels",
        name: list_incoming_channels,
        path: "/teams/{{RID}}/incomingChannels"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_incoming_channels_count,
        path: "/teams/{{RID}}/incomingChannels/$count"
    );
    get!(
        doc: "Get incomingChannels from teams",
        name: get_incoming_channels,
        path: "/teams/{{RID}}/incomingChannels/{{id}}",
        params: channel_id
    );
    post!(
        doc: "Add app to team",
        name: create_installed_apps,
        path: "/teams/{{RID}}/installedApps",
        body: true
    );
    get!(
        doc: "List apps in team",
        name: list_installed_apps,
        path: "/teams/{{RID}}/installedApps"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_installed_apps_count,
        path: "/teams/{{RID}}/installedApps/$count"
    );
    delete!(
        doc: "Delete navigation property installedApps for teams",
        name: delete_installed_apps,
        path: "/teams/{{RID}}/installedApps/{{id}}",
        params: teams_app_installation_id
    );
    get!(
        doc: "Get installedApps from teams",
        name: get_installed_apps,
        path: "/teams/{{RID}}/installedApps/{{id}}",
        params: teams_app_installation_id
    );
    patch!(
        doc: "Update the navigation property installedApps in teams",
        name: update_installed_apps,
        path: "/teams/{{RID}}/installedApps/{{id}}",
        body: true,
        params: teams_app_installation_id
    );
    get!(
        doc: "Get teamsApp from teams",
        name: get_teams_app,
        path: "/teams/{{RID}}/installedApps/{{id}}/teamsApp",
        params: teams_app_installation_id
    );
    get!(
        doc: "Get teamsAppDefinition from teams",
        name: get_teams_app_definition,
        path: "/teams/{{RID}}/installedApps/{{id}}/teamsAppDefinition",
        params: teams_app_installation_id
    );
    post!(
        doc: "Invoke action upgrade",
        name: upgrade,
        path: "/teams/{{RID}}/installedApps/{{id}}/upgrade",
        params: teams_app_installation_id
    );
    post!(
        doc: "Create new navigation property to operations for teams",
        name: create_operations,
        path: "/teams/{{RID}}/operations",
        body: true
    );
    get!(
        doc: "Get operations from teams",
        name: list_operations,
        path: "/teams/{{RID}}/operations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_operations_count,
        path: "/teams/{{RID}}/operations/$count"
    );
    delete!(
        doc: "Delete navigation property operations for teams",
        name: delete_operations,
        path: "/teams/{{RID}}/operations/{{id}}",
        params: teams_async_operation_id
    );
    get!(
        doc: "Get operations from teams",
        name: get_operations,
        path: "/teams/{{RID}}/operations/{{id}}",
        params: teams_async_operation_id
    );
    patch!(
        doc: "Update the navigation property operations in teams",
        name: update_operations,
        path: "/teams/{{RID}}/operations/{{id}}",
        body: true,
        params: teams_async_operation_id
    );
    delete!(
        doc: "Delete navigation property photo for teams",
        name: delete_photo,
        path: "/teams/{{RID}}/photo"
    );
    get!(
        doc: "Get photo from teams",
        name: get_photo,
        path: "/teams/{{RID}}/photo"
    );
    patch!(
        doc: "Update the navigation property photo in teams",
        name: update_photo,
        path: "/teams/{{RID}}/photo",
        body: true
    );
    get!(
        doc: "Get media content for the navigation property photo from teams",
        name: get_photo_content,
        path: "/teams/{{RID}}/photo/$value"
    );
    put!(
        doc: "Update media content for the navigation property photo in teams",
        name: update_photo_content,
        path: "/teams/{{RID}}/photo/$value",
        body: true
    );
    post!(
        doc: "Invoke action sendActivityNotification",
        name: send_activity_notification,
        path: "/teams/{{RID}}/sendActivityNotification",
        body: true
    );
    post!(
        doc: "Create teamworkTag",
        name: create_tags,
        path: "/teams/{{RID}}/tags",
        body: true
    );
    get!(
        doc: "List teamworkTags",
        name: list_tags,
        path: "/teams/{{RID}}/tags"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_tags_count,
        path: "/teams/{{RID}}/tags/$count"
    );
    delete!(
        doc: "Delete navigation property tags for teams",
        name: delete_tags,
        path: "/teams/{{RID}}/tags/{{id}}",
        params: teamwork_tag_id
    );
    get!(
        doc: "Get tags from teams",
        name: get_tags,
        path: "/teams/{{RID}}/tags/{{id}}",
        params: teamwork_tag_id
    );
    patch!(
        doc: "Update the navigation property tags in teams",
        name: update_tags,
        path: "/teams/{{RID}}/tags/{{id}}",
        body: true,
        params: teamwork_tag_id
    );
    get!(
        doc: "Get template from teams",
        name: get_template,
        path: "/teams/{{RID}}/template"
    );
    post!(
        doc: "Invoke action unarchive",
        name: unarchive,
        path: "/teams/{{RID}}/unarchive"
    );
}
