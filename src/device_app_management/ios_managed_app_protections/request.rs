// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    IosManagedAppProtectionsApiClient,
    IosManagedAppProtectionsIdApiClient,
    ResourceIdentity::IosManagedAppProtections
);

impl IosManagedAppProtectionsApiClient {
    post!(
        doc: "Create new navigation property to iosManagedAppProtections for deviceAppManagement",
        name: create_ios_managed_app_protections,
        path: "/iosManagedAppProtections",
        body: true
    );
    get!(
        doc: "Get iosManagedAppProtections from deviceAppManagement",
        name: list_ios_managed_app_protections,
        path: "/iosManagedAppProtections"
    );
    get!(
        doc: "Get the number of the resource",
        name: ios_managed_app_protections_dcdc,
        path: "/iosManagedAppProtections/$count"
    );
}

impl IosManagedAppProtectionsIdApiClient {
    delete!(
        doc: "Delete navigation property iosManagedAppProtections for deviceAppManagement",
        name: delete_ios_managed_app_protections,
        path: "/iosManagedAppProtections/{{RID}}"
    );
    get!(
        doc: "Get iosManagedAppProtections from deviceAppManagement",
        name: get_ios_managed_app_protections,
        path: "/iosManagedAppProtections/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property iosManagedAppProtections in deviceAppManagement",
        name: update_ios_managed_app_protections,
        path: "/iosManagedAppProtections/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to apps for deviceAppManagement",
        name: create_apps,
        path: "/iosManagedAppProtections/{{RID}}/apps",
        body: true
    );
    get!(
        doc: "Get apps from deviceAppManagement",
        name: list_apps,
        path: "/iosManagedAppProtections/{{RID}}/apps"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_apps_count,
        path: "/iosManagedAppProtections/{{RID}}/apps/$count"
    );
    delete!(
        doc: "Delete navigation property apps for deviceAppManagement",
        name: delete_apps,
        path: "/iosManagedAppProtections/{{RID}}/apps/{{id}}",
        params: managed_mobile_app_id
    );
    get!(
        doc: "Get apps from deviceAppManagement",
        name: get_apps,
        path: "/iosManagedAppProtections/{{RID}}/apps/{{id}}",
        params: managed_mobile_app_id
    );
    patch!(
        doc: "Update the navigation property apps in deviceAppManagement",
        name: update_apps,
        path: "/iosManagedAppProtections/{{RID}}/apps/{{id}}",
        body: true,
        params: managed_mobile_app_id
    );
    delete!(
        doc: "Delete navigation property deploymentSummary for deviceAppManagement",
        name: delete_deployment_summary,
        path: "/iosManagedAppProtections/{{RID}}/deploymentSummary"
    );
    get!(
        doc: "Get deploymentSummary from deviceAppManagement",
        name: get_deployment_summary,
        path: "/iosManagedAppProtections/{{RID}}/deploymentSummary"
    );
    patch!(
        doc: "Update the navigation property deploymentSummary in deviceAppManagement",
        name: update_deployment_summary,
        path: "/iosManagedAppProtections/{{RID}}/deploymentSummary",
        body: true
    );
}
