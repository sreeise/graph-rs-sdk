// GENERATED CODE

use crate::api_default_imports::*;
use crate::chats_channels_messages::{
    ChatsAndChannelsMessagesIdRequest, ChatsAndChannelsMessagesRequest,
};
use graph_http::types::NoContent;

register_client!(ChannelsRequest,);
register_client!(ChannelsIdRequest, ());

impl<'a, Client> ChannelsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn messages(&self) -> ChatsAndChannelsMessagesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Messages);
        ChatsAndChannelsMessagesRequest::new(self.client)
    }

    pub fn message<ID: AsRef<str>>(&self, id: ID) -> ChatsAndChannelsMessagesIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Messages);
        ChatsAndChannelsMessagesIdRequest::new(id.as_ref(), self.client)
    }

    post!({
        doc: "Create new navigation property to channels for teams",
        name: create_channels,
        response: serde_json::Value,
        path: "/channels",
        has_body: true
    });
    get!({
        doc: "Get channels from teams",
        name: list_channels,
        response: serde_json::Value,
        path: "/channels",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/channels/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function getAllMessages",
        name: get_all_messages,
        response: serde_json::Value,
        path: "/channels/microsoft.graph.getAllMessages()",
        has_body: false
    });
}

impl<'a, Client> ChannelsIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "Get channels from teams",
        name: get_channels,
        response: serde_json::Value,
        path: "/channels/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property channels in teams",
        name: update_channels,
        response: NoContent,
        path: "/channels/{{RID}}",
        has_body: true
    });
    delete!({
        doc: "Delete navigation property channels for teams",
        name: delete_channels,
        response: NoContent,
        path: "/channels/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get filesFolder from teams",
        name: get_files_folder,
        response: serde_json::Value,
        path: "/channels/{{RID}}/filesFolder",
        has_body: false
    });
    get!({
        doc: "Get content for the navigation property filesFolder from teams",
        name: get_files_folder_content,
        response: serde_json::Value,
        path: "/channels/{{RID}}/filesFolder/content",
        has_body: false
    });
    put!({
        doc: "Update content for the navigation property filesFolder in teams",
        name: update_files_folder_content,
        response: NoContent,
        path: "/channels/{{RID}}/filesFolder/content",
        has_body: true
    });
    post!({
        doc: "Create new navigation property to members for teams",
        name: create_members,
        response: serde_json::Value,
        path: "/channels/{{RID}}/members",
        has_body: true
    });
    get!({
        doc: "Get members from teams",
        name: list_members,
        response: serde_json::Value,
        path: "/channels/{{RID}}/members",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_members_count,
        response: serde_json::Value,
        path: "/channels/{{RID}}/members/$count",
        has_body: false
    });
    post!({
        doc: "Invoke action add",
        name: add,
        response: serde_json::Value,
        path: "/channels/{{RID}}/members/microsoft.graph.add",
        has_body: true
    });
    patch!({
        doc: "Update the navigation property members in teams",
        name: update_members,
        response: NoContent,
        path: "/channels/{{RID}}/members/{{id}}",
        params: [ conversation_member_id ],
        has_body: true
    });
    delete!({
        doc: "Delete navigation property members for teams",
        name: delete_members,
        response: NoContent,
        path: "/channels/{{RID}}/members/{{id}}",
        params: [ conversation_member_id ],
        has_body: false
    });
    get!({
        doc: "Get members from teams",
        name: get_members,
        response: serde_json::Value,
        path: "/channels/{{RID}}/members/{{id}}",
        params: [ conversation_member_id ],
        has_body: false
    });
    post!({
        doc: "Invoke action completeMigration",
        name: complete_migration,
        response: NoContent,
        path: "/channels/{{RID}}/microsoft.graph.completeMigration",
        has_body: false
    });
    get!({
        doc: "Invoke function doesUserHaveAccess",
        name: does_user_have_access,
        response: serde_json::Value,
        path: "/channels/{{RID}}/microsoft.graph.doesUserHaveAccess(userId='{{id}}',tenantId='{{id2}}',userPrincipalName='{{id3}}')",
        params: [ user_id tenant_id user_principal_name ],
        has_body: false
    });
    post!({
        doc: "Invoke action provisionEmail",
        name: provision_email,
        response: serde_json::Value,
        path: "/channels/{{RID}}/microsoft.graph.provisionEmail",
        has_body: false
    });
    post!({
        doc: "Invoke action removeEmail",
        name: remove_email,
        response: NoContent,
        path: "/channels/{{RID}}/microsoft.graph.removeEmail",
        has_body: false
    });
}
