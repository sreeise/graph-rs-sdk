// GENERATED CODE

use crate::api_default_imports::*;
use crate::identity_governance::{
    AccessReviewsDefinitionsInstancesApiClient, AccessReviewsDefinitionsInstancesIdApiClient,
};

resource_api_client!(
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
        path: "/definitions/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: on
    );
}

impl AccessReviewsDefinitionsIdApiClient {
    api_client_link_id!(
        instance,
        ResourceIdentity::AccessReviewsDefinitionsInstances,
        AccessReviewsDefinitionsInstancesIdApiClient
    );
    api_client_link!(
        instances,
        ResourceIdentity::AccessReviewsDefinitionsInstances,
        AccessReviewsDefinitionsInstancesApiClient
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
        doc: "Create new navigation property to instances for identityGovernance",
        name: create_instances,
        path: "/definitions/{{RID}}/instances",
        body: true
    );
    get!(
        doc: "List instances",
        name: list_instances,
        path: "/definitions/{{RID}}/instances"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_instances_count,
        path: "/definitions/{{RID}}/instances/$count"
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_instances_by_current_user,
        path: "/definitions/{{RID}}/instances/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: on
    );
    delete!(
        doc: "Delete navigation property instances for identityGovernance",
        name: delete_instances,
        path: "/definitions/{{RID}}/instances/{{id}}",
        params: access_review_instance_id
    );
    get!(
        doc: "Get instances from identityGovernance",
        name: get_instances,
        path: "/definitions/{{RID}}/instances/{{id}}",
        params: access_review_instance_id
    );
    patch!(
        doc: "Update the navigation property instances in identityGovernance",
        name: update_instances,
        path: "/definitions/{{RID}}/instances/{{id}}",
        body: true,
        params: access_review_instance_id
    );
    post!(
        doc: "Create new navigation property to contactedReviewers for identityGovernance",
        name: create_contacted_reviewers,
        path: "/definitions/{{RID}}/instances/{{id}}/contactedReviewers",
        body: true,
        params: access_review_instance_id
    );
    get!(
        doc: "List contactedReviewers",
        name: list_contacted_reviewers,
        path: "/definitions/{{RID}}/instances/{{id}}/contactedReviewers",
        params: access_review_instance_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_contacted_reviewers_count,
        path: "/definitions/{{RID}}/instances/{{id}}/contactedReviewers/$count",
        params: access_review_instance_id
    );
    delete!(
        doc: "Delete navigation property contactedReviewers for identityGovernance",
        name: delete_contacted_reviewers,
        path: "/definitions/{{RID}}/instances/{{id}}/contactedReviewers/{{id2}}",
        params: access_review_instance_id, access_review_reviewer_id
    );
    get!(
        doc: "Get contactedReviewers from identityGovernance",
        name: get_contacted_reviewers,
        path: "/definitions/{{RID}}/instances/{{id}}/contactedReviewers/{{id2}}",
        params: access_review_instance_id, access_review_reviewer_id
    );
    patch!(
        doc: "Update the navigation property contactedReviewers in identityGovernance",
        name: update_contacted_reviewers,
        path: "/definitions/{{RID}}/instances/{{id}}/contactedReviewers/{{id2}}",
        body: true,
        params: access_review_instance_id, access_review_reviewer_id
    );
    post!(
        doc: "Create new navigation property to decisions for identityGovernance",
        name: create_decisions,
        path: "/definitions/{{RID}}/instances/{{id}}/decisions",
        body: true,
        params: access_review_instance_id
    );
    get!(
        doc: "List decisions",
        name: list_decisions,
        path: "/definitions/{{RID}}/instances/{{id}}/decisions",
        params: access_review_instance_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_decisions_count,
        path: "/definitions/{{RID}}/instances/{{id}}/decisions/$count",
        params: access_review_instance_id
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        path: "/definitions/{{RID}}/instances/{{id}}/decisions/microsoft.graph.filterByCurrentUser(on='{{id2}}')",
        params: on
    );
    delete!(
        doc: "Delete navigation property decisions for identityGovernance",
        name: delete_decisions,
        path: "/definitions/{{RID}}/instances/{{id}}/decisions/{{id2}}",
        params: access_review_instance_id, access_review_instance_decision_item_id
    );
    get!(
        doc: "Get decisions from identityGovernance",
        name: get_decisions,
        path: "/definitions/{{RID}}/instances/{{id}}/decisions/{{id2}}",
        params: access_review_instance_id, access_review_instance_decision_item_id
    );
    patch!(
        doc: "Update the navigation property decisions in identityGovernance",
        name: update_decisions,
        path: "/definitions/{{RID}}/instances/{{id}}/decisions/{{id2}}",
        body: true,
        params: access_review_instance_id, access_review_instance_decision_item_id
    );
    post!(
        doc: "Invoke action acceptRecommendations",
        name: accept_recommendations,
        path: "/definitions/{{RID}}/instances/{{id}}/microsoft.graph.acceptRecommendations",
        params: access_review_instance_id
    );
    post!(
        doc: "Invoke action applyDecisions",
        name: apply_decisions,
        path: "/definitions/{{RID}}/instances/{{id}}/microsoft.graph.applyDecisions",
        params: access_review_instance_id
    );
    post!(
        doc: "Invoke action batchRecordDecisions",
        name: batch_record_decisions,
        path: "/definitions/{{RID}}/instances/{{id}}/microsoft.graph.batchRecordDecisions",
        body: true,
        params: access_review_instance_id
    );
    post!(
        doc: "Invoke action resetDecisions",
        name: reset_decisions,
        path: "/definitions/{{RID}}/instances/{{id}}/microsoft.graph.resetDecisions",
        params: access_review_instance_id
    );
    post!(
        doc: "Invoke action sendReminder",
        name: send_reminder,
        path: "/definitions/{{RID}}/instances/{{id}}/microsoft.graph.sendReminder",
        params: access_review_instance_id
    );
    post!(
        doc: "Invoke action stop",
        name: stop_instances,
        path: "/definitions/{{RID}}/instances/{{id}}/microsoft.graph.stop",
        params: access_review_instance_id
    );
    post!(
        doc: "Invoke action stop",
        name: stop,
        path: "/definitions/{{RID}}/microsoft.graph.stop"
    );
}
