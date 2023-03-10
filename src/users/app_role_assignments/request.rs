// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    AppRoleAssignmentsApiClient,
    AppRoleAssignmentsIdApiClient,
    ResourceIdentity::AppRoleAssignments
);

impl AppRoleAssignmentsApiClient {
    get!(
        doc: "Get appRoleAssignments from users",
        name: list_app_role_assignments,
        path: "/appRoleAssignments"
    );
    post!(
        doc: "Grant an appRoleAssignment to a user",
        name: create_app_role_assignments,
        path: "/appRoleAssignments",
        body: true
    );
    get!(
        doc: "Get the number of the resource",
        name: get_app_role_assignments_count,
        path: "/appRoleAssignments/$count"
    );
}

impl AppRoleAssignmentsIdApiClient {
    delete!(
        doc: "Delete navigation property appRoleAssignments for users",
        name: delete_app_role_assignments,
        path: "/appRoleAssignments/{{RID}}"
    );
    get!(
        doc: "Get appRoleAssignments from users",
        name: get_app_role_assignments,
        path: "/appRoleAssignments/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property appRoleAssignments in users",
        name: update_app_role_assignments,
        path: "/appRoleAssignments/{{RID}}",
        body: true
    );
}
