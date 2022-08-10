// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;
use crate::connected_organizations::{ConnectedOrganizationsIdRequest, ConnectedOrganizationsRequest};

register_client!(EntitlementManagementRequest,);

impl<'a, Client> EntitlementManagementRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
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
    post!({
        doc: "Create new navigation property to accessPackageAssignmentApprovals for identityGovernance",
        name: create_access_package_assignment_approvals,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackageAssignmentApprovals",
        has_body: true
    });
    get!({
        doc: "Get accessPackageAssignmentApprovals from identityGovernance",
        name: list_access_package_assignment_approvals,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackageAssignmentApprovals",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackageAssignmentApprovals/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackageAssignmentApprovals/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: [ on ],
        has_body: false
    });
    delete!({
        doc: "Delete navigation property accessPackageAssignmentApprovals for identityGovernance",
        name: delete_access_package_assignment_approvals,
        response: NoContent,
        path: "/entitlementManagement/accessPackageAssignmentApprovals/{{id}}",
        params: [ approval_id ],
        has_body: false
    });
    get!({
        doc: "Get accessPackageAssignmentApprovals from identityGovernance",
        name: get_access_package_assignment_approvals,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackageAssignmentApprovals/{{id}}",
        params: [ approval_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property accessPackageAssignmentApprovals in identityGovernance",
        name: update_access_package_assignment_approvals,
        response: NoContent,
        path: "/entitlementManagement/accessPackageAssignmentApprovals/{{id}}",
        params: [ approval_id ],
        has_body: true
    });
    post!({
        doc: "Create new navigation property to stages for identityGovernance",
        name: create_stages,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackageAssignmentApprovals/{{id}}/stages",
        params: [ approval_id ],
        has_body: true
    });
    get!({
        doc: "Get stages from identityGovernance",
        name: list_stages,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackageAssignmentApprovals/{{id}}/stages",
        params: [ approval_id ],
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackageAssignmentApprovals/{{id}}/stages/$count",
        params: [ approval_id ],
        has_body: false
    });
    delete!({
        doc: "Delete navigation property stages for identityGovernance",
        name: delete_stages,
        response: NoContent,
        path: "/entitlementManagement/accessPackageAssignmentApprovals/{{id}}/stages/{{id2}}",
        params: [ approval_id  approval_stage_id ],
        has_body: false
    });
    get!({
        doc: "Get stages from identityGovernance",
        name: get_stages,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackageAssignmentApprovals/{{id}}/stages/{{id2}}",
        params: [ approval_id  approval_stage_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property stages in identityGovernance",
        name: update_stages,
        response: NoContent,
        path: "/entitlementManagement/accessPackageAssignmentApprovals/{{id}}/stages/{{id2}}",
        params: [ approval_id  approval_stage_id ],
        has_body: true
    });
    post!({
        doc: "Create new navigation property to accessPackages for identityGovernance",
        name: create_access_packages,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackages",
        has_body: true
    });
    get!({
        doc: "Get accessPackages from identityGovernance",
        name: list_access_packages,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackages",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackages/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackages/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: [ on ],
        has_body: false
    });
    delete!({
        doc: "Delete navigation property accessPackages for identityGovernance",
        name: delete_access_packages,
        response: NoContent,
        path: "/entitlementManagement/accessPackages/{{id}}",
        params: [ access_package_id ],
        has_body: false
    });
    get!({
        doc: "Get accessPackages from identityGovernance",
        name: get_access_packages,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackages/{{id}}",
        params: [ access_package_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property accessPackages in identityGovernance",
        name: update_access_packages,
        response: NoContent,
        path: "/entitlementManagement/accessPackages/{{id}}",
        params: [ access_package_id ],
        has_body: true
    });
    post!({
        doc: "Create new navigation property to assignmentPolicies for identityGovernance",
        name: create_assignment_policies,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackages/{{id}}/assignmentPolicies",
        params: [ access_package_id ],
        has_body: true
    });
    get!({
        doc: "Get assignmentPolicies from identityGovernance",
        name: list_assignment_policies,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackages/{{id}}/assignmentPolicies",
        params: [ access_package_id ],
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackages/{{id}}/assignmentPolicies/$count",
        params: [ access_package_id ],
        has_body: false
    });
    delete!({
        doc: "Delete navigation property assignmentPolicies for identityGovernance",
        name: delete_assignment_policies,
        response: NoContent,
        path: "/entitlementManagement/accessPackages/{{id}}/assignmentPolicies/{{id2}}",
        params: [ access_package_id  access_package_assignment_policy_id ],
        has_body: false
    });
    get!({
        doc: "Get assignmentPolicies from identityGovernance",
        name: get_assignment_policies,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackages/{{id}}/assignmentPolicies/{{id2}}",
        params: [ access_package_id  access_package_assignment_policy_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property assignmentPolicies in identityGovernance",
        name: update_assignment_policies,
        response: NoContent,
        path: "/entitlementManagement/accessPackages/{{id}}/assignmentPolicies/{{id2}}",
        params: [ access_package_id  access_package_assignment_policy_id ],
        has_body: true
    });
    get!({
        doc: "Get accessPackage from identityGovernance",
        name: get_access_package,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackages/{{id}}/assignmentPolicies/{{id2}}/accessPackage",
        params: [ access_package_id  access_package_assignment_policy_id ],
        has_body: false
    });
    get!({
        doc: "Get catalog from identityGovernance",
        name: get_catalog,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackages/{{id}}/assignmentPolicies/{{id2}}/catalog",
        params: [ access_package_id  access_package_assignment_policy_id ],
        has_body: false
    });
    get!({
        doc: "Get catalog from identityGovernance",
        name: get_catalog,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackages/{{id}}/catalog",
        params: [ access_package_id ],
        has_body: false
    });
    post!({
        doc: "Invoke action getApplicablePolicyRequirements",
        name: get_applicable_policy_requirements,
        response: serde_json::Value,
        path: "/entitlementManagement/accessPackages/{{id}}/microsoft.graph.getApplicablePolicyRequirements",
        params: [ access_package_id ],
        has_body: false
    });
    post!({
        doc: "Create new navigation property to assignmentPolicies for identityGovernance",
        name: create_assignment_policies,
        response: serde_json::Value,
        path: "/entitlementManagement/assignmentPolicies",
        has_body: true
    });
    get!({
        doc: "Get assignmentPolicies from identityGovernance",
        name: list_assignment_policies,
        response: serde_json::Value,
        path: "/entitlementManagement/assignmentPolicies",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/entitlementManagement/assignmentPolicies/$count",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property assignmentPolicies for identityGovernance",
        name: delete_assignment_policies,
        response: NoContent,
        path: "/entitlementManagement/assignmentPolicies/{{id}}",
        params: [ access_package_assignment_policy_id ],
        has_body: false
    });
    get!({
        doc: "Get assignmentPolicies from identityGovernance",
        name: get_assignment_policies,
        response: serde_json::Value,
        path: "/entitlementManagement/assignmentPolicies/{{id}}",
        params: [ access_package_assignment_policy_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property assignmentPolicies in identityGovernance",
        name: update_assignment_policies,
        response: NoContent,
        path: "/entitlementManagement/assignmentPolicies/{{id}}",
        params: [ access_package_assignment_policy_id ],
        has_body: true
    });
    get!({
        doc: "Get accessPackage from identityGovernance",
        name: get_access_package,
        response: serde_json::Value,
        path: "/entitlementManagement/assignmentPolicies/{{id}}/accessPackage",
        params: [ access_package_assignment_policy_id ],
        has_body: false
    });
    get!({
        doc: "Get catalog from identityGovernance",
        name: get_catalog,
        response: serde_json::Value,
        path: "/entitlementManagement/assignmentPolicies/{{id}}/catalog",
        params: [ access_package_assignment_policy_id ],
        has_body: false
    });
    post!({
        doc: "Create new navigation property to assignmentRequests for identityGovernance",
        name: create_assignment_requests,
        response: serde_json::Value,
        path: "/entitlementManagement/assignmentRequests",
        has_body: true
    });
    get!({
        doc: "Get assignmentRequests from identityGovernance",
        name: list_assignment_requests,
        response: serde_json::Value,
        path: "/entitlementManagement/assignmentRequests",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/entitlementManagement/assignmentRequests/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        response: serde_json::Value,
        path: "/entitlementManagement/assignmentRequests/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: [ on ],
        has_body: false
    });
    delete!({
        doc: "Delete navigation property assignmentRequests for identityGovernance",
        name: delete_assignment_requests,
        response: NoContent,
        path: "/entitlementManagement/assignmentRequests/{{id}}",
        params: [ access_package_assignment_request_id ],
        has_body: false
    });
    get!({
        doc: "Get assignmentRequests from identityGovernance",
        name: get_assignment_requests,
        response: serde_json::Value,
        path: "/entitlementManagement/assignmentRequests/{{id}}",
        params: [ access_package_assignment_request_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property assignmentRequests in identityGovernance",
        name: update_assignment_requests,
        response: NoContent,
        path: "/entitlementManagement/assignmentRequests/{{id}}",
        params: [ access_package_assignment_request_id ],
        has_body: true
    });
    get!({
        doc: "Get accessPackage from identityGovernance",
        name: get_access_package,
        response: serde_json::Value,
        path: "/entitlementManagement/assignmentRequests/{{id}}/accessPackage",
        params: [ access_package_assignment_request_id ],
        has_body: false
    });
    get!({
        doc: "Get assignment from identityGovernance",
        name: get_assignment,
        response: serde_json::Value,
        path: "/entitlementManagement/assignmentRequests/{{id}}/assignment",
        params: [ access_package_assignment_request_id ],
        has_body: false
    });
    post!({
        doc: "Invoke action cancel",
        name: cancel,
        response: NoContent,
        path: "/entitlementManagement/assignmentRequests/{{id}}/microsoft.graph.cancel",
        params: [ access_package_assignment_request_id ],
        has_body: false
    });
    post!({
        doc: "Invoke action reprocess",
        name: reprocess,
        response: NoContent,
        path: "/entitlementManagement/assignmentRequests/{{id}}/microsoft.graph.reprocess",
        params: [ access_package_assignment_request_id ],
        has_body: false
    });
    get!({
        doc: "Get requestor from identityGovernance",
        name: get_requestor,
        response: serde_json::Value,
        path: "/entitlementManagement/assignmentRequests/{{id}}/requestor",
        params: [ access_package_assignment_request_id ],
        has_body: false
    });
    post!({
        doc: "Create new navigation property to assignments for identityGovernance",
        name: create_assignments,
        response: serde_json::Value,
        path: "/entitlementManagement/assignments",
        has_body: true
    });
    get!({
        doc: "Get assignments from identityGovernance",
        name: list_assignments,
        response: serde_json::Value,
        path: "/entitlementManagement/assignments",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/entitlementManagement/assignments/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        response: serde_json::Value,
        path: "/entitlementManagement/assignments/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: [ on ],
        has_body: false
    });
    delete!({
        doc: "Delete navigation property assignments for identityGovernance",
        name: delete_assignments,
        response: NoContent,
        path: "/entitlementManagement/assignments/{{id}}",
        params: [ access_package_assignment_id ],
        has_body: false
    });
    get!({
        doc: "Get assignments from identityGovernance",
        name: get_assignments,
        response: serde_json::Value,
        path: "/entitlementManagement/assignments/{{id}}",
        params: [ access_package_assignment_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property assignments in identityGovernance",
        name: update_assignments,
        response: NoContent,
        path: "/entitlementManagement/assignments/{{id}}",
        params: [ access_package_assignment_id ],
        has_body: true
    });
    get!({
        doc: "Get accessPackage from identityGovernance",
        name: get_access_package,
        response: serde_json::Value,
        path: "/entitlementManagement/assignments/{{id}}/accessPackage",
        params: [ access_package_assignment_id ],
        has_body: false
    });
    get!({
        doc: "Get assignmentPolicy from identityGovernance",
        name: get_assignment_policy,
        response: serde_json::Value,
        path: "/entitlementManagement/assignments/{{id}}/assignmentPolicy",
        params: [ access_package_assignment_id ],
        has_body: false
    });
    post!({
        doc: "Invoke action reprocess",
        name: reprocess,
        response: NoContent,
        path: "/entitlementManagement/assignments/{{id}}/microsoft.graph.reprocess",
        params: [ access_package_assignment_id ],
        has_body: false
    });
    get!({
        doc: "Get target from identityGovernance",
        name: get_target,
        response: serde_json::Value,
        path: "/entitlementManagement/assignments/{{id}}/target",
        params: [ access_package_assignment_id ],
        has_body: false
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
