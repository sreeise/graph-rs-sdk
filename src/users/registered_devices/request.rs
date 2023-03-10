// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
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
        doc: "Get the items of type microsoft.graph.appRoleAssignment in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_app_role_assignment_type,
        path: "/registeredDevices/graph.appRoleAssignment"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_app_role_assignment_count,
        path: "/registeredDevices/graph.appRoleAssignment/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.device in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_device_type,
        path: "/registeredDevices/graph.device"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_count,
        path: "/registeredDevices/graph.device/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.endpoint in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_endpoint_type,
        path: "/registeredDevices/graph.endpoint"
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
