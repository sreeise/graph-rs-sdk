// GENERATED CODE

use crate::api_default_imports::*;
use crate::users::*;

resource_api_client!(
    CalendarViewApiClient,
    CalendarViewIdApiClient,
    ResourceIdentity::CalendarView
);

impl CalendarViewApiClient {
    get!(
        doc: "Get calendarView from users",
        name: list_calendar_view,
        path: "/calendarView"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_calendar_view_count,
        path: "/calendarView/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/calendarView/delta()"
    );
}

impl CalendarViewIdApiClient {
    api_client_link_id!(instance, EventsInstancesIdApiClient);
    api_client_link!(instances, EventsInstancesApiClient);

    get!(
        doc: "Get calendarView from users",
        name: get_calendar_view,
        path: "/calendarView/{{RID}}"
    );
    post!(
        doc: "Invoke action accept",
        name: accept,
        path: "/calendarView/{{RID}}/accept",
        body: true
    );
    post!(
        doc: "Add attachment",
        name: create_attachments,
        path: "/calendarView/{{RID}}/attachments",
        body: true
    );
    get!(
        doc: "List attachments",
        name: list_attachments,
        path: "/calendarView/{{RID}}/attachments"
    );
    get!(
        doc: "Get calendar from users",
        name: get_calendar,
        path: "/calendarView/{{RID}}/calendar"
    );
    post!(
        doc: "Invoke action cancel",
        name: cancel,
        path: "/calendarView/{{RID}}/cancel",
        body: true
    );
    post!(
        doc: "Invoke action decline",
        name: decline,
        path: "/calendarView/{{RID}}/decline",
        body: true
    );
    post!(
        doc: "Invoke action dismissReminder",
        name: dismiss_reminder,
        path: "/calendarView/{{RID}}/dismissReminder"
    );
    post!(
        doc: "Create open extension",
        name: create_extensions,
        path: "/calendarView/{{RID}}/extensions",
        body: true
    );
    get!(
        doc: "Get extensions from users",
        name: list_extensions,
        path: "/calendarView/{{RID}}/extensions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_extensions_count,
        path: "/calendarView/{{RID}}/extensions/$count"
    );
    delete!(
        doc: "Delete navigation property extensions for users",
        name: delete_extensions,
        path: "/calendarView/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    get!(
        doc: "Get extensions from users",
        name: get_extensions,
        path: "/calendarView/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    patch!(
        doc: "Update the navigation property extensions in users",
        name: update_extensions,
        path: "/calendarView/{{RID}}/extensions/{{id}}",
        body: true,
        params: extension_id
    );
    post!(
        doc: "Invoke action forward",
        name: forward,
        path: "/calendarView/{{RID}}/forward",
        body: true
    );
    post!(
        doc: "Invoke action snoozeReminder",
        name: snooze_reminder,
        path: "/calendarView/{{RID}}/snoozeReminder",
        body: true
    );
    post!(
        doc: "Invoke action tentativelyAccept",
        name: tentatively_accept,
        path: "/calendarView/{{RID}}/tentativelyAccept",
        body: true
    );
}
