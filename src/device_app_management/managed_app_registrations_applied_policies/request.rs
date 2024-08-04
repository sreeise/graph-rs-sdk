// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    ManagedAppRegistrationsAppliedPoliciesApiClient,
    ManagedAppRegistrationsAppliedPoliciesIdApiClient,
    ResourceIdentity::ManagedAppRegistrationsAppliedPolicies
);

impl ManagedAppRegistrationsAppliedPoliciesApiClient {
    post!(
        doc: "Create new navigation property to appliedPolicies for deviceAppManagement",
        name: create_applied_policies,
        path: "/appliedPolicies",
        body: true
    );
    get!(
        doc: "Get appliedPolicies from deviceAppManagement",
        name: list_applied_policies,
        path: "/appliedPolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_applied_policies_count,
        path: "/appliedPolicies/$count"
    );
}

impl ManagedAppRegistrationsAppliedPoliciesIdApiClient {
    delete!(
        doc: "Delete navigation property appliedPolicies for deviceAppManagement",
        name: delete_applied_policies,
        path: "/appliedPolicies/{{RID}}"
    );
    get!(
        doc: "Get appliedPolicies from deviceAppManagement",
        name: get_applied_policies,
        path: "/appliedPolicies/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property appliedPolicies in deviceAppManagement",
        name: update_applied_policies,
        path: "/appliedPolicies/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action targetApps",
        name: target_apps,
        path: "/appliedPolicies/{{RID}}/targetApps",
        body: true
    );
}
