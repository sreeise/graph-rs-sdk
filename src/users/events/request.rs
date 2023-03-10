// GENERATED CODE

use crate::api_default_imports::*;
use crate::users::*;

resource_api_client!(EventsApiClient, EventsIdApiClient, ResourceIdentity::Events);

impl EventsApiClient {
    post!(
        doc: "Create Event",
        name: create_events,
        path: "/events",
        body: true
    );
    get!(
        doc: "List events",
        name: list_events,
        path: "/events"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_events_count,
        path: "/events/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/events/delta()"
    );
}

impl EventsIdApiClient {
    api_client_link!(instances, EventsInstancesApiClient);
    api_client_link_id!(instance, EventsInstancesIdApiClient);

    delete!(
        doc: "Delete navigation property events for users",
        name: delete_events,
        path: "/events/{{RID}}"
    );
    get!(
        doc: "Get events from users",
        name: get_events,
        path: "/events/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property events in users",
        name: update_events,
        path: "/events/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action accept",
        name: accept,
        path: "/events/{{RID}}/accept",
        body: true
    );
    post!(
        doc: "Add attachment",
        name: create_attachments,
        path: "/events/{{RID}}/attachments",
        body: true
    );
    get!(
        doc: "List attachments",
        name: list_attachments,
        path: "/events/{{RID}}/attachments"
    );
    get!(
        doc: "Get calendar from users",
        name: get_calendar,
        path: "/events/{{RID}}/calendar"
    );
    post!(
        doc: "Invoke action cancel",
        name: cancel,
        path: "/events/{{RID}}/cancel",
        body: true
    );
    post!(
        doc: "Invoke action decline",
        name: decline,
        path: "/events/{{RID}}/decline",
        body: true
    );
    post!(
        doc: "Invoke action dismissReminder",
        name: dismiss_reminder,
        path: "/events/{{RID}}/dismissReminder"
    );
    post!(
        doc: "Create open extension",
        name: create_extensions,
        path: "/events/{{RID}}/extensions",
        body: true
    );
    get!(
        doc: "Get extensions from users",
        name: list_extensions,
        path: "/events/{{RID}}/extensions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_extensions_count,
        path: "/events/{{RID}}/extensions/$count"
    );
    delete!(
        doc: "Delete navigation property extensions for users",
        name: delete_extensions,
        path: "/events/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    get!(
        doc: "Get extensions from users",
        name: get_extensions,
        path: "/events/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    patch!(
        doc: "Update the navigation property extensions in users",
        name: update_extensions,
        path: "/events/{{RID}}/extensions/{{id}}",
        body: true,
        params: extension_id
    );
    post!(
        doc: "Invoke action forward",
        name: forward,
        path: "/events/{{RID}}/forward",
        body: true
    );
    post!(
        doc: "Invoke action snoozeReminder",
        name: snooze_reminder,
        path: "/events/{{RID}}/snoozeReminder",
        body: true
    );
    post!(
        doc: "Invoke action tentativelyAccept",
        name: tentatively_accept,
        path: "/events/{{RID}}/tentativelyAccept",
        body: true
    );
}
