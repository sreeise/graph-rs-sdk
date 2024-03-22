// GENERATED CODE

use crate::api_default_imports::*;
use crate::directory::*;

api_client!(
    AdministrativeUnitsApiClient,
    AdministrativeUnitsIdApiClient,
    ResourceIdentity::AdministrativeUnits
);

impl AdministrativeUnitsApiClient {
    post!(
        doc: "Create administrativeUnit",
        name: create_administrative_units,
        path: "/administrativeUnits",
        body: true
    );
    get!(
        doc: "List administrativeUnits",
        name: list_administrative_units,
        path: "/administrativeUnits"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_administrative_units_count,
        path: "/administrativeUnits/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/administrativeUnits/delta()"
    );
}

impl AdministrativeUnitsIdApiClient {
    api_client_link!(members, DirectoryMembersApiClient);
    api_client_link_id!(member, DirectoryMembersIdApiClient);

    delete!(
        doc: "Delete navigation property administrativeUnits for directory",
        name: delete_administrative_units,
        path: "/administrativeUnits/{{RID}}"
    );
    get!(
        doc: "Get administrativeUnits from directory",
        name: get_administrative_units,
        path: "/administrativeUnits/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property administrativeUnits in directory",
        name: update_administrative_units,
        path: "/administrativeUnits/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to extensions for directory",
        name: create_extensions,
        path: "/administrativeUnits/{{RID}}/extensions",
        body: true
    );
    get!(
        doc: "Get extensions from directory",
        name: list_extensions,
        path: "/administrativeUnits/{{RID}}/extensions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_extensions_count,
        path: "/administrativeUnits/{{RID}}/extensions/$count"
    );
    delete!(
        doc: "Delete navigation property extensions for directory",
        name: delete_extensions,
        path: "/administrativeUnits/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    get!(
        doc: "Get extensions from directory",
        name: get_extensions,
        path: "/administrativeUnits/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    patch!(
        doc: "Update the navigation property extensions in directory",
        name: update_extensions,
        path: "/administrativeUnits/{{RID}}/extensions/{{id}}",
        body: true,
        params: extension_id
    );
    post!(
        doc: "Add a scopedRoleMember",
        name: create_scoped_role_members,
        path: "/administrativeUnits/{{RID}}/scopedRoleMembers",
        body: true
    );
    get!(
        doc: "List scopedRoleMembers",
        name: list_scoped_role_members,
        path: "/administrativeUnits/{{RID}}/scopedRoleMembers"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_scoped_role_members_count,
        path: "/administrativeUnits/{{RID}}/scopedRoleMembers/$count"
    );
    delete!(
        doc: "Delete navigation property scopedRoleMembers for directory",
        name: delete_scoped_role_members,
        path: "/administrativeUnits/{{RID}}/scopedRoleMembers/{{id}}",
        params: scoped_role_membership_id
    );
    get!(
        doc: "Get scopedRoleMembers from directory",
        name: get_scoped_role_members,
        path: "/administrativeUnits/{{RID}}/scopedRoleMembers/{{id}}",
        params: scoped_role_membership_id
    );
    patch!(
        doc: "Update the navigation property scopedRoleMembers in directory",
        name: update_scoped_role_members,
        path: "/administrativeUnits/{{RID}}/scopedRoleMembers/{{id}}",
        body: true,
        params: scoped_role_membership_id
    );
}
