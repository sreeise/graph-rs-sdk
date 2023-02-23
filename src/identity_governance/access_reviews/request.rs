// GENERATED CODE

use crate::api_default_imports::*;
use crate::identity_governance::access_review_definitions::{
    AccessReviewDefinitionsApiClient, AccessReviewDefinitionsIdApiClient,
};

resource_api_client!(AccessReviewsApiClient, ResourceIdentity::AccessReviews);

impl AccessReviewsApiClient {
    api_client_link_id!(
        access_review_definition,
        ResourceIdentity::AccessReviewDefinitions,
        AccessReviewDefinitionsIdApiClient
    );
    api_client_link!(
        access_review_definitions,
        ResourceIdentity::AccessReviewDefinitions,
        AccessReviewDefinitionsApiClient
    );

    delete!(
        doc: "Delete navigation property accessReviews for identityGovernance",
        name: delete_access_reviews,
        path: "/accessReviews"
    );
    get!(
        doc: "Get accessReviews from identityGovernance",
        name: get_access_reviews,
        path: "/accessReviews"
    );
    patch!(
        doc: "Update the navigation property accessReviews in identityGovernance",
        name: update_access_reviews,
        path: "/accessReviews",
        body: true
    );
    post!(
        doc: "Create historyDefinitions",
        name: create_history_definitions,
        path: "/accessReviews/historyDefinitions",
        body: true
    );
    get!(
        doc: "List historyDefinitions",
        name: list_history_definitions,
        path: "/accessReviews/historyDefinitions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_history_definitions_count,
        path: "/accessReviews/historyDefinitions/$count"
    );
    delete!(
        doc: "Delete navigation property historyDefinitions for identityGovernance",
        name: delete_history_definitions,
        path: "/accessReviews/historyDefinitions/{{id}}",
        params: access_review_history_definition_id
    );
    get!(
        doc: "Get historyDefinitions from identityGovernance",
        name: get_history_definitions,
        path: "/accessReviews/historyDefinitions/{{id}}",
        params: access_review_history_definition_id
    );
    patch!(
        doc: "Update the navigation property historyDefinitions in identityGovernance",
        name: update_history_definitions,
        path: "/accessReviews/historyDefinitions/{{id}}",
        body: true,
        params: access_review_history_definition_id
    );
    post!(
        doc: "Create new navigation property to instances for identityGovernance",
        name: create_instances,
        path: "/accessReviews/historyDefinitions/{{id}}/instances",
        body: true,
        params: access_review_history_definition_id
    );
    get!(
        doc: "List instances (of an accessReviewHistoryDefinition)",
        name: list_instances,
        path: "/accessReviews/historyDefinitions/{{id}}/instances",
        params: access_review_history_definition_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_instances_count,
        path: "/accessReviews/historyDefinitions/{{id}}/instances/$count",
        params: access_review_history_definition_id
    );
    delete!(
        doc: "Delete navigation property instances for identityGovernance",
        name: delete_instances,
        path: "/accessReviews/historyDefinitions/{{id}}/instances/{{id2}}",
        params: access_review_history_definition_id, access_review_history_instance_id
    );
    get!(
        doc: "Get instances from identityGovernance",
        name: get_instances,
        path: "/accessReviews/historyDefinitions/{{id}}/instances/{{id2}}",
        params: access_review_history_definition_id, access_review_history_instance_id
    );
    patch!(
        doc: "Update the navigation property instances in identityGovernance",
        name: update_instances,
        path: "/accessReviews/historyDefinitions/{{id}}/instances/{{id2}}",
        body: true,
        params: access_review_history_definition_id, access_review_history_instance_id
    );
    post!(
        doc: "Invoke action generateDownloadUri",
        name: generate_download_uri,
        path: "/accessReviews/historyDefinitions/{{id}}/instances/{{id2}}/microsoft.graph.generateDownloadUri",
        params: access_review_history_definition_id, access_review_history_instance_id
    );
}
