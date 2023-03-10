// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    GroupLifecyclePoliciesApiClient,
    GroupLifecyclePoliciesIdApiClient,
    ResourceIdentity::GroupLifecyclePolicies
);

impl GroupLifecyclePoliciesApiClient {
    post!(
        doc: "Create groupLifecyclePolicy",
        name: create_group_lifecycle_policy,
        path: "/groupLifecyclePolicies",
        body: true
    );
    get!(
        doc: "List groupLifecyclePolicies",
        name: list_group_lifecycle_policy,
        path: "/groupLifecyclePolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_group_lifecycle_policies_count,
        path: "/groupLifecyclePolicies/$count"
    );
}

impl GroupLifecyclePoliciesIdApiClient {
    delete!(
        doc: "Delete groupLifecyclePolicy",
        name: delete_group_lifecycle_policy,
        path: "/groupLifecyclePolicies/{{RID}}"
    );
    get!(
        doc: "Get groupLifecyclePolicy",
        name: get_group_lifecycle_policy,
        path: "/groupLifecyclePolicies/{{RID}}"
    );
    patch!(
        doc: "Update groupLifecyclePolicy",
        name: update_group_lifecycle_policy,
        path: "/groupLifecyclePolicies/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action addGroup",
        name: add_group,
        path: "/groupLifecyclePolicies/{{RID}}/addGroup",
        body: true
    );
    post!(
        doc: "Invoke action removeGroup",
        name: remove_group,
        path: "/groupLifecyclePolicies/{{RID}}/removeGroup",
        body: true
    );
}
