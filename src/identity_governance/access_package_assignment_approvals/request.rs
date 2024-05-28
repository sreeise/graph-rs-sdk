// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    AccessPackageAssignmentApprovalsApiClient,
    AccessPackageAssignmentApprovalsIdApiClient,
    ResourceIdentity::AccessPackageAssignmentApprovals
);

impl AccessPackageAssignmentApprovalsApiClient {
    post!(
        doc: "Create new navigation property to accessPackageAssignmentApprovals for identityGovernance",
        name: create_access_package_assignment_approvals,
        path: "/accessPackageAssignmentApprovals",
        body: true
    );
    get!(
        doc: "Get accessPackageAssignmentApprovals from identityGovernance",
        name: list_access_package_assignment_approvals,
        path: "/accessPackageAssignmentApprovals"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_access_package_assignment_approvals_count,
        path: "/accessPackageAssignmentApprovals/$count"
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        path: "/accessPackageAssignmentApprovals/filterByCurrentUser(on='{{id}}')",
        params: on
    );
}

impl AccessPackageAssignmentApprovalsIdApiClient {
    delete!(
        doc: "Delete navigation property accessPackageAssignmentApprovals for identityGovernance",
        name: delete_access_package_assignment_approvals,
        path: "/accessPackageAssignmentApprovals/{{RID}}"
    );
    get!(
        doc: "Get accessPackageAssignmentApprovals from identityGovernance",
        name: get_access_package_assignment_approvals,
        path: "/accessPackageAssignmentApprovals/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property accessPackageAssignmentApprovals in identityGovernance",
        name: update_access_package_assignment_approvals,
        path: "/accessPackageAssignmentApprovals/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to stages for identityGovernance",
        name: create_stages,
        path: "/accessPackageAssignmentApprovals/{{RID}}/stages",
        body: true
    );
    get!(
        doc: "List approval stages",
        name: list_stages,
        path: "/accessPackageAssignmentApprovals/{{RID}}/stages"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_stages_count,
        path: "/accessPackageAssignmentApprovals/{{RID}}/stages/$count"
    );
    delete!(
        doc: "Delete navigation property stages for identityGovernance",
        name: delete_stages,
        path: "/accessPackageAssignmentApprovals/{{RID}}/stages/{{id}}",
        params: approval_stage_id
    );
    get!(
        doc: "Get stages from identityGovernance",
        name: get_stages,
        path: "/accessPackageAssignmentApprovals/{{RID}}/stages/{{id}}",
        params: approval_stage_id
    );
    patch!(
        doc: "Update the navigation property stages in identityGovernance",
        name: update_stages,
        path: "/accessPackageAssignmentApprovals/{{RID}}/stages/{{id}}",
        body: true,
        params: approval_stage_id
    );
}
