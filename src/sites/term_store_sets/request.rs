// GENERATED CODE

use crate::api_default_imports::*;
use crate::sites::*;

resource_api_client!(
    TermStoreSetsApiClient,
    TermStoreSetsIdApiClient,
    ResourceIdentity::TermStoreSets
);

impl TermStoreSetsApiClient {
    post!(
        doc: "Create new navigation property to sets for sites",
        name: create_sets,
        path: "/sets",
        body: true
    );
    get!(
        doc: "Get sets from sites",
        name: list_sets,
        path: "/sets"
    );
    get!(
        doc: "Get the number of the resource",
        name: sets_dbcc,
        path: "/sets/$count"
    );
}

impl TermStoreSetsIdApiClient {
    api_client_link!(terms, TermStoreSetsTermsApiClient);
    api_client_link_id!(term, TermStoreSetsTermsIdApiClient);
    api_client_link!(parent_group, TermStoreSetsParentGroupApiClient);
    api_client_link!(children, TermStoreSetsChildrenApiClient);
    api_client_link_id!(children_id, TermStoreSetsChildrenIdApiClient);

    delete!(
        doc: "Delete navigation property sets for sites",
        name: delete_sets,
        path: "/sets/{{RID}}"
    );
    get!(
        doc: "Get sets from sites",
        name: get_sets,
        path: "/sets/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property sets in sites",
        name: update_sets,
        path: "/sets/{{RID}}",
        body: true
    );
}
