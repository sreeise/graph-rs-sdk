use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::{GraphResponse, IntoResponse};
use reqwest::Method;

register_client!(DeviceAppManagementRequest,);
register_client!(DefaultManagedAppProtectionsRequest,);
register_client!(MobileAppsRequest,);
register_client!(IosManagedAppProtectionsRequest,);
register_client!(AppliedPoliciesRequest,);
register_client!(IntendedPoliciesRequest,);
register_client!(MobileAppConfigurationsRequest,);
register_client!(ManagedAppPoliciesRequest,);
register_client!(VppTokensRequest,);
register_client!(ManagedEBooksRequest,);
register_client!(ManagedAppRegistrationsRequest,);
register_client!(UserStateSummaryRequest,);
register_client!(AndroidManagedAppProtectionsRequest,);
register_client!(TargetedManagedAppConfigurationsRequest,);

#[allow(dead_code)]
impl<'a, Client> ManagedAppRegistrationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn applied_policies(&self) -> AppliedPoliciesRequest<'a, Client> {
        AppliedPoliciesRequest::new(&self.client)
    }
    pub fn intended_policies(&self) -> IntendedPoliciesRequest<'a, Client> {
        IntendedPoliciesRequest::new(&self.client)
    }
    get!({
        doc: "# Get intendedPolicies from deviceAppManagement",
        name: list_intended_policies,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/intendedPolicies",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to intendedPolicies for deviceAppManagement",
        name: create_intended_policies,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/intendedPolicies",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get appliedPolicies from deviceAppManagement",
        name: list_applied_policies,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/appliedPolicies",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to appliedPolicies for deviceAppManagement",
        name: create_applied_policies,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/appliedPolicies",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get appliedPolicies from deviceAppManagement",
        name: get_applied_policies,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/appliedPolicies/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property appliedPolicies in deviceAppManagement",
        name: update_applied_policies,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/appliedPolicies/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property appliedPolicies for deviceAppManagement",
        name: delete_applied_policies,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/appliedPolicies/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get operations from deviceAppManagement",
        name: list_operations,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/operations",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to operations for deviceAppManagement",
        name: create_operations,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/operations",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Invoke function getUserIdsWithFlaggedAppRegistration",
        name: get_user_ids_with_flagged_app_registration,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/managedAppRegistrations/microsoft.graph.getUserIdsWithFlaggedAppRegistration()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get intendedPolicies from deviceAppManagement",
        name: get_intended_policies,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/intendedPolicies/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property intendedPolicies in deviceAppManagement",
        name: update_intended_policies,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/intendedPolicies/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property intendedPolicies for deviceAppManagement",
        name: delete_intended_policies,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/intendedPolicies/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get operations from deviceAppManagement",
        name: get_operations,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/operations/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property operations in deviceAppManagement",
        name: update_operations,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/operations/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property operations for deviceAppManagement",
        name: delete_operations,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/operations/{{id2}}",
        params: 2,
        has_body: false
    });
}

#[allow(dead_code)]
impl<'a, Client> AndroidManagedAppProtectionsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get apps from deviceAppManagement",
        name: list_apps,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/androidManagedAppProtections/{{id}}/apps",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to apps for deviceAppManagement",
        name: create_apps,
        response: serde_json::Value,
        path: "/deviceAppManagement/androidManagedAppProtections/{{id}}/apps",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get deploymentSummary from deviceAppManagement",
        name: get_deployment_summary,
        response: serde_json::Value,
        path: "/deviceAppManagement/androidManagedAppProtections/{{id}}/deploymentSummary",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property deploymentSummary in deviceAppManagement",
        name: update_deployment_summary,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/androidManagedAppProtections/{{id}}/deploymentSummary",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property deploymentSummary for deviceAppManagement",
        name: delete_deployment_summary,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/androidManagedAppProtections/{{id}}/deploymentSummary",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get apps from deviceAppManagement",
        name: get_apps,
        response: serde_json::Value,
        path: "/deviceAppManagement/androidManagedAppProtections/{{id}}/apps/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property apps in deviceAppManagement",
        name: update_apps,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/androidManagedAppProtections/{{id}}/apps/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property apps for deviceAppManagement",
        name: delete_apps,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/androidManagedAppProtections/{{id}}/apps/{{id2}}",
        params: 2,
        has_body: false
    });
}

