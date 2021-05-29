use crate::client::Graph;
use graph_http::{types::NoContent, IntoResponse};
use handlebars::*;
use reqwest::Method;

register_client!(TeamsRequest, ());
register_client!(ChannelsRequest,);
register_client!(InstalledAppsRequest,);
register_client!(MessagesRequest,);
register_client!(PrimaryChannelRequest,);
register_client!(PrimaryChannelMessagesRequest,);
register_client!(PrimaryChannelTabsRequest,);
register_client!(ScheduleRequest,);
register_client!(TabsRequest,);
register_client!(TeamRequest,);

impl<'a, Client> TeamRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entity from teamsTemplates by key",
        name: get_teams_template,
        response: serde_json::Value,
        path: "/teamsTemplates/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update entity in teamsTemplates",
        name: update_teams_template,
        response: NoContent,
        path: "/teamsTemplates/{{id}}",
        params: 1,
        has_body: true
    });

    delete!({
        doc: "# Delete entity from teamsTemplates",
        name: delete_teams_template,
        response: NoContent,
        path: "/teamsTemplates/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get entities from teamsTemplates",
        name: list_teams_template,
        response: serde_json::Value,
        path: "/teamsTemplates",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Add new entity to teamsTemplates",
        name: create_teams_template,
        response: serde_json::Value,
        path: "/teamsTemplates",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get entities from teams",
        name: list_team,
        response: serde_json::Value,
        path: "/teams",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Add new entity to teams",
        name: create_team,
        response: serde_json::Value,
        path: "/teams",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Invoke function getAllMessages",
        name: get_all_messages,
        response: serde_json::Value,
        path: "/teams/getAllMessages()",
        params: 0,
        has_body: false
    });

    pub fn id<ID: AsRef<str>>(&self, id: ID) -> TeamsRequest<'a, Client> {
        TeamsRequest::new(id.as_ref(), self.client)
    }
}

impl<'a, Client> TeamsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get members from teams",
        name: get_members,
        response: serde_json::Value,
        path: "/teams/{{RID}}/members/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property members in teams",
        name: update_members,
        response: NoContent,
        path: "/teams/{{RID}}/members/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get primaryChannel from teams",
        name: get_primary_channel,
        response: serde_json::Value,
        path: "/teams/{{RID}}/primaryChannel",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property primaryChannel in teams",
        name: update_primary_channel,
        response: NoContent,
        path: "/teams/{{RID}}/primaryChannel",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get group from teams",
        name: get_group,
        response: serde_json::Value,
        path: "/teams/{{RID}}/group",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Invoke action archive",
        name: archive,
        response: NoContent,
        path: "/teams/{{RID}}/archive",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get schedule from teams",
        name: get_schedule,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property schedule in teams",
        name: update_schedule,
        response: NoContent,
        path: "/teams/{{RID}}/schedule",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action unarchive",
        name: unarchive,
        response: NoContent,
        path: "/teams/{{RID}}/unarchive",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get channels from teams",
        name: get_channels,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property channels in teams",
        name: update_channels,
        response: NoContent,
        path: "/teams/{{RID}}/channels/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get operations from teams",
        name: list_operations,
        response: serde_json::Value,
        path: "/teams/{{RID}}/operations",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to operations for teams",
        name: create_operations,
        response: serde_json::Value,
        path: "/teams/{{RID}}/operations",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get members from teams",
        name: list_members,
        response: serde_json::Value,
        path: "/teams/{{RID}}/members",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to members for teams",
        name: create_members,
        response: serde_json::Value,
        path: "/teams/{{RID}}/members",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get installedApps from teams",
        name: get_installed_apps,
        response: serde_json::Value,
        path: "/teams/{{RID}}/installedApps/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property installedApps in teams",
        name: update_installed_apps,
        response: NoContent,
        path: "/teams/{{RID}}/installedApps/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get entity from teams by key",
        name: get_team,
        response: serde_json::Value,
        path: "/teams/{{RID}}",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update entity in teams",
        name: update_team,
        response: NoContent,
        path: "/teams/{{RID}}",
        params: 0,
        has_body: true
    });

    delete!({
        doc: "# Delete entity from teams",
        name: delete_team,
        response: NoContent,
        path: "/teams/{{RID}}",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get channels from teams",
        name: list_channels,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to channels for teams",
        name: create_channels,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get operations from teams",
        name: get_operations,
        response: serde_json::Value,
        path: "/teams/{{RID}}/operations/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property operations in teams",
        name: update_operations,
        response: NoContent,
        path: "/teams/{{RID}}/operations/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get template from teams",
        name: get_template,
        response: serde_json::Value,
        path: "/teams/{{RID}}/template",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get installedApps from teams",
        name: list_installed_apps,
        response: serde_json::Value,
        path: "/teams/{{RID}}/installedApps",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to installedApps for teams",
        name: create_installed_apps,
        response: serde_json::Value,
        path: "/teams/{{RID}}/installedApps",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action clone",
        name: clone,
        response: NoContent,
        path: "/teams/{{RID}}/clone",
        params: 0,
        has_body: true
    });

    pub fn channels(&self) -> ChannelsRequest<'a, Client> {
        ChannelsRequest::new(&self.client)
    }

    pub fn installed_apps(&self) -> InstalledAppsRequest<'a, Client> {
        InstalledAppsRequest::new(&self.client)
    }

    pub fn primary_channel(&self) -> PrimaryChannelRequest<'a, Client> {
        PrimaryChannelRequest::new(&self.client)
    }

    pub fn schedule(&self) -> ScheduleRequest<'a, Client> {
        ScheduleRequest::new(&self.client)
    }
}

