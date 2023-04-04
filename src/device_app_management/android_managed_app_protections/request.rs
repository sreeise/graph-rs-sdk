// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    AndroidManagedAppProtectionsApiClient,
    AndroidManagedAppProtectionsIdApiClient,
    ResourceIdentity::AndroidManagedAppProtections
);

impl AndroidManagedAppProtectionsApiClient {
    post!(
        doc: "Create new navigation property to androidManagedAppProtections for deviceAppManagement",
        name: create_android_managed_app_protections,
        path: "/androidManagedAppProtections",
        body: true
    );
    get!(
        doc: "Get androidManagedAppProtections from deviceAppManagement",
        name: list_android_managed_app_protections,
        path: "/androidManagedAppProtections"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_android_managed_app_protections_count,
        path: "/androidManagedAppProtections/$count"
    );
}

impl AndroidManagedAppProtectionsIdApiClient {
    delete!(
        doc: "Delete navigation property androidManagedAppProtections for deviceAppManagement",
        name: delete_android_managed_app_protections,
        path: "/androidManagedAppProtections/{{RID}}"
    );
    get!(
        doc: "Get androidManagedAppProtections from deviceAppManagement",
        name: get_android_managed_app_protections,
        path: "/androidManagedAppProtections/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property androidManagedAppProtections in deviceAppManagement",
        name: update_android_managed_app_protections,
        path: "/androidManagedAppProtections/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to apps for deviceAppManagement",
        name: create_apps,
        path: "/androidManagedAppProtections/{{RID}}/apps",
        body: true
    );
    get!(
        doc: "Get apps from deviceAppManagement",
        name: list_apps,
        path: "/androidManagedAppProtections/{{RID}}/apps"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_apps_count,
        path: "/androidManagedAppProtections/{{RID}}/apps/$count"
    );
    delete!(
        doc: "Delete navigation property apps for deviceAppManagement",
        name: delete_apps,
        path: "/androidManagedAppProtections/{{RID}}/apps/{{id}}",
        params: managed_mobile_app_id
    );
    get!(
        doc: "Get apps from deviceAppManagement",
        name: get_apps,
        path: "/androidManagedAppProtections/{{RID}}/apps/{{id}}",
        params: managed_mobile_app_id
    );
    patch!(
        doc: "Update the navigation property apps in deviceAppManagement",
        name: update_apps,
        path: "/androidManagedAppProtections/{{RID}}/apps/{{id}}",
        body: true,
        params: managed_mobile_app_id
    );
    delete!(
        doc: "Delete navigation property deploymentSummary for deviceAppManagement",
        name: delete_deployment_summary,
        path: "/androidManagedAppProtections/{{RID}}/deploymentSummary"
    );
    get!(
        doc: "Get deploymentSummary from deviceAppManagement",
        name: get_deployment_summary,
        path: "/androidManagedAppProtections/{{RID}}/deploymentSummary"
    );
    patch!(
        doc: "Update the navigation property deploymentSummary in deviceAppManagement",
        name: update_deployment_summary,
        path: "/androidManagedAppProtections/{{RID}}/deploymentSummary",
        body: true
    );
}
