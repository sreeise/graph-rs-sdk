// GENERATED CODE

use crate::api_default_imports::*;
use crate::device_management::{
    DeviceConfigurationsApiClient, DeviceConfigurationsIdApiClient,
    DeviceEnrollmentConfigurationsApiClient, DeviceEnrollmentConfigurationsIdApiClient,
    ManagedDevicesApiClient, ManagedDevicesIdApiClient, RoleDefinitionsApiClient,
    RoleDefinitionsIdApiClient, TermsAndConditionsApiClient, TermsAndConditionsIdApiClient,
};

resource_api_client!(
    DeviceManagementApiClient,
    ResourceIdentity::DeviceManagement
);

impl DeviceManagementApiClient {
    api_client_link!(
        device_configurations,
        ResourceIdentity::DeviceConfigurations,
        DeviceConfigurationsApiClient
    );
    api_client_link!(
        device_enrollment_configurations,
        ResourceIdentity::DeviceEnrollmentConfigurations,
        DeviceEnrollmentConfigurationsApiClient
    );
    api_client_link_id!(
        managed_device,
        ResourceIdentity::ManagedDevices,
        ManagedDevicesIdApiClient
    );
    api_client_link_id!(
        role_definition,
        ResourceIdentity::RoleDefinitions,
        RoleDefinitionsIdApiClient
    );
    api_client_link!(
        managed_devices,
        ResourceIdentity::ManagedDevices,
        ManagedDevicesApiClient
    );
    api_client_link_id!(
        device_enrollment_configuration,
        ResourceIdentity::DeviceEnrollmentConfigurations,
        DeviceEnrollmentConfigurationsIdApiClient
    );
    api_client_link_id!(
        terms_and_condition,
        ResourceIdentity::TermsAndConditions,
        TermsAndConditionsIdApiClient
    );
    api_client_link!(
        role_definitions,
        ResourceIdentity::RoleDefinitions,
        RoleDefinitionsApiClient
    );
    api_client_link!(
        terms_and_conditions,
        ResourceIdentity::TermsAndConditions,
        TermsAndConditionsApiClient
    );
    api_client_link_id!(
        device_configuration,
        ResourceIdentity::DeviceConfigurations,
        DeviceConfigurationsIdApiClient
    );

