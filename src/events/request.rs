use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::types::DeltaPhantom;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use graph_http::UploadSessionClient;
use handlebars::*;
use reqwest::Method;

register_client!(AttachmentsRequest,);
register_client!(CalendarRequest,);
register_client!(CalendarViewRequest,);
register_client!(EventRequest,);
register_client!(EventsRequest, ());
register_client!(InstancesRequest,);

impl<'a, Client> AttachmentsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action createUploadSession",
        name: create_upload_session,
        response: UploadSessionClient<Client>,
        path: "/events/{{RID}}/attachments/createUploadSession",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> CalendarRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn calendar_view(&self) -> CalendarViewRequest<'a, Client> {
        CalendarViewRequest::new(self.client)
    }
    get!({
        doc: "# Get calendarPermissions from users",
        name: list_calendar_permissions,
        response: Collection<serde_json::Value>,
        path: "/events/{{RID}}/calendar/calendarPermissions",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to calendarPermissions for users",
        name: create_calendar_permissions,
        response: serde_json::Value,
        path: "/events/{{RID}}/calendar/calendarPermissions",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get calendarPermissions from users",
        name: get_calendar_permissions,
        response: serde_json::Value,
        path: "/events/{{RID}}/calendar/calendarPermissions/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendarPermissions in users",
        name: update_calendar_permissions,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/calendar/calendarPermissions/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get calendarView from users",
        name: list_calendar_view,
        response: Collection<serde_json::Value>,
        path: "/events/{{RID}}/calendar/calendarView",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to calendarView for users",
        name: create_calendar_view,
        response: serde_json::Value,
        path: "/events/{{RID}}/calendar/calendarView",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get calendarView from users",
        name: get_calendar_view,
        response: serde_json::Value,
        path: "/events/{{RID}}/calendar/calendarView/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendarView in users",
        name: update_calendar_view,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/calendar/calendarView/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get events from users",
        name: list_events,
        response: Collection<serde_json::Value>,
        path: "/events/{{RID}}/calendar/events",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to events for users",
        name: create_events,
        response: serde_json::Value,
        path: "/events/{{RID}}/calendar/events",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/events/{{RID}}/calendar/events/delta()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get events from users",
        name: get_events,
        response: serde_json::Value,
        path: "/events/{{RID}}/calendar/events/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property events in users",
        name: update_events,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/calendar/events/{{id}}",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action accept",
        name: accept,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/calendar/events/{{id}}/accept",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action decline",
        name: decline,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/calendar/events/{{id}}/decline",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action dismissReminder",
        name: dismiss_reminder,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/calendar/events/{{id}}/dismissReminder",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action snoozeReminder",
        name: snooze_reminder,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/calendar/events/{{id}}/snoozeReminder",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action tentativelyAccept",
        name: tentatively_accept,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/calendar/events/{{id}}/tentativelyAccept",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action getSchedule",
        name: get_schedule,
        response: Collection<serde_json::Value>,
        path: "/events/{{RID}}/calendar/getSchedule",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> CalendarViewRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/events/{{RID}}/calendar/calendarView/delta()",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action accept",
        name: accept,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/calendar/calendarView/{{id}}/accept",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action decline",
        name: decline,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/calendar/calendarView/{{id}}/decline",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action dismissReminder",
        name: dismiss_reminder,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/calendar/calendarView/{{id}}/dismissReminder",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action snoozeReminder",
        name: snooze_reminder,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/calendar/calendarView/{{id}}/snoozeReminder",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action tentativelyAccept",
        name: tentatively_accept,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/calendar/calendarView/{{id}}/tentativelyAccept",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> EventRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> EventsRequest<'a, Client> {
        EventsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get events from users",
        name: list_events,
        response: Collection<serde_json::Value>,
        path: "/events",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to events for users",
        name: create_events,
        response: serde_json::Value,
        path: "/events",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/events/delta()",
        params: 0,
        has_body: false
    });
}

impl<'a, Client> EventsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn attachments(&self) -> AttachmentsRequest<'a, Client> {
        AttachmentsRequest::new(self.client)
    }
    pub fn calendar(&self) -> CalendarRequest<'a, Client> {
        CalendarRequest::new(self.client)
    }
    pub fn instances(&self) -> InstancesRequest<'a, Client> {
        InstancesRequest::new(self.client)
    }
    get!({
        doc: "# Get events from users",
        name: get_events,
        response: serde_json::Value,
        path: "/events/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property events in users",
        name: update_events,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action accept",
        name: accept,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/accept",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get attachments from users",
        name: list_attachments,
        response: Collection<serde_json::Value>,
        path: "/events/{{RID}}/attachments",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to attachments for users",
        name: create_attachments,
        response: serde_json::Value,
        path: "/events/{{RID}}/attachments",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get attachments from users",
        name: get_attachments,
        response: serde_json::Value,
        path: "/events/{{RID}}/attachments/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property attachments in users",
        name: update_attachments,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/attachments/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get calendar from users",
        name: get_calendar,
        response: serde_json::Value,
        path: "/events/{{RID}}/calendar",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendar in users",
        name: update_calendar,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/calendar",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action decline",
        name: decline,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/decline",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action dismissReminder",
        name: dismiss_reminder,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/dismissReminder",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get extensions from users",
        name: list_extensions,
        response: Collection<serde_json::Value>,
        path: "/events/{{RID}}/extensions",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to extensions for users",
        name: create_extensions,
        response: serde_json::Value,
        path: "/events/{{RID}}/extensions",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get extensions from users",
        name: get_extensions,
        response: serde_json::Value,
        path: "/events/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property extensions in users",
        name: update_extensions,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get instances from users",
        name: list_instances,
        response: Collection<serde_json::Value>,
        path: "/events/{{RID}}/instances",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to instances for users",
        name: create_instances,
        response: serde_json::Value,
        path: "/events/{{RID}}/instances",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get instances from users",
        name: get_instances,
        response: serde_json::Value,
        path: "/events/{{RID}}/instances/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property instances in users",
        name: update_instances,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/instances/{{id}}",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action snoozeReminder",
        name: snooze_reminder,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/snoozeReminder",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action tentativelyAccept",
        name: tentatively_accept,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/tentativelyAccept",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> InstancesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/events/{{RID}}/instances/delta()",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action accept",
        name: accept,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/instances/{{id}}/accept",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action decline",
        name: decline,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/instances/{{id}}/decline",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action dismissReminder",
        name: dismiss_reminder,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/instances/{{id}}/dismissReminder",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action snoozeReminder",
        name: snooze_reminder,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/instances/{{id}}/snoozeReminder",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action tentativelyAccept",
        name: tentatively_accept,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/instances/{{id}}/tentativelyAccept",
        params: 1,
        has_body: true
    });
}
