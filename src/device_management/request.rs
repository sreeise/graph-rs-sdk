use crate::client::Graph;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(DeviceManagementRequest,);
register_client!(AcceptanceStatusesRequest,);
register_client!(AppManagedDevicesRequest,);
register_client!(DetectedAppsRequest,);
register_client!(DeviceCompliancePoliciesRequest,);
register_client!(DeviceCompliancePolicySettingStateSummariesRequest,);
register_client!(DeviceConfigurationsRequest,);
register_client!(DeviceEnrollmentConfigurationsRequest,);
register_client!(ExchangeConnectorsRequest,);
register_client!(ManagedDevicesRequest,);
register_client!(NotificationMessageTemplatesRequest,);
register_client!(RemoteAssistancePartnersRequest,);
register_client!(RoleAssignmentsRequest,);
register_client!(RoleDefinitionsRequest,);
register_client!(ScheduledActionsForRuleRequest,);
register_client!(TermsAndConditionsRequest,);

impl<'a, Client> DeviceManagementRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get deviceConfigurationDeviceStateSummaries from deviceManagement",
        name: get_device_configuration_device_state_summaries,
        response: serde_json::Value,
        path: "/deviceManagement/deviceConfigurationDeviceStateSummaries",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deviceConfigurationDeviceStateSummaries in deviceManagement",
        name: update_device_configuration_device_state_summaries,
        response: NoContent,
        path: "/deviceManagement/deviceConfigurationDeviceStateSummaries",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get resourceOperations from deviceManagement",
        name: list_resource_operations,
        response: serde_json::Value,
        path: "/deviceManagement/resourceOperations",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to resourceOperations for deviceManagement",
        name: create_resource_operations,
        response: serde_json::Value,
        path: "/deviceManagement/resourceOperations",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get applePushNotificationCertificate from deviceManagement",
        name: get_apple_push_notification_certificate,
        response: serde_json::Value,
        path: "/deviceManagement/applePushNotificationCertificate",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property applePushNotificationCertificate in deviceManagement",
        name: update_apple_push_notification_certificate,
        response: NoContent,
        path: "/deviceManagement/applePushNotificationCertificate",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get complianceManagementPartners from deviceManagement",
        name: get_compliance_management_partners,
        response: serde_json::Value,
        path: "/deviceManagement/complianceManagementPartners/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property complianceManagementPartners in deviceManagement",
        name: update_compliance_management_partners,
        response: NoContent,
        path: "/deviceManagement/complianceManagementPartners/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get telecomExpenseManagementPartners from deviceManagement",
        name: get_telecom_expense_management_partners,
        response: serde_json::Value,
        path: "/deviceManagement/telecomExpenseManagementPartners/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property telecomExpenseManagementPartners in deviceManagement",
        name: update_telecom_expense_management_partners,
        response: NoContent,
        path: "/deviceManagement/telecomExpenseManagementPartners/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get exchangeConnectors from deviceManagement",
        name: list_exchange_connectors,
        response: serde_json::Value,
        path: "/deviceManagement/exchangeConnectors",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to exchangeConnectors for deviceManagement",
        name: create_exchange_connectors,
        response: serde_json::Value,
        path: "/deviceManagement/exchangeConnectors",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get iosUpdateStatuses from deviceManagement",
        name: list_ios_update_statuses,
        response: serde_json::Value,
        path: "/deviceManagement/iosUpdateStatuses",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to iosUpdateStatuses for deviceManagement",
        name: create_ios_update_statuses,
        response: serde_json::Value,
        path: "/deviceManagement/iosUpdateStatuses",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get deviceCategories from deviceManagement",
        name: get_device_categories,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCategories/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deviceCategories in deviceManagement",
        name: update_device_categories,
        response: NoContent,
        path: "/deviceManagement/deviceCategories/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get deviceManagementPartners from deviceManagement",
        name: list_device_management_partners,
        response: serde_json::Value,
        path: "/deviceManagement/deviceManagementPartners",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to deviceManagementPartners for deviceManagement",
        name: create_device_management_partners,
        response: serde_json::Value,
        path: "/deviceManagement/deviceManagementPartners",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get exchangeConnectors from deviceManagement",
        name: get_exchange_connectors,
        response: serde_json::Value,
        path: "/deviceManagement/exchangeConnectors/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property exchangeConnectors in deviceManagement",
        name: update_exchange_connectors,
        response: NoContent,
        path: "/deviceManagement/exchangeConnectors/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get managedDevices from deviceManagement",
        name: get_managed_devices,
        response: serde_json::Value,
        path: "/deviceManagement/managedDevices/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property managedDevices in deviceManagement",
        name: update_managed_devices,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get notificationMessageTemplates from deviceManagement",
        name: list_notification_message_templates,
        response: serde_json::Value,
        path: "/deviceManagement/notificationMessageTemplates",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to notificationMessageTemplates for deviceManagement",
        name: create_notification_message_templates,
        response: serde_json::Value,
        path: "/deviceManagement/notificationMessageTemplates",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get deviceCompliancePolicySettingStateSummaries from deviceManagement",
        name: list_device_compliance_policy_setting_state_summaries,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to deviceCompliancePolicySettingStateSummaries for deviceManagement",
        name: create_device_compliance_policy_setting_state_summaries,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get deviceCompliancePolicySettingStateSummaries from deviceManagement",
        name: get_device_compliance_policy_setting_state_summaries,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deviceCompliancePolicySettingStateSummaries in deviceManagement",
        name: update_device_compliance_policy_setting_state_summaries,
        response: NoContent,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get remoteAssistancePartners from deviceManagement",
        name: get_remote_assistance_partners,
        response: serde_json::Value,
        path: "/deviceManagement/remoteAssistancePartners/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property remoteAssistancePartners in deviceManagement",
        name: update_remote_assistance_partners,
        response: NoContent,
        path: "/deviceManagement/remoteAssistancePartners/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get iosUpdateStatuses from deviceManagement",
        name: get_ios_update_statuses,
        response: serde_json::Value,
        path: "/deviceManagement/iosUpdateStatuses/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property iosUpdateStatuses in deviceManagement",
        name: update_ios_update_statuses,
        response: NoContent,
        path: "/deviceManagement/iosUpdateStatuses/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get telecomExpenseManagementPartners from deviceManagement",
        name: list_telecom_expense_management_partners,
        response: serde_json::Value,
        path: "/deviceManagement/telecomExpenseManagementPartners",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to telecomExpenseManagementPartners for deviceManagement",
        name: create_telecom_expense_management_partners,
        response: serde_json::Value,
        path: "/deviceManagement/telecomExpenseManagementPartners",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get termsAndConditions from deviceManagement",
        name: list_terms_and_conditions,
        response: serde_json::Value,
        path: "/deviceManagement/termsAndConditions",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to termsAndConditions for deviceManagement",
        name: create_terms_and_conditions,
        response: serde_json::Value,
        path: "/deviceManagement/termsAndConditions",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Invoke function downloadApplePushNotificationCertificateSigningRequest",
        name: download_apple_push_notification_certificate_signing_request,
        response: serde_json::Value,
        path: "/deviceManagement/applePushNotificationCertificate/downloadApplePushNotificationCertificateSigningRequest()",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get deviceCategories from deviceManagement",
        name: list_device_categories,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCategories",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to deviceCategories for deviceManagement",
        name: create_device_categories,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCategories",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get managedDeviceOverview from deviceManagement",
        name: get_managed_device_overview,
        response: serde_json::Value,
        path: "/deviceManagement/managedDeviceOverview",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get roleDefinitions from deviceManagement",
        name: get_role_definitions,
        response: serde_json::Value,
        path: "/deviceManagement/roleDefinitions/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property roleDefinitions in deviceManagement",
        name: update_role_definitions,
        response: NoContent,
        path: "/deviceManagement/roleDefinitions/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get mobileThreatDefenseConnectors from deviceManagement",
        name: list_mobile_threat_defense_connectors,
        response: serde_json::Value,
        path: "/deviceManagement/mobileThreatDefenseConnectors",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to mobileThreatDefenseConnectors for deviceManagement",
        name: create_mobile_threat_defense_connectors,
        response: serde_json::Value,
        path: "/deviceManagement/mobileThreatDefenseConnectors",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get detectedApps from deviceManagement",
        name: list_detected_apps,
        response: serde_json::Value,
        path: "/deviceManagement/detectedApps",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to detectedApps for deviceManagement",
        name: create_detected_apps,
        response: serde_json::Value,
        path: "/deviceManagement/detectedApps",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get deviceEnrollmentConfigurations from deviceManagement",
        name: get_device_enrollment_configurations,
        response: serde_json::Value,
        path: "/deviceManagement/deviceEnrollmentConfigurations/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deviceEnrollmentConfigurations in deviceManagement",
        name: update_device_enrollment_configurations,
        response: NoContent,
        path: "/deviceManagement/deviceEnrollmentConfigurations/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get complianceManagementPartners from deviceManagement",
        name: list_compliance_management_partners,
        response: serde_json::Value,
        path: "/deviceManagement/complianceManagementPartners",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to complianceManagementPartners for deviceManagement",
        name: create_compliance_management_partners,
        response: serde_json::Value,
        path: "/deviceManagement/complianceManagementPartners",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get roleAssignments from deviceManagement",
        name: list_role_assignments,
        response: serde_json::Value,
        path: "/deviceManagement/roleAssignments",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to roleAssignments for deviceManagement",
        name: create_role_assignments,
        response: serde_json::Value,
        path: "/deviceManagement/roleAssignments",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get termsAndConditions from deviceManagement",
        name: get_terms_and_conditions,
        response: serde_json::Value,
        path: "/deviceManagement/termsAndConditions/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property termsAndConditions in deviceManagement",
        name: update_terms_and_conditions,
        response: NoContent,
        path: "/deviceManagement/termsAndConditions/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get deviceManagementPartners from deviceManagement",
        name: get_device_management_partners,
        response: serde_json::Value,
        path: "/deviceManagement/deviceManagementPartners/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deviceManagementPartners in deviceManagement",
        name: update_device_management_partners,
        response: NoContent,
        path: "/deviceManagement/deviceManagementPartners/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get deviceCompliancePolicies from deviceManagement",
        name: get_device_compliance_policies,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deviceCompliancePolicies in deviceManagement",
        name: update_device_compliance_policies,
        response: NoContent,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get troubleshootingEvents from deviceManagement",
        name: get_troubleshooting_events,
        response: serde_json::Value,
        path: "/deviceManagement/troubleshootingEvents/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property troubleshootingEvents in deviceManagement",
        name: update_troubleshooting_events,
        response: NoContent,
        path: "/deviceManagement/troubleshootingEvents/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get windowsInformationProtectionAppLearningSummaries from deviceManagement",
        name: list_windows_information_protection_app_learning_summaries,
        response: serde_json::Value,
        path: "/deviceManagement/windowsInformationProtectionAppLearningSummaries",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to windowsInformationProtectionAppLearningSummaries for deviceManagement",
        name: create_windows_information_protection_app_learning_summaries,
        response: serde_json::Value,
        path: "/deviceManagement/windowsInformationProtectionAppLearningSummaries",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get windowsInformationProtectionNetworkLearningSummaries from deviceManagement",
        name: get_windows_information_protection_network_learning_summaries,
        response: serde_json::Value,
        path: "/deviceManagement/windowsInformationProtectionNetworkLearningSummaries/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property windowsInformationProtectionNetworkLearningSummaries in deviceManagement",
        name: update_windows_information_protection_network_learning_summaries,
        response: NoContent,
        path: "/deviceManagement/windowsInformationProtectionNetworkLearningSummaries/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get troubleshootingEvents from deviceManagement",
        name: list_troubleshooting_events,
        response: serde_json::Value,
        path: "/deviceManagement/troubleshootingEvents",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to troubleshootingEvents for deviceManagement",
        name: create_troubleshooting_events,
        response: serde_json::Value,
        path: "/deviceManagement/troubleshootingEvents",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get deviceConfigurations from deviceManagement",
        name: get_device_configurations,
        response: serde_json::Value,
        path: "/deviceManagement/deviceConfigurations/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deviceConfigurations in deviceManagement",
        name: update_device_configurations,
        response: NoContent,
        path: "/deviceManagement/deviceConfigurations/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get detectedApps from deviceManagement",
        name: get_detected_apps,
        response: serde_json::Value,
        path: "/deviceManagement/detectedApps/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property detectedApps in deviceManagement",
        name: update_detected_apps,
        response: NoContent,
        path: "/deviceManagement/detectedApps/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get windowsInformationProtectionNetworkLearningSummaries from deviceManagement",
        name: list_windows_information_protection_network_learning_summaries,
        response: serde_json::Value,
        path: "/deviceManagement/windowsInformationProtectionNetworkLearningSummaries",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to windowsInformationProtectionNetworkLearningSummaries for deviceManagement",
        name: create_windows_information_protection_network_learning_summaries,
        response: serde_json::Value,
        path: "/deviceManagement/windowsInformationProtectionNetworkLearningSummaries",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get remoteAssistancePartners from deviceManagement",
        name: list_remote_assistance_partners,
        response: serde_json::Value,
        path: "/deviceManagement/remoteAssistancePartners",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to remoteAssistancePartners for deviceManagement",
        name: create_remote_assistance_partners,
        response: serde_json::Value,
        path: "/deviceManagement/remoteAssistancePartners",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get deviceCompliancePolicies from deviceManagement",
        name: list_device_compliance_policies,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to deviceCompliancePolicies for deviceManagement",
        name: create_device_compliance_policies,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get roleDefinitions from deviceManagement",
        name: list_role_definitions,
        response: serde_json::Value,
        path: "/deviceManagement/roleDefinitions",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to roleDefinitions for deviceManagement",
        name: create_role_definitions,
        response: serde_json::Value,
        path: "/deviceManagement/roleDefinitions",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get resourceOperations from deviceManagement",
        name: get_resource_operations,
        response: serde_json::Value,
        path: "/deviceManagement/resourceOperations/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property resourceOperations in deviceManagement",
        name: update_resource_operations,
        response: NoContent,
        path: "/deviceManagement/resourceOperations/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get deviceEnrollmentConfigurations from deviceManagement",
        name: list_device_enrollment_configurations,
        response: serde_json::Value,
        path: "/deviceManagement/deviceEnrollmentConfigurations",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to deviceEnrollmentConfigurations for deviceManagement",
        name: create_device_enrollment_configurations,
        response: serde_json::Value,
        path: "/deviceManagement/deviceEnrollmentConfigurations",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get conditionalAccessSettings from deviceManagement",
        name: get_conditional_access_settings,
        response: serde_json::Value,
        path: "/deviceManagement/conditionalAccessSettings",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property conditionalAccessSettings in deviceManagement",
        name: update_conditional_access_settings,
        response: NoContent,
        path: "/deviceManagement/conditionalAccessSettings",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get softwareUpdateStatusSummary from deviceManagement",
        name: get_software_update_status_summary,
        response: serde_json::Value,
        path: "/deviceManagement/softwareUpdateStatusSummary",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get mobileThreatDefenseConnectors from deviceManagement",
        name: get_mobile_threat_defense_connectors,
        response: serde_json::Value,
        path: "/deviceManagement/mobileThreatDefenseConnectors/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property mobileThreatDefenseConnectors in deviceManagement",
        name: update_mobile_threat_defense_connectors,
        response: NoContent,
        path: "/deviceManagement/mobileThreatDefenseConnectors/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get deviceManagement",
        name: get_device_management,
        response: serde_json::Value,
        path: "/deviceManagement",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update deviceManagement",
        name: update_device_management,
        response: NoContent,
        path: "/deviceManagement",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get deviceCompliancePolicyDeviceStateSummary from deviceManagement",
        name: get_device_compliance_policy_device_state_summary,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicyDeviceStateSummary",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deviceCompliancePolicyDeviceStateSummary in deviceManagement",
        name: update_device_compliance_policy_device_state_summary,
        response: NoContent,
        path: "/deviceManagement/deviceCompliancePolicyDeviceStateSummary",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get windowsInformationProtectionAppLearningSummaries from deviceManagement",
        name: get_windows_information_protection_app_learning_summaries,
        response: serde_json::Value,
        path: "/deviceManagement/windowsInformationProtectionAppLearningSummaries/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property windowsInformationProtectionAppLearningSummaries in deviceManagement",
        name: update_windows_information_protection_app_learning_summaries,
        response: NoContent,
        path: "/deviceManagement/windowsInformationProtectionAppLearningSummaries/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get notificationMessageTemplates from deviceManagement",
        name: get_notification_message_templates,
        response: serde_json::Value,
        path: "/deviceManagement/notificationMessageTemplates/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property notificationMessageTemplates in deviceManagement",
        name: update_notification_message_templates,
        response: NoContent,
        path: "/deviceManagement/notificationMessageTemplates/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get managedDevices from deviceManagement",
        name: list_managed_devices,
        response: serde_json::Value,
        path: "/deviceManagement/managedDevices",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to managedDevices for deviceManagement",
        name: create_managed_devices,
        response: serde_json::Value,
        path: "/deviceManagement/managedDevices",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get deviceConfigurations from deviceManagement",
        name: list_device_configurations,
        response: serde_json::Value,
        path: "/deviceManagement/deviceConfigurations",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to deviceConfigurations for deviceManagement",
        name: create_device_configurations,
        response: serde_json::Value,
        path: "/deviceManagement/deviceConfigurations",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get roleAssignments from deviceManagement",
        name: get_role_assignments,
        response: serde_json::Value,
        path: "/deviceManagement/roleAssignments/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property roleAssignments in deviceManagement",
        name: update_role_assignments,
        response: NoContent,
        path: "/deviceManagement/roleAssignments/{{id}}",
        params: 1,
        has_body: true
    });

    pub fn detected_apps(&self) -> DetectedAppsRequest<'a, Client> {
        DetectedAppsRequest::new(&self.client)
    }

    pub fn device_compliance_policies(&self) -> DeviceCompliancePoliciesRequest<'a, Client> {
        DeviceCompliancePoliciesRequest::new(&self.client)
    }

    pub fn device_compliance_policy_setting_state_summaries(
        &self,
    ) -> DeviceCompliancePolicySettingStateSummariesRequest<'a, Client> {
        DeviceCompliancePolicySettingStateSummariesRequest::new(&self.client)
    }

    pub fn device_configurations(&self) -> DeviceConfigurationsRequest<'a, Client> {
        DeviceConfigurationsRequest::new(&self.client)
    }

    pub fn device_enrollment_configurations(
        &self,
    ) -> DeviceEnrollmentConfigurationsRequest<'a, Client> {
        DeviceEnrollmentConfigurationsRequest::new(&self.client)
    }

    pub fn exchange_connectors(&self) -> ExchangeConnectorsRequest<'a, Client> {
        ExchangeConnectorsRequest::new(&self.client)
    }

    pub fn managed_devices(&self) -> ManagedDevicesRequest<'a, Client> {
        ManagedDevicesRequest::new(&self.client)
    }

    pub fn notification_message_templates(
        &self,
    ) -> NotificationMessageTemplatesRequest<'a, Client> {
        NotificationMessageTemplatesRequest::new(&self.client)
    }

    pub fn remote_assistance_partners(&self) -> RemoteAssistancePartnersRequest<'a, Client> {
        RemoteAssistancePartnersRequest::new(&self.client)
    }

    pub fn role_definitions(&self) -> RoleDefinitionsRequest<'a, Client> {
        RoleDefinitionsRequest::new(&self.client)
    }

    pub fn terms_and_conditions(&self) -> TermsAndConditionsRequest<'a, Client> {
        TermsAndConditionsRequest::new(&self.client)
    }
}