impl<'a, Client> ChannelsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get members from teams",
        name: list_members,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels/{{id}}/members",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to members for teams",
        name: create_members,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels/{{id}}/members",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get members from teams",
        name: get_members,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels/{{id}}/members/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property members in teams",
        name: update_members,
        response: NoContent,
        path: "/teams/{{RID}}/channels/{{id}}/members/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get messages from teams",
        name: get_messages,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels/{{id}}/messages/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property messages in teams",
        name: update_messages,
        response: NoContent,
        path: "/teams/{{RID}}/channels/{{id}}/messages/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get messages from teams",
        name: list_messages,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels/{{id}}/messages",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to messages for teams",
        name: create_messages,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels/{{id}}/messages",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get filesFolder from teams",
        name: get_files_folder,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels/{{id}}/filesFolder",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property filesFolder in teams",
        name: update_files_folder,
        response: NoContent,
        path: "/teams/{{RID}}/channels/{{id}}/filesFolder",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get tabs from teams",
        name: get_tabs,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels/{{id}}/tabs/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property tabs in teams",
        name: update_tabs,
        response: NoContent,
        path: "/teams/{{RID}}/channels/{{id}}/tabs/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get tabs from teams",
        name: list_tabs,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels/{{id}}/tabs",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to tabs for teams",
        name: create_tabs,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels/{{id}}/tabs",
        params: 1,
        has_body: true
    });

    pub fn messages(&self) -> MessagesRequest<'a, Client> {
        MessagesRequest::new(&self.client)
    }

    pub fn tabs(&self) -> TabsRequest<'a, Client> {
        TabsRequest::new(&self.client)
    }
}

impl<'a, Client> MessagesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get hostedContents from teams",
        name: list_hosted_contents,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels/{{id}}/messages/{{id2}}/hostedContents",
        params: 2,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to hostedContents for teams",
        name: create_hosted_contents,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels/{{id}}/messages/{{id2}}/hostedContents",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get replies from teams",
        name: get_replies,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels/{{id}}/messages/{{id2}}/replies/{{id2}}",
        params: 3,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property replies in teams",
        name: update_replies,
        response: NoContent,
        path: "/teams/{{RID}}/channels/{{id}}/messages/{{id2}}/replies/{{id2}}",
        params: 3,
        has_body: true
    });

    get!({
        doc: "# Get replies from teams",
        name: list_replies,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels/{{id}}/messages/{{id2}}/replies",
        params: 2,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to replies for teams",
        name: create_replies,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels/{{id}}/messages/{{id2}}/replies",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get hostedContents from teams",
        name: get_hosted_contents,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels/{{id}}/messages/{{id2}}/hostedContents/{{id3}}",
        params: 3,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property hostedContents in teams",
        name: update_hosted_contents,
        response: NoContent,
        path: "/teams/{{RID}}/channels/{{id}}/messages/{{id2}}/hostedContents/{{id3}}",
        params: 3,
        has_body: true
    });
}

impl<'a, Client> TabsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get teamsApp from teams",
        name: get_teams_app,
        response: serde_json::Value,
        path: "/teams/{{RID}}/channels/{{id}}/tabs/{{id2}}/teamsApp",
        params: 2,
        has_body: false
    });
}

