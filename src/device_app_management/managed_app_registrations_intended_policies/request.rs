// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    ManagedAppRegistrationsIntendedPoliciesApiClient,
    ManagedAppRegistrationsIntendedPoliciesIdApiClient,
    ResourceIdentity::ManagedAppRegistrationsIntendedPolicies
);

impl ManagedAppRegistrationsIntendedPoliciesApiClient {
    post!(
        doc: "Create new navigation property to intendedPolicies for deviceAppManagement",
        name: create_intended_policies,
        path: "/intendedPolicies",
        body: true
    );
    get!(
        doc: "Get intendedPolicies from deviceAppManagement",
        name: list_intended_policies,
        path: "/intendedPolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_intended_policies_count,
        path: "/intendedPolicies/$count"
    );
}

impl ManagedAppRegistrationsIntendedPoliciesIdApiClient {
    delete!(
        doc: "Delete navigation property intendedPolicies for deviceAppManagement",
        name: delete_intended_policies,
        path: "/intendedPolicies/{{RID}}"
    );
    get!(
        doc: "Get intendedPolicies from deviceAppManagement",
        name: get_intended_policies,
        path: "/intendedPolicies/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property intendedPolicies in deviceAppManagement",
        name: update_intended_policies,
        path: "/intendedPolicies/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action targetApps",
        name: target_apps,
        path: "/intendedPolicies/{{RID}}/targetApps",
        body: true
    );
}
