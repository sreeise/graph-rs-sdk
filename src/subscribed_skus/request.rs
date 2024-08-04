// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    SubscribedSkusApiClient,
    SubscribedSkusIdApiClient,
    ResourceIdentity::SubscribedSkus
);

impl SubscribedSkusApiClient {
    post!(
        doc: "Add new entity to subscribedSkus",
        name: create_subscribed_sku,
        path: "/subscribedSkus",
        body: true
    );
    get!(
        doc: "List subscribedSkus",
        name: list_subscribed_sku,
        path: "/subscribedSkus"
    );
}

impl SubscribedSkusIdApiClient {
    delete!(
        doc: "Delete entity from subscribedSkus",
        name: delete_subscribed_sku,
        path: "/subscribedSkus/{{RID}}"
    );
    get!(
        doc: "Get subscribedSku",
        name: get_subscribed_sku,
        path: "/subscribedSkus/{{RID}}"
    );
    patch!(
        doc: "Update entity in subscribedSkus",
        name: update_subscribed_sku,
        path: "/subscribedSkus/{{RID}}",
        body: true
    );
}