impl<'a, Client> InstalledAppsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get teamsAppDefinition from teams",
        name: get_teams_app_definition,
        response: serde_json::Value,
        path: "/teams/{{RID}}/installedApps/{{id}}/teamsAppDefinition",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action upgrade",
        name: upgrade,
        response: NoContent,
        path: "/teams/{{RID}}/installedApps/{{id}}/upgrade",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get teamsApp from teams",
        name: get_teams_app,
        response: serde_json::Value,
        path: "/teams/{{RID}}/installedApps/{{id}}/teamsApp",
        params: 1,
        has_body: false
    });
}

impl<'a, Client> PrimaryChannelRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get members from teams",
        name: get_members,
        response: serde_json::Value,
        path: "/teams/{{RID}}/primaryChannel/members/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property members in teams",
        name: update_members,
        response: NoContent,
        path: "/teams/{{RID}}/primaryChannel/members/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get filesFolder from teams",
        name: get_files_folder,
        response: serde_json::Value,
        path: "/teams/{{RID}}/primaryChannel/filesFolder",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property filesFolder in teams",
        name: update_files_folder,
        response: NoContent,
        path: "/teams/{{RID}}/primaryChannel/filesFolder",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get messages from teams",
        name: get_messages,
        response: serde_json::Value,
        path: "/teams/{{RID}}/primaryChannel/messages/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property messages in teams",
        name: update_messages,
        response: NoContent,
        path: "/teams/{{RID}}/primaryChannel/messages/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get tabs from teams",
        name: get_tabs,
        response: serde_json::Value,
        path: "/teams/{{RID}}/primaryChannel/tabs/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property tabs in teams",
        name: update_tabs,
        response: NoContent,
        path: "/teams/{{RID}}/primaryChannel/tabs/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get members from teams",
        name: list_members,
        response: serde_json::Value,
        path: "/teams/{{RID}}/primaryChannel/members",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to members for teams",
        name: create_members,
        response: serde_json::Value,
        path: "/teams/{{RID}}/primaryChannel/members",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get messages from teams",
        name: list_messages,
        response: serde_json::Value,
        path: "/teams/{{RID}}/primaryChannel/messages",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to messages for teams",
        name: create_messages,
        response: serde_json::Value,
        path: "/teams/{{RID}}/primaryChannel/messages",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get tabs from teams",
        name: list_tabs,
        response: serde_json::Value,
        path: "/teams/{{RID}}/primaryChannel/tabs",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to tabs for teams",
        name: create_tabs,
        response: serde_json::Value,
        path: "/teams/{{RID}}/primaryChannel/tabs",
        params: 0,
        has_body: true
    });

    pub fn primary_channel_messages(&self) -> PrimaryChannelMessagesRequest<'a, Client> {
        PrimaryChannelMessagesRequest::new(&self.client)
    }

    pub fn primary_channel_tabs(&self) -> PrimaryChannelTabsRequest<'a, Client> {
        PrimaryChannelTabsRequest::new(&self.client)
    }
}

impl<'a, Client> PrimaryChannelMessagesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get hostedContents from teams",
        name: get_hosted_contents,
        response: serde_json::Value,
        path: "/teams/{{RID}}/primaryChannel/messages/{{id}}/hostedContents/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property hostedContents in teams",
        name: update_hosted_contents,
        response: NoContent,
        path: "/teams/{{RID}}/primaryChannel/messages/{{id}}/hostedContents/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get hostedContents from teams",
        name: list_hosted_contents,
        response: serde_json::Value,
        path: "/teams/{{RID}}/primaryChannel/messages/{{id}}/hostedContents",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to hostedContents for teams",
        name: create_hosted_contents,
        response: serde_json::Value,
        path: "/teams/{{RID}}/primaryChannel/messages/{{id}}/hostedContents",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get replies from teams",
        name: get_replies,
        response: serde_json::Value,
        path: "/teams/{{RID}}/primaryChannel/messages/{{id}}/replies/{{id}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property replies in teams",
        name: update_replies,
        response: NoContent,
        path: "/teams/{{RID}}/primaryChannel/messages/{{id}}/replies/{{id}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get replies from teams",
        name: list_replies,
        response: serde_json::Value,
        path: "/teams/{{RID}}/primaryChannel/messages/{{id}}/replies",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to replies for teams",
        name: create_replies,
        response: serde_json::Value,
        path: "/teams/{{RID}}/primaryChannel/messages/{{id}}/replies",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> PrimaryChannelTabsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get teamsApp from teams",
        name: get_teams_app,
        response: serde_json::Value,
        path: "/teams/{{RID}}/primaryChannel/tabs/{{id}}/teamsApp",
        params: 1,
        has_body: false
    });
}

