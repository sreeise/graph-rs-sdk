// GENERATED CODE

use crate::api_default_imports::*;
use crate::directory::*;

api_client!(
    DirectoryRolesApiClient,
    DirectoryRolesIdApiClient,
    ResourceIdentity::DirectoryRoles
);

impl DirectoryRolesApiClient {
    post!(
        doc: "Activate directoryRole",
        name: create_directory_role,
        path: "/directoryRoles",
        body: true
    );
    get!(
        doc: "List directoryRoles",
        name: list_directory_role,
        path: "/directoryRoles"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_directory_roles_count,
        path: "/directoryRoles/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/directoryRoles/delta()"
    );
    post!(
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        path: "/directoryRoles/getAvailableExtensionProperties",
        body: true
    );
    post!(
        doc: "Invoke action getByIds",
        name: get_by_ids,
        path: "/directoryRoles/getByIds",
        body: true
    );
    post!(
        doc: "Invoke action validateProperties",
        name: validate_properties,
        path: "/directoryRoles/validateProperties",
        body: true
    );
}

impl DirectoryRolesIdApiClient {
    api_client_link_id!(member, DirectoryMembersIdApiClient);
    api_client_link!(members, DirectoryMembersApiClient);

    delete!(
        doc: "Delete entity from directoryRoles",
        name: delete_directory_role,
        path: "/directoryRoles/{{RID}}"
    );
    get!(
        doc: "Get directoryRole",
        name: get_directory_role,
        path: "/directoryRoles/{{RID}}"
    );
    patch!(
        doc: "Update entity in directoryRoles",
        name: update_directory_role,
        path: "/directoryRoles/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberGroups",
        name: check_member_groups,
        path: "/directoryRoles/{{RID}}/checkMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberObjects",
        name: check_member_objects,
        path: "/directoryRoles/{{RID}}/checkMemberObjects",
        body: true
    );
    post!(
        doc: "Invoke action getMemberGroups",
        name: get_member_groups,
        path: "/directoryRoles/{{RID}}/getMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action getMemberObjects",
        name: get_member_objects,
        path: "/directoryRoles/{{RID}}/getMemberObjects",
        body: true
    );
    post!(
        doc: "Invoke action restore",
        name: restore,
        path: "/directoryRoles/{{RID}}/restore"
    );
    post!(
        doc: "Create new navigation property to scopedMembers for directoryRoles",
        name: create_scoped_members,
        path: "/directoryRoles/{{RID}}/scopedMembers",
        body: true
    );
    get!(
        doc: "List scopedMembers for a directory role",
        name: list_scoped_members,
        path: "/directoryRoles/{{RID}}/scopedMembers"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_scoped_members_count,
        path: "/directoryRoles/{{RID}}/scopedMembers/$count"
    );
    delete!(
        doc: "Delete navigation property scopedMembers for directoryRoles",
        name: delete_scoped_members,
        path: "/directoryRoles/{{RID}}/scopedMembers/{{id}}",
        params: scoped_role_membership_id
    );
    get!(
        doc: "Get scopedMembers from directoryRoles",
        name: get_scoped_members,
        path: "/directoryRoles/{{RID}}/scopedMembers/{{id}}",
        params: scoped_role_membership_id
    );
    patch!(
        doc: "Update the navigation property scopedMembers in directoryRoles",
        name: update_scoped_members,
        path: "/directoryRoles/{{RID}}/scopedMembers/{{id}}",
        body: true,
        params: scoped_role_membership_id
    );
}
