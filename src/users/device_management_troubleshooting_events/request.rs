// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    DeviceManagementTroubleshootingEventsApiClient,
    DeviceManagementTroubleshootingEventsIdApiClient,
    ResourceIdentity::TroubleshootingEvents
);

impl DeviceManagementTroubleshootingEventsApiClient {
    post!(
        doc: "Create new navigation property to deviceManagementTroubleshootingEvents for users",
        name: create_device_management_troubleshooting_events,
        path: "/deviceManagementTroubleshootingEvents",
        body: true
    );
    get!(
        doc: "Get deviceManagementTroubleshootingEvents from users",
        name: list_device_management_troubleshooting_events,
        path: "/deviceManagementTroubleshootingEvents"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_management_troubleshooting_events_count,
        path: "/deviceManagementTroubleshootingEvents/$count"
    );
}

impl DeviceManagementTroubleshootingEventsIdApiClient {
    delete!(
        doc: "Delete navigation property deviceManagementTroubleshootingEvents for users",
        name: delete_device_management_troubleshooting_events,
        path: "/deviceManagementTroubleshootingEvents/{{RID}}"
    );
    get!(
        doc: "Get deviceManagementTroubleshootingEvents from users",
        name: get_device_management_troubleshooting_events,
        path: "/deviceManagementTroubleshootingEvents/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property deviceManagementTroubleshootingEvents in users",
        name: update_device_management_troubleshooting_events,
        path: "/deviceManagementTroubleshootingEvents/{{RID}}",
        body: true
    );
}
