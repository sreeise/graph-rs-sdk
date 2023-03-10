// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    OwnedDevicesApiClient,
    OwnedDevicesIdApiClient,
    ResourceIdentity::OwnedDevices
);

impl OwnedDevicesApiClient {
    get!(
        doc: "Get ownedDevices from users",
        name: list_owned_devices,
        path: "/ownedDevices"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_owned_devices_count,
        path: "/ownedDevices/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.appRoleAssignment in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_app_role_assignment_type,
        path: "/ownedDevices/graph.appRoleAssignment"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_app_role_assignment_count,
        path: "/ownedDevices/graph.appRoleAssignment/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.device in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_device_type,
        path: "/ownedDevices/graph.device"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_count,
        path: "/ownedDevices/graph.device/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.endpoint in the microsoft.graph.directoryObject collection",
        name: get_directory_object_items_as_endpoint_type,
        path: "/ownedDevices/graph.endpoint"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_endpoint_count,
        path: "/ownedDevices/graph.endpoint/$count"
    );
}

impl OwnedDevicesIdApiClient {
    get!(
        doc: "Get ownedDevices from users",
        name: get_owned_devices,
        path: "/ownedDevices/{{RID}}"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.appRoleAssignment",
        name: get_directory_object_item_as_app_role_assignment_type,
        path: "/ownedDevices/{{RID}}/graph.appRoleAssignment"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.device",
        name: get_directory_object_item_as_device_type,
        path: "/ownedDevices/{{RID}}/graph.device"
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.endpoint",
        name: get_directory_object_item_as_endpoint_type,
        path: "/ownedDevices/{{RID}}/graph.endpoint"
    );
}
