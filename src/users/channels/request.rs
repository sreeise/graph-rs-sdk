// GENERATED CODE

use crate::api_default_imports::*;
use crate::chats::*;
use crate::teams::*;

resource_api_client!(
    ChannelsApiClient,
    ChannelsIdApiClient,
    ResourceIdentity::Channels
);

impl ChannelsApiClient {
    post!(
        doc: "Create channel",
        name: create_channels,
        path: "/channels",
        body: true
    );
    get!(
        doc: "List channels",
        name: list_channels,
        path: "/channels"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_channels_count,
        path: "/channels/$count"
    );
    get!(
        doc: "Invoke function getAllMessages",
        name: get_all_messages,
        path: "/channels/getAllMessages()"
    );
}

impl ChannelsIdApiClient {
    api_client_link_id!(message, ChatsMessagesIdApiClient);
    api_client_link_id!(member, TeamsMembersIdApiClient);
    api_client_link!(messages, ChatsMessagesApiClient);
    api_client_link!(shared_with_teams, SharedWithTeamsApiClient);
    api_client_link!(members, TeamsMembersApiClient);
    api_client_link_id!(shared_with_team, SharedWithTeamsIdApiClient);

    delete!(
        doc: "Delete navigation property channels for users",
        name: delete_channels,
        path: "/channels/{{RID}}"
    );
    get!(
        doc: "Get channels from users",
        name: get_channels,
        path: "/channels/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property channels in users",
        name: update_channels,
        path: "/channels/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action completeMigration",
        name: complete_migration,
        path: "/channels/{{RID}}/completeMigration"
    );
    get!(
        doc: "Get filesFolder",
        name: get_files_folder,
        path: "/channels/{{RID}}/filesFolder"
    );
    get!(
        doc: "Get content for the navigation property filesFolder from users",
        name: get_files_folder_content,
        path: "/channels/{{RID}}/filesFolder/content"
    );
    put!(
        doc: "Update content for the navigation property filesFolder in users",
        name: update_files_folder_content,
        path: "/channels/{{RID}}/filesFolder/content",
        body: true
    );
    post!(
        doc: "Invoke action provisionEmail",
        name: provision_email,
        path: "/channels/{{RID}}/provisionEmail"
    );
    post!(
        doc: "Invoke action removeEmail",
        name: remove_email,
        path: "/channels/{{RID}}/removeEmail"
    );
    post!(
        doc: "Create new navigation property to tabs for users",
        name: create_tabs,
        path: "/channels/{{RID}}/tabs",
        body: true
    );
    get!(
        doc: "List tabs in channel",
        name: list_tabs,
        path: "/channels/{{RID}}/tabs"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_tabs_count,
        path: "/channels/{{RID}}/tabs/$count"
    );
    delete!(
        doc: "Delete navigation property tabs for users",
        name: delete_tabs,
        path: "/channels/{{RID}}/tabs/{{id}}",
        params: teams_tab_id
    );
    get!(
        doc: "Get tabs from users",
        name: get_tabs,
        path: "/channels/{{RID}}/tabs/{{id}}",
        params: teams_tab_id
    );
    patch!(
        doc: "Update the navigation property tabs in users",
        name: update_tabs,
        path: "/channels/{{RID}}/tabs/{{id}}",
        body: true,
        params: teams_tab_id
    );
    get!(
        doc: "Get teamsApp from users",
        name: get_teams_app,
        path: "/channels/{{RID}}/tabs/{{id}}/teamsApp",
        params: teams_tab_id
    );
}
