use crate::attachments::{CalendarAttachmentRequest, CalendarGroupAttachmentRequest};
use crate::client::Graph;
use crate::http::{GraphResponse, IntoResponse};
use crate::types::{collection::Collection, content::Content};
use reqwest::Method;

register_client!(CalendarRequest,);

impl<'a, Client> CalendarRequest<'a, Client>
where
    Client: crate::http::RequestClient,
{
    get!( list, Collection<serde_json::Value> => "calendars" );
    get!( get_default, serde_json::Value => "calendar" );
    get!( | get, serde_json::Value => "calendars/{{id}}" );
    get!( list_events, Collection<serde_json::Value> => "calendars/events" );
    patch!( [ update_default, serde_json::Value => "calendar" ] );
    patch!( [ | update, serde_json::Value => "calendars/{{id}}" ] );
    post!( | create, serde_json::Value => "calendars" );
    delete!( | delete, GraphResponse<Content> => "calendars/{{id}}" );

    pub fn attachments(&'a self) -> CalendarAttachmentRequest<'a, Client> {
        CalendarAttachmentRequest::new(self.client)
    }

    pub fn views(&self) -> CalendarViewRequest<'a, Client> {
        CalendarViewRequest::new(self.client)
    }

    pub fn groups(&self) -> CalendarGroupRequest<'a, Client> {
        CalendarGroupRequest::new(self.client)
    }
}

register_client!(CalendarViewRequest,);

impl<'a, Client> CalendarViewRequest<'a, Client>
where
    Client: crate::http::RequestClient,
{
    pub fn list_default_view(
        &self,
        start_date_time: &str,
        end_date_time: &str,
    ) -> IntoResponse<'a, Collection<serde_json::Value>, Client> {
        self.client
            .client()
            .set_method(Method::GET)
            .as_mut()
            .extend_path(&["calendar", "calendarView"]);
        self.client
            .client()
            .as_mut()
            .append_query_pair("startDateTime", start_date_time);
        self.client
            .client()
            .as_mut()
            .append_query_pair("endDateTime", end_date_time);
        IntoResponse::new(self.client)
    }

    pub fn list_view(
        &self,
        id: &str,
        start_date_time: &str,
        end_date_time: &str,
    ) -> IntoResponse<'a, Collection<serde_json::Value>, Client> {
        self.client
            .client()
            .set_method(Method::GET)
            .as_mut()
            .extend_path(&["calendars", id, "calendarView"]);
        self.client
            .client()
            .as_mut()
            .append_query_pair("startDateTime", start_date_time);
        self.client
            .client()
            .as_mut()
            .append_query_pair("endDateTime", end_date_time);
        IntoResponse::new(self.client)
    }

    pub fn list_default_group_view(
        &self,
        calendar_id: &str,
        start_date_time: &str,
        end_date_time: &str,
    ) -> IntoResponse<'a, Collection<serde_json::Value>, Client> {
        self.client
            .client()
            .set_method(Method::GET)
            .as_mut()
            .extend_path(&["calendarGroup", "calendars", calendar_id, "calendarView"]);
        self.client
            .client()
            .as_mut()
            .append_query_pair("startDateTime", start_date_time);
        self.client
            .client()
            .as_mut()
            .append_query_pair("endDateTime", end_date_time);
        IntoResponse::new(self.client)
    }

    pub fn list_group_view(
        &self,
        calendar_group_id: &str,
        calendar_id: &str,
        start_date_time: &str,
        end_date_time: &str,
    ) -> IntoResponse<'a, Collection<serde_json::Value>, Client> {
        self.client
            .client()
            .set_method(Method::GET)
            .as_mut()
            .extend_path(&[
                "calendarGroup",
                calendar_group_id,
                "calendars",
                calendar_id,
                "calendarView",
            ]);
        self.client
            .client()
            .as_mut()
            .append_query_pair("startDateTime", start_date_time);
        self.client
            .client()
            .as_mut()
            .append_query_pair("endDateTime", end_date_time);
        IntoResponse::new(self.client)
    }
}

register_client!(CalendarGroupRequest,);

impl<'a, Client> CalendarGroupRequest<'a, Client>
where
    Client: crate::http::RequestClient,
{
    get!( list, Collection<serde_json::Value> => "calendarGroups" );
    get!( | get, serde_json::Value => "calendarGroups/{{id}}" );
    get!( list_default_calendars, Collection<serde_json::Value> => "calendarGroup/calendars" );
    get!( | list_calendars, Collection<serde_json::Value> => "calendarGroups/{{id}}/calendars" );
    get!( || list_events, Collection<serde_json::Value> => "calendarGroup/{{id}}/calendars/{{id2}}/events" );
    post!( [ create, serde_json::Value => "calendarGroups" ] );
    post!([
        create_default_calendar,
        serde_json::Value =>
        "calendarGroups/calendars"
    ]);
    post!( [ | create_calendar, serde_json::Value => "calendarGroups/{{id}}/calendars" ] );
    patch!( [ | update, serde_json::Value => "calendarGroups/{{id}}" ] );
    delete!( | delete, GraphResponse<Content> => "calendarGroups/{{id}}" );

    pub fn attachments(&'a self) -> CalendarGroupAttachmentRequest<'a, Client> {
        CalendarGroupAttachmentRequest::new(self.client)
    }
}