impl<'a, Client> DetectedAppsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get managedDevices from deviceManagement",
        name: get_managed_devices,
        response: serde_json::Value,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices/{{id2}}",
        params: 2,
        has_body: false
    });

    get!({
        doc: "# Get managedDevices from deviceManagement",
        name: list_managed_devices,
        response: serde_json::Value,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices",
        params: 1,
        has_body: false
    });

    pub fn app_managed_devices(&self) -> AppManagedDevicesRequest<'a, Client> {
        AppManagedDevicesRequest::new(&self.client)
    }
}

impl<'a, Client> AppManagedDevicesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action requestRemoteAssistance",
        name: request_remote_assistance,
        response: NoContent,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices/{{id2}}/requestRemoteAssistance",
        params: 2,
        has_body: false
    });

    post!({
        doc: "# Invoke action rebootNow",
        name: reboot_now,
        response: NoContent,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices/{{id2}}/rebootNow",
        params: 2,
        has_body: false
    });

    post!({
        doc: "# Invoke action cleanWindowsDevice",
        name: clean_windows_device,
        response: NoContent,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices/{{id2}}/cleanWindowsDevice",
        params: 2,
        has_body: true
    });

    post!({
        doc: "# Invoke action deleteUserFromSharedAppleDevice",
        name: delete_user_from_shared_apple_device,
        response: NoContent,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices/{{id2}}/deleteUserFromSharedAppleDevice",
        params: 2,
        has_body: true
    });

    post!({
        doc: "# Invoke action disableLostMode",
        name: disable_lost_mode,
        response: NoContent,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices/{{id2}}/disableLostMode",
        params: 2,
        has_body: false
    });

    post!({
        doc: "# Invoke action syncDevice",
        name: sync_device,
        response: NoContent,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices/{{id2}}/syncDevice",
        params: 2,
        has_body: false
    });

    post!({
        doc: "# Invoke action wipe",
        name: wipe,
        response: NoContent,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices/{{id2}}/wipe",
        params: 2,
        has_body: true
    });

    post!({
        doc: "# Invoke action updateWindowsDeviceAccount",
        name: update_windows_device_account,
        response: NoContent,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices/{{id2}}/updateWindowsDeviceAccount",
        params: 2,
        has_body: true
    });

    post!({
        doc: "# Invoke action windowsDefenderUpdateSignatures",
        name: windows_defender_update_signatures,
        response: NoContent,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices/{{id2}}/windowsDefenderUpdateSignatures",
        params: 2,
        has_body: false
    });

    post!({
        doc: "# Invoke action retire",
        name: retire,
        response: NoContent,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices/{{id2}}/retire",
        params: 2,
        has_body: false
    });

    post!({
        doc: "# Invoke action locateDevice",
        name: locate_device,
        response: NoContent,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices/{{id2}}/locateDevice",
        params: 2,
        has_body: false
    });

    post!({
        doc: "# Invoke action resetPasscode",
        name: reset_passcode,
        response: NoContent,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices/{{id2}}/resetPasscode",
        params: 2,
        has_body: false
    });

    post!({
        doc: "# Invoke action logoutSharedAppleDeviceActiveUser",
        name: logout_shared_apple_device_active_user,
        response: NoContent,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices/{{id2}}/logoutSharedAppleDeviceActiveUser",
        params: 2,
        has_body: false
    });

    post!({
        doc: "# Invoke action windowsDefenderScan",
        name: windows_defender_scan,
        response: NoContent,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices/{{id2}}/windowsDefenderScan",
        params: 2,
        has_body: true
    });

    post!({
        doc: "# Invoke action bypassActivationLock",
        name: bypass_activation_lock,
        response: NoContent,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices/{{id2}}/bypassActivationLock",
        params: 2,
        has_body: false
    });

    post!({
        doc: "# Invoke action shutDown",
        name: shut_down,
        response: NoContent,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices/{{id2}}/shutDown",
        params: 2,
        has_body: false
    });

    post!({
        doc: "# Invoke action remoteLock",
        name: remote_lock,
        response: NoContent,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices/{{id2}}/remoteLock",
        params: 2,
        has_body: false
    });

    post!({
        doc: "# Invoke action recoverPasscode",
        name: recover_passcode,
        response: NoContent,
        path: "/deviceManagement/detectedApps/{{id}}/managedDevices/{{id2}}/recoverPasscode",
        params: 2,
        has_body: false
    });
}

