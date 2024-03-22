// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    SubscriptionsApiClient,
    SubscriptionsIdApiClient,
    ResourceIdentity::Subscriptions
);

impl SubscriptionsApiClient {
    post!(
        doc: "Create subscription",
        name: create_subscription,
        path: "/subscriptions",
        body: true
    );
    get!(
        doc: "List subscriptions",
        name: list_subscription,
        path: "/subscriptions"
    );
}

impl SubscriptionsIdApiClient {
    delete!(
        doc: "Delete subscription",
        name: delete_subscription,
        path: "/subscriptions/{{RID}}"
    );
    get!(
        doc: "Get subscription",
        name: get_subscription,
        path: "/subscriptions/{{RID}}"
    );
    patch!(
        doc: "Update subscription",
        name: update_subscription,
        path: "/subscriptions/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action reauthorize",
        name: reauthorize,
        path: "/subscriptions/{{RID}}/reauthorize"
    );
}
