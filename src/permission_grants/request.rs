// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    PermissionGrantsApiClient,
    PermissionGrantsIdApiClient,
    ResourceIdentity::PermissionGrants
);

impl PermissionGrantsApiClient {
    post!(
        doc: "Add new entity to permissionGrants",
        name: create_resource_specific_permission_grant,
        path: "/permissionGrants",
        body: true
    );
    get!(
        doc: "Get entities from permissionGrants",
        name: list_resource_specific_permission_grant,
        path: "/permissionGrants"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/permissionGrants/delta()"
    );
    post!(
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        path: "/permissionGrants/getAvailableExtensionProperties",
        body: true
    );
    post!(
        doc: "Invoke action getByIds",
        name: get_by_ids,
        path: "/permissionGrants/getByIds",
        body: true
    );
    post!(
        doc: "Invoke action validateProperties",
        name: validate_properties,
        path: "/permissionGrants/validateProperties",
        body: true
    );
}

impl PermissionGrantsIdApiClient {
    delete!(
        doc: "Delete entity from permissionGrants",
        name: delete_resource_specific_permission_grant,
        path: "/permissionGrants/{{RID}}"
    );
    get!(
        doc: "Get entity from permissionGrants by key",
        name: get_resource_specific_permission_grant,
        path: "/permissionGrants/{{RID}}"
    );
    patch!(
        doc: "Update entity in permissionGrants",
        name: update_resource_specific_permission_grant,
        path: "/permissionGrants/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberGroups",
        name: check_member_groups,
        path: "/permissionGrants/{{RID}}/checkMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberObjects",
        name: check_member_objects,
        path: "/permissionGrants/{{RID}}/checkMemberObjects",
        body: true
    );
    post!(
        doc: "Invoke action getMemberGroups",
        name: get_member_groups,
        path: "/permissionGrants/{{RID}}/getMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action getMemberObjects",
        name: get_member_objects,
        path: "/permissionGrants/{{RID}}/getMemberObjects",
        body: true
    );
    post!(
        doc: "Invoke action restore",
        name: restore,
        path: "/permissionGrants/{{RID}}/restore"
    );
}
