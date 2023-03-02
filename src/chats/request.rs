// GENERATED CODE

use crate::api_default_imports::*;

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
        path: "/chats/microsoft.graph.getAllMessages()"
    );
}

impl ChatsIdApiClient {
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
    post!(
        doc: "Invoke action upgrade",
        name: upgrade,
        path: "/chats/{{RID}}/installedApps/{{id}}/microsoft.graph.upgrade",
        params: teams_app_installation_id
    );
    get!(
        doc: "Get teamsApp from chats",
        name: get_teams_app,
        path: "/chats/{{RID}}/installedApps/{{id}}/teamsApp",
        params: teams_app_installation_id
    );
    get!(
        doc: "Get teamsAppDefinition from chats",
        name: get_teams_app_definition,
        path: "/chats/{{RID}}/installedApps/{{id}}/teamsAppDefinition",
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
        doc: "Add member to a chat",
        name: create_members,
        path: "/chats/{{RID}}/members",
        body: true
    );
    get!(
        doc: "List members of a chat",
        name: list_members,
        path: "/chats/{{RID}}/members"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_members_count,
        path: "/chats/{{RID}}/members/$count"
    );
    post!(
        doc: "Invoke action add",
        name: add,
        path: "/chats/{{RID}}/members/microsoft.graph.add",
        body: true
    );
    delete!(
        doc: "Delete navigation property members for chats",
        name: delete_members,
        path: "/chats/{{RID}}/members/{{id}}",
        params: conversation_member_id
    );
    get!(
        doc: "Get members from chats",
        name: get_members,
        path: "/chats/{{RID}}/members/{{id}}",
        params: conversation_member_id
    );
    patch!(
        doc: "Update the navigation property members in chats",
        name: update_members,
        path: "/chats/{{RID}}/members/{{id}}",
        body: true,
        params: conversation_member_id
    );
    post!(
        doc: "Invoke action hideForUser",
        name: hide_for_user,
        path: "/chats/{{RID}}/microsoft.graph.hideForUser",
        body: true
    );
    post!(
        doc: "Invoke action markChatReadForUser",
        name: mark_chat_read_for_user,
        path: "/chats/{{RID}}/microsoft.graph.markChatReadForUser",
        body: true
    );
    post!(
        doc: "Invoke action markChatUnreadForUser",
        name: mark_chat_unread_for_user,
        path: "/chats/{{RID}}/microsoft.graph.markChatUnreadForUser",
        body: true
    );
    post!(
        doc: "Invoke action sendActivityNotification",
        name: send_activity_notification,
        path: "/chats/{{RID}}/microsoft.graph.sendActivityNotification",
        body: true
    );
    post!(
        doc: "Invoke action unhideForUser",
        name: unhide_for_user,
        path: "/chats/{{RID}}/microsoft.graph.unhideForUser",
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
        doc: "Add tab to chat",
        name: create_tabs,
        path: "/chats/{{RID}}/tabs",
        body: true
    );
    get!(
        doc: "List tabs in chat",
        name: list_tabs,
        path: "/chats/{{RID}}/tabs"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_tabs_count,
        path: "/chats/{{RID}}/tabs/$count"
    );
    delete!(
        doc: "Delete navigation property tabs for chats",
        name: delete_tabs,
        path: "/chats/{{RID}}/tabs/{{id}}",
        params: teams_tab_id
    );
    get!(
        doc: "Get tabs from chats",
        name: get_tabs,
        path: "/chats/{{RID}}/tabs/{{id}}",
        params: teams_tab_id
    );
    patch!(
        doc: "Update the navigation property tabs in chats",
        name: update_tabs,
        path: "/chats/{{RID}}/tabs/{{id}}",
        body: true,
        params: teams_tab_id
    );
}
