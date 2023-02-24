// GENERATED CODE

use crate::api_default_imports::*;
use crate::identity_governance::{
    AccessPackageAssignmentApprovalsApiClient, AccessPackageAssignmentApprovalsIdApiClient,
    AccessPackagesApiClient, AccessPackagesIdApiClient, ConnectedOrganizationsApiClient,
    ConnectedOrganizationsIdApiClient,
};
use crate::identity_governance::{
    AssignmentPoliciesApiClient, AssignmentPoliciesIdApiClient, AssignmentRequestsApiClient,
    AssignmentRequestsIdApiClient, EntitlementManagementAssignmentsApiClient,
    EntitlementManagementAssignmentsIdApiClient,
};
use crate::identity_governance::{
    EntitlementManagementCatalogsApiClient, EntitlementManagementCatalogsIdApiClient,
};

resource_api_client!(
    EntitlementManagementApiClient,
    ResourceIdentity::EntitlementManagement
);

impl EntitlementManagementApiClient {
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
    api_client_link!(
        connected_organizations,
        ResourceIdentity::ConnectedOrganizations,
        ConnectedOrganizationsApiClient
    );
    api_client_link!(
        access_package_assignment_approvals,
        ResourceIdentity::AccessPackageAssignmentApprovals,
        AccessPackageAssignmentApprovalsApiClient
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
    api_client_link_id!(
        assignment_policy,
        ResourceIdentity::AssignmentPolicies,
        AssignmentPoliciesIdApiClient
    );
    api_client_link!(
        assignments,
        ResourceIdentity::EntitlementManagementAssignments,
        EntitlementManagementAssignmentsApiClient
    );
    api_client_link!(
        access_packages,
        ResourceIdentity::AccessPackages,
        AccessPackagesApiClient
    );
    api_client_link_id!(
        assignment,
        ResourceIdentity::EntitlementManagementAssignments,
        EntitlementManagementAssignmentsIdApiClient
    );
    api_client_link!(
        assignment_policies,
        ResourceIdentity::AssignmentPolicies,
        AssignmentPoliciesApiClient
    );
    api_client_link!(
        catalogs,
        ResourceIdentity::EntitlementManagementCatalogs,
        EntitlementManagementCatalogsApiClient
    );
    api_client_link_id!(
        catalog,
        ResourceIdentity::EntitlementManagementCatalogs,
        EntitlementManagementCatalogsIdApiClient
    );
    api_client_link!(
        assignment_requests,
        ResourceIdentity::AssignmentRequests,
        AssignmentRequestsApiClient
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
