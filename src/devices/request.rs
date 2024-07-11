// GENERATED CODE

use crate::api_default_imports::*;
use crate::devices::*;
use crate::users::MemberOfApiClient;
use crate::users::MemberOfIdApiClient;
use crate::users::TransitiveMemberOfApiClient;
use crate::users::TransitiveMemberOfIdApiClient;

api_client!(
    DevicesApiClient,
    DevicesIdApiClient,
    ResourceIdentity::Devices
);

impl DevicesApiClient {
    post!(
        doc: "Create device",
        name: create_device,
        path: "/devices",
        body: true
    );
    get!(
        doc: "List devices",
        name: list_device,
        path: "/devices"
    );
    delete!(
        doc: "Delete device",
        name: delete_device_by_device_id,
        path: "/devices(deviceId='{{id}}')",
        params: device_id
    );
    get!(
        doc: "Get device",
        name: get_device_by_device_id,
        path: "/devices(deviceId='{{id}}')",
        params: device_id
    );
    patch!(
        doc: "Update device",
        name: update_device_by_device_id,
        path: "/devices(deviceId='{{id}}')",
        body: true,
        params: device_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_devices_count,
        path: "/devices/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/devices/delta()"
    );
    post!(
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        path: "/devices/getAvailableExtensionProperties",
        body: true
    );
    post!(
        doc: "Invoke action getByIds",
        name: get_by_ids,
        path: "/devices/getByIds",
        body: true
    );
    post!(
        doc: "Invoke action validateProperties",
        name: validate_properties,
        path: "/devices/validateProperties",
        body: true
    );
}

impl DevicesIdApiClient {
    api_client_link!(registered_users, DevicesRegisteredUsersApiClient);
    api_client_link_id!(registered_user, DevicesRegisteredUsersIdApiClient);
    api_client_link!(members_of, MemberOfApiClient);
    api_client_link_id!(registered_owner, DevicesRegisteredOwnersIdApiClient);
    api_client_link!(transitive_members_of, TransitiveMemberOfApiClient);
    api_client_link_id!(member_of, MemberOfIdApiClient);
    api_client_link_id!(transitive_member_of, TransitiveMemberOfIdApiClient);
    api_client_link!(registered_owners, DevicesRegisteredOwnersApiClient);

    delete!(
        doc: "Delete device",
        name: delete_device,
        path: "/devices/{{RID}}"
    );
    get!(
        doc: "Get device",
        name: get_device,
        path: "/devices/{{RID}}"
    );
    patch!(
        doc: "Update device",
        name: update_device,
        path: "/devices/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberGroups",
        name: check_member_groups,
        path: "/devices/{{RID}}/checkMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberObjects",
        name: check_member_objects,
        path: "/devices/{{RID}}/checkMemberObjects",
        body: true
    );
    post!(
        doc: "Create new navigation property to extensions for devices",
        name: create_extensions,
        path: "/devices/{{RID}}/extensions",
        body: true
    );
    get!(
        doc: "Get extensions from devices",
        name: list_extensions,
        path: "/devices/{{RID}}/extensions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_extensions_count,
        path: "/devices/{{RID}}/extensions/$count"
    );
    delete!(
        doc: "Delete navigation property extensions for devices",
        name: delete_extensions,
        path: "/devices/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    get!(
        doc: "Get extensions from devices",
        name: get_extensions,
        path: "/devices/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    patch!(
        doc: "Update the navigation property extensions in devices",
        name: update_extensions,
        path: "/devices/{{RID}}/extensions/{{id}}",
        body: true,
        params: extension_id
    );
    post!(
        doc: "Invoke action getMemberGroups",
        name: get_member_groups,
        path: "/devices/{{RID}}/getMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action getMemberObjects",
        name: get_member_objects,
        path: "/devices/{{RID}}/getMemberObjects",
        body: true
    );
    post!(
        doc: "Invoke action restore",
        name: restore,
        path: "/devices/{{RID}}/restore"
    );
}
