// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    TroubleshootingEventsApiClient,
    TroubleshootingEventsIdApiClient,
    ResourceIdentity::TroubleshootingEvents
);

impl TroubleshootingEventsApiClient {
    post!(
        doc: "Create new navigation property to troubleshootingEvents for deviceManagement",
        name: create_troubleshooting_events,
        path: "/troubleshootingEvents",
        body: true
    );
    get!(
        doc: "Get troubleshootingEvents from deviceManagement",
        name: list_troubleshooting_events,
        path: "/troubleshootingEvents"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_troubleshooting_events_count,
        path: "/troubleshootingEvents/$count"
    );
}

impl TroubleshootingEventsIdApiClient {
    delete!(
        doc: "Delete navigation property troubleshootingEvents for deviceManagement",
        name: delete_troubleshooting_events,
        path: "/troubleshootingEvents/{{RID}}"
    );
    get!(
        doc: "Get troubleshootingEvents from deviceManagement",
        name: get_troubleshooting_events,
        path: "/troubleshootingEvents/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property troubleshootingEvents in deviceManagement",
        name: update_troubleshooting_events,
        path: "/troubleshootingEvents/{{RID}}",
        body: true
    );
}
