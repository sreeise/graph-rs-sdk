// GENERATED CODE

use crate::api_default_imports::*;
use crate::sites::*;

resource_api_client!(
    TermStoresApiClient,
    TermStoresIdApiClient,
    ResourceIdentity::TermStores
);

impl TermStoresApiClient {
    post!(
        doc: "Create new navigation property to termStores for sites",
        name: create_term_stores,
        path: "/termStores",
        body: true
    );
    get!(
        doc: "Get termStores from sites",
        name: list_term_stores,
        path: "/termStores"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_term_stores_count,
        path: "/termStores/$count"
    );
}

impl TermStoresIdApiClient {
    api_client_link!(sets, TermStoreSetsApiClient);
    api_client_link_id!(group, TermStoreGroupsIdApiClient);
    api_client_link_id!(set, TermStoreSetsIdApiClient);
    api_client_link!(groups, TermStoreGroupsApiClient);

    delete!(
        doc: "Delete navigation property termStores for sites",
        name: delete_term_stores,
        path: "/termStores/{{RID}}"
    );
    get!(
        doc: "Get termStores from sites",
        name: get_term_stores,
        path: "/termStores/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property termStores in sites",
        name: update_term_stores,
        path: "/termStores/{{RID}}",
        body: true
    );
}