#[allow(dead_code)]
impl<'a, Client> MobileAppsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get categories from deviceAppManagement",
        name: get_categories,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileApps/{{id}}/categories/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get categories from deviceAppManagement",
        name: list_categories,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/mobileApps/{{id}}/categories",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get assignments from deviceAppManagement",
        name: list_assignments,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/mobileApps/{{id}}/assignments",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to assignments for deviceAppManagement",
        name: create_assignments,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileApps/{{id}}/assignments",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get ref of categories from deviceAppManagement",
        name: list_ref_categories,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/mobileApps/{{id}}/categories/$ref",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property ref to categories for deviceAppManagement",
        name: create_ref_categories,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileApps/{{id}}/categories/$ref",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete ref of navigation property categories for deviceAppManagement",
        name: delete_ref_categories,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileApps/{{id}}/categories/$ref",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action assign",
        name: assign,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileApps/{{id}}/microsoft.graph.assign",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get assignments from deviceAppManagement",
        name: get_assignments,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileApps/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property assignments in deviceAppManagement",
        name: update_assignments,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileApps/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property assignments for deviceAppManagement",
        name: delete_assignments,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileApps/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: false
    });
}

#[allow(dead_code)]
impl<'a, Client> UserStateSummaryRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get deviceStates from deviceAppManagement",
        name: get_device_states,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedEBooks/{{id}}/userStateSummary/{{id2}}/deviceStates/{{id3}}",
        params: 3,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property deviceStates in deviceAppManagement",
        name: update_device_states,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedEBooks/{{id}}/userStateSummary/{{id2}}/deviceStates/{{id3}}",
        params: 3,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property deviceStates for deviceAppManagement",
        name: delete_device_states,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedEBooks/{{id}}/userStateSummary/{{id2}}/deviceStates/{{id3}}",
        params: 3,
        has_body: false
    });
    get!({
        doc: "# Get deviceStates from deviceAppManagement",
        name: list_device_states,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/managedEBooks/{{id}}/userStateSummary/{{id2}}/deviceStates",
        params: 2,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to deviceStates for deviceAppManagement",
        name: create_device_states,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedEBooks/{{id}}/userStateSummary/{{id2}}/deviceStates",
        params: 2,
        has_body: true
    });
}

#[allow(dead_code)]
impl<'a, Client> DefaultManagedAppProtectionsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get apps from deviceAppManagement",
        name: get_apps,
        response: serde_json::Value,
        path: "/deviceAppManagement/defaultManagedAppProtections/{{id}}/apps/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property apps in deviceAppManagement",
        name: update_apps,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/defaultManagedAppProtections/{{id}}/apps/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property apps for deviceAppManagement",
        name: delete_apps,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/defaultManagedAppProtections/{{id}}/apps/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get deploymentSummary from deviceAppManagement",
        name: get_deployment_summary,
        response: serde_json::Value,
        path: "/deviceAppManagement/defaultManagedAppProtections/{{id}}/deploymentSummary",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property deploymentSummary in deviceAppManagement",
        name: update_deployment_summary,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/defaultManagedAppProtections/{{id}}/deploymentSummary",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property deploymentSummary for deviceAppManagement",
        name: delete_deployment_summary,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/defaultManagedAppProtections/{{id}}/deploymentSummary",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get apps from deviceAppManagement",
        name: list_apps,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/defaultManagedAppProtections/{{id}}/apps",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to apps for deviceAppManagement",
        name: create_apps,
        response: serde_json::Value,
        path: "/deviceAppManagement/defaultManagedAppProtections/{{id}}/apps",
        params: 1,
        has_body: true
    });
}

