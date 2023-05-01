// GENERATED CODE

use crate::api_default_imports::*;
use crate::identity_governance::{
    AccessReviewsDefinitionsInstancesApiClient, AccessReviewsDefinitionsInstancesIdApiClient,
};

api_client!(
    AccessReviewsDefinitionsApiClient,
    AccessReviewsDefinitionsIdApiClient,
    ResourceIdentity::AccessReviewsDefinitions
);

impl AccessReviewsDefinitionsApiClient {
    post!(
        doc: "Create definitions",
        name: create_definitions,
        path: "/definitions",
        body: true
    );
    get!(
        doc: "List definitions",
        name: list_definitions,
        path: "/definitions"
    );
    get!(
        doc: "Get the number of the resource",
        name: count,
        path: "/definitions/$count"
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        path: "/definitions/filterByCurrentUser(on='{{id}}')",
        params: on
    );
}

impl AccessReviewsDefinitionsIdApiClient {
    api_client_link!(
        instances,
        ResourceIdentity::AccessReviewsDefinitionsInstances,
        AccessReviewsDefinitionsInstancesApiClient
    );
    api_client_link_id!(
        instance,
        ResourceIdentity::AccessReviewsDefinitionsInstances,
        AccessReviewsDefinitionsInstancesIdApiClient
    );

    delete!(
        doc: "Delete navigation property definitions for identityGovernance",
        name: delete_definitions,
        path: "/definitions/{{RID}}"
    );
    get!(
        doc: "Get definitions from identityGovernance",
        name: get_definitions,
        path: "/definitions/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property definitions in identityGovernance",
        name: update_definitions,
        path: "/definitions/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action stop",
        name: stop,
        path: "/definitions/{{RID}}/stop"
    );
}
