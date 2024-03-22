// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    TargetedManagedAppConfigurationsApiClient,
    TargetedManagedAppConfigurationsIdApiClient,
    ResourceIdentity::TargetedManagedAppConfigurations
);

impl TargetedManagedAppConfigurationsApiClient {
    post!(
        doc: "Create new navigation property to targetedManagedAppConfigurations for deviceAppManagement",
        name: create_targeted_managed_app_configurations,
        path: "/targetedManagedAppConfigurations",
        body: true
    );
    get!(
        doc: "Get targetedManagedAppConfigurations from deviceAppManagement",
        name: list_targeted_managed_app_configurations,
        path: "/targetedManagedAppConfigurations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_targeted_managed_app_configurations_count,
        path: "/targetedManagedAppConfigurations/$count"
    );
}

impl TargetedManagedAppConfigurationsIdApiClient {
    delete!(
        doc: "Delete navigation property targetedManagedAppConfigurations for deviceAppManagement",
        name: delete_targeted_managed_app_configurations,
        path: "/targetedManagedAppConfigurations/{{RID}}"
    );
    get!(
        doc: "Get targetedManagedAppConfigurations from deviceAppManagement",
        name: get_targeted_managed_app_configurations,
        path: "/targetedManagedAppConfigurations/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property targetedManagedAppConfigurations in deviceAppManagement",
        name: update_targeted_managed_app_configurations,
        path: "/targetedManagedAppConfigurations/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to apps for deviceAppManagement",
        name: create_apps,
        path: "/targetedManagedAppConfigurations/{{RID}}/apps",
        body: true
    );
    get!(
        doc: "Get apps from deviceAppManagement",
        name: list_apps,
        path: "/targetedManagedAppConfigurations/{{RID}}/apps"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_apps_count,
        path: "/targetedManagedAppConfigurations/{{RID}}/apps/$count"
    );
    delete!(
        doc: "Delete navigation property apps for deviceAppManagement",
        name: delete_apps,
        path: "/targetedManagedAppConfigurations/{{RID}}/apps/{{id}}",
        params: managed_mobile_app_id
    );
    get!(
        doc: "Get apps from deviceAppManagement",
        name: get_apps,
        path: "/targetedManagedAppConfigurations/{{RID}}/apps/{{id}}",
        params: managed_mobile_app_id
    );
    patch!(
        doc: "Update the navigation property apps in deviceAppManagement",
        name: update_apps,
        path: "/targetedManagedAppConfigurations/{{RID}}/apps/{{id}}",
        body: true,
        params: managed_mobile_app_id
    );
    post!(
        doc: "Invoke action assign",
        name: assign,
        path: "/targetedManagedAppConfigurations/{{RID}}/assign",
        body: true
    );
    post!(
        doc: "Create new navigation property to assignments for deviceAppManagement",
        name: create_assignments,
        path: "/targetedManagedAppConfigurations/{{RID}}/assignments",
        body: true
    );
    get!(
        doc: "Get assignments from deviceAppManagement",
        name: list_assignments,
        path: "/targetedManagedAppConfigurations/{{RID}}/assignments"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_assignments_count,
        path: "/targetedManagedAppConfigurations/{{RID}}/assignments/$count"
    );
    delete!(
        doc: "Delete navigation property assignments for deviceAppManagement",
        name: delete_assignments,
        path: "/targetedManagedAppConfigurations/{{RID}}/assignments/{{id}}",
        params: targeted_managed_app_policy_assignment_id
    );
    get!(
        doc: "Get assignments from deviceAppManagement",
        name: get_assignments,
        path: "/targetedManagedAppConfigurations/{{RID}}/assignments/{{id}}",
        params: targeted_managed_app_policy_assignment_id
    );
    patch!(
        doc: "Update the navigation property assignments in deviceAppManagement",
        name: update_assignments,
        path: "/targetedManagedAppConfigurations/{{RID}}/assignments/{{id}}",
        body: true,
        params: targeted_managed_app_policy_assignment_id
    );
    delete!(
        doc: "Delete navigation property deploymentSummary for deviceAppManagement",
        name: delete_deployment_summary,
        path: "/targetedManagedAppConfigurations/{{RID}}/deploymentSummary"
    );
    get!(
        doc: "Get deploymentSummary from deviceAppManagement",
        name: get_deployment_summary,
        path: "/targetedManagedAppConfigurations/{{RID}}/deploymentSummary"
    );
    patch!(
        doc: "Update the navigation property deploymentSummary in deviceAppManagement",
        name: update_deployment_summary,
        path: "/targetedManagedAppConfigurations/{{RID}}/deploymentSummary",
        body: true
    );
    post!(
        doc: "Invoke action targetApps",
        name: target_apps,
        path: "/targetedManagedAppConfigurations/{{RID}}/targetApps",
        body: true
    );
}