impl<'a, Client> DeviceCompliancePoliciesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get deviceSettingStateSummaries from deviceManagement",
        name: get_device_setting_state_summaries,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/deviceSettingStateSummaries/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deviceSettingStateSummaries in deviceManagement",
        name: update_device_setting_state_summaries,
        response: NoContent,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/deviceSettingStateSummaries/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get deviceStatuses from deviceManagement",
        name: list_device_statuses,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/deviceStatuses",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to deviceStatuses for deviceManagement",
        name: create_device_statuses,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/deviceStatuses",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action assign",
        name: assign,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/assign",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get deviceSettingStateSummaries from deviceManagement",
        name: list_device_setting_state_summaries,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/deviceSettingStateSummaries",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to deviceSettingStateSummaries for deviceManagement",
        name: create_device_setting_state_summaries,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/deviceSettingStateSummaries",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get assignments from deviceManagement",
        name: get_assignments,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property assignments in deviceManagement",
        name: update_assignments,
        response: NoContent,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: true
    });

    post!({
        doc: "# Invoke action scheduleActionsForRules",
        name: schedule_actions_for_rules,
        response: NoContent,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/scheduleActionsForRules",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get scheduledActionsForRule from deviceManagement",
        name: get_scheduled_actions_for_rule,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/scheduledActionsForRule/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property scheduledActionsForRule in deviceManagement",
        name: update_scheduled_actions_for_rule,
        response: NoContent,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/scheduledActionsForRule/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get deviceStatusOverview from deviceManagement",
        name: get_device_status_overview,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/deviceStatusOverview",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deviceStatusOverview in deviceManagement",
        name: update_device_status_overview,
        response: NoContent,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/deviceStatusOverview",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get scheduledActionsForRule from deviceManagement",
        name: list_scheduled_actions_for_rule,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/scheduledActionsForRule",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to scheduledActionsForRule for deviceManagement",
        name: create_scheduled_actions_for_rule,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/scheduledActionsForRule",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get deviceStatuses from deviceManagement",
        name: get_device_statuses,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/deviceStatuses/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deviceStatuses in deviceManagement",
        name: update_device_statuses,
        response: NoContent,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/deviceStatuses/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get assignments from deviceManagement",
        name: list_assignments,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/assignments",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to assignments for deviceManagement",
        name: create_assignments,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/assignments",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get userStatusOverview from deviceManagement",
        name: get_user_status_overview,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/userStatusOverview",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property userStatusOverview in deviceManagement",
        name: update_user_status_overview,
        response: NoContent,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/userStatusOverview",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get userStatuses from deviceManagement",
        name: get_user_statuses,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/userStatuses/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property userStatuses in deviceManagement",
        name: update_user_statuses,
        response: NoContent,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/userStatuses/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get userStatuses from deviceManagement",
        name: list_user_statuses,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/userStatuses",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to userStatuses for deviceManagement",
        name: create_user_statuses,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/userStatuses",
        params: 1,
        has_body: true
    });

    pub fn scheduled_actions_for_rule(&self) -> ScheduledActionsForRuleRequest<'a, Client> {
        ScheduledActionsForRuleRequest::new(&self.client)
    }
}