impl<'a, Client> ScheduleRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get swapShiftsChangeRequests from teams",
        name: get_swap_shifts_change_requests,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/swapShiftsChangeRequests/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property swapShiftsChangeRequests in teams",
        name: update_swap_shifts_change_requests,
        response: NoContent,
        path: "/teams/{{RID}}/schedule/swapShiftsChangeRequests/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get openShifts from teams",
        name: list_open_shifts,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/openShifts",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to openShifts for teams",
        name: create_open_shifts,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/openShifts",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get shifts from teams",
        name: list_shifts,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/shifts",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to shifts for teams",
        name: create_shifts,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/shifts",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get timeOffReasons from teams",
        name: list_time_off_reasons,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/timeOffReasons",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to timeOffReasons for teams",
        name: create_time_off_reasons,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/timeOffReasons",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get timeOffRequests from teams",
        name: get_time_off_requests,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/timeOffRequests/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property timeOffRequests in teams",
        name: update_time_off_requests,
        response: NoContent,
        path: "/teams/{{RID}}/schedule/timeOffRequests/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get shifts from teams",
        name: get_shifts,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/shifts/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property shifts in teams",
        name: update_shifts,
        response: NoContent,
        path: "/teams/{{RID}}/schedule/shifts/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get offerShiftRequests from teams",
        name: get_offer_shift_requests,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/offerShiftRequests/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property offerShiftRequests in teams",
        name: update_offer_shift_requests,
        response: NoContent,
        path: "/teams/{{RID}}/schedule/offerShiftRequests/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get timeOffRequests from teams",
        name: list_time_off_requests,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/timeOffRequests",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to timeOffRequests for teams",
        name: create_time_off_requests,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/timeOffRequests",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get schedulingGroups from teams",
        name: get_scheduling_groups,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/schedulingGroups/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property schedulingGroups in teams",
        name: update_scheduling_groups,
        response: NoContent,
        path: "/teams/{{RID}}/schedule/schedulingGroups/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get schedulingGroups from teams",
        name: list_scheduling_groups,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/schedulingGroups",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to schedulingGroups for teams",
        name: create_scheduling_groups,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/schedulingGroups",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get swapShiftsChangeRequests from teams",
        name: list_swap_shifts_change_requests,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/swapShiftsChangeRequests",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to swapShiftsChangeRequests for teams",
        name: create_swap_shifts_change_requests,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/swapShiftsChangeRequests",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get openShifts from teams",
        name: get_open_shifts,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/openShifts/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property openShifts in teams",
        name: update_open_shifts,
        response: NoContent,
        path: "/teams/{{RID}}/schedule/openShifts/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get timeOffReasons from teams",
        name: get_time_off_reasons,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/timeOffReasons/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property timeOffReasons in teams",
        name: update_time_off_reasons,
        response: NoContent,
        path: "/teams/{{RID}}/schedule/timeOffReasons/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get timesOff from teams",
        name: get_times_off,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/timesOff/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property timesOff in teams",
        name: update_times_off,
        response: NoContent,
        path: "/teams/{{RID}}/schedule/timesOff/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get openShiftChangeRequests from teams",
        name: get_open_shift_change_requests,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/openShiftChangeRequests/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property openShiftChangeRequests in teams",
        name: update_open_shift_change_requests,
        response: NoContent,
        path: "/teams/{{RID}}/schedule/openShiftChangeRequests/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get openShiftChangeRequests from teams",
        name: list_open_shift_change_requests,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/openShiftChangeRequests",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to openShiftChangeRequests for teams",
        name: create_open_shift_change_requests,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/openShiftChangeRequests",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get offerShiftRequests from teams",
        name: list_offer_shift_requests,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/offerShiftRequests",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to offerShiftRequests for teams",
        name: create_offer_shift_requests,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/offerShiftRequests",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get timesOff from teams",
        name: list_times_off,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/timesOff",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to timesOff for teams",
        name: create_times_off,
        response: serde_json::Value,
        path: "/teams/{{RID}}/schedule/timesOff",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action share",
        name: share,
        response: NoContent,
        path: "/teams/{{RID}}/schedule/share",
        params: 0,
        has_body: true
    });
}
