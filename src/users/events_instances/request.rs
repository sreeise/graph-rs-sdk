// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    EventsInstancesApiClient,
    EventsInstancesIdApiClient,
    ResourceIdentity::EventsInstances
);

impl EventsInstancesApiClient {
    get!(
        doc: "List instances",
        name: list_instances,
        path: "/instances"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_instances_count,
        path: "/instances/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/instances/delta()"
    );
}

impl EventsInstancesIdApiClient {
    get!(
        doc: "Get instances from users",
        name: get_instances,
        path: "/instances/{{RID}}"
    );
    post!(
        doc: "Invoke action accept",
        name: accept,
        path: "/instances/{{RID}}/accept",
        body: true
    );
    post!(
        doc: "Add attachment",
        name: create_attachments,
        path: "/instances/{{RID}}/attachments",
        body: true
    );
    get!(
        doc: "List attachments",
        name: list_attachments,
        path: "/instances/{{RID}}/attachments"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_attachments_count,
        path: "/instances/{{RID}}/attachments/$count"
    );
    post!(
        doc: "Invoke action createUploadSession",
        name: create_upload_session,
        path: "/instances/{{RID}}/attachments/createUploadSession",
        body: true
    );
    delete!(
        doc: "Delete navigation property attachments for users",
        name: delete_attachments,
        path: "/instances/{{RID}}/attachments/{{id}}",
        params: attachment_id
    );
    get!(
        doc: "Get attachments from users",
        name: get_attachments,
        path: "/instances/{{RID}}/attachments/{{id}}",
        params: attachment_id
    );
    get!(
        doc: "Get calendar from users",
        name: get_calendar,
        path: "/instances/{{RID}}/calendar"
    );
    post!(
        doc: "Invoke action cancel",
        name: cancel,
        path: "/instances/{{RID}}/cancel",
        body: true
    );
    post!(
        doc: "Invoke action decline",
        name: decline,
        path: "/instances/{{RID}}/decline",
        body: true
    );
    post!(
        doc: "Invoke action dismissReminder",
        name: dismiss_reminder,
        path: "/instances/{{RID}}/dismissReminder"
    );
    post!(
        doc: "Create open extension",
        name: create_extensions,
        path: "/instances/{{RID}}/extensions",
        body: true
    );
    get!(
        doc: "Get extensions from users",
        name: list_extensions,
        path: "/instances/{{RID}}/extensions"
    );
    get!(
        doc: "Get the number of the resource",
        name: count,
        path: "/instances/{{RID}}/extensions/$count"
    );
    delete!(
        doc: "Delete navigation property extensions for users",
        name: delete_extensions,
        path: "/instances/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    get!(
        doc: "Get extensions from users",
        name: get_extensions,
        path: "/instances/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    patch!(
        doc: "Update the navigation property extensions in users",
        name: update_extensions,
        path: "/instances/{{RID}}/extensions/{{id}}",
        body: true,
        params: extension_id
    );
    post!(
        doc: "Invoke action forward",
        name: forward,
        path: "/instances/{{RID}}/forward",
        body: true
    );
    post!(
        doc: "Invoke action snoozeReminder",
        name: snooze_reminder,
        path: "/instances/{{RID}}/snoozeReminder",
        body: true
    );
    post!(
        doc: "Invoke action tentativelyAccept",
        name: tentatively_accept,
        path: "/instances/{{RID}}/tentativelyAccept",
        body: true
    );
}
