// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    MobileAppsApiClient,
    MobileAppsIdApiClient,
    ResourceIdentity::MobileApps
);

impl MobileAppsApiClient {
    post!(
        doc: "Create new navigation property to mobileApps for deviceAppManagement",
        name: create_mobile_apps,
        path: "/mobileApps",
        body: true
    );
    get!(
        doc: "Get mobileApps from deviceAppManagement",
        name: list_mobile_apps,
        path: "/mobileApps"
    );
    get!(
        doc: "Get the number of the resource",
        name: mobile_apps_dcef,
        path: "/mobileApps/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.managedMobileLobApp in the microsoft.graph.mobileApp collection",
        name: get_mobile_app_items_as_managed_mobile_lob_app_type,
        path: "/mobileApps/graph.managedMobileLobApp"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_managed_mobile_lob_app_count,
        path: "/mobileApps/graph.managedMobileLobApp/$count"
    );
    get!(
        doc: "Get the items of type microsoft.graph.mobileLobApp in the microsoft.graph.mobileApp collection",
        name: graph,
        path: "/mobileApps/graph.mobileLobApp"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_mobile_lob_app_count,
        path: "/mobileApps/graph.mobileLobApp/$count"
    );
}

impl MobileAppsIdApiClient {
    delete!(
        doc: "Delete navigation property mobileApps for deviceAppManagement",
        name: delete_mobile_apps,
        path: "/mobileApps/{{RID}}"
    );
    get!(
        doc: "Get mobileApps from deviceAppManagement",
        name: get_mobile_apps,
        path: "/mobileApps/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property mobileApps in deviceAppManagement",
        name: update_mobile_apps,
        path: "/mobileApps/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action assign",
        name: assign,
        path: "/mobileApps/{{RID}}/assign",
        body: true
    );
    post!(
        doc: "Create new navigation property to assignments for deviceAppManagement",
        name: create_assignments,
        path: "/mobileApps/{{RID}}/assignments",
        body: true
    );
    get!(
        doc: "Get assignments from deviceAppManagement",
        name: list_assignments,
        path: "/mobileApps/{{RID}}/assignments"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_assignments_count,
        path: "/mobileApps/{{RID}}/assignments/$count"
    );
    delete!(
        doc: "Delete navigation property assignments for deviceAppManagement",
        name: delete_assignments,
        path: "/mobileApps/{{RID}}/assignments/{{id}}",
        params: mobile_app_assignment_id
    );
    get!(
        doc: "Get assignments from deviceAppManagement",
        name: get_assignments,
        path: "/mobileApps/{{RID}}/assignments/{{id}}",
        params: mobile_app_assignment_id
    );
    patch!(
        doc: "Update the navigation property assignments in deviceAppManagement",
        name: update_assignments,
        path: "/mobileApps/{{RID}}/assignments/{{id}}",
        body: true,
        params: mobile_app_assignment_id
    );
    get!(
        doc: "Get categories from deviceAppManagement",
        name: list_categories,
        path: "/mobileApps/{{RID}}/categories"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_categories_count,
        path: "/mobileApps/{{RID}}/categories/$count"
    );
    get!(
        doc: "Get categories from deviceAppManagement",
        name: get_categories,
        path: "/mobileApps/{{RID}}/categories/{{id}}",
        params: mobile_app_category_id
    );
    get!(
        doc: "Get the item of type microsoft.graph.mobileApp as microsoft.graph.managedMobileLobApp",
        name: get_mobile_app_item_as_managed_mobile_lob_app_type,
        path: "/mobileApps/{{RID}}/graph.managedMobileLobApp"
    );
    get!(
        doc: "Get the item of type microsoft.graph.mobileApp as microsoft.graph.mobileLobApp",
        name: get_mobile_app_item_as_mobile_lob_app_type,
        path: "/mobileApps/{{RID}}/graph.mobileLobApp"
    );
}
