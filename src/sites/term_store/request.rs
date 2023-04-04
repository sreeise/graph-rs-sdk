// GENERATED CODE

use crate::api_default_imports::*;
use crate::sites::*;

resource_api_client!(TermStoreApiClient, ResourceIdentity::TermStore);

impl TermStoreApiClient {
    api_client_link_id!(set, TermStoreSetsIdApiClient);
    api_client_link!(groups, TermStoreGroupsApiClient);
    api_client_link_id!(group, TermStoreGroupsIdApiClient);
    api_client_link!(sets, TermStoreSetsApiClient);

    delete!(
        doc: "Delete navigation property termStore for sites",
        name: delete_term_store,
        path: "/termStore"
    );
    get!(
        doc: "Get store",
        name: get_term_store,
        path: "/termStore"
    );
    patch!(
        doc: "Update store",
        name: update_term_store,
        path: "/termStore",
        body: true
    );
}
