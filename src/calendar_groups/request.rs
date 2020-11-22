use crate::calendar::{CalendarRequest, CalendarsRequest};
use crate::client::Graph;
use crate::core::ResourceIdentity;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(CalendarGroupRequest,);
register_client!(CalendarGroupsRequest, ());

impl<'a, Client> CalendarGroupRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> CalendarGroupsRequest<'a, Client> {
        CalendarGroupsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get calendarGroups from users",
        name: list_calendar_groups,
        response: Collection<serde_json::Value>,
        path: "/calendarGroups",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to calendarGroups for users",
        name: create_calendar_groups,
        response: serde_json::Value,
        path: "/calendarGroups",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> CalendarGroupsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn calendars(&self) -> CalendarRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Calendar);
        CalendarRequest::new(self.client)
    }
    pub fn calendar<ID: AsRef<str>>(&self, id: ID) -> CalendarsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Calendars);
        CalendarsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get calendarGroups from users",
        name: get_calendar_groups,
        response: serde_json::Value,
        path: "/calendarGroups/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendarGroups in users",
        name: update_calendar_groups,
        response: GraphResponse<Content>,
        path: "/calendarGroups/{{RID}}",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get calendars from users",
        name: list_calendars,
        response: Collection<serde_json::Value>,
        path: "/calendarGroups/{{RID}}/calendars",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to calendars for users",
        name: create_calendars,
        response: serde_json::Value,
        path: "/calendarGroups/{{RID}}/calendars",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get calendars from users",
        name: get_calendars,
        response: serde_json::Value,
        path: "/calendarGroups/{{RID}}/calendars/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendars in users",
        name: update_calendars,
        response: GraphResponse<Content>,
        path: "/calendarGroups/{{RID}}/calendars/{{id}}",
        params: 1,
        has_body: true
    });
}
