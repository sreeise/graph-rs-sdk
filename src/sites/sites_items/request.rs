// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    SitesItemsApiClient,
    SitesItemsIdApiClient,
    ResourceIdentity::SitesItems
);

impl SitesItemsApiClient {
    get!(
        doc: "Get items from sites",
        name: list_items,
        path: "/items"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_items_count,
        path: "/items/$count"
    );
}

impl SitesItemsIdApiClient {
    get!(
        doc: "Get items from sites",
        name: get_items,
        path: "/items/{{RID}}"
    );
}