#[allow(dead_code)]
impl<'a, Client> VppTokensRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action syncLicenses",
        name: sync_licenses,
        response: serde_json::Value,
        path: "/deviceAppManagement/vppTokens/{{id}}/microsoft.graph.syncLicenses",
        params: 1,
        has_body: false
    });
}

#[allow(dead_code)]
impl<'a, Client> ManagedAppPoliciesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action targetApps",
        name: target_apps,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedAppPolicies/{{id}}/microsoft.graph.targetApps",
        params: 1,
        has_body: true
    });
}

#[allow(dead_code)]
impl<'a, Client> IosManagedAppProtectionsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get apps from deviceAppManagement",
        name: get_apps,
        response: serde_json::Value,
        path: "/deviceAppManagement/iosManagedAppProtections/{{id}}/apps/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property apps in deviceAppManagement",
        name: update_apps,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/iosManagedAppProtections/{{id}}/apps/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property apps for deviceAppManagement",
        name: delete_apps,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/iosManagedAppProtections/{{id}}/apps/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get apps from deviceAppManagement",
        name: list_apps,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/iosManagedAppProtections/{{id}}/apps",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to apps for deviceAppManagement",
        name: create_apps,
        response: serde_json::Value,
        path: "/deviceAppManagement/iosManagedAppProtections/{{id}}/apps",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get deploymentSummary from deviceAppManagement",
        name: get_deployment_summary,
        response: serde_json::Value,
        path: "/deviceAppManagement/iosManagedAppProtections/{{id}}/deploymentSummary",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property deploymentSummary in deviceAppManagement",
        name: update_deployment_summary,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/iosManagedAppProtections/{{id}}/deploymentSummary",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property deploymentSummary for deviceAppManagement",
        name: delete_deployment_summary,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/iosManagedAppProtections/{{id}}/deploymentSummary",
        params: 1,
        has_body: false
    });
}

#[allow(dead_code)]
impl<'a, Client> MobileAppConfigurationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get userStatuses from deviceAppManagement",
        name: list_user_statuses,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/userStatuses",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to userStatuses for deviceAppManagement",
        name: create_user_statuses,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/userStatuses",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get assignments from deviceAppManagement",
        name: get_assignments,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property assignments in deviceAppManagement",
        name: update_assignments,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property assignments for deviceAppManagement",
        name: delete_assignments,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get deviceStatusSummary from deviceAppManagement",
        name: get_device_status_summary,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/deviceStatusSummary",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property deviceStatusSummary in deviceAppManagement",
        name: update_device_status_summary,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/deviceStatusSummary",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property deviceStatusSummary for deviceAppManagement",
        name: delete_device_status_summary,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/deviceStatusSummary",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get assignments from deviceAppManagement",
        name: list_assignments,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/assignments",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to assignments for deviceAppManagement",
        name: create_assignments,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/assignments",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get deviceStatuses from deviceAppManagement",
        name: get_device_statuses,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/deviceStatuses/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property deviceStatuses in deviceAppManagement",
        name: update_device_statuses,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/deviceStatuses/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property deviceStatuses for deviceAppManagement",
        name: delete_device_statuses,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/deviceStatuses/{{id2}}",
        params: 2,
        has_body: false
    });
    post!({
        doc: "# Invoke action assign",
        name: assign,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/microsoft.graph.assign",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get userStatuses from deviceAppManagement",
        name: get_user_statuses,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/userStatuses/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property userStatuses in deviceAppManagement",
        name: update_user_statuses,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/userStatuses/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property userStatuses for deviceAppManagement",
        name: delete_user_statuses,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/userStatuses/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get userStatusSummary from deviceAppManagement",
        name: get_user_status_summary,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/userStatusSummary",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property userStatusSummary in deviceAppManagement",
        name: update_user_status_summary,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/userStatusSummary",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property userStatusSummary for deviceAppManagement",
        name: delete_user_status_summary,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/userStatusSummary",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get deviceStatuses from deviceAppManagement",
        name: list_device_statuses,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/deviceStatuses",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to deviceStatuses for deviceAppManagement",
        name: create_device_statuses,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/deviceStatuses",
        params: 1,
        has_body: true
    });
}

