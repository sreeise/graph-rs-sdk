// GENERATED CODE

use crate::api_default_imports::*;
use crate::chats_channels_messages::{
    ChatsAndChannelsMessagesIdRequest, ChatsAndChannelsMessagesRequest,
};
use crate::shared_with_teams::{SharedWithTeamsIdRequest, SharedWithTeamsRequest};
use crate::tabs::{TabsIdRequest, TabsRequest};
use graph_http::types::NoContent;

register_client!(PrimaryChannelRequest,);

impl<'a, Client> PrimaryChannelRequest<'a, Client>
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

    pub fn shared_with_teams(&self) -> SharedWithTeamsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::SharedWithTeams);
        SharedWithTeamsRequest::new(self.client)
    }

    pub fn shared_with_team<ID: AsRef<str>>(&self, id: ID) -> SharedWithTeamsIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::SharedWithTeams);
        SharedWithTeamsIdRequest::new(id.as_ref(), self.client)
    }

    pub fn tabs(&self) -> TabsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Tabs);
        TabsRequest::new(self.client)
    }

    pub fn tab<ID: AsRef<str>>(&self, id: ID) -> TabsIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Tabs);
        TabsIdRequest::new(id.as_ref(), self.client)
    }

    delete!({
        doc: "Delete navigation property primaryChannel for teams",
        name: delete_primary_channel,
        response: NoContent,
        path: "/primaryChannel",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property primaryChannel in teams",
        name: update_primary_channel,
        response: NoContent,
        path: "/primaryChannel",
        has_body: true
    });
    get!({
        doc: "Get primaryChannel from teams",
        name: get_primary_channel,
        response: serde_json::Value,
        path: "/primaryChannel",
        has_body: false
    });
    get!({
        doc: "Get filesFolder from teams",
        name: get_files_folder,
        response: serde_json::Value,
        path: "/primaryChannel/filesFolder",
        has_body: false
    });
    get!({
        doc: "Get content for the navigation property filesFolder from teams",
        name: get_files_folder_content,
        response: serde_json::Value,
        path: "/primaryChannel/filesFolder/content",
        has_body: false
    });
    put!({
        doc: "Update content for the navigation property filesFolder in teams",
        name: update_files_folder_content,
        response: NoContent,
        path: "/primaryChannel/filesFolder/content",
        has_body: true
    });
    post!({
        doc: "Create new navigation property to members for teams",
        name: create_members,
        response: serde_json::Value,
        path: "/primaryChannel/members",
        has_body: true
    });
    get!({
        doc: "Get members from teams",
        name: list_members,
        response: serde_json::Value,
        path: "/primaryChannel/members",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/primaryChannel/members/$count",
        has_body: false
    });
    post!({
        doc: "Invoke action add",
        name: add,
        response: serde_json::Value,
        path: "/primaryChannel/members/microsoft.graph.add",
        has_body: true
    });
    delete!({
        doc: "Delete navigation property members for teams",
        name: delete_members,
        response: NoContent,
        path: "/primaryChannel/members/{{id}}",
        params: [ conversation_member_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property members in teams",
        name: update_members,
        response: NoContent,
        path: "/primaryChannel/members/{{id}}",
        params: [ conversation_member_id ],
        has_body: true
    });
    get!({
        doc: "Get members from teams",
        name: get_members,
        response: serde_json::Value,
        path: "/primaryChannel/members/{{id}}",
        params: [ conversation_member_id ],
        has_body: false
    });
    post!({
        doc: "Invoke action completeMigration",
        name: complete_migration,
        response: NoContent,
        path: "/primaryChannel/microsoft.graph.completeMigration",
        has_body: false
    });
    get!({
        doc: "Invoke function doesUserHaveAccess",
        name: does_user_have_access,
        response: serde_json::Value,
        path: "/primaryChannel/microsoft.graph.doesUserHaveAccess(userId='{{id}}',tenantId='{{id2}}',userPrincipalName='{{id3}}')",
        params: [ user_id  tenant_id  user_principal_name ],
        has_body: false
    });
    post!({
        doc: "Invoke action provisionEmail",
        name: provision_email,
        response: serde_json::Value,
        path: "/primaryChannel/microsoft.graph.provisionEmail",
        has_body: false
    });
    post!({
        doc: "Invoke action removeEmail",
        name: remove_email,
        response: NoContent,
        path: "/primaryChannel/microsoft.graph.removeEmail",
        has_body: false
    });
}
