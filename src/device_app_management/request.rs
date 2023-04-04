// GENERATED CODE

use crate::api_default_imports::*;
use crate::device_app_management::*;

resource_api_client!(
    DeviceAppManagementApiClient,
    ResourceIdentity::DeviceAppManagement
);

impl DeviceAppManagementApiClient {
    api_client_link_id!(mobile_app_categories_id, MobileAppCategoriesIdApiClient);
    api_client_link!(mobile_app_configurations, MobileAppConfigurationsApiClient);
    api_client_link_id!(
        android_managed_app_protection,
        AndroidManagedAppProtectionsIdApiClient
    );
    api_client_link_id!(vpp_token, VppTokensIdApiClient);
    api_client_link!(
        targeted_managed_app_configurations,
        TargetedManagedAppConfigurationsApiClient
    );
    api_client_link_id!(
        windows_information_protection_policies_id,
        WindowsInformationProtectionPoliciesIdApiClient
    );
    api_client_link_id!(managed_app_statuses_id, ManagedAppStatusesIdApiClient);
    api_client_link!(
        android_managed_app_protections,
        AndroidManagedAppProtectionsApiClient
    );
    api_client_link!(
        mdm_windows_information_protection_policies,
        MdmWindowsInformationProtectionPoliciesApiClient
    );
    api_client_link!(mobile_apps, MobileAppsApiClient);
    api_client_link!(managed_app_statuses, ManagedAppStatusesApiClient);
    api_client_link!(vpp_tokens, VppTokensApiClient);
    api_client_link_id!(mobile_app_configuration, MobileAppConfigurationsIdApiClient);
    api_client_link_id!(
        ios_managed_app_protection,
        IosManagedAppProtectionsIdApiClient
    );
    api_client_link_id!(
        targeted_managed_app_configuration,
        TargetedManagedAppConfigurationsIdApiClient
    );
    api_client_link!(managed_e_books, ManagedEBooksApiClient);
    api_client_link!(managed_app_policies, ManagedAppPoliciesApiClient);
    api_client_link_id!(
        default_managed_app_protection,
        DefaultManagedAppProtectionsIdApiClient
    );
    api_client_link_id!(managed_e_book, ManagedEBooksIdApiClient);
    api_client_link_id!(mobile_app, MobileAppsIdApiClient);
    api_client_link!(
        windows_information_protection_policies,
        WindowsInformationProtectionPoliciesApiClient
    );
    api_client_link!(
        default_managed_app_protections,
        DefaultManagedAppProtectionsApiClient
    );
    api_client_link!(
        ios_managed_app_protections,
        IosManagedAppProtectionsApiClient
    );
    api_client_link_id!(
        mdm_windows_information_protection_policy,
        MdmWindowsInformationProtectionPoliciesIdApiClient
    );
    api_client_link_id!(managed_app_policies_id, ManagedAppPoliciesIdApiClient);
    api_client_link!(mobile_app_categories, MobileAppCategoriesApiClient);
    api_client_link_id!(managed_app_registration, ManagedAppRegistrationsIdApiClient);
    api_client_link!(managed_app_registrations, ManagedAppRegistrationsApiClient);

    get!(
        doc: "Get deviceAppManagement",
        name: get_device_app_management,
        path: "/deviceAppManagement"
    );
    patch!(
        doc: "Update deviceAppManagement",
        name: update_device_app_management,
        path: "/deviceAppManagement",
        body: true
    );
    post!(
        doc: "Invoke action syncMicrosoftStoreForBusinessApps",
        name: sync_microsoft_store_for_business_apps,
        path: "/deviceAppManagement/syncMicrosoftStoreForBusinessApps"
    );
}
