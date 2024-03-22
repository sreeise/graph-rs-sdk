// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    DeviceManagementReportsApiClient,
    ResourceIdentity::DeviceManagementReports
);

impl DeviceManagementReportsApiClient {
    delete!(
        doc: "Delete navigation property reports for deviceManagement",
        name: delete_reports,
        path: "/reports"
    );
    get!(
        doc: "Get reports from deviceManagement",
        name: get_reports,
        path: "/reports"
    );
    patch!(
        doc: "Update the navigation property reports in deviceManagement",
        name: update_reports,
        path: "/reports",
        body: true
    );
    post!(
        doc: "Create new navigation property to exportJobs for deviceManagement",
        name: create_export_jobs,
        path: "/reports/exportJobs",
        body: true
    );
    get!(
        doc: "Get exportJobs from deviceManagement",
        name: list_export_jobs,
        path: "/reports/exportJobs"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_export_jobs_count,
        path: "/reports/exportJobs/$count"
    );
    delete!(
        doc: "Delete navigation property exportJobs for deviceManagement",
        name: delete_export_jobs,
        path: "/reports/exportJobs/{{id}}",
        params: device_management_export_job_id
    );
    get!(
        doc: "Get exportJobs from deviceManagement",
        name: get_export_jobs,
        path: "/reports/exportJobs/{{id}}",
        params: device_management_export_job_id
    );
    patch!(
        doc: "Update the navigation property exportJobs in deviceManagement",
        name: update_export_jobs,
        path: "/reports/exportJobs/{{id}}",
        body: true,
        params: device_management_export_job_id
    );
    post!(
        doc: "Invoke action getCachedReport",
        name: get_cached_report,
        path: "/reports/getCachedReport",
        body: true
    );
    post!(
        doc: "Invoke action getCompliancePolicyNonComplianceReport",
        name: get_compliance_policy_non_compliance_report,
        path: "/reports/getCompliancePolicyNonComplianceReport",
        body: true
    );
    post!(
        doc: "Invoke action getCompliancePolicyNonComplianceSummaryReport",
        name: get_compliance_policy_non_compliance_summary_report,
        path: "/reports/getCompliancePolicyNonComplianceSummaryReport",
        body: true
    );
    post!(
        doc: "Invoke action getComplianceSettingNonComplianceReport",
        name: get_compliance_setting_non_compliance_report,
        path: "/reports/getComplianceSettingNonComplianceReport",
        body: true
    );
    post!(
        doc: "Invoke action getConfigurationPolicyNonComplianceReport",
        name: get_configuration_policy_non_compliance_report,
        path: "/reports/getConfigurationPolicyNonComplianceReport",
        body: true
    );
    post!(
        doc: "Invoke action getConfigurationPolicyNonComplianceSummaryReport",
        name: get_configuration_policy_non_compliance_summary_report,
        path: "/reports/getConfigurationPolicyNonComplianceSummaryReport",
        body: true
    );
    post!(
        doc: "Invoke action getConfigurationSettingNonComplianceReport",
        name: get_configuration_setting_non_compliance_report,
        path: "/reports/getConfigurationSettingNonComplianceReport",
        body: true
    );
    post!(
        doc: "Invoke action getDeviceManagementIntentPerSettingContributingProfiles",
        name: get_device_management_intent_per_setting_contributing_profiles,
        path: "/reports/getDeviceManagementIntentPerSettingContributingProfiles",
        body: true
    );
    post!(
        doc: "Invoke action getDeviceManagementIntentSettingsReport",
        name: get_device_management_intent_settings_report,
        path: "/reports/getDeviceManagementIntentSettingsReport",
        body: true
    );
    post!(
        doc: "Invoke action getDeviceNonComplianceReport",
        name: get_device_non_compliance_report,
        path: "/reports/getDeviceNonComplianceReport",
        body: true
    );
    post!(
        doc: "Invoke action getDevicesWithoutCompliancePolicyReport",
        name: get_devices_without_compliance_policy_report,
        path: "/reports/getDevicesWithoutCompliancePolicyReport",
        body: true
    );
    post!(
        doc: "Invoke action getHistoricalReport",
        name: get_historical_report,
        path: "/reports/getHistoricalReport",
        body: true
    );
    post!(
        doc: "Invoke action getNoncompliantDevicesAndSettingsReport",
        name: get_noncompliant_devices_and_settings_report,
        path: "/reports/getNoncompliantDevicesAndSettingsReport",
        body: true
    );
    post!(
        doc: "Invoke action getPolicyNonComplianceMetadata",
        name: get_policy_non_compliance_metadata,
        path: "/reports/getPolicyNonComplianceMetadata",
        body: true
    );
    post!(
        doc: "Invoke action getPolicyNonComplianceReport",
        name: get_policy_non_compliance_report,
        path: "/reports/getPolicyNonComplianceReport",
        body: true
    );
    post!(
        doc: "Invoke action getPolicyNonComplianceSummaryReport",
        name: get_policy_non_compliance_summary_report,
        path: "/reports/getPolicyNonComplianceSummaryReport",
        body: true
    );
    post!(
        doc: "Invoke action getReportFilters",
        name: get_report_filters,
        path: "/reports/getReportFilters",
        body: true
    );
    post!(
        doc: "Invoke action getSettingNonComplianceReport",
        name: get_setting_non_compliance_report,
        path: "/reports/getSettingNonComplianceReport",
        body: true
    );
}
