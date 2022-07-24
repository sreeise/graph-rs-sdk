// GENERATED CODE

use crate::api_default_imports::*;
use crate::channels::{ChannelsIdRequest, ChannelsRequest};
use crate::primary_channel::PrimaryChannelRequest;
use graph_http::types::NoContent;

register_client!(TeamsRequest,);
register_client!(TeamsIdRequest, ());

impl<'a, Client> TeamsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn primary_channel(&self) -> PrimaryChannelRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::PrimaryChannel);
        PrimaryChannelRequest::new(self.client)
    }

    pub fn id<ID: AsRef<str>>(&self, teams_id: ID) -> TeamsIdRequest<'a, Client> {
        TeamsIdRequest::new(teams_id.as_ref(), self.client)
    }

    post!({
        doc: "Create team",
        name: create_team,
        response: serde_json::Value,
        path: "/teams",
        has_body: true
    });
    get!({
        doc: "List teams",
        name: list_team,
        response: serde_json::Value,
        path: "/teams",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/teams/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function getAllMessages",
        name: get_all_messages,
        response: serde_json::Value,
        path: "/teams/microsoft.graph.getAllMessages()",
        has_body: false
    });
}

impl<'a, Client> TeamsIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn channels(&self) -> ChannelsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Channels);
        ChannelsRequest::new(self.client)
    }

    pub fn channel<ID: AsRef<str>>(&self, id: ID) -> ChannelsIdRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Channels);
        ChannelsIdRequest::new(id.as_ref(), self.client)
    }

    get!({
        doc: "Get team",
        name: get_team,
        response: serde_json::Value,
        path: "/teams/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update team",
        name: update_team,
        response: NoContent,
        path: "/teams/{{RID}}",
        has_body: true
    });
    delete!({
        doc: "Delete entity from teams",
        name: delete_team,
        response: NoContent,
        path: "/teams/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get allChannels from teams",
        name: list_all_channels,
        response: serde_json::Value,
        path: "/teams/{{RID}}/allChannels",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_all_channels_count,
        response: serde_json::Value,
        path: "/teams/{{RID}}/allChannels/$count",
        has_body: false
    });
    get!({
        doc: "Get allChannels from teams",
        name: get_all_channels,
        response: serde_json::Value,
        path: "/teams/{{RID}}/allChannels/{{id}}",
        params: [ channel_id ],
        has_body: false
    });
    get!({
        doc: "Get group from teams",
        name: get_group,
        response: serde_json::Value,
        path: "/teams/{{RID}}/group",
        has_body: false
    });
    get!({
        doc: "Get incomingChannels from teams",
        name: list_incoming_channels,
        response: serde_json::Value,
        path: "/teams/{{RID}}/incomingChannels",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_incoming_channels_count,
        response: serde_json::Value,
        path: "/teams/{{RID}}/incomingChannels/$count",
        has_body: false
    });
    get!({
        doc: "Get incomingChannels from teams",
        name: get_incoming_channels,
        response: serde_json::Value,
        path: "/teams/{{RID}}/incomingChannels/{{id}}",
        params: [ channel_id ],
        has_body: false
    });
    post!({
        doc: "Invoke action archive",
        name: archive,
        response: NoContent,
        path: "/teams/{{RID}}/microsoft.graph.archive",
        has_body: true
    });
    post!({
        doc: "Invoke action clone",
        name: clone,
        response: NoContent,
        path: "/teams/{{RID}}/microsoft.graph.clone",
        has_body: true
    });
    post!({
        doc: "Invoke action completeMigration",
        name: complete_migration,
        response: NoContent,
        path: "/teams/{{RID}}/microsoft.graph.completeMigration",
        has_body: false
    });
    post!({
        doc: "Invoke action sendActivityNotification",
        name: send_activity_notification,
        response: NoContent,
        path: "/teams/{{RID}}/microsoft.graph.sendActivityNotification",
        has_body: true
    });
    post!({
        doc: "Invoke action unarchive",
        name: unarchive,
        response: NoContent,
        path: "/teams/{{RID}}/microsoft.graph.unarchive",
        has_body: false
    });
    get!({
        doc: "Get photo from teams",
        name: get_photo,
        response: serde_json::Value,
        path: "/teams/{{RID}}/photo",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property photo in teams",
        name: update_photo,
        response: NoContent,
        path: "/teams/{{RID}}/photo",
        has_body: true
    });
    delete!({
        doc: "Delete navigation property photo for teams",
        name: delete_photo,
        response: NoContent,
        path: "/teams/{{RID}}/photo",
        has_body: false
    });
    get!({
        doc: "Get media content for the navigation property photo from teams",
        name: get_photo_content,
        response: serde_json::Value,
        path: "/teams/{{RID}}/photo/$value",
        has_body: false
    });
    put!({
        doc: "Update media content for the navigation property photo in teams",
        name: update_photo_content,
        response: NoContent,
        path: "/teams/{{RID}}/photo/$value",
        has_body: true
    });
    get!({
        doc: "Get template from teams",
        name: get_template,
        response: serde_json::Value,
        path: "/teams/{{RID}}/template",
        has_body: false
    });
}
