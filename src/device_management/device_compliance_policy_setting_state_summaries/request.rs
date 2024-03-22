// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    DeviceCompliancePolicySettingStateSummariesApiClient,
    DeviceCompliancePolicySettingStateSummariesIdApiClient,
    ResourceIdentity::DeviceCompliancePolicySettingStateSummaries
);

impl DeviceCompliancePolicySettingStateSummariesApiClient {
    post!(
        doc: "Create new navigation property to deviceCompliancePolicySettingStateSummaries for deviceManagement",
        name: create_device_compliance_policy_setting_state_summaries,
        path: "/deviceCompliancePolicySettingStateSummaries",
        body: true
    );
    get!(
        doc: "Get deviceCompliancePolicySettingStateSummaries from deviceManagement",
        name: list_device_compliance_policy_setting_state_summaries,
        path: "/deviceCompliancePolicySettingStateSummaries"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_compliance_policy_setting_state_summaries_count,
        path: "/deviceCompliancePolicySettingStateSummaries/$count"
    );
}

impl DeviceCompliancePolicySettingStateSummariesIdApiClient {
    delete!(
        doc: "Delete navigation property deviceCompliancePolicySettingStateSummaries for deviceManagement",
        name: delete_device_compliance_policy_setting_state_summaries,
        path: "/deviceCompliancePolicySettingStateSummaries/{{RID}}"
    );
    get!(
        doc: "Get deviceCompliancePolicySettingStateSummaries from deviceManagement",
        name: get_device_compliance_policy_setting_state_summaries,
        path: "/deviceCompliancePolicySettingStateSummaries/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property deviceCompliancePolicySettingStateSummaries in deviceManagement",
        name: update_device_compliance_policy_setting_state_summaries,
        path: "/deviceCompliancePolicySettingStateSummaries/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to deviceComplianceSettingStates for deviceManagement",
        name: create_device_compliance_setting_states,
        path: "/deviceCompliancePolicySettingStateSummaries/{{RID}}/deviceComplianceSettingStates",
        body: true
    );
    get!(
        doc: "Get deviceComplianceSettingStates from deviceManagement",
        name: list_device_compliance_setting_states,
        path: "/deviceCompliancePolicySettingStateSummaries/{{RID}}/deviceComplianceSettingStates"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_compliance_setting_states_count,
        path: "/deviceCompliancePolicySettingStateSummaries/{{RID}}/deviceComplianceSettingStates/$count"
    );
    delete!(
        doc: "Delete navigation property deviceComplianceSettingStates for deviceManagement",
        name: delete_device_compliance_setting_states,
        path: "/deviceCompliancePolicySettingStateSummaries/{{RID}}/deviceComplianceSettingStates/{{id}}",
        params: device_compliance_setting_state_id
    );
    get!(
        doc: "Get deviceComplianceSettingStates from deviceManagement",
        name: get_device_compliance_setting_states,
        path: "/deviceCompliancePolicySettingStateSummaries/{{RID}}/deviceComplianceSettingStates/{{id}}",
        params: device_compliance_setting_state_id
    );
    patch!(
        doc: "Update the navigation property deviceComplianceSettingStates in deviceManagement",
        name: update_device_compliance_setting_states,
        path: "/deviceCompliancePolicySettingStateSummaries/{{RID}}/deviceComplianceSettingStates/{{id}}",
        body: true,
        params: device_compliance_setting_state_id
    );
}
