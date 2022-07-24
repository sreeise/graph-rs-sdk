// GENERATED CODE

use crate::api_default_imports::*;
use crate::channels::{ChannelsIdRequest, ChannelsRequest};
use crate::members::{MembersIdRequest, MembersRequest};
use crate::primary_channel::PrimaryChannelRequest;
use crate::schedule::ScheduleRequest;
use crate::teams_templates::{TeamsTemplatesIdRequest, TeamsTemplatesRequest};
use graph_http::types::NoContent;

register_client!(TeamsRequest,);
register_client!(TeamsIdRequest, ());

impl<'a, Client> TeamsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, teams_id: ID) -> TeamsIdRequest<'a, Client> {
        TeamsIdRequest::new(teams_id.as_ref(), self.client)
    }

    get!({
        doc: "List teams",
        name: list_team,
        response: serde_json::Value,
        path: "/teams",
        has_body: false
    });
    post!({
        doc: "Create team",
        name: create_team,
        response: NoContent,
        path: "/teams",
        has_body: true
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
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Channels);
        ChannelsRequest::new(self.client)
    }

    pub fn channel<ID: AsRef<str>>(&self, id: ID) -> ChannelsIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Channels);
        ChannelsIdRequest::new(id.as_ref(), self.client)
    }

    pub fn members(&self) -> MembersRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Members);
        MembersRequest::new(self.client)
    }

    pub fn member<ID: AsRef<str>>(&self, id: ID) -> MembersIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Members);
        MembersIdRequest::new(id.as_ref(), self.client)
    }

    pub fn primary_channel(&self) -> PrimaryChannelRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::PrimaryChannel);
        PrimaryChannelRequest::new(self.client)
    }

    pub fn schedule(&self) -> ScheduleRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Schedule);
        ScheduleRequest::new(self.client)
    }

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
        doc: "Get team",
        name: get_team,
        response: serde_json::Value,
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
        doc: "Create new navigation property to installedApps for teams",
        name: create_installed_apps,
        response: serde_json::Value,
        path: "/teams/{{RID}}/installedApps",
        has_body: true
    });
    get!({
        doc: "Get installedApps from teams",
        name: list_installed_apps,
        response: serde_json::Value,
        path: "/teams/{{RID}}/installedApps",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_installed_apps_count,
        response: serde_json::Value,
        path: "/teams/{{RID}}/installedApps/$count",
        has_body: false
    });
    get!({
        doc: "Get installedApps from teams",
        name: get_installed_apps,
        response: serde_json::Value,
        path: "/teams/{{RID}}/installedApps/{{id}}",
        params: [ teams_app_installation_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property installedApps in teams",
        name: update_installed_apps,
        response: NoContent,
        path: "/teams/{{RID}}/installedApps/{{id}}",
        params: [ teams_app_installation_id ],
        has_body: true
    });
    delete!({
        doc: "Delete navigation property installedApps for teams",
        name: delete_installed_apps,
        response: NoContent,
        path: "/teams/{{RID}}/installedApps/{{id}}",
        params: [ teams_app_installation_id ],
        has_body: false
    });
    post!({
        doc: "Invoke action upgrade",
        name: upgrade,
        response: NoContent,
        path: "/teams/{{RID}}/installedApps/{{id}}/microsoft.graph.upgrade",
        params: [ teams_app_installation_id ],
        has_body: false
    });
    get!({
        doc: "Get teamsApp from teams",
        name: get_teams_app,
        response: serde_json::Value,
        path: "/teams/{{RID}}/installedApps/{{id}}/teamsApp",
        params: [ teams_app_installation_id ],
        has_body: false
    });
    get!({
        doc: "Get teamsAppDefinition from teams",
        name: get_teams_app_definition,
        response: serde_json::Value,
        path: "/teams/{{RID}}/installedApps/{{id}}/teamsAppDefinition",
        params: [ teams_app_installation_id ],
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
        doc: "Get operations from teams",
        name: list_operations,
        response: serde_json::Value,
        path: "/teams/{{RID}}/operations",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to operations for teams",
        name: create_operations,
        response: serde_json::Value,
        path: "/teams/{{RID}}/operations",
        has_body: true
    });
    get!({
        doc: "Get the number of the resource",
        name: get_operations_count,
        response: serde_json::Value,
        path: "/teams/{{RID}}/operations/$count",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property operations for teams",
        name: delete_operations,
        response: NoContent,
        path: "/teams/{{RID}}/operations/{{id}}",
        params: [ teams_async_operation_id ],
        has_body: false
    });
    get!({
        doc: "Get operations from teams",
        name: get_operations,
        response: serde_json::Value,
        path: "/teams/{{RID}}/operations/{{id}}",
        params: [ teams_async_operation_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property operations in teams",
        name: update_operations,
        response: NoContent,
        path: "/teams/{{RID}}/operations/{{id}}",
        params: [ teams_async_operation_id ],
        has_body: true
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
        doc: "Get photo from teams",
        name: get_photo,
        response: serde_json::Value,
        path: "/teams/{{RID}}/photo",
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
        doc: "Get media content for the navigation property photo from teams",
        name: get_photo_content,
        response: serde_json::Value,
        path: "/teams/{{RID}}/photo/$value",
        has_body: false
    });
    get!({
        doc: "Get template from teams",
        name: get_template,
        response: serde_json::Value,
        path: "/teams/{{RID}}/template",
        has_body: false
    });
}
