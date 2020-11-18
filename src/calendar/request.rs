use crate::calendars::CalendarsRequest;
use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::events::EventsRequest;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::types::DeltaPhantom;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use graph_http::UploadSessionClient;
use reqwest::Method;

register_client!(AttachmentsRequest,);
register_client!(CalendarRequest,);
register_client!(CalendarViewRequest,);

impl<'a, Client> AttachmentsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action createUploadSession",
        name: create_upload_session,
        response: UploadSessionClient<Client>,
        path: "/calendar/calendarView/{{id}}/attachments/createUploadSession",
        params: 1,
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
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> CalendarsRequest<'a, Client> {
        CalendarsRequest::new(id.as_ref(), self.client)
    }
    pub fn event<ID: AsRef<str>>(&self, id: ID) -> EventsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Events);
        EventsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get calendarPermissions from users",
        name: list_calendar_permissions,
        response: Collection<serde_json::Value>,
        path: "/calendar/calendarPermissions",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to calendarPermissions for users",
        name: create_calendar_permissions,
        response: serde_json::Value,
        path: "/calendar/calendarPermissions",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get calendarPermissions from users",
        name: get_calendar_permissions,
        response: serde_json::Value,
        path: "/calendar/calendarPermissions/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendarPermissions in users",
        name: update_calendar_permissions,
        response: GraphResponse<Content>,
        path: "/calendar/calendarPermissions/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get calendarView from users",
        name: list_calendar_view,
        response: Collection<serde_json::Value>,
        path: "/calendar/calendarView",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to calendarView for users",
        name: create_calendar_view,
        response: serde_json::Value,
        path: "/calendar/calendarView",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get calendarView from users",
        name: get_calendar_view,
        response: serde_json::Value,
        path: "/calendar/calendarView/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendarView in users",
        name: update_calendar_view,
        response: GraphResponse<Content>,
        path: "/calendar/calendarView/{{id}}",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action getSchedule",
        name: get_schedule,
        response: Collection<serde_json::Value>,
        path: "/calendar/getSchedule",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> CalendarViewRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn attachments(&self) -> AttachmentsRequest<'a, Client> {
        AttachmentsRequest::new(self.client)
    }
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/calendar/calendarView/delta()",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action accept",
        name: accept,
        response: GraphResponse<Content>,
        path: "/calendar/calendarView/{{id}}/accept",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get attachments from users",
        name: list_attachments,
        response: Collection<serde_json::Value>,
        path: "/calendar/calendarView/{{id}}/attachments",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to attachments for users",
        name: create_attachments,
        response: serde_json::Value,
        path: "/calendar/calendarView/{{id}}/attachments",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get attachments from users",
        name: get_attachments,
        response: serde_json::Value,
        path: "/calendar/calendarView/{{id}}/attachments/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property attachments in users",
        name: update_attachments,
        response: GraphResponse<Content>,
        path: "/calendar/calendarView/{{id}}/attachments/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get calendar from users",
        name: get_calendar,
        response: serde_json::Value,
        path: "/calendar/calendarView/{{id}}/calendar",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendar in users",
        name: update_calendar,
        response: GraphResponse<Content>,
        path: "/calendar/calendarView/{{id}}/calendar",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action getSchedule",
        name: get_schedule,
        response: Collection<serde_json::Value>,
        path: "/calendar/calendarView/{{id}}/calendar/getSchedule",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action decline",
        name: decline,
        response: GraphResponse<Content>,
        path: "/calendar/calendarView/{{id}}/decline",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action dismissReminder",
        name: dismiss_reminder,
        response: GraphResponse<Content>,
        path: "/calendar/calendarView/{{id}}/dismissReminder",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get extensions from users",
        name: list_extensions,
        response: Collection<serde_json::Value>,
        path: "/calendar/calendarView/{{id}}/extensions",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to extensions for users",
        name: create_extensions,
        response: serde_json::Value,
        path: "/calendar/calendarView/{{id}}/extensions",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get extensions from users",
        name: get_extensions,
        response: serde_json::Value,
        path: "/calendar/calendarView/{{id}}/extensions/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property extensions in users",
        name: update_extensions,
        response: GraphResponse<Content>,
        path: "/calendar/calendarView/{{id}}/extensions/{{id2}}",
        params: 2,
        has_body: true
    });
    post!({
        doc: "# Invoke action snoozeReminder",
        name: snooze_reminder,
        response: GraphResponse<Content>,
        path: "/calendar/calendarView/{{id}}/snoozeReminder",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action tentativelyAccept",
        name: tentatively_accept,
        response: GraphResponse<Content>,
        path: "/calendar/calendarView/{{id}}/tentativelyAccept",
        params: 1,
        has_body: true
    });
}
