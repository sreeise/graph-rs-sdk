// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    TermStoreGroupsApiClient,
    TermStoreGroupsIdApiClient,
    ResourceIdentity::TermStoreGroups
);

impl TermStoreGroupsApiClient {
    post!(
        doc: "Create termStore group",
        name: create_groups,
        path: "/groups",
        body: true
    );
    get!(
        doc: "List termStore groups",
        name: list_groups,
        path: "/groups"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_groups_count,
        path: "/groups/$count"
    );
}

impl TermStoreGroupsIdApiClient {
    delete!(
        doc: "Delete navigation property groups for sites",
        name: delete_groups,
        path: "/groups/{{RID}}"
    );
    get!(
        doc: "Get groups from sites",
        name: get_groups,
        path: "/groups/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property groups in sites",
        name: update_groups,
        path: "/groups/{{RID}}",
        body: true
    );
}
