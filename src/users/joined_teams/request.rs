// GENERATED CODE

use crate::api_default_imports::*;
use crate::teams::*;
use crate::users::*;

resource_api_client!(
    JoinedTeamsApiClient,
    JoinedTeamsIdApiClient,
    ResourceIdentity::JoinedTeams
);

impl JoinedTeamsApiClient {
    post!(
        doc: "Create new navigation property to joinedTeams for users",
        name: create_joined_teams,
        path: "/joinedTeams",
        body: true
    );
    get!(
        doc: "List joinedTeams",
        name: list_joined_teams,
        path: "/joinedTeams"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_joined_teams_count,
        path: "/joinedTeams/$count"
    );
    get!(
        doc: "Invoke function getAllMessages",
        name: get_all_messages,
        path: "/joinedTeams/getAllMessages()"
    );
}

impl JoinedTeamsIdApiClient {
    api_client_link!(channels, ChannelsApiClient);
    api_client_link_id!(tag, TeamsTagsIdApiClient);
    api_client_link!(member, TeamsMembersIdApiClient);
    api_client_link!(members, TeamsMembersApiClient);
    api_client_link_id!(channel, ChannelsIdApiClient);
    api_client_link!(tags, TeamsTagsApiClient);
    api_client_link!(primary_channel, PrimaryChannelApiClient);
    api_client_link!(schedule, ScheduleApiClient);

