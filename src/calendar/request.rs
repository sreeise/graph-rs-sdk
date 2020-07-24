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
    get!( list_events, Collection<serde_json::Value> => "calendar/events" );
    patch!( [ update_default, serde_json::Value => "calendar" ] );
    patch!( [ | update, serde_json::Value => "calendars/{{id}}" ] );
    post!( [ create, serde_json::Value => "calendars" ] );
    post!( [ create_event, serde_json::Value => "calendar/events" ] );
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
        let request = self.client.request();
        request.set_method(Method::GET);
        request.url_mut(|url| {
            url.extend_path(&["calendar", "calendarView"]);
            url.append_query_pair("startDateTime", start_date_time);
            url.append_query_pair("endDateTime", end_date_time);
        });
        IntoResponse::new(self.client)
    }

    pub fn list_view(
        &self,
        id: &str,
        start_date_time: &str,
        end_date_time: &str,
    ) -> IntoResponse<'a, Collection<serde_json::Value>, Client> {
        let request = self.client.request();
        request.set_method(Method::GET);
        request.url_mut(|url| {
            url.extend_path(&["calendars", id, "calendarView"]);
            url.append_query_pair("startDateTime", start_date_time);
            url.append_query_pair("endDateTime", end_date_time);
        });
        IntoResponse::new(self.client)
    }

    pub fn list_default_group_view(
        &self,
        calendar_id: &str,
        start_date_time: &str,
        end_date_time: &str,
    ) -> IntoResponse<'a, Collection<serde_json::Value>, Client> {
        let request = self.client.request();
        request.set_method(Method::GET);
        request.url_mut(|url| {
            url.extend_path(&["calendarGroup", "calendars", calendar_id, "calendarView"]);
            url.append_query_pair("startDateTime", start_date_time);
            url.append_query_pair("endDateTime", end_date_time);
        });
        IntoResponse::new(self.client)
    }

    pub fn list_group_view(
        &self,
        calendar_group_id: &str,
        calendar_id: &str,
        start_date_time: &str,
        end_date_time: &str,
    ) -> IntoResponse<'a, Collection<serde_json::Value>, Client> {
        let request = self.client.request();
        request.set_method(Method::GET);
        request.url_mut(|url| {
            url.extend_path(&[
                "calendarGroup",
                calendar_group_id,
                "calendars",
                calendar_id,
                "calendarView",
            ]);
            url.append_query_pair("startDateTime", start_date_time);
            url.append_query_pair("endDateTime", end_date_time);
        });
        IntoResponse::new(self.client)
    }
}

register_client!(CalendarGroupRequest,);

impl<'a, Client> CalendarGroupRequest<'a, Client>
where
    Client: crate::http::RequestClient,
{
    get!( list, Collection<serde_json::Value> => "calendarGroups" );
    get!( list_default_calendars, Collection<serde_json::Value> => "calendarGroup/calendars" );
    get!( | get, serde_json::Value => "calendarGroups/{{id}}" );
    get!( | list_calendars, Collection<serde_json::Value> => "calendarGroups/{{id}}/calendars" );
    get!( || list_events, Collection<serde_json::Value> => "calendarGroups/{{id}}/calendars/{{id2}}/events" );
    get!( | list_default_events, Collection<serde_json::Value> => "calendarGroup/calendars/{{id}}/events" );
    post!( [ || create_event, serde_json::Value => "calendarGroups/{{id}}/calendars/{{id2}}/events" ] );
    post!( [ | create_default_event, serde_json::Value => "calendarGroup/calendars/{{id}}/events" ] );
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
