use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::types::DeltaPhantom;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(CalendarsRequest, ());
register_client!(CalendarViewRequest,);
register_client!(EventsRequest,);

impl<'a, Client> CalendarsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn calendar_view(&self) -> CalendarViewRequest<'a, Client> {
        CalendarViewRequest::new(&self.client)
    }
    pub fn events(&self) -> EventsRequest<'a, Client> {
        EventsRequest::new(&self.client)
    }
    get!({
        doc: "# Get calendarPermissions from users",
        name: list_calendar_permissions,
        response: Collection<serde_json::Value>,
        path: "/calendars/{{RID}}/calendarPermissions",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to calendarPermissions for users",
        name: create_calendar_permissions,
        response: serde_json::Value,
        path: "/calendars/{{RID}}/calendarPermissions",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get calendarView from users",
        name: get_calendar_view,
        response: serde_json::Value,
        path: "/calendars/{{RID}}/calendarView/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendarView in users",
        name: update_calendar_view,
        response: GraphResponse<Content>,
        path: "/calendars/{{RID}}/calendarView/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get calendars from users",
        name: get_calendars,
        response: serde_json::Value,
        path: "/calendars/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendars in users",
        name: update_calendars,
        response: GraphResponse<Content>,
        path: "/calendars/{{RID}}",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get calendarPermissions from users",
        name: get_calendar_permissions,
        response: serde_json::Value,
        path: "/calendars/{{RID}}/calendarPermissions/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendarPermissions in users",
        name: update_calendar_permissions,
        response: GraphResponse<Content>,
        path: "/calendars/{{RID}}/calendarPermissions/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get events from users",
        name: list_events,
        response: Collection<serde_json::Value>,
        path: "/calendars/{{RID}}/events",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to events for users",
        name: create_events,
        response: serde_json::Value,
        path: "/calendars/{{RID}}/events",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get events from users",
        name: get_events,
        response: serde_json::Value,
        path: "/calendars/{{RID}}/events/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property events in users",
        name: update_events,
        response: GraphResponse<Content>,
        path: "/calendars/{{RID}}/events/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get calendarView from users",
        name: list_calendar_view,
        response: Collection<serde_json::Value>,
        path: "/calendars/{{RID}}/calendarView",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to calendarView for users",
        name: create_calendar_view,
        response: serde_json::Value,
        path: "/calendars/{{RID}}/calendarView",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action getSchedule",
        name: get_schedule,
        response: Collection<serde_json::Value>,
        path: "/calendars/{{RID}}/getSchedule",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> CalendarViewRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action decline",
        name: decline,
        response: GraphResponse<Content>,
        path: "/calendars/{{RID}}/calendarView/{{id}}/decline",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action accept",
        name: accept,
        response: GraphResponse<Content>,
        path: "/calendars/{{RID}}/calendarView/{{id}}/accept",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get calendar from users",
        name: get_calendar,
        response: serde_json::Value,
        path: "/calendars/{{RID}}/calendarView/{{id}}/calendar",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendar in users",
        name: update_calendar,
        response: GraphResponse<Content>,
        path: "/calendars/{{RID}}/calendarView/{{id}}/calendar",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/calendars/{{RID}}/calendarView/delta()",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action snoozeReminder",
        name: snooze_reminder,
        response: GraphResponse<Content>,
        path: "/calendars/{{RID}}/calendarView/{{id}}/snoozeReminder",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get extensions from users",
        name: list_extensions,
        response: Collection<serde_json::Value>,
        path: "/calendars/{{RID}}/calendarView/{{id}}/extensions",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to extensions for users",
        name: create_extensions,
        response: serde_json::Value,
        path: "/calendars/{{RID}}/calendarView/{{id}}/extensions",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action tentativelyAccept",
        name: tentatively_accept,
        response: GraphResponse<Content>,
        path: "/calendars/{{RID}}/calendarView/{{id}}/tentativelyAccept",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get extensions from users",
        name: get_extensions,
        response: serde_json::Value,
        path: "/calendars/{{RID}}/calendarView/{{id}}/extensions/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property extensions in users",
        name: update_extensions,
        response: GraphResponse<Content>,
        path: "/calendars/{{RID}}/calendarView/{{id}}/extensions/{{id2}}",
        params: 2,
        has_body: true
    });
    post!({
        doc: "# Invoke action dismissReminder",
        name: dismiss_reminder,
        response: GraphResponse<Content>,
        path: "/calendars/{{RID}}/calendarView/{{id}}/dismissReminder",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action getSchedule",
        name: get_schedule,
        response: Collection<serde_json::Value>,
        path: "/calendars/{{RID}}/calendarView/{{id}}/calendar/getSchedule",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> EventsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/calendars/{{RID}}/events/delta()",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action accept",
        name: accept,
        response: GraphResponse<Content>,
        path: "/calendars/{{RID}}/events/{{id}}/accept",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get calendar from users",
        name: get_calendar,
        response: serde_json::Value,
        path: "/calendars/{{RID}}/events/{{id}}/calendar",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendar in users",
        name: update_calendar,
        response: GraphResponse<Content>,
        path: "/calendars/{{RID}}/events/{{id}}/calendar",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get extensions from users",
        name: list_extensions,
        response: Collection<serde_json::Value>,
        path: "/calendars/{{RID}}/events/{{id}}/extensions",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to extensions for users",
        name: create_extensions,
        response: serde_json::Value,
        path: "/calendars/{{RID}}/events/{{id}}/extensions",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action decline",
        name: decline,
        response: GraphResponse<Content>,
        path: "/calendars/{{RID}}/events/{{id}}/decline",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action snoozeReminder",
        name: snooze_reminder,
        response: GraphResponse<Content>,
        path: "/calendars/{{RID}}/events/{{id}}/snoozeReminder",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action dismissReminder",
        name: dismiss_reminder,
        response: GraphResponse<Content>,
        path: "/calendars/{{RID}}/events/{{id}}/dismissReminder",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get extensions from users",
        name: get_extensions,
        response: serde_json::Value,
        path: "/calendars/{{RID}}/events/{{id}}/extensions/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property extensions in users",
        name: update_extensions,
        response: GraphResponse<Content>,
        path: "/calendars/{{RID}}/events/{{id}}/extensions/{{id2}}",
        params: 2,
        has_body: true
    });
    post!({
        doc: "# Invoke action tentativelyAccept",
        name: tentatively_accept,
        response: GraphResponse<Content>,
        path: "/calendars/{{RID}}/events/{{id}}/tentativelyAccept",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action getSchedule",
        name: get_schedule,
        response: Collection<serde_json::Value>,
        path: "/calendars/{{RID}}/events/{{id}}/calendar/getSchedule",
        params: 1,
        has_body: true
    });
}
