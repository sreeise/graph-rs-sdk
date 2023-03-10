// GENERATED CODE

use crate::api_default_imports::*;
use crate::extended_properties::*;
use crate::users::*;

resource_api_client!(
    CalendarsApiClient,
    CalendarsIdApiClient,
    ResourceIdentity::Calendars
);

impl CalendarsApiClient {
    post!(
        doc: "Create calendar",
        name: create_calendars,
        path: "/calendars",
        body: true
    );
    get!(
        doc: "List calendars",
        name: list_calendars,
        path: "/calendars"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_calendars_count,
        path: "/calendars/$count"
    );
}

impl CalendarsIdApiClient {
    api_client_link!(calendar_views, CalendarViewApiClient);
    api_client_link_id!(calendar_view, CalendarViewIdApiClient);
    api_client_link_id!(event, EventsIdApiClient);
    api_client_link!(events, EventsApiClient);
    api_client_link!(extended_properties, ExtendedPropertiesApiClient);

    delete!(
        doc: "Delete navigation property calendars for users",
        name: delete_calendars,
        path: "/calendars/{{RID}}"
    );
    get!(
        doc: "Get calendars from users",
        name: get_calendars,
        path: "/calendars/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property calendars in users",
        name: update_calendars,
        path: "/calendars/{{RID}}",
        body: true
    );
    get!(
        doc: "Invoke function allowedCalendarSharingRoles",
        name: allowed_calendar_sharing_roles,
        path: "/calendars/{{RID}}/allowedCalendarSharingRoles(User='{{id}}')",
        params: user
    );
    post!(
        doc: "Create calendarPermission",
        name: create_calendar_permissions,
        path: "/calendars/{{RID}}/calendarPermissions",
        body: true
    );
    get!(
        doc: "Get calendarPermissions from users",
        name: list_calendar_permissions,
        path: "/calendars/{{RID}}/calendarPermissions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_calendar_permissions_count,
        path: "/calendars/{{RID}}/calendarPermissions/$count"
    );
    delete!(
        doc: "Delete navigation property calendarPermissions for users",
        name: delete_calendar_permissions,
        path: "/calendars/{{RID}}/calendarPermissions/{{id}}",
        params: calendar_permission_id
    );
    get!(
        doc: "Get calendarPermissions from users",
        name: get_calendar_permissions,
        path: "/calendars/{{RID}}/calendarPermissions/{{id}}",
        params: calendar_permission_id
    );
    patch!(
        doc: "Update the navigation property calendarPermissions in users",
        name: update_calendar_permissions,
        path: "/calendars/{{RID}}/calendarPermissions/{{id}}",
        body: true,
        params: calendar_permission_id
    );
    post!(
        doc: "Invoke action getSchedule",
        name: get_schedule,
        path: "/calendars/{{RID}}/getSchedule",
        body: true
    );
}
