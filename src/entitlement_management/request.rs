// GENERATED CODE

use crate::access_package_assignment_approvals::{
    AccessPackageAssignmentApprovalsIdRequest, AccessPackageAssignmentApprovalsRequest,
};
use crate::access_packages::{AccessPackagesIdRequest, AccessPackagesRequest};
use crate::api_default_imports::*;
use crate::assignment_policies::{AssignmentPoliciesIdRequest, AssignmentPoliciesRequest};
use crate::assignment_requests::{AssignmentRequestsIdRequest, AssignmentRequestsRequest};
use crate::assignments::{AssignmentsIdRequest, AssignmentsRequest};
use crate::connected_organizations::{
    ConnectedOrganizationsIdRequest, ConnectedOrganizationsRequest,
};
use graph_http::types::NoContent;

register_client!(EntitlementManagementRequest,);

impl<'a, Client> EntitlementManagementRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn access_package_assignment_approvals(
        &self,
    ) -> AccessPackageAssignmentApprovalsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client
            .set_ident(ResourceIdentity::AccessPackageAssignmentApprovals);
        AccessPackageAssignmentApprovalsRequest::new(self.client)
    }

    pub fn access_package_assignment_approval<ID: AsRef<str>>(
        &self,
        id: ID,
    ) -> AccessPackageAssignmentApprovalsIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client
            .set_ident(ResourceIdentity::AccessPackageAssignmentApprovals);
        AccessPackageAssignmentApprovalsIdRequest::new(id.as_ref(), self.client)
    }

    pub fn access_packages(&self) -> AccessPackagesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::AccessPackages);
        AccessPackagesRequest::new(self.client)
    }

    pub fn access_package<ID: AsRef<str>>(&self, id: ID) -> AccessPackagesIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::AccessPackages);
        AccessPackagesIdRequest::new(id.as_ref(), self.client)
    }

    pub fn assignment_policies(&self) -> AssignmentPoliciesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::AssignmentPolicies);
        AssignmentPoliciesRequest::new(self.client)
    }

    pub fn assignment_policy<ID: AsRef<str>>(
        &self,
        id: ID,
    ) -> AssignmentPoliciesIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::AssignmentPolicies);
        AssignmentPoliciesIdRequest::new(id.as_ref(), self.client)
    }

    pub fn assignment_requests(&self) -> AssignmentRequestsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::AssignmentRequests);
        AssignmentRequestsRequest::new(self.client)
    }

    pub fn assignment_request<ID: AsRef<str>>(
        &self,
        id: ID,
    ) -> AssignmentRequestsIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::AssignmentRequests);
        AssignmentRequestsIdRequest::new(id.as_ref(), self.client)
    }

    pub fn assignments(&self) -> AssignmentsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Assignments);
        AssignmentsRequest::new(self.client)
    }

    pub fn assignment<ID: AsRef<str>>(&self, id: ID) -> AssignmentsIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Assignments);
        AssignmentsIdRequest::new(id.as_ref(), self.client)
    }

    pub fn connected_organizations(&self) -> ConnectedOrganizationsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client
            .set_ident(ResourceIdentity::ConnectedOrganizations);
        ConnectedOrganizationsRequest::new(self.client)
    }

    pub fn connected_organization<ID: AsRef<str>>(
        &self,
        id: ID,
    ) -> ConnectedOrganizationsIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client
            .set_ident(ResourceIdentity::ConnectedOrganizations);
        ConnectedOrganizationsIdRequest::new(id.as_ref(), self.client)
    }

    delete!({
        doc: "Delete navigation property entitlementManagement for identityGovernance",
        name: delete_entitlement_management,
        response: NoContent,
        path: "/entitlementManagement",
        has_body: false
    });
    get!({
        doc: "Get entitlementManagement from identityGovernance",
        name: get_entitlement_management,
        response: serde_json::Value,
        path: "/entitlementManagement",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property entitlementManagement in identityGovernance",
        name: update_entitlement_management,
        response: NoContent,
        path: "/entitlementManagement",
        has_body: true
    });
    delete!({
        doc: "Delete navigation property settings for identityGovernance",
        name: delete_settings,
        response: NoContent,
        path: "/entitlementManagement/settings",
        has_body: false
    });
    get!({
        doc: "Get settings from identityGovernance",
        name: get_settings,
        response: serde_json::Value,
        path: "/entitlementManagement/settings",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property settings in identityGovernance",
        name: update_settings,
        response: NoContent,
        path: "/entitlementManagement/settings",
        has_body: true
    });
}
