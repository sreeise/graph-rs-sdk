// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    RegisteredDevicesApiClient,
    RegisteredDevicesIdApiClient,
    ResourceIdentity::RegisteredDevices
);

impl RegisteredDevicesApiClient {
    get!(
        doc: "Get registeredDevices from users",
        name: list_registered_devices,
        path: "/registeredDevices"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_registered_devices_count,
        path: "/registeredDevices/$count"
    );

    get!(
        doc: "Get the number of the resource",
        name: get_endpoint_count,
        path: "/registeredDevices/graph.endpoint/$count"
    );
}

impl RegisteredDevicesIdApiClient {
    get!(
        doc: "Get registeredDevices from users",
        name: get_registered_devices,
        path: "/registeredDevices/{{RID}}"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.appRoleAssignment",
        name: get_directory_object_item_as_app_role_assignment_type,
        path: "/registeredDevices/{{RID}}/graph.appRoleAssignment"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.device",
        name: get_directory_object_item_as_device_type,
        path: "/registeredDevices/{{RID}}/graph.device"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.endpoint",
        name: get_directory_object_item_as_endpoint_type,
        path: "/registeredDevices/{{RID}}/graph.endpoint"
    );
}