#[allow(dead_code)]
impl<'a, Client> TargetedManagedAppConfigurationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get assignments from deviceAppManagement",
        name: list_assignments,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/assignments",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to assignments for deviceAppManagement",
        name: create_assignments,
        response: serde_json::Value,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/assignments",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get assignments from deviceAppManagement",
        name: get_assignments,
        response: serde_json::Value,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property assignments in deviceAppManagement",
        name: update_assignments,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property assignments for deviceAppManagement",
        name: delete_assignments,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get deploymentSummary from deviceAppManagement",
        name: get_deployment_summary,
        response: serde_json::Value,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/deploymentSummary",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property deploymentSummary in deviceAppManagement",
        name: update_deployment_summary,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/deploymentSummary",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property deploymentSummary for deviceAppManagement",
        name: delete_deployment_summary,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/deploymentSummary",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action assign",
        name: assign,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/microsoft.graph.assign",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action targetApps",
        name: target_apps,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/microsoft.graph.targetApps",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get apps from deviceAppManagement",
        name: get_apps,
        response: serde_json::Value,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/apps/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property apps in deviceAppManagement",
        name: update_apps,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/apps/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property apps for deviceAppManagement",
        name: delete_apps,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/apps/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get apps from deviceAppManagement",
        name: list_apps,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/apps",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to apps for deviceAppManagement",
        name: create_apps,
        response: serde_json::Value,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/apps",
        params: 1,
        has_body: true
    });
}

#[allow(dead_code)]
impl<'a, Client> AppliedPoliciesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action targetApps",
        name: target_apps,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/appliedPolicies/{{id2}}/microsoft.graph.targetApps",
        params: 2,
        has_body: true
    });
}

#[allow(dead_code)]
impl<'a, Client> ManagedEBooksRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn user_state_summary(&self) -> UserStateSummaryRequest<'a, Client> {
        UserStateSummaryRequest::new(&self.client)
    }
    post!({
        doc: "# Invoke action assign",
        name: assign,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedEBooks/{{id}}/microsoft.graph.assign",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get deviceStates from deviceAppManagement",
        name: get_device_states,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedEBooks/{{id}}/deviceStates/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property deviceStates in deviceAppManagement",
        name: update_device_states,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedEBooks/{{id}}/deviceStates/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property deviceStates for deviceAppManagement",
        name: delete_device_states,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedEBooks/{{id}}/deviceStates/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get installSummary from deviceAppManagement",
        name: get_install_summary,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedEBooks/{{id}}/installSummary",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property installSummary in deviceAppManagement",
        name: update_install_summary,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedEBooks/{{id}}/installSummary",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property installSummary for deviceAppManagement",
        name: delete_install_summary,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedEBooks/{{id}}/installSummary",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get userStateSummary from deviceAppManagement",
        name: get_user_state_summary,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedEBooks/{{id}}/userStateSummary/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property userStateSummary in deviceAppManagement",
        name: update_user_state_summary,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedEBooks/{{id}}/userStateSummary/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property userStateSummary for deviceAppManagement",
        name: delete_user_state_summary,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedEBooks/{{id}}/userStateSummary/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get deviceStates from deviceAppManagement",
        name: list_device_states,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/managedEBooks/{{id}}/deviceStates",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to deviceStates for deviceAppManagement",
        name: create_device_states,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedEBooks/{{id}}/deviceStates",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get assignments from deviceAppManagement",
        name: get_assignments,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedEBooks/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property assignments in deviceAppManagement",
        name: update_assignments,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedEBooks/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property assignments for deviceAppManagement",
        name: delete_assignments,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedEBooks/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get assignments from deviceAppManagement",
        name: list_assignments,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/managedEBooks/{{id}}/assignments",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to assignments for deviceAppManagement",
        name: create_assignments,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedEBooks/{{id}}/assignments",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get userStateSummary from deviceAppManagement",
        name: list_user_state_summary,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/managedEBooks/{{id}}/userStateSummary",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to userStateSummary for deviceAppManagement",
        name: create_user_state_summary,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedEBooks/{{id}}/userStateSummary",
        params: 1,
        has_body: true
    });
}

