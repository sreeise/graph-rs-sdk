// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    EntitlementManagementAssignmentsApiClient,
    EntitlementManagementAssignmentsIdApiClient,
    ResourceIdentity::EntitlementManagementAssignments
);

impl EntitlementManagementAssignmentsApiClient {
    post!(
        doc: "Create new navigation property to assignments for identityGovernance",
        name: create_assignments,
        path: "/assignments",
        body: true
    );
    get!(
        doc: "List assignments",
        name: list_assignments,
        path: "/assignments"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_assignments_count,
        path: "/assignments/$count"
    );
    get!(
        doc: "Invoke function additionalAccess",
        name: additional_access,
        path: "/assignments/additionalAccess(accessPackageId='{{id}}',incompatibleAccessPackageId='{{id2}}')",
        params: access_package_id, incompatible_access_package_id
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        path: "/assignments/filterByCurrentUser(on='{{id}}')",
        params: on
    );
}

impl EntitlementManagementAssignmentsIdApiClient {
    delete!(
        doc: "Delete navigation property assignments for identityGovernance",
        name: delete_assignments,
        path: "/assignments/{{RID}}"
    );
    get!(
        doc: "Get assignments from identityGovernance",
        name: get_assignments,
        path: "/assignments/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property assignments in identityGovernance",
        name: update_assignments,
        path: "/assignments/{{RID}}",
        body: true
    );
    get!(
        doc: "Get accessPackage from identityGovernance",
        name: get_access_package,
        path: "/assignments/{{RID}}/accessPackage"
    );
    get!(
        doc: "Get assignmentPolicy from identityGovernance",
        name: get_assignment_policy,
        path: "/assignments/{{RID}}/assignmentPolicy"
    );
    post!(
        doc: "Invoke action reprocess",
        name: reprocess,
        path: "/assignments/{{RID}}/reprocess"
    );
    get!(
        doc: "Get target from identityGovernance",
        name: get_target,
        path: "/assignments/{{RID}}/target"
    );
}
