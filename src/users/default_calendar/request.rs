// GENERATED CODE

use crate::api_default_imports::*;
use crate::extended_properties::*;
use crate::users::*;

resource_api_client!(DefaultCalendarApiClient, ResourceIdentity::DefaultCalendar);

impl DefaultCalendarApiClient {
    api_client_link_id!(event, EventsIdApiClient);
    api_client_link_id!(calendar_view, CalendarViewIdApiClient);
    api_client_link!(extended_properties, ExtendedPropertiesApiClient);
    api_client_link!(calendar_views, CalendarViewApiClient);
    api_client_link!(events, EventsApiClient);

    get!(
        doc: "Invoke function allowedCalendarSharingRoles",
        name: allowed_calendar_sharing_roles,
        path: "/calendar/allowedCalendarSharingRoles(User='{{id}}')",
        params: user
    );
    post!(
        doc: "Create calendarPermission",
        name: create_calendar_permissions,
        path: "/calendar/calendarPermissions",
        body: true
    );
    get!(
        doc: "Get calendarPermissions from users",
        name: list_calendar_permissions,
        path: "/calendar/calendarPermissions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_calendar_permissions_count,
        path: "/calendar/calendarPermissions/$count"
    );
    delete!(
        doc: "Delete navigation property calendarPermissions for users",
        name: delete_calendar_permissions,
        path: "/calendar/calendarPermissions/{{id}}",
        params: calendar_permission_id
    );
    get!(
        doc: "Get calendarPermissions from users",
        name: get_calendar_permissions,
        path: "/calendar/calendarPermissions/{{id}}",
        params: calendar_permission_id
    );
    patch!(
        doc: "Update the navigation property calendarPermissions in users",
        name: update_calendar_permissions,
        path: "/calendar/calendarPermissions/{{id}}",
        body: true,
        params: calendar_permission_id
    );
    post!(
        doc: "Create event",
        name: create_events,
        path: "/calendar/events",
        body: true
    );
    get!(
        doc: "List events",
        name: list_events,
        path: "/calendar/events"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_events_count,
        path: "/calendar/events/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/calendar/events/delta()"
    );
    post!(
        doc: "Invoke action getSchedule",
        name: get_schedule,
        path: "/calendar/getSchedule",
        body: true
    );
}
