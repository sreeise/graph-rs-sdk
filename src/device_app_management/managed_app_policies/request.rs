// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    ManagedAppPoliciesApiClient,
    ManagedAppPoliciesIdApiClient,
    ResourceIdentity::ManagedAppPolicies
);

impl ManagedAppPoliciesApiClient {
    post!(
        doc: "Create new navigation property to managedAppPolicies for deviceAppManagement",
        name: create_managed_app_policies,
        path: "/managedAppPolicies",
        body: true
    );
    get!(
        doc: "Get managedAppPolicies from deviceAppManagement",
        name: list_managed_app_policies,
        path: "/managedAppPolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_managed_app_policies_count,
        path: "/managedAppPolicies/$count"
    );
}

impl ManagedAppPoliciesIdApiClient {
    delete!(
        doc: "Delete navigation property managedAppPolicies for deviceAppManagement",
        name: delete_managed_app_policies,
        path: "/managedAppPolicies/{{RID}}"
    );
    get!(
        doc: "Get managedAppPolicies from deviceAppManagement",
        name: get_managed_app_policies,
        path: "/managedAppPolicies/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property managedAppPolicies in deviceAppManagement",
        name: update_managed_app_policies,
        path: "/managedAppPolicies/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action targetApps",
        name: target_apps,
        path: "/managedAppPolicies/{{RID}}/targetApps",
        body: true
    );
}
