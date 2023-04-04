// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    DefaultManagedAppProtectionsApiClient,
    DefaultManagedAppProtectionsIdApiClient,
    ResourceIdentity::DefaultManagedAppProtections
);

impl DefaultManagedAppProtectionsApiClient {
    post!(
        doc: "Create new navigation property to defaultManagedAppProtections for deviceAppManagement",
        name: create_default_managed_app_protections,
        path: "/defaultManagedAppProtections",
        body: true
    );
    get!(
        doc: "Get defaultManagedAppProtections from deviceAppManagement",
        name: list_default_managed_app_protections,
        path: "/defaultManagedAppProtections"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_default_managed_app_protections_count,
        path: "/defaultManagedAppProtections/$count"
    );
}

impl DefaultManagedAppProtectionsIdApiClient {
    delete!(
        doc: "Delete navigation property defaultManagedAppProtections for deviceAppManagement",
        name: delete_default_managed_app_protections,
        path: "/defaultManagedAppProtections/{{RID}}"
    );
    get!(
        doc: "Get defaultManagedAppProtections from deviceAppManagement",
        name: get_default_managed_app_protections,
        path: "/defaultManagedAppProtections/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property defaultManagedAppProtections in deviceAppManagement",
        name: update_default_managed_app_protections,
        path: "/defaultManagedAppProtections/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to apps for deviceAppManagement",
        name: create_apps,
        path: "/defaultManagedAppProtections/{{RID}}/apps",
        body: true
    );
    get!(
        doc: "Get apps from deviceAppManagement",
        name: list_apps,
        path: "/defaultManagedAppProtections/{{RID}}/apps"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_apps_count,
        path: "/defaultManagedAppProtections/{{RID}}/apps/$count"
    );
    delete!(
        doc: "Delete navigation property apps for deviceAppManagement",
        name: delete_apps,
        path: "/defaultManagedAppProtections/{{RID}}/apps/{{id}}",
        params: managed_mobile_app_id
    );
    get!(
        doc: "Get apps from deviceAppManagement",
        name: get_apps,
        path: "/defaultManagedAppProtections/{{RID}}/apps/{{id}}",
        params: managed_mobile_app_id
    );
    patch!(
        doc: "Update the navigation property apps in deviceAppManagement",
        name: update_apps,
        path: "/defaultManagedAppProtections/{{RID}}/apps/{{id}}",
        body: true,
        params: managed_mobile_app_id
    );
    delete!(
        doc: "Delete navigation property deploymentSummary for deviceAppManagement",
        name: delete_deployment_summary,
        path: "/defaultManagedAppProtections/{{RID}}/deploymentSummary"
    );
    get!(
        doc: "Get deploymentSummary from deviceAppManagement",
        name: get_deployment_summary,
        path: "/defaultManagedAppProtections/{{RID}}/deploymentSummary"
    );
    patch!(
        doc: "Update the navigation property deploymentSummary in deviceAppManagement",
        name: update_deployment_summary,
        path: "/defaultManagedAppProtections/{{RID}}/deploymentSummary",
        body: true
    );
}
