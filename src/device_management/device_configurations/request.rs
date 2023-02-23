// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    DeviceConfigurationsApiClient,
    DeviceConfigurationsIdApiClient,
    ResourceIdentity::DeviceConfigurations
);

impl DeviceConfigurationsApiClient {
    post!(
        doc: "Create new navigation property to deviceConfigurations for deviceManagement",
        name: create_device_configurations,
        path: "/deviceConfigurations",
        body: true
    );
    get!(
        doc: "Get deviceConfigurations from deviceManagement",
        name: list_device_configurations,
        path: "/deviceConfigurations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_configurations_count,
        path: "/deviceConfigurations/$count"
    );
}

impl DeviceConfigurationsIdApiClient {
    delete!(
        doc: "Delete navigation property deviceConfigurations for deviceManagement",
        name: delete_device_configurations,
        path: "/deviceConfigurations/{{RID}}"
    );
    get!(
        doc: "Get deviceConfigurations from deviceManagement",
        name: get_device_configurations,
        path: "/deviceConfigurations/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property deviceConfigurations in deviceManagement",
        name: update_device_configurations,
        path: "/deviceConfigurations/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to assignments for deviceManagement",
        name: create_assignments,
        path: "/deviceConfigurations/{{RID}}/assignments",
        body: true
    );
    get!(
        doc: "Get assignments from deviceManagement",
        name: list_assignments,
        path: "/deviceConfigurations/{{RID}}/assignments"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_assignments_count,
        path: "/deviceConfigurations/{{RID}}/assignments/$count"
    );
    delete!(
        doc: "Delete navigation property assignments for deviceManagement",
        name: delete_assignments,
        path: "/deviceConfigurations/{{RID}}/assignments/{{id}}",
        params: device_configuration_assignment_id
    );
    get!(
        doc: "Get assignments from deviceManagement",
        name: get_assignments,
        path: "/deviceConfigurations/{{RID}}/assignments/{{id}}",
        params: device_configuration_assignment_id
    );
    patch!(
        doc: "Update the navigation property assignments in deviceManagement",
        name: update_assignments,
        path: "/deviceConfigurations/{{RID}}/assignments/{{id}}",
        body: true,
        params: device_configuration_assignment_id
    );
    post!(
        doc: "Create new navigation property to deviceSettingStateSummaries for deviceManagement",
        name: create_device_setting_state_summaries,
        path: "/deviceConfigurations/{{RID}}/deviceSettingStateSummaries",
        body: true
    );
    get!(
        doc: "Get deviceSettingStateSummaries from deviceManagement",
        name: list_device_setting_state_summaries,
        path: "/deviceConfigurations/{{RID}}/deviceSettingStateSummaries"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_setting_state_summaries_count,
        path: "/deviceConfigurations/{{RID}}/deviceSettingStateSummaries/$count"
    );
    delete!(
        doc: "Delete navigation property deviceSettingStateSummaries for deviceManagement",
        name: delete_device_setting_state_summaries,
        path: "/deviceConfigurations/{{RID}}/deviceSettingStateSummaries/{{id}}",
        params: setting_state_device_summary_id
    );
    get!(
        doc: "Get deviceSettingStateSummaries from deviceManagement",
        name: get_device_setting_state_summaries,
        path: "/deviceConfigurations/{{RID}}/deviceSettingStateSummaries/{{id}}",
        params: setting_state_device_summary_id
    );
    patch!(
        doc: "Update the navigation property deviceSettingStateSummaries in deviceManagement",
        name: update_device_setting_state_summaries,
        path: "/deviceConfigurations/{{RID}}/deviceSettingStateSummaries/{{id}}",
        body: true,
        params: setting_state_device_summary_id
    );
    delete!(
        doc: "Delete navigation property deviceStatusOverview for deviceManagement",
        name: delete_device_status_overview,
        path: "/deviceConfigurations/{{RID}}/deviceStatusOverview"
    );
    get!(
        doc: "Get deviceStatusOverview from deviceManagement",
        name: get_device_status_overview,
        path: "/deviceConfigurations/{{RID}}/deviceStatusOverview"
    );
    patch!(
        doc: "Update the navigation property deviceStatusOverview in deviceManagement",
        name: update_device_status_overview,
        path: "/deviceConfigurations/{{RID}}/deviceStatusOverview",
        body: true
    );
    post!(
        doc: "Create new navigation property to deviceStatuses for deviceManagement",
        name: create_device_statuses,
        path: "/deviceConfigurations/{{RID}}/deviceStatuses",
        body: true
    );
    get!(
        doc: "Get deviceStatuses from deviceManagement",
        name: list_device_statuses,
        path: "/deviceConfigurations/{{RID}}/deviceStatuses"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_statuses_count,
        path: "/deviceConfigurations/{{RID}}/deviceStatuses/$count"
    );
    delete!(
        doc: "Delete navigation property deviceStatuses for deviceManagement",
        name: delete_device_statuses,
        path: "/deviceConfigurations/{{RID}}/deviceStatuses/{{id}}",
        params: device_configuration_device_status_id
    );
    get!(
        doc: "Get deviceStatuses from deviceManagement",
        name: get_device_statuses,
        path: "/deviceConfigurations/{{RID}}/deviceStatuses/{{id}}",
        params: device_configuration_device_status_id
    );
    patch!(
        doc: "Update the navigation property deviceStatuses in deviceManagement",
        name: update_device_statuses,
        path: "/deviceConfigurations/{{RID}}/deviceStatuses/{{id}}",
        body: true,
        params: device_configuration_device_status_id
    );
    post!(
        doc: "Invoke action assign",
        name: assign,
        path: "/deviceConfigurations/{{RID}}/microsoft.graph.assign",
        body: true
    );
    get!(
        doc: "Invoke function getOmaSettingPlainTextValue",
        name: get_oma_setting_plain_text_value,
        path: "/deviceConfigurations/{{RID}}/microsoft.graph.getOmaSettingPlainTextValue(secretReferenceValueId='{{id}}')",
        params: secret_reference_value_id
    );
    delete!(
        doc: "Delete navigation property userStatusOverview for deviceManagement",
        name: delete_user_status_overview,
        path: "/deviceConfigurations/{{RID}}/userStatusOverview"
    );
    get!(
        doc: "Get userStatusOverview from deviceManagement",
        name: get_user_status_overview,
        path: "/deviceConfigurations/{{RID}}/userStatusOverview"
    );
    patch!(
        doc: "Update the navigation property userStatusOverview in deviceManagement",
        name: update_user_status_overview,
        path: "/deviceConfigurations/{{RID}}/userStatusOverview",
        body: true
    );
    post!(
        doc: "Create new navigation property to userStatuses for deviceManagement",
        name: create_user_statuses,
        path: "/deviceConfigurations/{{RID}}/userStatuses",
        body: true
    );
    get!(
        doc: "Get userStatuses from deviceManagement",
        name: list_user_statuses,
        path: "/deviceConfigurations/{{RID}}/userStatuses"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_user_statuses_count,
        path: "/deviceConfigurations/{{RID}}/userStatuses/$count"
    );
    delete!(
        doc: "Delete navigation property userStatuses for deviceManagement",
        name: delete_user_statuses,
        path: "/deviceConfigurations/{{RID}}/userStatuses/{{id}}",
        params: device_configuration_user_status_id
    );
    get!(
        doc: "Get userStatuses from deviceManagement",
        name: get_user_statuses,
        path: "/deviceConfigurations/{{RID}}/userStatuses/{{id}}",
        params: device_configuration_user_status_id
    );
    patch!(
        doc: "Update the navigation property userStatuses in deviceManagement",
        name: update_user_statuses,
        path: "/deviceConfigurations/{{RID}}/userStatuses/{{id}}",
        body: true,
        params: device_configuration_user_status_id
    );
}
