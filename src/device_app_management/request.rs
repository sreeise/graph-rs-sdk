use crate::client::Graph;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(DeviceAppManagementRequest,);
register_client!(AndroidManagedAppProtectionsRequest,);
register_client!(AppliedPoliciesRequest,);
register_client!(DefaultManagedAppProtectionsRequest,);
register_client!(IntendedPoliciesRequest,);
register_client!(IosManagedAppProtectionsRequest,);
register_client!(ManagedAppPoliciesRequest,);
register_client!(ManagedAppRegistrationsRequest,);
register_client!(ManagedEBooksRequest,);
register_client!(MobileAppConfigurationsRequest,);
register_client!(MobileAppsRequest,);
register_client!(TargetedManagedAppConfigurationsRequest,);
register_client!(UserStateSummaryRequest,);
register_client!(VppTokensRequest,);

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
        response: NoContent,
        path: "/deviceAppManagement/windowsInformationProtectionPolicies/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get androidManagedAppProtections from deviceAppManagement",
        name: list_android_managed_app_protections,
        response: serde_json::Value,
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
    post!({
        doc: "# Invoke action syncMicrosoftStoreForBusinessApps",
        name: sync_microsoft_store_for_business_apps,
        response: NoContent,
        path: "/deviceAppManagement/syncMicrosoftStoreForBusinessApps",
        params: 0,
        has_body: false
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
        response: NoContent,
        path: "/deviceAppManagement/mobileApps/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get vppTokens from deviceAppManagement",
        name: list_vpp_tokens,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/deviceAppManagement/managedEBooks/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get mobileAppCategories from deviceAppManagement",
        name: list_mobile_app_categories,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/deviceAppManagement/iosManagedAppProtections/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get mobileAppConfigurations from deviceAppManagement",
        name: list_mobile_app_configurations,
        response: serde_json::Value,
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
        doc: "# Get windowsInformationProtectionPolicies from deviceAppManagement",
        name: list_windows_information_protection_policies,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Invoke function getUserIdsWithFlaggedAppRegistration",
        name: get_user_ids_with_flagged_app_registration,
        response: serde_json::Value,
        path: "/deviceAppManagement/managedAppRegistrations/getUserIdsWithFlaggedAppRegistration()",
        params: 0,
        has_body: false
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
        response: NoContent,
        path: "/deviceAppManagement/vppTokens/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get targetedManagedAppConfigurations from deviceAppManagement",
        name: list_targeted_managed_app_configurations,
        response: serde_json::Value,
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
        doc: "# Get managedAppPolicies from deviceAppManagement",
        name: list_managed_app_policies,
        response: serde_json::Value,
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
        doc: "# Get managedEBooks from deviceAppManagement",
        name: list_managed_e_books,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/deviceAppManagement/androidManagedAppProtections/{{id}}",
        params: 1,
        has_body: true
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
        response: NoContent,
        path: "/deviceAppManagement/defaultManagedAppProtections/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get defaultManagedAppProtections from deviceAppManagement",
        name: list_default_managed_app_protections,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/deviceAppManagement",
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
        response: NoContent,
        path: "/deviceAppManagement/mobileAppCategories/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get mobileApps from deviceAppManagement",
        name: list_mobile_apps,
        response: serde_json::Value,
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
        doc: "# Get managedAppRegistrations from deviceAppManagement",
        name: list_managed_app_registrations,
        response: serde_json::Value,
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
        doc: "# Get mdmWindowsInformationProtectionPolicies from deviceAppManagement",
        name: list_mdm_windows_information_protection_policies,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/deviceAppManagement/managedAppStatuses/{{id}}",
        params: 1,
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
        response: NoContent,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get iosManagedAppProtections from deviceAppManagement",
        name: list_ios_managed_app_protections,
        response: serde_json::Value,
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
        doc: "# Get managedAppStatuses from deviceAppManagement",
        name: list_managed_app_statuses,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/deviceAppManagement/mdmWindowsInformationProtectionPolicies/{{id}}",
        params: 1,
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
        response: NoContent,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}",
        params: 1,
        has_body: true
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
        response: NoContent,
        path: "/deviceAppManagement/managedAppPolicies/{{id}}",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> AndroidManagedAppProtectionsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
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
        response: NoContent,
        path: "/deviceAppManagement/androidManagedAppProtections/{{id}}/deploymentSummary",
        params: 1,
        has_body: true
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
        response: NoContent,
        path: "/deviceAppManagement/androidManagedAppProtections/{{id}}/apps/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get apps from deviceAppManagement",
        name: list_apps,
        response: serde_json::Value,
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
}

impl<'a, Client> DefaultManagedAppProtectionsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get apps from deviceAppManagement",
        name: list_apps,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/deviceAppManagement/defaultManagedAppProtections/{{id}}/deploymentSummary",
        params: 1,
        has_body: true
    });
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
        response: NoContent,
        path: "/deviceAppManagement/defaultManagedAppProtections/{{id}}/apps/{{id2}}",
        params: 2,
        has_body: true
    });
}

impl<'a, Client> IosManagedAppProtectionsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get apps from deviceAppManagement",
        name: list_apps,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/deviceAppManagement/iosManagedAppProtections/{{id}}/apps/{{id2}}",
        params: 2,
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
        response: NoContent,
        path: "/deviceAppManagement/iosManagedAppProtections/{{id}}/deploymentSummary",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> ManagedAppPoliciesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action targetApps",
        name: target_apps,
        response: NoContent,
        path: "/deviceAppManagement/managedAppPolicies/{{id}}/targetApps",
        params: 1,
        has_body: true
    });
}

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
        doc: "# Get operations from deviceAppManagement",
        name: list_operations,
        response: serde_json::Value,
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
        doc: "# Get intendedPolicies from deviceAppManagement",
        name: list_intended_policies,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/operations/{{id2}}",
        params: 2,
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
        response: NoContent,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/appliedPolicies/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get appliedPolicies from deviceAppManagement",
        name: list_applied_policies,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/intendedPolicies/{{id2}}",
        params: 2,
        has_body: true
    });
}

impl<'a, Client> AppliedPoliciesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action targetApps",
        name: target_apps,
        response: NoContent,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/appliedPolicies/{{id2}}/targetApps",
        params: 2,
        has_body: true
    });
}

impl<'a, Client> IntendedPoliciesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action targetApps",
        name: target_apps,
        response: NoContent,
        path: "/deviceAppManagement/managedAppRegistrations/{{id}}/intendedPolicies/{{id2}}/targetApps",
        params: 2,
        has_body: true
    });
}

impl<'a, Client> ManagedEBooksRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn user_state_summary(&self) -> UserStateSummaryRequest<'a, Client> {
        UserStateSummaryRequest::new(&self.client)
    }
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
        response: NoContent,
        path: "/deviceAppManagement/managedEBooks/{{id}}/userStateSummary/{{id2}}",
        params: 2,
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
        response: NoContent,
        path: "/deviceAppManagement/managedEBooks/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get assignments from deviceAppManagement",
        name: list_assignments,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/deviceAppManagement/managedEBooks/{{id}}/installSummary",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get deviceStates from deviceAppManagement",
        name: list_device_states,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/deviceAppManagement/managedEBooks/{{id}}/deviceStates/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get userStateSummary from deviceAppManagement",
        name: list_user_state_summary,
        response: serde_json::Value,
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
    post!({
        doc: "# Invoke action assign",
        name: assign,
        response: NoContent,
        path: "/deviceAppManagement/managedEBooks/{{id}}/assign",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> UserStateSummaryRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get deviceStates from deviceAppManagement",
        name: list_device_states,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/deviceAppManagement/managedEBooks/{{id}}/userStateSummary/{{id2}}/deviceStates/{{id3}}",
        params: 3,
        has_body: true
    });
}

impl<'a, Client> MobileAppConfigurationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action assign",
        name: assign,
        response: NoContent,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/assign",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get userStatuses from deviceAppManagement",
        name: list_user_statuses,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/userStatuses/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get assignments from deviceAppManagement",
        name: list_assignments,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get deviceStatuses from deviceAppManagement",
        name: list_device_statuses,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/deviceStatuses/{{id2}}",
        params: 2,
        has_body: true
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
        response: NoContent,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/userStatusSummary",
        params: 1,
        has_body: true
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
        response: NoContent,
        path: "/deviceAppManagement/mobileAppConfigurations/{{id}}/deviceStatusSummary",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> MobileAppsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
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
        response: NoContent,
        path: "/deviceAppManagement/mobileApps/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get categories from deviceAppManagement",
        name: get_categories,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileApps/{{id}}/categories/{{id2}}",
        params: 2,
        has_body: false
    });
    post!({
        doc: "# Invoke action assign",
        name: assign,
        response: NoContent,
        path: "/deviceAppManagement/mobileApps/{{id}}/assign",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get categories from deviceAppManagement",
        name: list_categories,
        response: serde_json::Value,
        path: "/deviceAppManagement/mobileApps/{{id}}/categories",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get assignments from deviceAppManagement",
        name: list_assignments,
        response: serde_json::Value,
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
}

impl<'a, Client> TargetedManagedAppConfigurationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get assignments from deviceAppManagement",
        name: list_assignments,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get apps from deviceAppManagement",
        name: list_apps,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/apps/{{id2}}",
        params: 2,
        has_body: true
    });
    post!({
        doc: "# Invoke action assign",
        name: assign,
        response: NoContent,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/assign",
        params: 1,
        has_body: true
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
        response: NoContent,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/deploymentSummary",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action targetApps",
        name: target_apps,
        response: NoContent,
        path: "/deviceAppManagement/targetedManagedAppConfigurations/{{id}}/targetApps",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> VppTokensRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action syncLicenses",
        name: sync_licenses,
        response: serde_json::Value,
        path: "/deviceAppManagement/vppTokens/{{id}}/syncLicenses",
        params: 1,
        has_body: false
    });
}