#[allow(dead_code)]
impl<'a, Client> IntendedPoliciesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action targetApps",
        name: target_apps,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/intendedPolicies/{{id2}}/microsoft.graph.targetApps",
        params: 2,
        has_body: true
    });
}

#[allow(dead_code)]
impl<'a, Client> DeviceAppManagementRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn android_managed_app_protections(
        &self,
    ) -> AndroidManagedAppProtectionsRequest<'a, Client> {
        AndroidManagedAppProtectionsRequest::new(&self.client)
    }
    pub fn default_managed_app_protections(
        &self,
    ) -> DefaultManagedAppProtectionsRequest<'a, Client> {
        DefaultManagedAppProtectionsRequest::new(&self.client)
    }
    pub fn ios_managed_app_protections(&self) -> IosManagedAppProtectionsRequest<'a, Client> {
        IosManagedAppProtectionsRequest::new(&self.client)
    }
    pub fn managed_app_policies(&self) -> ManagedAppPoliciesRequest<'a, Client> {
        ManagedAppPoliciesRequest::new(&self.client)
    }
    pub fn managed_app_registrations(&self) -> ManagedAppRegistrationsRequest<'a, Client> {
        ManagedAppRegistrationsRequest::new(&self.client)
    }
    pub fn managed_e_books(&self) -> ManagedEBooksRequest<'a, Client> {
        ManagedEBooksRequest::new(&self.client)
    }
    pub fn mobile_app_configurations(&self) -> MobileAppConfigurationsRequest<'a, Client> {
        MobileAppConfigurationsRequest::new(&self.client)
    }
    pub fn mobile_apps(&self) -> MobileAppsRequest<'a, Client> {
        MobileAppsRequest::new(&self.client)
    }
    pub fn targeted_managed_app_configurations(
        &self,
    ) -> TargetedManagedAppConfigurationsRequest<'a, Client> {
        TargetedManagedAppConfigurationsRequest::new(&self.client)
    }
    pub fn vpp_tokens(&self) -> VppTokensRequest<'a, Client> {
        VppTokensRequest::new(&self.client)
    }
    get!({
        doc: "# Get mobileAppConfigurations from deviceAppManagement",
        name: list_mobile_app_configurations,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/mobileAppConfigurations",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to mobileAppConfigurations for deviceAppManagement",
        name: create_mobile_app_configurations,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileAppConfigurations",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get managedAppStatuses from deviceAppManagement",
        name: list_managed_app_statuses,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/managedAppStatuses",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to managedAppStatuses for deviceAppManagement",
        name: create_managed_app_statuses,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedAppStatuses",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get vppTokens from deviceAppManagement",
        name: get_vpp_tokens,
        response: serde_json::Value,
        path: "/deviceAppManagement/vppTokens/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property vppTokens in deviceAppManagement",
        name: update_vpp_tokens,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/vppTokens/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property vppTokens for deviceAppManagement",
        name: delete_vpp_tokens,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/vppTokens/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get managedAppPolicies from deviceAppManagement",
        name: get_managed_app_policies,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedAppPolicies/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property managedAppPolicies in deviceAppManagement",
        name: update_managed_app_policies,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedAppPolicies/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property managedAppPolicies for deviceAppManagement",
        name: delete_managed_app_policies,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedAppPolicies/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get mobileApps from deviceAppManagement",
        name: list_mobile_apps,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/mobileApps",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to mobileApps for deviceAppManagement",
        name: create_mobile_apps,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileApps",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get windowsInformationProtectionPolicies from deviceAppManagement",
        name: get_windows_information_protection_policies,
        response: serde_json::Value,
        path: "/deviceAppManagement/windowsInformationProtectionPolicies/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property windowsInformationProtectionPolicies in deviceAppManagement",
        name: update_windows_information_protection_policies,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/windowsInformationProtectionPolicies/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property windowsInformationProtectionPolicies for deviceAppManagement",
        name: delete_windows_information_protection_policies,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/windowsInformationProtectionPolicies/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get targetedManagedAppConfigurations from deviceAppManagement",
        name: list_targeted_managed_app_configurations,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/targetedManagedAppConfigurations",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to targetedManagedAppConfigurations for deviceAppManagement",
        name: create_targeted_managed_app_configurations,
        response: serde_json::Value,
        path: "/deviceAppManagement/targetedManagedAppConfigurations",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get mobileAppConfigurations from deviceAppManagement",
        name: get_mobile_app_configurations,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property mobileAppConfigurations in deviceAppManagement",
        name: update_mobile_app_configurations,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property mobileAppConfigurations for deviceAppManagement",
        name: delete_mobile_app_configurations,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get defaultManagedAppProtections from deviceAppManagement",
        name: get_default_managed_app_protections,
        response: serde_json::Value,
        path: "/deviceAppManagement/defaultManagedAppProtections/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property defaultManagedAppProtections in deviceAppManagement",
        name: update_default_managed_app_protections,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/defaultManagedAppProtections/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property defaultManagedAppProtections for deviceAppManagement",
        name: delete_default_managed_app_protections,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/defaultManagedAppProtections/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get deviceAppManagement",
        name: get_device_app_management,
        response: serde_json::Value,
        path: "/deviceAppManagement",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update deviceAppManagement",
        name: update_device_app_management,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get managedAppRegistrations from deviceAppManagement",
        name: list_managed_app_registrations,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/managedAppRegistrations",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to managedAppRegistrations for deviceAppManagement",
        name: create_managed_app_registrations,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedAppRegistrations",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get mobileApps from deviceAppManagement",
        name: get_mobile_apps,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileApps/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property mobileApps in deviceAppManagement",
        name: update_mobile_apps,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileApps/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property mobileApps for deviceAppManagement",
        name: delete_mobile_apps,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileApps/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get mobileAppCategories from deviceAppManagement",
        name: list_mobile_app_categories,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/mobileAppCategories",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to mobileAppCategories for deviceAppManagement",
        name: create_mobile_app_categories,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileAppCategories",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get iosManagedAppProtections from deviceAppManagement",
        name: get_ios_managed_app_protections,
        response: serde_json::Value,
        path: "/deviceAppManagement/iosManagedAppProtections/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property iosManagedAppProtections in deviceAppManagement",
        name: update_ios_managed_app_protections,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/iosManagedAppProtections/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property iosManagedAppProtections for deviceAppManagement",
        name: delete_ios_managed_app_protections,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/iosManagedAppProtections/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get defaultManagedAppProtections from deviceAppManagement",
        name: list_default_managed_app_protections,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/defaultManagedAppProtections",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to defaultManagedAppProtections for deviceAppManagement",
        name: create_default_managed_app_protections,
        response: serde_json::Value,
        path: "/deviceAppManagement/defaultManagedAppProtections",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get mobileAppCategories from deviceAppManagement",
        name: get_mobile_app_categories,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileAppCategories/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property mobileAppCategories in deviceAppManagement",
        name: update_mobile_app_categories,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileAppCategories/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property mobileAppCategories for deviceAppManagement",
        name: delete_mobile_app_categories,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mobileAppCategories/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get mdmWindowsInformationProtectionPolicies from deviceAppManagement",
        name: list_mdm_windows_information_protection_policies,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/mdmWindowsInformationProtectionPolicies",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to mdmWindowsInformationProtectionPolicies for deviceAppManagement",
        name: create_mdm_windows_information_protection_policies,
        response: serde_json::Value,
        path: "/deviceAppManagement/mdmWindowsInformationProtectionPolicies",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get managedAppStatuses from deviceAppManagement",
        name: get_managed_app_statuses,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedAppStatuses/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property managedAppStatuses in deviceAppManagement",
        name: update_managed_app_statuses,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedAppStatuses/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property managedAppStatuses for deviceAppManagement",
        name: delete_managed_app_statuses,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedAppStatuses/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get androidManagedAppProtections from deviceAppManagement",
        name: get_android_managed_app_protections,
        response: serde_json::Value,
        path: "/deviceAppManagement/androidManagedAppProtections/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property androidManagedAppProtections in deviceAppManagement",
        name: update_android_managed_app_protections,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/androidManagedAppProtections/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property androidManagedAppProtections for deviceAppManagement",
        name: delete_android_managed_app_protections,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/androidManagedAppProtections/{{id}}",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action syncMicrosoftStoreForBusinessApps",
        name: sync_microsoft_store_for_business_apps,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/microsoft.graph.syncMicrosoftStoreForBusinessApps",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get managedAppPolicies from deviceAppManagement",
        name: list_managed_app_policies,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/managedAppPolicies",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to managedAppPolicies for deviceAppManagement",
        name: create_managed_app_policies,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedAppPolicies",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get managedAppRegistrations from deviceAppManagement",
        name: get_managed_app_registrations,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property managedAppRegistrations in deviceAppManagement",
        name: update_managed_app_registrations,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property managedAppRegistrations for deviceAppManagement",
        name: delete_managed_app_registrations,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get iosManagedAppProtections from deviceAppManagement",
        name: list_ios_managed_app_protections,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/iosManagedAppProtections",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to iosManagedAppProtections for deviceAppManagement",
        name: create_ios_managed_app_protections,
        response: serde_json::Value,
        path: "/deviceAppManagement/iosManagedAppProtections",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get vppTokens from deviceAppManagement",
        name: list_vpp_tokens,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/vppTokens",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to vppTokens for deviceAppManagement",
        name: create_vpp_tokens,
        response: serde_json::Value,
        path: "/deviceAppManagement/vppTokens",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get managedEBooks from deviceAppManagement",
        name: get_managed_e_books,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedEBooks/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property managedEBooks in deviceAppManagement",
        name: update_managed_e_books,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedEBooks/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property managedEBooks for deviceAppManagement",
        name: delete_managed_e_books,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/managedEBooks/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get androidManagedAppProtections from deviceAppManagement",
        name: list_android_managed_app_protections,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/androidManagedAppProtections",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to androidManagedAppProtections for deviceAppManagement",
        name: create_android_managed_app_protections,
        response: serde_json::Value,
        path: "/deviceAppManagement/androidManagedAppProtections",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get managedEBooks from deviceAppManagement",
        name: list_managed_e_books,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/managedEBooks",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to managedEBooks for deviceAppManagement",
        name: create_managed_e_books,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedEBooks",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get targetedManagedAppConfigurations from deviceAppManagement",
        name: get_targeted_managed_app_configurations,
        response: serde_json::Value,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property targetedManagedAppConfigurations in deviceAppManagement",
        name: update_targeted_managed_app_configurations,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property targetedManagedAppConfigurations for deviceAppManagement",
        name: delete_targeted_managed_app_configurations,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get windowsInformationProtectionPolicies from deviceAppManagement",
        name: list_windows_information_protection_policies,
        response: Collection<serde_json::Value>,
        path: "/deviceAppManagement/windowsInformationProtectionPolicies",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to windowsInformationProtectionPolicies for deviceAppManagement",
        name: create_windows_information_protection_policies,
        response: serde_json::Value,
        path: "/deviceAppManagement/windowsInformationProtectionPolicies",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get mdmWindowsInformationProtectionPolicies from deviceAppManagement",
        name: get_mdm_windows_information_protection_policies,
        response: serde_json::Value,
        path: "/deviceAppManagement/mdmWindowsInformationProtectionPolicies/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property mdmWindowsInformationProtectionPolicies in deviceAppManagement",
        name: update_mdm_windows_information_protection_policies,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mdmWindowsInformationProtectionPolicies/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property mdmWindowsInformationProtectionPolicies for deviceAppManagement",
        name: delete_mdm_windows_information_protection_policies,
        response: GraphResponse<Content>,
        path: "/deviceAppManagement/mdmWindowsInformationProtectionPolicies/{{id}}",
        params: 1,
        has_body: false
    });
}