    delete!(
        doc: "Delete navigation property joinedTeams for users",
        name: delete_joined_teams,
        path: "/joinedTeams/{{RID}}"
    );
    get!(
        doc: "Get joinedTeams from users",
        name: get_joined_teams,
        path: "/joinedTeams/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property joinedTeams in users",
        name: update_joined_teams,
        path: "/joinedTeams/{{RID}}",
        body: true
    );
    get!(
        doc: "List allChannels",
        name: list_all_channels,
        path: "/joinedTeams/{{RID}}/allChannels"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_all_channels_count,
        path: "/joinedTeams/{{RID}}/allChannels/$count"
    );
    get!(
        doc: "Get allChannels from users",
        name: get_all_channels,
        path: "/joinedTeams/{{RID}}/allChannels/{{id}}",
        params: channel_id
    );
    post!(
        doc: "Invoke action archive",
        name: archive,
        path: "/joinedTeams/{{RID}}/archive",
        body: true
    );
    post!(
        doc: "Invoke action clone",
        name: clone,
        path: "/joinedTeams/{{RID}}/clone",
        body: true
    );
    post!(
        doc: "Invoke action completeMigration",
        name: complete_migration,
        path: "/joinedTeams/{{RID}}/completeMigration"
    );
    get!(
        doc: "Get group from users",
        name: get_group,
        path: "/joinedTeams/{{RID}}/group"
    );
    get!(
        doc: "List incomingChannels",
        name: list_incoming_channels,
        path: "/joinedTeams/{{RID}}/incomingChannels"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_incoming_channels_count,
        path: "/joinedTeams/{{RID}}/incomingChannels/$count"
    );
    get!(
        doc: "Get incomingChannels from users",
        name: get_incoming_channels,
        path: "/joinedTeams/{{RID}}/incomingChannels/{{id}}",
        params: channel_id
    );
    post!(
        doc: "Add app to team",
        name: create_installed_apps,
        path: "/joinedTeams/{{RID}}/installedApps",
        body: true
    );
    get!(
        doc: "List apps in team",
        name: list_installed_apps,
        path: "/joinedTeams/{{RID}}/installedApps"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_installed_apps_count,
        path: "/joinedTeams/{{RID}}/installedApps/$count"
    );
    delete!(
        doc: "Delete navigation property installedApps for users",
        name: delete_installed_apps,
        path: "/joinedTeams/{{RID}}/installedApps/{{id}}",
        params: teams_app_installation_id
    );
    get!(
        doc: "Get installedApps from users",
        name: get_installed_apps,
        path: "/joinedTeams/{{RID}}/installedApps/{{id}}",
        params: teams_app_installation_id
    );
    patch!(
        doc: "Update the navigation property installedApps in users",
        name: update_installed_apps,
        path: "/joinedTeams/{{RID}}/installedApps/{{id}}",
        body: true,
        params: teams_app_installation_id
    );
    get!(
        doc: "Get teamsApp from users",
        name: get_teams_app,
        path: "/joinedTeams/{{RID}}/installedApps/{{id}}/teamsApp",
        params: teams_app_installation_id
    );
    get!(
        doc: "Get teamsAppDefinition from users",
        name: get_teams_app_definition,
        path: "/joinedTeams/{{RID}}/installedApps/{{id}}/teamsAppDefinition",
        params: teams_app_installation_id
    );
    post!(
        doc: "Invoke action upgrade",
        name: upgrade,
        path: "/joinedTeams/{{RID}}/installedApps/{{id}}/upgrade",
        params: teams_app_installation_id
    );
    post!(
        doc: "Add member to team",
        name: create_members,
        path: "/joinedTeams/{{RID}}/members",
        body: true
    );
    get!(
        doc: "List members of team",
        name: list_members,
        path: "/joinedTeams/{{RID}}/members"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_members_count,
        path: "/joinedTeams/{{RID}}/members/$count"
    );
    post!(
        doc: "Invoke action add",
        name: add,
        path: "/joinedTeams/{{RID}}/members/add",
        body: true
    );
    delete!(
        doc: "Delete navigation property members for users",
        name: delete_members,
        path: "/joinedTeams/{{RID}}/members/{{id}}",
        params: conversation_member_id
    );
    get!(
        doc: "Get members from users",
        name: get_members,
        path: "/joinedTeams/{{RID}}/members/{{id}}",
        params: conversation_member_id
    );
    patch!(
        doc: "Update the navigation property members in users",
        name: update_members,
        path: "/joinedTeams/{{RID}}/members/{{id}}",
        body: true,
        params: conversation_member_id
    );
    post!(
        doc: "Create new navigation property to operations for users",
        name: create_operations,
        path: "/joinedTeams/{{RID}}/operations",
        body: true
    );
    get!(
        doc: "Get operations from users",
        name: list_operations,
        path: "/joinedTeams/{{RID}}/operations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_operations_count,
        path: "/joinedTeams/{{RID}}/operations/$count"
    );
    delete!(
        doc: "Delete navigation property operations for users",
        name: delete_operations,
        path: "/joinedTeams/{{RID}}/operations/{{id}}",
        params: teams_async_operation_id
    );
    get!(
        doc: "Get operations from users",
        name: get_operations,
        path: "/joinedTeams/{{RID}}/operations/{{id}}",
        params: teams_async_operation_id
    );
    patch!(
        doc: "Update the navigation property operations in users",
        name: update_operations,
        path: "/joinedTeams/{{RID}}/operations/{{id}}",
        body: true,
        params: teams_async_operation_id
    );
    delete!(
        doc: "Delete navigation property photo for users",
        name: delete_photo,
        path: "/joinedTeams/{{RID}}/photo"
    );
    get!(
        doc: "Get photo from users",
        name: get_photo,
        path: "/joinedTeams/{{RID}}/photo"
    );
    patch!(
        doc: "Update the navigation property photo in users",
        name: update_photo,
        path: "/joinedTeams/{{RID}}/photo",
        body: true
    );
    get!(
        doc: "Get media content for the navigation property photo from users",
        name: get_photo_content,
        path: "/joinedTeams/{{RID}}/photo/$value"
    );
    put!(
        doc: "Update media content for the navigation property photo in users",
        name: update_photo_content,
        path: "/joinedTeams/{{RID}}/photo/$value",
        body: true
    );
    post!(
        doc: "Invoke action sendActivityNotification",
        name: send_activity_notification,
        path: "/joinedTeams/{{RID}}/sendActivityNotification",
        body: true
    );
    get!(
        doc: "Get template from users",
        name: get_template,
        path: "/joinedTeams/{{RID}}/template"
    );
    post!(
        doc: "Invoke action unarchive",
        name: unarchive,
        path: "/joinedTeams/{{RID}}/unarchive"
    );
}