impl<'a, Client> ScheduledActionsForRuleRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get scheduledActionConfigurations from deviceManagement",
        name: list_scheduled_action_configurations,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/scheduledActionsForRule/{{id2}}/scheduledActionConfigurations",
        params: 2,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to scheduledActionConfigurations for deviceManagement",
        name: create_scheduled_action_configurations,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/scheduledActionsForRule/{{id2}}/scheduledActionConfigurations",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get scheduledActionConfigurations from deviceManagement",
        name: get_scheduled_action_configurations,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/scheduledActionsForRule/{{id2}}/scheduledActionConfigurations/{{id3}}",
        params: 3,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property scheduledActionConfigurations in deviceManagement",
        name: update_scheduled_action_configurations,
        response: NoContent,
        path: "/deviceManagement/deviceCompliancePolicies/{{id}}/scheduledActionsForRule/{{id2}}/scheduledActionConfigurations/{{id3}}",
        params: 3,
        has_body: true
    });
}

impl<'a, Client> DeviceCompliancePolicySettingStateSummariesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get deviceComplianceSettingStates from deviceManagement",
        name: get_device_compliance_setting_states,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries/{{id}}/deviceComplianceSettingStates/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deviceComplianceSettingStates in deviceManagement",
        name: update_device_compliance_setting_states,
        response: NoContent,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries/{{id}}/deviceComplianceSettingStates/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get deviceComplianceSettingStates from deviceManagement",
        name: list_device_compliance_setting_states,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries/{{id}}/deviceComplianceSettingStates",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to deviceComplianceSettingStates for deviceManagement",
        name: create_device_compliance_setting_states,
        response: serde_json::Value,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries/{{id}}/deviceComplianceSettingStates",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> DeviceConfigurationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get userStatuses from deviceManagement",
        name: get_user_statuses,
        response: serde_json::Value,
        path: "/deviceManagement/deviceConfigurations/{{id}}/userStatuses/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property userStatuses in deviceManagement",
        name: update_user_statuses,
        response: NoContent,
        path: "/deviceManagement/deviceConfigurations/{{id}}/userStatuses/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get deviceStatuses from deviceManagement",
        name: list_device_statuses,
        response: serde_json::Value,
        path: "/deviceManagement/deviceConfigurations/{{id}}/deviceStatuses",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to deviceStatuses for deviceManagement",
        name: create_device_statuses,
        response: serde_json::Value,
        path: "/deviceManagement/deviceConfigurations/{{id}}/deviceStatuses",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get userStatusOverview from deviceManagement",
        name: get_user_status_overview,
        response: serde_json::Value,
        path: "/deviceManagement/deviceConfigurations/{{id}}/userStatusOverview",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property userStatusOverview in deviceManagement",
        name: update_user_status_overview,
        response: NoContent,
        path: "/deviceManagement/deviceConfigurations/{{id}}/userStatusOverview",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get assignments from deviceManagement",
        name: get_assignments,
        response: serde_json::Value,
        path: "/deviceManagement/deviceConfigurations/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property assignments in deviceManagement",
        name: update_assignments,
        response: NoContent,
        path: "/deviceManagement/deviceConfigurations/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get deviceSettingStateSummaries from deviceManagement",
        name: list_device_setting_state_summaries,
        response: serde_json::Value,
        path: "/deviceManagement/deviceConfigurations/{{id}}/deviceSettingStateSummaries",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to deviceSettingStateSummaries for deviceManagement",
        name: create_device_setting_state_summaries,
        response: serde_json::Value,
        path: "/deviceManagement/deviceConfigurations/{{id}}/deviceSettingStateSummaries",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get assignments from deviceManagement",
        name: list_assignments,
        response: serde_json::Value,
        path: "/deviceManagement/deviceConfigurations/{{id}}/assignments",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to assignments for deviceManagement",
        name: create_assignments,
        response: serde_json::Value,
        path: "/deviceManagement/deviceConfigurations/{{id}}/assignments",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get userStatuses from deviceManagement",
        name: list_user_statuses,
        response: serde_json::Value,
        path: "/deviceManagement/deviceConfigurations/{{id}}/userStatuses",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to userStatuses for deviceManagement",
        name: create_user_statuses,
        response: serde_json::Value,
        path: "/deviceManagement/deviceConfigurations/{{id}}/userStatuses",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action assign",
        name: assign,
        response: serde_json::Value,
        path: "/deviceManagement/deviceConfigurations/{{id}}/assign",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get deviceSettingStateSummaries from deviceManagement",
        name: get_device_setting_state_summaries,
        response: serde_json::Value,
        path: "/deviceManagement/deviceConfigurations/{{id}}/deviceSettingStateSummaries/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deviceSettingStateSummaries in deviceManagement",
        name: update_device_setting_state_summaries,
        response: NoContent,
        path: "/deviceManagement/deviceConfigurations/{{id}}/deviceSettingStateSummaries/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get deviceStatusOverview from deviceManagement",
        name: get_device_status_overview,
        response: serde_json::Value,
        path: "/deviceManagement/deviceConfigurations/{{id}}/deviceStatusOverview",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deviceStatusOverview in deviceManagement",
        name: update_device_status_overview,
        response: NoContent,
        path: "/deviceManagement/deviceConfigurations/{{id}}/deviceStatusOverview",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get deviceStatuses from deviceManagement",
        name: get_device_statuses,
        response: serde_json::Value,
        path: "/deviceManagement/deviceConfigurations/{{id}}/deviceStatuses/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deviceStatuses in deviceManagement",
        name: update_device_statuses,
        response: NoContent,
        path: "/deviceManagement/deviceConfigurations/{{id}}/deviceStatuses/{{id2}}",
        params: 2,
        has_body: true
    });
}

