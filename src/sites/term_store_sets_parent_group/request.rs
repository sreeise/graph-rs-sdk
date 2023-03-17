// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    TermStoreSetsParentGroupApiClient,
    ResourceIdentity::TermStoreSetsParentGroup
);

impl TermStoreSetsParentGroupApiClient {
    delete!(
        doc: "Delete navigation property parentGroup for sites",
        name: delete_parent_group,
        path: "/parentGroup"
    );
    get!(
        doc: "Get parentGroup from sites",
        name: get_parent_group,
        path: "/parentGroup"
    );
    patch!(
        doc: "Update the navigation property parentGroup in sites",
        name: update_parent_group,
        path: "/parentGroup",
        body: true
    );
    post!(
        doc: "Create new navigation property to sets for sites",
        name: create_sets,
        path: "/parentGroup/sets",
        body: true
    );
    get!(
        doc: "List sets",
        name: list_sets,
        path: "/parentGroup/sets"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_sets_count,
        path: "/parentGroup/sets/$count"
    );
    delete!(
        doc: "Delete navigation property sets for sites",
        name: delete_sets,
        path: "/parentGroup/sets/{{id}}",
        params: set_id_1
    );
    get!(
        doc: "Get sets from sites",
        name: get_sets,
        path: "/parentGroup/sets/{{id}}",
        params: set_id_1
    );
    patch!(
        doc: "Update the navigation property sets in sites",
        name: update_sets,
        path: "/parentGroup/sets/{{id}}",
        body: true,
        params: set_id_1
    );
}
