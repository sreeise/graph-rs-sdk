// GENERATED CODE

use crate::calendar_view::{CalendarViewRequest, CalendarViewsRequest};
use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::events::{EventRequest, EventsRequest};
use crate::extended_properties::ExtendedPropertiesRequest;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(CalendarRequest,);
register_client!(CalendarsRequest, ());

impl<'a, Client> CalendarRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn calendar_view<ID: AsRef<str>>(&self, id: ID) -> CalendarViewRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::CalendarView);
        CalendarViewRequest::new(id.as_ref(), self.client)
    }
    pub fn calendar_views(&self) -> CalendarViewsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::CalendarViews);
        CalendarViewsRequest::new(self.client)
    }
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> CalendarsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Calendars);
        CalendarsRequest::new(id.as_ref(), self.client)
    }
    pub fn events(&self) -> EventRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Event);
        EventRequest::new(self.client)
    }
    pub fn event<ID: AsRef<str>>(&self, id: ID) -> EventsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Events);
        EventsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get calendar from users",
        name: get_calendar,
        response: serde_json::Value,
        path: "/calendar",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendar in users",
        name: update_calendar,
        response: NoContent,
        path: "/calendar",
        params: 0,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property calendar for users",
        name: delete_calendar,
        response: NoContent,
        path: "/calendar",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Invoke function allowedCalendarSharingRoles",
        name: allowed_calendar_sharing_roles,
        response: serde_json::Value,
        path: "/calendar/allowedCalendarSharingRoles(User={{id}})",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get calendarPermissions from users",
        name: list_calendar_permissions,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/calendar/calendarPermissions/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property calendarPermissions for users",
        name: delete_calendar_permissions,
        response: NoContent,
        path: "/calendar/calendarPermissions/{{id}}",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action getSchedule",
        name: get_schedule,
        response: serde_json::Value,
        path: "/calendar/getSchedule",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get calendars from users",
        name: list_calendars,
        response: serde_json::Value,
        path: "/calendars",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to calendars for users",
        name: create_calendars,
        response: serde_json::Value,
        path: "/calendars",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> CalendarsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn calendar_view<ID: AsRef<str>>(&self, id: ID) -> CalendarViewRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::CalendarView);
        CalendarViewRequest::new(id.as_ref(), self.client)
    }
    pub fn calendar_views(&self) -> CalendarViewsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::CalendarViews);
        CalendarViewsRequest::new(self.client)
    }
    pub fn events(&self) -> EventRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Event);
        EventRequest::new(self.client)
    }
    pub fn event<ID: AsRef<str>>(&self, id: ID) -> EventsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Events);
        EventsRequest::new(id.as_ref(), self.client)
    }
    pub fn extended_properties(&self) -> ExtendedPropertiesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::ExtendedProperties);
        ExtendedPropertiesRequest::new(self.client)
    }
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
        response: NoContent,
        path: "/calendars/{{RID}}",
        params: 0,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property calendars for users",
        name: delete_calendars,
        response: NoContent,
        path: "/calendars/{{RID}}",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Invoke function allowedCalendarSharingRoles",
        name: allowed_calendar_sharing_roles,
        response: serde_json::Value,
        path: "/calendars/{{RID}}/allowedCalendarSharingRoles(User={{id}})",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get calendarPermissions from users",
        name: list_calendar_permissions,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/calendars/{{RID}}/calendarPermissions/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property calendarPermissions for users",
        name: delete_calendar_permissions,
        response: NoContent,
        path: "/calendars/{{RID}}/calendarPermissions/{{id}}",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action getSchedule",
        name: get_schedule,
        response: serde_json::Value,
        path: "/calendars/{{RID}}/getSchedule",
        params: 0,
        has_body: true
    });
}
