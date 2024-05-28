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

api_client!(
    EntitlementManagementApiClient,
    ResourceIdentity::EntitlementManagement
);

impl EntitlementManagementApiClient {
    api_client_link!(assignment_requests, AssignmentRequestsApiClient);
    api_client_link_id!(assignment, EntitlementManagementAssignmentsIdApiClient);
    api_client_link!(assignments, EntitlementManagementAssignmentsApiClient);
    api_client_link_id!(
        access_package_assignment_approval,
        AccessPackageAssignmentApprovalsIdApiClient
    );
    api_client_link_id!(assignment_policy, AssignmentPoliciesIdApiClient);
    api_client_link!(connected_organizations, ConnectedOrganizationsApiClient);
    api_client_link!(assignment_policies, AssignmentPoliciesApiClient);
    api_client_link!(access_packages, AccessPackagesApiClient);
    api_client_link!(catalogs, EntitlementManagementCatalogsApiClient);
    api_client_link_id!(assignment_request, AssignmentRequestsIdApiClient);
    api_client_link_id!(access_package, AccessPackagesIdApiClient);
    api_client_link!(
        access_package_assignment_approvals,
        AccessPackageAssignmentApprovalsApiClient
    );
    api_client_link_id!(connected_organization, ConnectedOrganizationsIdApiClient);
    api_client_link_id!(catalog, EntitlementManagementCatalogsIdApiClient);

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