impl<'a, Client> DeviceEnrollmentConfigurationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action setPriority",
        name: set_priority,
        response: NoContent,
        path: "/deviceManagement/deviceEnrollmentConfigurations/{{id}}/setPriority",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action assign",
        name: assign,
        response: NoContent,
        path: "/deviceManagement/deviceEnrollmentConfigurations/{{id}}/assign",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get assignments from deviceManagement",
        name: get_assignments,
        response: serde_json::Value,
        path: "/deviceManagement/deviceEnrollmentConfigurations/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property assignments in deviceManagement",
        name: update_assignments,
        response: NoContent,
        path: "/deviceManagement/deviceEnrollmentConfigurations/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get assignments from deviceManagement",
        name: list_assignments,
        response: serde_json::Value,
        path: "/deviceManagement/deviceEnrollmentConfigurations/{{id}}/assignments",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to assignments for deviceManagement",
        name: create_assignments,
        response: serde_json::Value,
        path: "/deviceManagement/deviceEnrollmentConfigurations/{{id}}/assignments",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> ExchangeConnectorsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action sync",
        name: sync,
        response: NoContent,
        path: "/deviceManagement/exchangeConnectors/{{id}}/sync",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> ManagedDevicesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action shutDown",
        name: shut_down,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/shutDown",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action bypassActivationLock",
        name: bypass_activation_lock,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/bypassActivationLock",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action retire",
        name: retire,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/retire",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action deleteUserFromSharedAppleDevice",
        name: delete_user_from_shared_apple_device,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/deleteUserFromSharedAppleDevice",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get deviceCompliancePolicyStates from deviceManagement",
        name: list_device_compliance_policy_states,
        response: serde_json::Value,
        path: "/deviceManagement/managedDevices/{{id}}/deviceCompliancePolicyStates",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to deviceCompliancePolicyStates for deviceManagement",
        name: create_device_compliance_policy_states,
        response: serde_json::Value,
        path: "/deviceManagement/managedDevices/{{id}}/deviceCompliancePolicyStates",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get deviceConfigurationStates from deviceManagement",
        name: list_device_configuration_states,
        response: serde_json::Value,
        path: "/deviceManagement/managedDevices/{{id}}/deviceConfigurationStates",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to deviceConfigurationStates for deviceManagement",
        name: create_device_configuration_states,
        response: serde_json::Value,
        path: "/deviceManagement/managedDevices/{{id}}/deviceConfigurationStates",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action wipe",
        name: wipe,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/wipe",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get deviceConfigurationStates from deviceManagement",
        name: get_device_configuration_states,
        response: serde_json::Value,
        path: "/deviceManagement/managedDevices/{{id}}/deviceConfigurationStates/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deviceConfigurationStates in deviceManagement",
        name: update_device_configuration_states,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/deviceConfigurationStates/{{id2}}",
        params: 2,
        has_body: true
    });

    post!({
        doc: "# Invoke action remoteLock",
        name: remote_lock,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/remoteLock",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action recoverPasscode",
        name: recover_passcode,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/recoverPasscode",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action disableLostMode",
        name: disable_lost_mode,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/disableLostMode",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get deviceCategory from deviceManagement",
        name: get_device_category,
        response: serde_json::Value,
        path: "/deviceManagement/managedDevices/{{id}}/deviceCategory",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deviceCategory in deviceManagement",
        name: update_device_category,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/deviceCategory",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action locateDevice",
        name: locate_device,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/locateDevice",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action logoutSharedAppleDeviceActiveUser",
        name: logout_shared_apple_device_active_user,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/logoutSharedAppleDeviceActiveUser",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action windowsDefenderUpdateSignatures",
        name: windows_defender_update_signatures,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/windowsDefenderUpdateSignatures",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action rebootNow",
        name: reboot_now,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/rebootNow",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action requestRemoteAssistance",
        name: request_remote_assistance,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/requestRemoteAssistance",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action updateWindowsDeviceAccount",
        name: update_windows_device_account,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/updateWindowsDeviceAccount",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action cleanWindowsDevice",
        name: clean_windows_device,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/cleanWindowsDevice",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action windowsDefenderScan",
        name: windows_defender_scan,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/windowsDefenderScan",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action resetPasscode",
        name: reset_passcode,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/resetPasscode",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action syncDevice",
        name: sync_device,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/syncDevice",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get deviceCompliancePolicyStates from deviceManagement",
        name: get_device_compliance_policy_states,
        response: serde_json::Value,
        path: "/deviceManagement/managedDevices/{{id}}/deviceCompliancePolicyStates/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deviceCompliancePolicyStates in deviceManagement",
        name: update_device_compliance_policy_states,
        response: NoContent,
        path: "/deviceManagement/managedDevices/{{id}}/deviceCompliancePolicyStates/{{id2}}",
        params: 2,
        has_body: true
    });
}

impl<'a, Client> NotificationMessageTemplatesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get localizedNotificationMessages from deviceManagement",
        name: get_localized_notification_messages,
        response: serde_json::Value,
        path: "/deviceManagement/notificationMessageTemplates/{{id}}/localizedNotificationMessages/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property localizedNotificationMessages in deviceManagement",
        name: update_localized_notification_messages,
        response: NoContent,
        path: "/deviceManagement/notificationMessageTemplates/{{id}}/localizedNotificationMessages/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get localizedNotificationMessages from deviceManagement",
        name: list_localized_notification_messages,
        response: serde_json::Value,
        path: "/deviceManagement/notificationMessageTemplates/{{id}}/localizedNotificationMessages",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to localizedNotificationMessages for deviceManagement",
        name: create_localized_notification_messages,
        response: serde_json::Value,
        path: "/deviceManagement/notificationMessageTemplates/{{id}}/localizedNotificationMessages",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action sendTestMessage",
        name: send_test_message,
        response: NoContent,
        path: "/deviceManagement/notificationMessageTemplates/{{id}}/sendTestMessage",
        params: 1,
        has_body: false
    });
}

impl<'a, Client> RemoteAssistancePartnersRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action disconnect",
        name: disconnect,
        response: NoContent,
        path: "/deviceManagement/remoteAssistancePartners/{{id}}/disconnect",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action beginOnboarding",
        name: begin_onboarding,
        response: NoContent,
        path: "/deviceManagement/remoteAssistancePartners/{{id}}/beginOnboarding",
        params: 1,
        has_body: false
    });
}

