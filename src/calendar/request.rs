use crate::attachments::{CalendarAttachmentRequest, CalendarGroupAttachmentRequest};
use crate::client::Graph;
use crate::http::{GraphResponse, IntoResponse};
use crate::types::{collection::Collection, content::Content};
use graph_rs_types::entitytypes::{Calendar, CalendarGroup, Event};
use reqwest::Method;

register_client!(CalendarRequest,);

impl<'a> CalendarRequest<'a> {
    get!( list, Collection<Calendar> => "calendars" );
    get!( get_default, Calendar => "calendar" );
    get!( | get, Calendar => "calendars/{{id}}" );
    get!( list_events, Collection<Event> => "calendars/events" );
    patch!( [ update_default, Calendar => "calendar" ] );
    patch!( [ | update, Calendar => "calendars/{{id}}" ] );
    post!( | create, Calendar => "calendars" );
    delete!( | delete, GraphResponse<Content> => "calendars/{{id}}" );

    pub fn attachments(&'a self) -> CalendarAttachmentRequest<'a> {
        CalendarAttachmentRequest::new(self.client)
    }

    pub fn views(&self) -> CalendarViewRequest<'a> {
        CalendarViewRequest::new(self.client)
    }

    pub fn groups(&self) -> CalendarGroupRequest<'a> {
        CalendarGroupRequest::new(self.client)
    }
}

register_client!(CalendarViewRequest,);

impl<'a> CalendarViewRequest<'a> {
    pub fn list_default_view(
        &self,
        start_date_time: &str,
        end_date_time: &str,
    ) -> IntoResponse<'a, Collection<Calendar>> {
        self.client
            .builder()
            .set_method(Method::GET)
            .as_mut()
            .extend_path(&["calendar", "calendarView"]);
        self.client
            .builder()
            .as_mut()
            .append_query_pair("startDateTime", start_date_time);
        self.client
            .builder()
            .as_mut()
            .append_query_pair("endDateTime", end_date_time);
        IntoResponse::new(self.client)
    }

    pub fn list_view(
        &self,
        id: &str,
        start_date_time: &str,
        end_date_time: &str,
    ) -> IntoResponse<'a, Collection<Calendar>> {
        self.client
            .builder()
            .set_method(Method::GET)
            .as_mut()
            .extend_path(&["calendars", id, "calendarView"]);
        self.client
            .builder()
            .as_mut()
            .append_query_pair("startDateTime", start_date_time);
        self.client
            .builder()
            .as_mut()
            .append_query_pair("endDateTime", end_date_time);
        IntoResponse::new(self.client)
    }

    pub fn list_default_group_view(
        &self,
        calendar_id: &str,
        start_date_time: &str,
        end_date_time: &str,
    ) -> IntoResponse<'a, Collection<Calendar>> {
        self.client
            .builder()
            .set_method(Method::GET)
            .as_mut()
            .extend_path(&["calendarGroup", "calendars", calendar_id, "calendarView"]);
        self.client
            .builder()
            .as_mut()
            .append_query_pair("startDateTime", start_date_time);
        self.client
            .builder()
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
    ) -> IntoResponse<'a, Collection<Calendar>> {
        self.client
            .builder()
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
            .builder()
            .as_mut()
            .append_query_pair("startDateTime", start_date_time);
        self.client
            .builder()
            .as_mut()
            .append_query_pair("endDateTime", end_date_time);
        IntoResponse::new(self.client)
    }
}

register_client!(CalendarGroupRequest,);

impl<'a> CalendarGroupRequest<'a> {
    get!( list, Collection<CalendarGroup> => "calendarGroups" );
    get!( | get, CalendarGroup => "calendarGroups/{{id}}" );
    get!( list_default_calendars, Collection<Calendar> => "calendarGroup/calendars" );
    get!( | list_calendars, Collection<Calendar> => "calendarGroups/{{id}}/calendars" );
    get!( || list_events, Collection<Event> => "calendarGroup/{{id}}/calendars/{{id2}}/events" );
    post!( [ create, CalendarGroup => "calendarGroups" ] );
    post!([
        create_default_calendar,
        CalendarGroup =>
        "calendarGroups/calendars"
    ]);
    post!( [ | create_calendar, Calendar => "calendarGroups/{{id}}/calendars" ] );
    patch!( [ | update, CalendarGroup => "calendarGroups/{{id}}" ] );
    delete!( | delete, GraphResponse<Content> => "calendarGroups/{{id}}" );

    pub fn attachments(&'a self) -> CalendarGroupAttachmentRequest<'a> {
        CalendarGroupAttachmentRequest::new(self.client)
    }
}