    get!(
        doc: "Get deviceManagement",
        name: get_device_management,
        path: "/deviceManagement"
    );
    patch!(
        doc: "Update deviceManagement",
        name: update_device_management,
        path: "/deviceManagement",
        body: true
    );
    delete!(
        doc: "Delete navigation property applePushNotificationCertificate for deviceManagement",
        name: delete_apple_push_notification_certificate,
        path: "/deviceManagement/applePushNotificationCertificate"
    );
    get!(
        doc: "Get applePushNotificationCertificate from deviceManagement",
        name: get_apple_push_notification_certificate,
        path: "/deviceManagement/applePushNotificationCertificate"
    );
    patch!(
        doc: "Update the navigation property applePushNotificationCertificate in deviceManagement",
        name: update_apple_push_notification_certificate,
        path: "/deviceManagement/applePushNotificationCertificate",
        body: true
    );
    get!(
        doc: "Invoke function downloadApplePushNotificationCertificateSigningRequest",
        name: download_apple_push_notification_certificate_signing_request,
        path: "/deviceManagement/applePushNotificationCertificate/microsoft.graph.downloadApplePushNotificationCertificateSigningRequest()"
    );
    post!(
        doc: "Create new navigation property to auditEvents for deviceManagement",
        name: create_audit_events,
        path: "/deviceManagement/auditEvents",
        body: true
    );
    get!(
        doc: "Get auditEvents from deviceManagement",
        name: list_audit_events,
        path: "/deviceManagement/auditEvents"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_audit_events_count,
        path: "/deviceManagement/auditEvents/$count"
    );
    get!(
        doc: "Invoke function getAuditActivityTypes",
        name: get_audit_activity_types,
        path: "/deviceManagement/auditEvents/microsoft.graph.getAuditActivityTypes(category='{{id}}')",
        params: category
    );
    get!(
        doc: "Invoke function getAuditCategories",
        name: get_audit_categories,
        path: "/deviceManagement/auditEvents/microsoft.graph.getAuditCategories()"
    );
    delete!(
        doc: "Delete navigation property auditEvents for deviceManagement",
        name: delete_audit_events,
        path: "/deviceManagement/auditEvents/{{id}}",
        params: audit_event_id
    );
    get!(
        doc: "Get auditEvents from deviceManagement",
        name: get_audit_events,
        path: "/deviceManagement/auditEvents/{{id}}",
        params: audit_event_id
    );
    patch!(
        doc: "Update the navigation property auditEvents in deviceManagement",
        name: update_audit_events,
        path: "/deviceManagement/auditEvents/{{id}}",
        body: true,
        params: audit_event_id
    );
    post!(
        doc: "Create new navigation property to complianceManagementPartners for deviceManagement",
        name: create_compliance_management_partners,
        path: "/deviceManagement/complianceManagementPartners",
        body: true
    );
    get!(
        doc: "Get complianceManagementPartners from deviceManagement",
        name: list_compliance_management_partners,
        path: "/deviceManagement/complianceManagementPartners"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_compliance_management_partners_count,
        path: "/deviceManagement/complianceManagementPartners/$count"
    );
    delete!(
        doc: "Delete navigation property complianceManagementPartners for deviceManagement",
        name: delete_compliance_management_partners,
        path: "/deviceManagement/complianceManagementPartners/{{id}}",
        params: compliance_management_partner_id
    );
    get!(
        doc: "Get complianceManagementPartners from deviceManagement",
        name: get_compliance_management_partners,
        path: "/deviceManagement/complianceManagementPartners/{{id}}",
        params: compliance_management_partner_id
    );
    patch!(
        doc: "Update the navigation property complianceManagementPartners in deviceManagement",
        name: update_compliance_management_partners,
        path: "/deviceManagement/complianceManagementPartners/{{id}}",
        body: true,
        params: compliance_management_partner_id
    );
    delete!(
        doc: "Delete navigation property conditionalAccessSettings for deviceManagement",
        name: delete_conditional_access_settings,
        path: "/deviceManagement/conditionalAccessSettings"
    );
    get!(
        doc: "Get conditionalAccessSettings from deviceManagement",
        name: get_conditional_access_settings,
        path: "/deviceManagement/conditionalAccessSettings"
    );
    patch!(
        doc: "Update the navigation property conditionalAccessSettings in deviceManagement",
        name: update_conditional_access_settings,
        path: "/deviceManagement/conditionalAccessSettings",
        body: true
    );
    post!(
        doc: "Create new navigation property to detectedApps for deviceManagement",
        name: create_detected_apps,
        path: "/deviceManagement/detectedApps",
        body: true
    );
    get!(
        doc: "Get detectedApps from deviceManagement",
        name: list_detected_apps,
        path: "/deviceManagement/detectedApps"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_detected_apps_count,
        path: "/deviceManagement/detectedApps/$count"
    );
    delete!(
        doc: "Delete navigation property detectedApps for deviceManagement",
        name: delete_detected_apps,
        path: "/deviceManagement/detectedApps/{{id}}",
        params: detected_app_id
    );
    get!(
        doc: "Get detectedApps from deviceManagement",
        name: get_detected_apps,
        path: "/deviceManagement/detectedApps/{{id}}",
        params: detected_app_id
    );
    patch!(
        doc: "Update the navigation property detectedApps in deviceManagement",
        name: update_detected_apps,
        path: "/deviceManagement/detectedApps/{{id}}",
        body: true,
        params: detected_app_id
    );
    post!(
        doc: "Create new navigation property to deviceCategories for deviceManagement",
        name: create_device_categories,
        path: "/deviceManagement/deviceCategories",
        body: true
    );
    get!(
        doc: "Get deviceCategories from deviceManagement",
        name: list_device_categories,
        path: "/deviceManagement/deviceCategories"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_categories_count,
        path: "/deviceManagement/deviceCategories/$count"
    );
    delete!(
        doc: "Delete navigation property deviceCategories for deviceManagement",
        name: delete_device_categories,
        path: "/deviceManagement/deviceCategories/{{id}}",
        params: device_category_id
    );
    get!(
        doc: "Get deviceCategories from deviceManagement",
        name: get_device_categories,
        path: "/deviceManagement/deviceCategories/{{id}}",
        params: device_category_id
    );
    patch!(
        doc: "Update the navigation property deviceCategories in deviceManagement",
        name: update_device_categories,
        path: "/deviceManagement/deviceCategories/{{id}}",
        body: true,
        params: device_category_id
    );
    delete!(
        doc: "Delete navigation property deviceCompliancePolicyDeviceStateSummary for deviceManagement",
        name: delete_device_compliance_policy_device_state_summary,
        path: "/deviceManagement/deviceCompliancePolicyDeviceStateSummary"
    );
    get!(
        doc: "Get deviceCompliancePolicyDeviceStateSummary from deviceManagement",
        name: get_device_compliance_policy_device_state_summary,
        path: "/deviceManagement/deviceCompliancePolicyDeviceStateSummary"
    );
    patch!(
        doc: "Update the navigation property deviceCompliancePolicyDeviceStateSummary in deviceManagement",
        name: update_device_compliance_policy_device_state_summary,
        path: "/deviceManagement/deviceCompliancePolicyDeviceStateSummary",
        body: true
    );
    post!(
        doc: "Create new navigation property to deviceCompliancePolicySettingStateSummaries for deviceManagement",
        name: create_device_compliance_policy_setting_state_summaries,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries",
        body: true
    );
    get!(
        doc: "Get deviceCompliancePolicySettingStateSummaries from deviceManagement",
        name: list_device_compliance_policy_setting_state_summaries,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_compliance_policy_setting_state_summaries_count,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries/$count"
    );
    delete!(
        doc: "Delete navigation property deviceCompliancePolicySettingStateSummaries for deviceManagement",
        name: delete_device_compliance_policy_setting_state_summaries,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries/{{id}}",
        params: device_compliance_policy_setting_state_summary_id
    );
    get!(
        doc: "Get deviceCompliancePolicySettingStateSummaries from deviceManagement",
        name: get_device_compliance_policy_setting_state_summaries,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries/{{id}}",
        params: device_compliance_policy_setting_state_summary_id
    );
    patch!(
        doc: "Update the navigation property deviceCompliancePolicySettingStateSummaries in deviceManagement",
        name: update_device_compliance_policy_setting_state_summaries,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries/{{id}}",
        body: true,
        params: device_compliance_policy_setting_state_summary_id
    );
    post!(
        doc: "Create new navigation property to deviceComplianceSettingStates for deviceManagement",
        name: create_device_compliance_setting_states,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries/{{id}}/deviceComplianceSettingStates",
        body: true,
        params: device_compliance_policy_setting_state_summary_id
    );
    get!(
        doc: "Get deviceComplianceSettingStates from deviceManagement",
        name: list_device_compliance_setting_states,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries/{{id}}/deviceComplianceSettingStates",
        params: device_compliance_policy_setting_state_summary_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_compliance_setting_states_count,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries/{{id}}/deviceComplianceSettingStates/$count",
        params: device_compliance_policy_setting_state_summary_id
    );
    delete!(
        doc: "Delete navigation property deviceComplianceSettingStates for deviceManagement",
        name: delete_device_compliance_setting_states,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries/{{id}}/deviceComplianceSettingStates/{{id2}}",
        params: device_compliance_policy_setting_state_summary_id, device_compliance_setting_state_id
    );
    get!(
        doc: "Get deviceComplianceSettingStates from deviceManagement",
        name: get_device_compliance_setting_states,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries/{{id}}/deviceComplianceSettingStates/{{id2}}",
        params: device_compliance_policy_setting_state_summary_id, device_compliance_setting_state_id
    );
    patch!(
        doc: "Update the navigation property deviceComplianceSettingStates in deviceManagement",
        name: update_device_compliance_setting_states,
        path: "/deviceManagement/deviceCompliancePolicySettingStateSummaries/{{id}}/deviceComplianceSettingStates/{{id2}}",
        body: true,
        params: device_compliance_policy_setting_state_summary_id, device_compliance_setting_state_id
    );
    delete!(
        doc: "Delete navigation property deviceConfigurationDeviceStateSummaries for deviceManagement",
        name: delete_device_configuration_device_state_summaries,
        path: "/deviceManagement/deviceConfigurationDeviceStateSummaries"
    );
    get!(
        doc: "Get deviceConfigurationDeviceStateSummaries from deviceManagement",
        name: get_device_configuration_device_state_summaries,
        path: "/deviceManagement/deviceConfigurationDeviceStateSummaries"
    );
    patch!(
        doc: "Update the navigation property deviceConfigurationDeviceStateSummaries in deviceManagement",
        name: update_device_configuration_device_state_summaries,
        path: "/deviceManagement/deviceConfigurationDeviceStateSummaries",
        body: true
    );
    post!(
        doc: "Create new navigation property to deviceManagementPartners for deviceManagement",
        name: create_device_management_partners,
        path: "/deviceManagement/deviceManagementPartners",
        body: true
    );
    get!(
        doc: "Get deviceManagementPartners from deviceManagement",
        name: list_device_management_partners,
        path: "/deviceManagement/deviceManagementPartners"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_management_partners_count,
        path: "/deviceManagement/deviceManagementPartners/$count"
    );
    delete!(
        doc: "Delete navigation property deviceManagementPartners for deviceManagement",
        name: delete_device_management_partners,
        path: "/deviceManagement/deviceManagementPartners/{{id}}",
        params: device_management_partner_id
    );
    get!(
        doc: "Get deviceManagementPartners from deviceManagement",
        name: get_device_management_partners,
        path: "/deviceManagement/deviceManagementPartners/{{id}}",
        params: device_management_partner_id
    );
    patch!(
        doc: "Update the navigation property deviceManagementPartners in deviceManagement",
        name: update_device_management_partners,
        path: "/deviceManagement/deviceManagementPartners/{{id}}",
        body: true,
        params: device_management_partner_id
    );
    post!(
        doc: "Invoke action terminate",
        name: terminate,
        path: "/deviceManagement/deviceManagementPartners/{{id}}/microsoft.graph.terminate",
        params: device_management_partner_id
    );
    post!(
        doc: "Create new navigation property to exchangeConnectors for deviceManagement",
        name: create_exchange_connectors,
        path: "/deviceManagement/exchangeConnectors",
        body: true
    );
    get!(
        doc: "Get exchangeConnectors from deviceManagement",
        name: list_exchange_connectors,
        path: "/deviceManagement/exchangeConnectors"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_exchange_connectors_count,
        path: "/deviceManagement/exchangeConnectors/$count"
    );
    delete!(
        doc: "Delete navigation property exchangeConnectors for deviceManagement",
        name: delete_exchange_connectors,
        path: "/deviceManagement/exchangeConnectors/{{id}}",
        params: device_management_exchange_connector_id
    );
    get!(
        doc: "Get exchangeConnectors from deviceManagement",
        name: get_exchange_connectors,
        path: "/deviceManagement/exchangeConnectors/{{id}}",
        params: device_management_exchange_connector_id
    );
    patch!(
        doc: "Update the navigation property exchangeConnectors in deviceManagement",
        name: update_exchange_connectors,
        path: "/deviceManagement/exchangeConnectors/{{id}}",
        body: true,
        params: device_management_exchange_connector_id
    );
    post!(
        doc: "Invoke action sync",
        name: sync,
        path: "/deviceManagement/exchangeConnectors/{{id}}/microsoft.graph.sync",
        body: true,
        params: device_management_exchange_connector_id
    );
    post!(
        doc: "Create new navigation property to importedWindowsAutopilotDeviceIdentities for deviceManagement",
        name: create_imported_windows_autopilot_device_identities,
        path: "/deviceManagement/importedWindowsAutopilotDeviceIdentities",
        body: true
    );
    get!(
        doc: "Get importedWindowsAutopilotDeviceIdentities from deviceManagement",
        name: list_imported_windows_autopilot_device_identities,
        path: "/deviceManagement/importedWindowsAutopilotDeviceIdentities"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_imported_windows_autopilot_device_identities_count,
        path: "/deviceManagement/importedWindowsAutopilotDeviceIdentities/$count"
    );
    post!(
        doc: "Invoke action import",
        name: import,
        path: "/deviceManagement/importedWindowsAutopilotDeviceIdentities/microsoft.graph.import",
        body: true
    );
    delete!(
        doc: "Delete navigation property importedWindowsAutopilotDeviceIdentities for deviceManagement",
        name: delete_imported_windows_autopilot_device_identities,
        path: "/deviceManagement/importedWindowsAutopilotDeviceIdentities/{{id}}",
        params: imported_windows_autopilot_device_identity_id
    );
    get!(
        doc: "Get importedWindowsAutopilotDeviceIdentities from deviceManagement",
        name: get_imported_windows_autopilot_device_identities,
        path: "/deviceManagement/importedWindowsAutopilotDeviceIdentities/{{id}}",
        params: imported_windows_autopilot_device_identity_id
    );
    patch!(
        doc: "Update the navigation property importedWindowsAutopilotDeviceIdentities in deviceManagement",
        name: update_imported_windows_autopilot_device_identities,
        path: "/deviceManagement/importedWindowsAutopilotDeviceIdentities/{{id}}",
        body: true,
        params: imported_windows_autopilot_device_identity_id
    );
    post!(
        doc: "Create new navigation property to iosUpdateStatuses for deviceManagement",
        name: create_ios_update_statuses,
        path: "/deviceManagement/iosUpdateStatuses",
        body: true
    );
    get!(
        doc: "Get iosUpdateStatuses from deviceManagement",
        name: list_ios_update_statuses,
        path: "/deviceManagement/iosUpdateStatuses"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_ios_update_statuses_count,
        path: "/deviceManagement/iosUpdateStatuses/$count"
    );
    delete!(
        doc: "Delete navigation property iosUpdateStatuses for deviceManagement",
        name: delete_ios_update_statuses,
        path: "/deviceManagement/iosUpdateStatuses/{{id}}",
        params: ios_update_device_status_id
    );
    get!(
        doc: "Get iosUpdateStatuses from deviceManagement",
        name: get_ios_update_statuses,
        path: "/deviceManagement/iosUpdateStatuses/{{id}}",
        params: ios_update_device_status_id
    );
    patch!(
        doc: "Update the navigation property iosUpdateStatuses in deviceManagement",
        name: update_ios_update_statuses,
        path: "/deviceManagement/iosUpdateStatuses/{{id}}",
        body: true,
        params: ios_update_device_status_id
    );
    get!(
        doc: "Get managedDeviceOverview from deviceManagement",
        name: get_managed_device_overview,
        path: "/deviceManagement/managedDeviceOverview"
    );
    get!(
        doc: "Invoke function getEffectivePermissions",
        name: get_effective_permissions,
        path: "/deviceManagement/microsoft.graph.getEffectivePermissions(scope='{{id}}')",
        params: scope
    );
    get!(
        doc: "Invoke function verifyWindowsEnrollmentAutoDiscovery",
        name: verify_windows_enrollment_auto_discovery,
        path: "/deviceManagement/microsoft.graph.verifyWindowsEnrollmentAutoDiscovery(domainName='{{id}}')",
        params: domain_name
    );
    post!(
        doc: "Create new navigation property to mobileThreatDefenseConnectors for deviceManagement",
        name: create_mobile_threat_defense_connectors,
        path: "/deviceManagement/mobileThreatDefenseConnectors",
        body: true
    );
    get!(
        doc: "Get mobileThreatDefenseConnectors from deviceManagement",
        name: list_mobile_threat_defense_connectors,
        path: "/deviceManagement/mobileThreatDefenseConnectors"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_mobile_threat_defense_connectors_count,
        path: "/deviceManagement/mobileThreatDefenseConnectors/$count"
    );
    delete!(
        doc: "Delete navigation property mobileThreatDefenseConnectors for deviceManagement",
        name: delete_mobile_threat_defense_connectors,
        path: "/deviceManagement/mobileThreatDefenseConnectors/{{id}}",
        params: mobile_threat_defense_connector_id
    );
    get!(
        doc: "Get mobileThreatDefenseConnectors from deviceManagement",
        name: get_mobile_threat_defense_connectors,
        path: "/deviceManagement/mobileThreatDefenseConnectors/{{id}}",
        params: mobile_threat_defense_connector_id
    );
    patch!(
        doc: "Update the navigation property mobileThreatDefenseConnectors in deviceManagement",
        name: update_mobile_threat_defense_connectors,
        path: "/deviceManagement/mobileThreatDefenseConnectors/{{id}}",
        body: true,
        params: mobile_threat_defense_connector_id
    );
    post!(
        doc: "Create new navigation property to notificationMessageTemplates for deviceManagement",
        name: create_notification_message_templates,
        path: "/deviceManagement/notificationMessageTemplates",
        body: true
    );
    get!(
        doc: "Get notificationMessageTemplates from deviceManagement",
        name: list_notification_message_templates,
        path: "/deviceManagement/notificationMessageTemplates"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_notification_message_templates_count,
        path: "/deviceManagement/notificationMessageTemplates/$count"
    );
    delete!(
        doc: "Delete navigation property notificationMessageTemplates for deviceManagement",
        name: delete_notification_message_templates,
        path: "/deviceManagement/notificationMessageTemplates/{{id}}",
        params: notification_message_template_id
    );
    get!(
        doc: "Get notificationMessageTemplates from deviceManagement",
        name: get_notification_message_templates,
        path: "/deviceManagement/notificationMessageTemplates/{{id}}",
        params: notification_message_template_id
    );
    patch!(
        doc: "Update the navigation property notificationMessageTemplates in deviceManagement",
        name: update_notification_message_templates,
        path: "/deviceManagement/notificationMessageTemplates/{{id}}",
        body: true,
        params: notification_message_template_id
    );
    post!(
        doc: "Create new navigation property to localizedNotificationMessages for deviceManagement",
        name: create_localized_notification_messages,
        path: "/deviceManagement/notificationMessageTemplates/{{id}}/localizedNotificationMessages",
        body: true,
        params: notification_message_template_id
    );
    get!(
        doc: "Get localizedNotificationMessages from deviceManagement",
        name: list_localized_notification_messages,
        path: "/deviceManagement/notificationMessageTemplates/{{id}}/localizedNotificationMessages",
        params: notification_message_template_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_localized_notification_messages_count,
        path: "/deviceManagement/notificationMessageTemplates/{{id}}/localizedNotificationMessages/$count",
        params: notification_message_template_id
    );
    delete!(
        doc: "Delete navigation property localizedNotificationMessages for deviceManagement",
        name: delete_localized_notification_messages,
        path: "/deviceManagement/notificationMessageTemplates/{{id}}/localizedNotificationMessages/{{id2}}",
        params: notification_message_template_id, localized_notification_message_id
    );
    get!(
        doc: "Get localizedNotificationMessages from deviceManagement",
        name: get_localized_notification_messages,
        path: "/deviceManagement/notificationMessageTemplates/{{id}}/localizedNotificationMessages/{{id2}}",
        params: notification_message_template_id, localized_notification_message_id
    );
    patch!(
        doc: "Update the navigation property localizedNotificationMessages in deviceManagement",
        name: update_localized_notification_messages,
        path: "/deviceManagement/notificationMessageTemplates/{{id}}/localizedNotificationMessages/{{id2}}",
        body: true,
        params: notification_message_template_id, localized_notification_message_id
    );
    post!(
        doc: "Invoke action sendTestMessage",
        name: send_test_message,
        path: "/deviceManagement/notificationMessageTemplates/{{id}}/microsoft.graph.sendTestMessage",
        params: notification_message_template_id
    );
    post!(
        doc: "Create new navigation property to remoteAssistancePartners for deviceManagement",
        name: create_remote_assistance_partners,
        path: "/deviceManagement/remoteAssistancePartners",
        body: true
    );
    get!(
        doc: "Get remoteAssistancePartners from deviceManagement",
        name: list_remote_assistance_partners,
        path: "/deviceManagement/remoteAssistancePartners"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_remote_assistance_partners_count,
        path: "/deviceManagement/remoteAssistancePartners/$count"
    );
    delete!(
        doc: "Delete navigation property remoteAssistancePartners for deviceManagement",
        name: delete_remote_assistance_partners,
        path: "/deviceManagement/remoteAssistancePartners/{{id}}",
        params: remote_assistance_partner_id
    );
    get!(
        doc: "Get remoteAssistancePartners from deviceManagement",
        name: get_remote_assistance_partners,
        path: "/deviceManagement/remoteAssistancePartners/{{id}}",
        params: remote_assistance_partner_id
    );
    patch!(
        doc: "Update the navigation property remoteAssistancePartners in deviceManagement",
        name: update_remote_assistance_partners,
        path: "/deviceManagement/remoteAssistancePartners/{{id}}",
        body: true,
        params: remote_assistance_partner_id
    );
    post!(
        doc: "Invoke action beginOnboarding",
        name: begin_onboarding,
        path: "/deviceManagement/remoteAssistancePartners/{{id}}/microsoft.graph.beginOnboarding",
        params: remote_assistance_partner_id
    );
    post!(
        doc: "Invoke action disconnect",
        name: disconnect,
        path: "/deviceManagement/remoteAssistancePartners/{{id}}/microsoft.graph.disconnect",
        params: remote_assistance_partner_id
    );
    delete!(
        doc: "Delete navigation property reports for deviceManagement",
        name: delete_reports,
        path: "/deviceManagement/reports"
    );
    get!(
        doc: "Get reports from deviceManagement",
        name: get_reports,
        path: "/deviceManagement/reports"
    );
    patch!(
        doc: "Update the navigation property reports in deviceManagement",
        name: update_reports,
        path: "/deviceManagement/reports",
        body: true
    );
    post!(
        doc: "Create new navigation property to exportJobs for deviceManagement",
        name: create_export_jobs,
        path: "/deviceManagement/reports/exportJobs",
        body: true
    );
    get!(
        doc: "Get exportJobs from deviceManagement",
        name: list_export_jobs,
        path: "/deviceManagement/reports/exportJobs"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_export_jobs_count,
        path: "/deviceManagement/reports/exportJobs/$count"
    );
    delete!(
        doc: "Delete navigation property exportJobs for deviceManagement",
        name: delete_export_jobs,
        path: "/deviceManagement/reports/exportJobs/{{id}}",
        params: device_management_export_job_id
    );
    get!(
        doc: "Get exportJobs from deviceManagement",
        name: get_export_jobs,
        path: "/deviceManagement/reports/exportJobs/{{id}}",
        params: device_management_export_job_id
    );
    patch!(
        doc: "Update the navigation property exportJobs in deviceManagement",
        name: update_export_jobs,
        path: "/deviceManagement/reports/exportJobs/{{id}}",
        body: true,
        params: device_management_export_job_id
    );
    post!(
        doc: "Invoke action getCachedReport",
        name: get_cached_report,
        path: "/deviceManagement/reports/microsoft.graph.getCachedReport",
        body: true
    );
    post!(
        doc: "Invoke action getCompliancePolicyNonComplianceReport",
        name: get_compliance_policy_non_compliance_report,
        path: "/deviceManagement/reports/microsoft.graph.getCompliancePolicyNonComplianceReport",
        body: true
    );
    post!(
        doc: "Invoke action getCompliancePolicyNonComplianceSummaryReport",
        name: get_compliance_policy_non_compliance_summary_report,
        path: "/deviceManagement/reports/microsoft.graph.getCompliancePolicyNonComplianceSummaryReport",
        body: true
    );
    post!(
        doc: "Invoke action getComplianceSettingNonComplianceReport",
        name: get_compliance_setting_non_compliance_report,
        path: "/deviceManagement/reports/microsoft.graph.getComplianceSettingNonComplianceReport",
        body: true
    );
    post!(
        doc: "Invoke action getConfigurationPolicyNonComplianceReport",
        name: get_configuration_policy_non_compliance_report,
        path: "/deviceManagement/reports/microsoft.graph.getConfigurationPolicyNonComplianceReport",
        body: true
    );
    post!(
        doc: "Invoke action getConfigurationPolicyNonComplianceSummaryReport",
        name: get_configuration_policy_non_compliance_summary_report,
        path: "/deviceManagement/reports/microsoft.graph.getConfigurationPolicyNonComplianceSummaryReport",
        body: true
    );
    post!(
        doc: "Invoke action getConfigurationSettingNonComplianceReport",
        name: get_configuration_setting_non_compliance_report,
        path: "/deviceManagement/reports/microsoft.graph.getConfigurationSettingNonComplianceReport",
        body: true
    );
    post!(
        doc: "Invoke action getDeviceManagementIntentPerSettingContributingProfiles",
        name: get_device_management_intent_per_setting_contributing_profiles,
        path: "/deviceManagement/reports/microsoft.graph.getDeviceManagementIntentPerSettingContributingProfiles",
        body: true
    );
    post!(
        doc: "Invoke action getDeviceManagementIntentSettingsReport",
        name: get_device_management_intent_settings_report,
        path: "/deviceManagement/reports/microsoft.graph.getDeviceManagementIntentSettingsReport",
        body: true
    );
    post!(
        doc: "Invoke action getDeviceNonComplianceReport",
        name: get_device_non_compliance_report,
        path: "/deviceManagement/reports/microsoft.graph.getDeviceNonComplianceReport",
        body: true
    );
    post!(
        doc: "Invoke action getDevicesWithoutCompliancePolicyReport",
        name: get_devices_without_compliance_policy_report,
        path: "/deviceManagement/reports/microsoft.graph.getDevicesWithoutCompliancePolicyReport",
        body: true
    );
    post!(
        doc: "Invoke action getHistoricalReport",
        name: get_historical_report,
        path: "/deviceManagement/reports/microsoft.graph.getHistoricalReport",
        body: true
    );
    post!(
        doc: "Invoke action getNoncompliantDevicesAndSettingsReport",
        name: get_noncompliant_devices_and_settings_report,
        path: "/deviceManagement/reports/microsoft.graph.getNoncompliantDevicesAndSettingsReport",
        body: true
    );
    post!(
        doc: "Invoke action getPolicyNonComplianceMetadata",
        name: get_policy_non_compliance_metadata,
        path: "/deviceManagement/reports/microsoft.graph.getPolicyNonComplianceMetadata",
        body: true
    );
    post!(
        doc: "Invoke action getPolicyNonComplianceReport",
        name: get_policy_non_compliance_report,
        path: "/deviceManagement/reports/microsoft.graph.getPolicyNonComplianceReport",
        body: true
    );
    post!(
        doc: "Invoke action getPolicyNonComplianceSummaryReport",
        name: get_policy_non_compliance_summary_report,
        path: "/deviceManagement/reports/microsoft.graph.getPolicyNonComplianceSummaryReport",
        body: true
    );
    post!(
        doc: "Invoke action getReportFilters",
        name: get_report_filters,
        path: "/deviceManagement/reports/microsoft.graph.getReportFilters",
        body: true
    );
    post!(
        doc: "Invoke action getSettingNonComplianceReport",
        name: get_setting_non_compliance_report,
        path: "/deviceManagement/reports/microsoft.graph.getSettingNonComplianceReport",
        body: true
    );
    post!(
        doc: "Create new navigation property to resourceOperations for deviceManagement",
        name: create_resource_operations,
        path: "/deviceManagement/resourceOperations",
        body: true
    );
    get!(
        doc: "Get resourceOperations from deviceManagement",
        name: list_resource_operations,
        path: "/deviceManagement/resourceOperations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_resource_operations_count,
        path: "/deviceManagement/resourceOperations/$count"
    );
    delete!(
        doc: "Delete navigation property resourceOperations for deviceManagement",
        name: delete_resource_operations,
        path: "/deviceManagement/resourceOperations/{{id}}",
        params: resource_operation_id
    );
    get!(
        doc: "Get resourceOperations from deviceManagement",
        name: get_resource_operations,
        path: "/deviceManagement/resourceOperations/{{id}}",
        params: resource_operation_id
    );
    patch!(
        doc: "Update the navigation property resourceOperations in deviceManagement",
        name: update_resource_operations,
        path: "/deviceManagement/resourceOperations/{{id}}",
        body: true,
        params: resource_operation_id
    );
    post!(
        doc: "Create new navigation property to roleAssignments for deviceManagement",
        name: create_role_assignments,
        path: "/deviceManagement/roleAssignments",
        body: true
    );
    get!(
        doc: "Get roleAssignments from deviceManagement",
        name: list_role_assignments,
        path: "/deviceManagement/roleAssignments"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_role_assignments_count,
        path: "/deviceManagement/roleAssignments/$count"
    );
    delete!(
        doc: "Delete navigation property roleAssignments for deviceManagement",
        name: delete_role_assignments,
        path: "/deviceManagement/roleAssignments/{{id}}",
        params: device_and_app_management_role_assignment_id
    );
    get!(
        doc: "Get roleAssignments from deviceManagement",
        name: get_role_assignments,
        path: "/deviceManagement/roleAssignments/{{id}}",
        params: device_and_app_management_role_assignment_id
    );
    patch!(
        doc: "Update the navigation property roleAssignments in deviceManagement",
        name: update_role_assignments,
        path: "/deviceManagement/roleAssignments/{{id}}",
        body: true,
        params: device_and_app_management_role_assignment_id
    );
    get!(
        doc: "Get softwareUpdateStatusSummary from deviceManagement",
        name: get_software_update_status_summary,
        path: "/deviceManagement/softwareUpdateStatusSummary"
    );
    post!(
        doc: "Create new navigation property to telecomExpenseManagementPartners for deviceManagement",
        name: create_telecom_expense_management_partners,
        path: "/deviceManagement/telecomExpenseManagementPartners",
        body: true
    );
    get!(
        doc: "Get telecomExpenseManagementPartners from deviceManagement",
        name: list_telecom_expense_management_partners,
        path: "/deviceManagement/telecomExpenseManagementPartners"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_telecom_expense_management_partners_count,
        path: "/deviceManagement/telecomExpenseManagementPartners/$count"
    );
    delete!(
        doc: "Delete navigation property telecomExpenseManagementPartners for deviceManagement",
        name: delete_telecom_expense_management_partners,
        path: "/deviceManagement/telecomExpenseManagementPartners/{{id}}",
        params: telecom_expense_management_partner_id
    );
    get!(
        doc: "Get telecomExpenseManagementPartners from deviceManagement",
        name: get_telecom_expense_management_partners,
        path: "/deviceManagement/telecomExpenseManagementPartners/{{id}}",
        params: telecom_expense_management_partner_id
    );
    patch!(
        doc: "Update the navigation property telecomExpenseManagementPartners in deviceManagement",
        name: update_telecom_expense_management_partners,
        path: "/deviceManagement/telecomExpenseManagementPartners/{{id}}",
        body: true,
        params: telecom_expense_management_partner_id
    );
    post!(
        doc: "Create new navigation property to troubleshootingEvents for deviceManagement",
        name: create_troubleshooting_events,
        path: "/deviceManagement/troubleshootingEvents",
        body: true
    );
    get!(
        doc: "Get troubleshootingEvents from deviceManagement",
        name: list_troubleshooting_events,
        path: "/deviceManagement/troubleshootingEvents"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_troubleshooting_events_count,
        path: "/deviceManagement/troubleshootingEvents/$count"
    );
    delete!(
        doc: "Delete navigation property troubleshootingEvents for deviceManagement",
        name: delete_troubleshooting_events,
        path: "/deviceManagement/troubleshootingEvents/{{id}}",
        params: device_management_troubleshooting_event_id
    );
    get!(
        doc: "Get troubleshootingEvents from deviceManagement",
        name: get_troubleshooting_events,
        path: "/deviceManagement/troubleshootingEvents/{{id}}",
        params: device_management_troubleshooting_event_id
    );
    patch!(
        doc: "Update the navigation property troubleshootingEvents in deviceManagement",
        name: update_troubleshooting_events,
        path: "/deviceManagement/troubleshootingEvents/{{id}}",
        body: true,
        params: device_management_troubleshooting_event_id
    );
    post!(
        doc: "Create new navigation property to windowsAutopilotDeviceIdentities for deviceManagement",
        name: create_windows_autopilot_device_identities,
        path: "/deviceManagement/windowsAutopilotDeviceIdentities",
        body: true
    );
    get!(
        doc: "Get windowsAutopilotDeviceIdentities from deviceManagement",
        name: list_windows_autopilot_device_identities,
        path: "/deviceManagement/windowsAutopilotDeviceIdentities"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_windows_autopilot_device_identities_count,
        path: "/deviceManagement/windowsAutopilotDeviceIdentities/$count"
    );
    delete!(
        doc: "Delete navigation property windowsAutopilotDeviceIdentities for deviceManagement",
        name: delete_windows_autopilot_device_identities,
        path: "/deviceManagement/windowsAutopilotDeviceIdentities/{{id}}",
        params: windows_autopilot_device_identity_id
    );
    get!(
        doc: "Get windowsAutopilotDeviceIdentities from deviceManagement",
        name: get_windows_autopilot_device_identities,
        path: "/deviceManagement/windowsAutopilotDeviceIdentities/{{id}}",
        params: windows_autopilot_device_identity_id
    );
    patch!(
        doc: "Update the navigation property windowsAutopilotDeviceIdentities in deviceManagement",
        name: update_windows_autopilot_device_identities,
        path: "/deviceManagement/windowsAutopilotDeviceIdentities/{{id}}",
        body: true,
        params: windows_autopilot_device_identity_id
    );
    post!(
        doc: "Invoke action assignUserToDevice",
        name: assign_user_to_device,
        path: "/deviceManagement/windowsAutopilotDeviceIdentities/{{id}}/microsoft.graph.assignUserToDevice",
        body: true,
        params: windows_autopilot_device_identity_id
    );
    post!(
        doc: "Invoke action unassignUserFromDevice",
        name: unassign_user_from_device,
        path: "/deviceManagement/windowsAutopilotDeviceIdentities/{{id}}/microsoft.graph.unassignUserFromDevice",
        params: windows_autopilot_device_identity_id
    );
    post!(
        doc: "Invoke action updateDeviceProperties",
        name: update_device_properties,
        path: "/deviceManagement/windowsAutopilotDeviceIdentities/{{id}}/microsoft.graph.updateDeviceProperties",
        body: true,
        params: windows_autopilot_device_identity_id
    );
    post!(
        doc: "Create new navigation property to windowsInformationProtectionAppLearningSummaries for deviceManagement",
        name: create_windows_information_protection_app_learning_summaries,
        path: "/deviceManagement/windowsInformationProtectionAppLearningSummaries",
        body: true
    );
    get!(
        doc: "Get windowsInformationProtectionAppLearningSummaries from deviceManagement",
        name: list_windows_information_protection_app_learning_summaries,
        path: "/deviceManagement/windowsInformationProtectionAppLearningSummaries"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_windows_information_protection_app_learning_summaries_count,
        path: "/deviceManagement/windowsInformationProtectionAppLearningSummaries/$count"
    );
    delete!(
        doc: "Delete navigation property windowsInformationProtectionAppLearningSummaries for deviceManagement",
        name: delete_windows_information_protection_app_learning_summaries,
        path: "/deviceManagement/windowsInformationProtectionAppLearningSummaries/{{id}}",
        params: windows_information_protection_app_learning_summary_id
    );
    get!(
        doc: "Get windowsInformationProtectionAppLearningSummaries from deviceManagement",
        name: get_windows_information_protection_app_learning_summaries,
        path: "/deviceManagement/windowsInformationProtectionAppLearningSummaries/{{id}}",
        params: windows_information_protection_app_learning_summary_id
    );
    patch!(
        doc: "Update the navigation property windowsInformationProtectionAppLearningSummaries in deviceManagement",
        name: update_windows_information_protection_app_learning_summaries,
        path: "/deviceManagement/windowsInformationProtectionAppLearningSummaries/{{id}}",
        body: true,
        params: windows_information_protection_app_learning_summary_id
    );
    post!(
        doc: "Create new navigation property to windowsInformationProtectionNetworkLearningSummaries for deviceManagement",
        name: create_windows_information_protection_network_learning_summaries,
        path: "/deviceManagement/windowsInformationProtectionNetworkLearningSummaries",
        body: true
    );
    get!(
        doc: "Get windowsInformationProtectionNetworkLearningSummaries from deviceManagement",
        name: list_windows_information_protection_network_learning_summaries,
        path: "/deviceManagement/windowsInformationProtectionNetworkLearningSummaries"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_windows_information_protection_network_learning_summaries_count,
        path: "/deviceManagement/windowsInformationProtectionNetworkLearningSummaries/$count"
    );
    delete!(
        doc: "Delete navigation property windowsInformationProtectionNetworkLearningSummaries for deviceManagement",
        name: delete_windows_information_protection_network_learning_summaries,
        path: "/deviceManagement/windowsInformationProtectionNetworkLearningSummaries/{{id}}",
        params: windows_information_protection_network_learning_summary_id
    );
    get!(
        doc: "Get windowsInformationProtectionNetworkLearningSummaries from deviceManagement",
        name: get_windows_information_protection_network_learning_summaries,
        path: "/deviceManagement/windowsInformationProtectionNetworkLearningSummaries/{{id}}",
        params: windows_information_protection_network_learning_summary_id
    );
    patch!(
        doc: "Update the navigation property windowsInformationProtectionNetworkLearningSummaries in deviceManagement",
        name: update_windows_information_protection_network_learning_summaries,
        path: "/deviceManagement/windowsInformationProtectionNetworkLearningSummaries/{{id}}",
        body: true,
        params: windows_information_protection_network_learning_summary_id
    );
}
