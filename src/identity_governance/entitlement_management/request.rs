// GENERATED CODE

use crate::api_default_imports::*;
use crate::assignment_requests::{AssignmentRequestsApiClient, AssignmentRequestsIdApiClient};
use crate::assignments::{AssignmentsApiClient, AssignmentsIdApiClient};
use crate::identity_governance::{
    AccessPackageAssignmentApprovalsApiClient, AccessPackageAssignmentApprovalsIdApiClient,
    AccessPackagesApiClient, AccessPackagesIdApiClient, AssignmentPoliciesApiClient,
    AssignmentPoliciesIdApiClient, ConnectedOrganizationsApiClient,
    ConnectedOrganizationsIdApiClient,
};

resource_api_client!(
    EntitlementManagementApiClient,
    ResourceIdentity::EntitlementManagement
);

impl EntitlementManagementApiClient {
    api_client_link_id!(
        access_package,
        ResourceIdentity::AccessPackages,
        AccessPackagesIdApiClient
    );
    api_client_link!(
        access_packages,
        ResourceIdentity::AccessPackages,
        AccessPackagesApiClient
    );
    api_client_link_id!(
        assignment_request,
        ResourceIdentity::AssignmentRequests,
        AssignmentRequestsIdApiClient
    );
    api_client_link_id!(
        connected_organization,
        ResourceIdentity::ConnectedOrganizations,
        ConnectedOrganizationsIdApiClient
    );
    api_client_link_id!(
        assignment_policy,
        ResourceIdentity::AssignmentPolicies,
        AssignmentPoliciesIdApiClient
    );
    api_client_link!(
        connected_organizations,
        ResourceIdentity::ConnectedOrganizations,
        ConnectedOrganizationsApiClient
    );
    api_client_link!(
        assignment_policies,
        ResourceIdentity::AssignmentPolicies,
        AssignmentPoliciesApiClient
    );
    api_client_link!(
        assignments,
        ResourceIdentity::Assignments,
        AssignmentsApiClient
    );
    api_client_link!(
        access_package_assignment_approvals,
        ResourceIdentity::AccessPackageAssignmentApprovals,
        AccessPackageAssignmentApprovalsApiClient
    );
    api_client_link!(
        assignment_requests,
        ResourceIdentity::AssignmentRequests,
        AssignmentRequestsApiClient
    );
    api_client_link_id!(
        assignment,
        ResourceIdentity::Assignments,
        AssignmentsIdApiClient
    );
    api_client_link_id!(
        access_package_assignment_approval,
        ResourceIdentity::AccessPackageAssignmentApprovals,
        AccessPackageAssignmentApprovalsIdApiClient
    );

    delete!(
        doc: "Delete navigation property entitlementManagement for identityGovernance",
        name: delete_entitlement_management,
        path: "/entitlementManagement"
    );
    get!(
        doc: "Get entitlementManagement from identityGovernance",
        name: get_entitlement_management,
        path: "/entitlementManagement"
    );
    patch!(
        doc: "Update the navigation property entitlementManagement in identityGovernance",
        name: update_entitlement_management,
        path: "/entitlementManagement",
        body: true
    );
    post!(
        doc: "Create accessPackageAssignmentRequest",
        name: create_assignment_requests,
        path: "/entitlementManagement/assignmentRequests",
        body: true
    );
    get!(
        doc: "List assignmentRequests",
        name: list_assignment_requests,
        path: "/entitlementManagement/assignmentRequests"
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        path: "/entitlementManagement/assignmentRequests/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: on
    );
    delete!(
        doc: "Delete navigation property assignmentRequests for identityGovernance",
        name: delete_assignment_requests,
        path: "/entitlementManagement/assignmentRequests/{{id}}",
        params: access_package_assignment_request_id
    );
    get!(
        doc: "Get assignmentRequests from identityGovernance",
        name: get_assignment_requests,
        path: "/entitlementManagement/assignmentRequests/{{id}}",
        params: access_package_assignment_request_id
    );
    patch!(
        doc: "Update the navigation property assignmentRequests in identityGovernance",
        name: update_assignment_requests,
        path: "/entitlementManagement/assignmentRequests/{{id}}",
        body: true,
        params: access_package_assignment_request_id
    );
    get!(
        doc: "Get accessPackage from identityGovernance",
        name: get_access_package,
        path: "/entitlementManagement/assignmentRequests/{{id}}/accessPackage",
        params: access_package_assignment_request_id
    );
    get!(
        doc: "Get assignment from identityGovernance",
        name: get_assignment,
        path: "/entitlementManagement/assignmentRequests/{{id}}/assignment",
        params: access_package_assignment_request_id
    );
    post!(
        doc: "Invoke action cancel",
        name: cancel,
        path: "/entitlementManagement/assignmentRequests/{{id}}/microsoft.graph.cancel",
        params: access_package_assignment_request_id
    );
    post!(
        doc: "Invoke action reprocess",
        name: reprocess,
        path: "/entitlementManagement/assignmentRequests/{{id}}/microsoft.graph.reprocess",
        params: access_package_assignment_request_id
    );
    get!(
        doc: "Get requestor from identityGovernance",
        name: get_requestor,
        path: "/entitlementManagement/assignmentRequests/{{id}}/requestor",
        params: access_package_assignment_request_id
    );
    delete!(
        doc: "Delete navigation property settings for identityGovernance",
        name: delete_settings,
        path: "/entitlementManagement/settings"
    );
    get!(
        doc: "Get entitlementManagementSettings",
        name: get_settings,
        path: "/entitlementManagement/settings"
    );
    patch!(
        doc: "Update entitlementManagementSettings",
        name: update_settings,
        path: "/entitlementManagement/settings",
        body: true
    );
}
