// GENERATED CODE

use crate::access_package_assignment_approvals::{
    AccessPackageAssignmentApprovalsApiClient, AccessPackageAssignmentApprovalsIdApiClient,
};
use crate::api_default_imports::*;
use crate::assignment_policies::{AssignmentPoliciesApiClient, AssignmentPoliciesIdApiClient};
use crate::assignment_requests::{AssignmentRequestsApiClient, AssignmentRequestsIdApiClient};
use crate::assignments::{AssignmentsApiClient, AssignmentsIdApiClient};
use crate::connected_organizations::{
    ConnectedOrganizationsApiClient, ConnectedOrganizationsIdApiClient,
};
use crate::identity_governance::{AccessPackagesApiClient, AccessPackagesIdApiClient};

resource_api_client!(
    EntitlementManagementApiClient,
    ResourceIdentity::EntitlementManagement
);

impl EntitlementManagementApiClient {
    api_client_link!(
        access_packages,
        ResourceIdentity::AccessPackages,
        AccessPackagesApiClient
    );
    api_client_link_id!(
        access_package,
        ResourceIdentity::AccessPackages,
        AccessPackagesIdApiClient
    );
    api_client_link_id!(
        access_package_assignment_approval,
        ResourceIdentity::AccessPackageAssignmentApprovals,
        AccessPackageAssignmentApprovalsIdApiClient
    );
    api_client_link!(
        connected_organizations,
        ResourceIdentity::ConnectedOrganizations,
        ConnectedOrganizationsApiClient
    );
    api_client_link_id!(
        connected_organization,
        ResourceIdentity::ConnectedOrganizations,
        ConnectedOrganizationsIdApiClient
    );
    api_client_link!(
        assignment_policies,
        ResourceIdentity::AssignmentPolicies,
        AssignmentPoliciesApiClient
    );
    api_client_link!(
        assignment_requests,
        ResourceIdentity::AssignmentRequests,
        AssignmentRequestsApiClient
    );
    api_client_link_id!(
        assignment_request,
        ResourceIdentity::AssignmentRequests,
        AssignmentRequestsIdApiClient
    );
    api_client_link!(
        assignments,
        ResourceIdentity::Assignments,
        AssignmentsApiClient
    );
    api_client_link_id!(
        assignment,
        ResourceIdentity::Assignments,
        AssignmentsIdApiClient
    );
    api_client_link!(
        access_package_assignment_approvals,
        ResourceIdentity::AccessPackageAssignmentApprovals,
        AccessPackageAssignmentApprovalsApiClient
    );
    api_client_link_id!(
        assignment_policy,
        ResourceIdentity::AssignmentPolicies,
        AssignmentPoliciesIdApiClient
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
