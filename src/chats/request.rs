// GENERATED CODE

use crate::api_default_imports::*;
use crate::chats::*;
use crate::teams::*;

resource_api_client!(ChatsApiClient, ChatsIdApiClient, ResourceIdentity::Chats);

impl ChatsApiClient {
    post!(
        doc: "Create chat",
        name: create_chat,
        path: "/chats",
        body: true
    );
    get!(
        doc: "List chats",
        name: list_chat,
        path: "/chats"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_chats_count,
        path: "/chats/$count"
    );
    get!(
        doc: "Invoke function getAllMessages",
        name: get_all_messages,
        path: "/chats/getAllMessages()"
    );
}

impl ChatsIdApiClient {
    api_client_link!(messages, MessagesApiClient);
    api_client_link_id!(message, MessagesIdApiClient);
    api_client_link!(members, TeamsMembersApiClient);

    delete!(
        doc: "Delete entity from chats",
        name: delete_chat,
        path: "/chats/{{RID}}"
    );
    get!(
        doc: "Get chat",
        name: get_chat,
        path: "/chats/{{RID}}"
    );
    patch!(
        doc: "Update chat",
        name: update_chat,
        path: "/chats/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action hideForUser",
        name: hide_for_user,
        path: "/chats/{{RID}}/hideForUser",
        body: true
    );
    post!(
        doc: "Add app to chat",
        name: create_installed_apps,
        path: "/chats/{{RID}}/installedApps",
        body: true
    );
    get!(
        doc: "List apps in chat",
        name: list_installed_apps,
        path: "/chats/{{RID}}/installedApps"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_installed_apps_count,
        path: "/chats/{{RID}}/installedApps/$count"
    );
    delete!(
        doc: "Delete navigation property installedApps for chats",
        name: delete_installed_apps,
        path: "/chats/{{RID}}/installedApps/{{id}}",
        params: teams_app_installation_id
    );
    get!(
        doc: "Get installedApps from chats",
        name: get_installed_apps,
        path: "/chats/{{RID}}/installedApps/{{id}}",
        params: teams_app_installation_id
    );
    patch!(
        doc: "Update the navigation property installedApps in chats",
        name: update_installed_apps,
        path: "/chats/{{RID}}/installedApps/{{id}}",
        body: true,
        params: teams_app_installation_id
    );
    get!(
        doc: "Get teamsApp from chats",
        name: get_installed_apps_teams_app,
        path: "/chats/{{RID}}/installedApps/{{id}}/teamsApp",
        params: teams_app_installation_id
    );
    get!(
        doc: "Get teamsAppDefinition from chats",
        name: get_teams_app_definition,
        path: "/chats/{{RID}}/installedApps/{{id}}/teamsAppDefinition",
        params: teams_app_installation_id
    );
    post!(
        doc: "Invoke action upgrade",
        name: upgrade,
        path: "/chats/{{RID}}/installedApps/{{id}}/upgrade",
        params: teams_app_installation_id
    );
    delete!(
        doc: "Delete navigation property lastMessagePreview for chats",
        name: delete_last_message_preview,
        path: "/chats/{{RID}}/lastMessagePreview"
    );
    get!(
        doc: "Get lastMessagePreview from chats",
        name: get_last_message_preview,
        path: "/chats/{{RID}}/lastMessagePreview"
    );
    patch!(
        doc: "Update the navigation property lastMessagePreview in chats",
        name: update_last_message_preview,
        path: "/chats/{{RID}}/lastMessagePreview",
        body: true
    );
    post!(
        doc: "Invoke action markChatReadForUser",
        name: mark_chat_read_for_user,
        path: "/chats/{{RID}}/markChatReadForUser",
        body: true
    );
    post!(
        doc: "Invoke action markChatUnreadForUser",
        name: mark_chat_unread_for_user,
        path: "/chats/{{RID}}/markChatUnreadForUser",
        body: true
    );
    get!(
        doc: "List pinnedChatMessages in a chat",
        name: list_pinned_messages,
        path: "/chats/{{RID}}/pinnedMessages"
    );
    post!(
        doc: "Pin a message in a chat",
        name: create_pinned_messages,
        path: "/chats/{{RID}}/pinnedMessages",
        body: true
    );
    get!(
        doc: "Get the number of the resource",
        name: get_pinned_messages_count,
        path: "/chats/{{RID}}/pinnedMessages/$count"
    );
    delete!(
        doc: "Delete navigation property pinnedMessages for chats",
        name: delete_pinned_messages,
        path: "/chats/{{RID}}/pinnedMessages/{{id}}",
        params: pinned_chat_message_info_id
    );
    get!(
        doc: "Get pinnedMessages from chats",
        name: get_pinned_messages,
        path: "/chats/{{RID}}/pinnedMessages/{{id}}",
        params: pinned_chat_message_info_id
    );
    patch!(
        doc: "Update the navigation property pinnedMessages in chats",
        name: update_pinned_messages,
        path: "/chats/{{RID}}/pinnedMessages/{{id}}",
        body: true,
        params: pinned_chat_message_info_id
    );
    get!(
        doc: "Get message from chats",
        name: get_message,
        path: "/chats/{{RID}}/pinnedMessages/{{id}}/message",
        params: pinned_chat_message_info_id
    );
    post!(
        doc: "Invoke action sendActivityNotification",
        name: send_activity_notification,
        path: "/chats/{{RID}}/sendActivityNotification",
        body: true
    );
    post!(
        doc: "Invoke action unhideForUser",
        name: unhide_for_user,
        path: "/chats/{{RID}}/unhideForUser",
        body: true
    );
}