impl<'a, Client> RoleDefinitionsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get roleAssignments from deviceManagement",
        name: list_role_assignments,
        response: serde_json::Value,
        path: "/deviceManagement/roleDefinitions/{{id}}/roleAssignments",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to roleAssignments for deviceManagement",
        name: create_role_assignments,
        response: serde_json::Value,
        path: "/deviceManagement/roleDefinitions/{{id}}/roleAssignments",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get roleAssignments from deviceManagement",
        name: get_role_assignments,
        response: serde_json::Value,
        path: "/deviceManagement/roleDefinitions/{{id}}/roleAssignments/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property roleAssignments in deviceManagement",
        name: update_role_assignments,
        response: NoContent,
        path: "/deviceManagement/roleDefinitions/{{id}}/roleAssignments/{{id2}}",
        params: 2,
        has_body: true
    });

    pub fn role_assignments(&self) -> RoleAssignmentsRequest<'a, Client> {
        RoleAssignmentsRequest::new(&self.client)
    }
}

impl<'a, Client> RoleAssignmentsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get roleDefinition from deviceManagement",
        name: get_role_definition,
        response: serde_json::Value,
        path: "/deviceManagement/roleDefinitions/{{id}}/roleAssignments/{{id2}}/roleDefinition",
        params: 2,
        has_body: false
    });
}

