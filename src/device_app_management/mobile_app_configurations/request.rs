// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    MobileAppConfigurationsApiClient,
    MobileAppConfigurationsIdApiClient,
    ResourceIdentity::MobileAppConfigurations
);

impl MobileAppConfigurationsApiClient {
    post!(
        doc: "Create new navigation property to mobileAppConfigurations for deviceAppManagement",
        name: create_mobile_app_configurations,
        path: "/mobileAppConfigurations",
        body: true
    );
    get!(
        doc: "Get mobileAppConfigurations from deviceAppManagement",
        name: list_mobile_app_configurations,
        path: "/mobileAppConfigurations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_mobile_app_configurations_count,
        path: "/mobileAppConfigurations/$count"
    );
}

impl MobileAppConfigurationsIdApiClient {
    delete!(
        doc: "Delete navigation property mobileAppConfigurations for deviceAppManagement",
        name: delete_mobile_app_configurations,
        path: "/mobileAppConfigurations/{{RID}}"
    );
    get!(
        doc: "Get mobileAppConfigurations from deviceAppManagement",
        name: get_mobile_app_configurations,
        path: "/mobileAppConfigurations/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property mobileAppConfigurations in deviceAppManagement",
        name: update_mobile_app_configurations,
        path: "/mobileAppConfigurations/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action assign",
        name: assign,
        path: "/mobileAppConfigurations/{{RID}}/assign",
        body: true
    );
    post!(
        doc: "Create new navigation property to assignments for deviceAppManagement",
        name: create_assignments,
        path: "/mobileAppConfigurations/{{RID}}/assignments",
        body: true
    );
    get!(
        doc: "Get assignments from deviceAppManagement",
        name: list_assignments,
        path: "/mobileAppConfigurations/{{RID}}/assignments"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_assignments_count,
        path: "/mobileAppConfigurations/{{RID}}/assignments/$count"
    );
    delete!(
        doc: "Delete navigation property assignments for deviceAppManagement",
        name: delete_assignments,
        path: "/mobileAppConfigurations/{{RID}}/assignments/{{id}}",
        params: managed_device_mobile_app_configuration_assignment_id
    );
    get!(
        doc: "Get assignments from deviceAppManagement",
        name: get_assignments,
        path: "/mobileAppConfigurations/{{RID}}/assignments/{{id}}",
        params: managed_device_mobile_app_configuration_assignment_id
    );
    patch!(
        doc: "Update the navigation property assignments in deviceAppManagement",
        name: update_assignments,
        path: "/mobileAppConfigurations/{{RID}}/assignments/{{id}}",
        body: true,
        params: managed_device_mobile_app_configuration_assignment_id
    );
    delete!(
        doc: "Delete navigation property deviceStatusSummary for deviceAppManagement",
        name: delete_device_status_summary,
        path: "/mobileAppConfigurations/{{RID}}/deviceStatusSummary"
    );
    get!(
        doc: "Get deviceStatusSummary from deviceAppManagement",
        name: get_device_status_summary,
        path: "/mobileAppConfigurations/{{RID}}/deviceStatusSummary"
    );
    patch!(
        doc: "Update the navigation property deviceStatusSummary in deviceAppManagement",
        name: update_device_status_summary,
        path: "/mobileAppConfigurations/{{RID}}/deviceStatusSummary",
        body: true
    );
    post!(
        doc: "Create new navigation property to deviceStatuses for deviceAppManagement",
        name: create_device_statuses,
        path: "/mobileAppConfigurations/{{RID}}/deviceStatuses",
        body: true
    );
    get!(
        doc: "Get deviceStatuses from deviceAppManagement",
        name: list_device_statuses,
        path: "/mobileAppConfigurations/{{RID}}/deviceStatuses"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_statuses_count,
        path: "/mobileAppConfigurations/{{RID}}/deviceStatuses/$count"
    );
    delete!(
        doc: "Delete navigation property deviceStatuses for deviceAppManagement",
        name: delete_device_statuses,
        path: "/mobileAppConfigurations/{{RID}}/deviceStatuses/{{id}}",
        params: managed_device_mobile_app_configuration_device_status_id
    );
    get!(
        doc: "Get deviceStatuses from deviceAppManagement",
        name: get_device_statuses,
        path: "/mobileAppConfigurations/{{RID}}/deviceStatuses/{{id}}",
        params: managed_device_mobile_app_configuration_device_status_id
    );
    patch!(
        doc: "Update the navigation property deviceStatuses in deviceAppManagement",
        name: update_device_statuses,
        path: "/mobileAppConfigurations/{{RID}}/deviceStatuses/{{id}}",
        body: true,
        params: managed_device_mobile_app_configuration_device_status_id
    );
    delete!(
        doc: "Delete navigation property userStatusSummary for deviceAppManagement",
        name: delete_user_status_summary,
        path: "/mobileAppConfigurations/{{RID}}/userStatusSummary"
    );
    get!(
        doc: "Get userStatusSummary from deviceAppManagement",
        name: get_user_status_summary,
        path: "/mobileAppConfigurations/{{RID}}/userStatusSummary"
    );
    patch!(
        doc: "Update the navigation property userStatusSummary in deviceAppManagement",
        name: update_user_status_summary,
        path: "/mobileAppConfigurations/{{RID}}/userStatusSummary",
        body: true
    );
    post!(
        doc: "Create new navigation property to userStatuses for deviceAppManagement",
        name: create_user_statuses,
        path: "/mobileAppConfigurations/{{RID}}/userStatuses",
        body: true
    );
    get!(
        doc: "Get userStatuses from deviceAppManagement",
        name: list_user_statuses,
        path: "/mobileAppConfigurations/{{RID}}/userStatuses"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_user_statuses_count,
        path: "/mobileAppConfigurations/{{RID}}/userStatuses/$count"
    );
    delete!(
        doc: "Delete navigation property userStatuses for deviceAppManagement",
        name: delete_user_statuses,
        path: "/mobileAppConfigurations/{{RID}}/userStatuses/{{id}}",
        params: managed_device_mobile_app_configuration_user_status_id
    );
    get!(
        doc: "Get userStatuses from deviceAppManagement",
        name: get_user_statuses,
        path: "/mobileAppConfigurations/{{RID}}/userStatuses/{{id}}",
        params: managed_device_mobile_app_configuration_user_status_id
    );
    patch!(
        doc: "Update the navigation property userStatuses in deviceAppManagement",
        name: update_user_statuses,
        path: "/mobileAppConfigurations/{{RID}}/userStatuses/{{id}}",
        body: true,
        params: managed_device_mobile_app_configuration_user_status_id
    );
}
