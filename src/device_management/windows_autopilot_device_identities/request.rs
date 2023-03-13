// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    WindowsAutopilotDeviceIdentitiesApiClient,
    WindowsAutopilotDeviceIdentitiesIdApiClient,
    ResourceIdentity::WindowsAutopilotDeviceIdentities
);

impl WindowsAutopilotDeviceIdentitiesApiClient {
    post!(
        doc: "Create new navigation property to windowsAutopilotDeviceIdentities for deviceManagement",
        name: create_windows_autopilot_device_identities,
        path: "/windowsAutopilotDeviceIdentities",
        body: true
    );
    get!(
        doc: "Get windowsAutopilotDeviceIdentities from deviceManagement",
        name: list_windows_autopilot_device_identities,
        path: "/windowsAutopilotDeviceIdentities"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_windows_autopilot_device_identities_count,
        path: "/windowsAutopilotDeviceIdentities/$count"
    );
}

impl WindowsAutopilotDeviceIdentitiesIdApiClient {
    delete!(
        doc: "Delete navigation property windowsAutopilotDeviceIdentities for deviceManagement",
        name: delete_windows_autopilot_device_identities,
        path: "/windowsAutopilotDeviceIdentities/{{RID}}"
    );
    get!(
        doc: "Get windowsAutopilotDeviceIdentities from deviceManagement",
        name: get_windows_autopilot_device_identities,
        path: "/windowsAutopilotDeviceIdentities/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property windowsAutopilotDeviceIdentities in deviceManagement",
        name: update_windows_autopilot_device_identities,
        path: "/windowsAutopilotDeviceIdentities/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action assignUserToDevice",
        name: assign_user_to_device,
        path: "/windowsAutopilotDeviceIdentities/{{RID}}/assignUserToDevice",
        body: true
    );
    post!(
        doc: "Invoke action unassignUserFromDevice",
        name: unassign_user_from_device,
        path: "/windowsAutopilotDeviceIdentities/{{RID}}/unassignUserFromDevice"
    );
    post!(
        doc: "Invoke action updateDeviceProperties",
        name: update_device_properties,
        path: "/windowsAutopilotDeviceIdentities/{{RID}}/updateDeviceProperties",
        body: true
    );
}