impl<'a, Client> TermsAndConditionsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get assignments from deviceManagement",
        name: get_assignments,
        response: serde_json::Value,
        path: "/deviceManagement/termsAndConditions/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property assignments in deviceManagement",
        name: update_assignments,
        response: NoContent,
        path: "/deviceManagement/termsAndConditions/{{id}}/assignments/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get assignments from deviceManagement",
        name: list_assignments,
        response: serde_json::Value,
        path: "/deviceManagement/termsAndConditions/{{id}}/assignments",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to assignments for deviceManagement",
        name: create_assignments,
        response: serde_json::Value,
        path: "/deviceManagement/termsAndConditions/{{id}}/assignments",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get acceptanceStatuses from deviceManagement",
        name: list_acceptance_statuses,
        response: serde_json::Value,
        path: "/deviceManagement/termsAndConditions/{{id}}/acceptanceStatuses",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to acceptanceStatuses for deviceManagement",
        name: create_acceptance_statuses,
        response: serde_json::Value,
        path: "/deviceManagement/termsAndConditions/{{id}}/acceptanceStatuses",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get acceptanceStatuses from deviceManagement",
        name: get_acceptance_statuses,
        response: serde_json::Value,
        path: "/deviceManagement/termsAndConditions/{{id}}/acceptanceStatuses/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property acceptanceStatuses in deviceManagement",
        name: update_acceptance_statuses,
        response: NoContent,
        path: "/deviceManagement/termsAndConditions/{{id}}/acceptanceStatuses/{{id2}}",
        params: 2,
        has_body: true
    });

    pub fn acceptance_statuses(&self) -> AcceptanceStatusesRequest<'a, Client> {
        AcceptanceStatusesRequest::new(&self.client)
    }
}

impl<'a, Client> AcceptanceStatusesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get termsAndConditions from deviceManagement",
        name: get_terms_and_conditions,
        response: serde_json::Value,
        path: "/deviceManagement/termsAndConditions/{{id}}/acceptanceStatuses/{{id2}}/termsAndConditions",
        params: 2,
        has_body: false
    });
}
