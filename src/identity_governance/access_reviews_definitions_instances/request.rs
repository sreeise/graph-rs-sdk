// GENERATED CODE

use crate::api_default_imports::*;
use crate::identity_governance::{
    AccessReviewsDefinitionsInstancesStagesApiClient,
    AccessReviewsDefinitionsInstancesStagesIdApiClient,
};

resource_api_client!(
    AccessReviewsDefinitionsInstancesApiClient,
    AccessReviewsDefinitionsInstancesIdApiClient,
    ResourceIdentity::AccessReviewsDefinitionsInstances
);

impl AccessReviewsDefinitionsInstancesApiClient {
    post!(
        doc: "Create new navigation property to instances for identityGovernance",
        name: create_instances,
        path: "/instances",
        body: true
    );
    get!(
        doc: "List instances",
        name: list_instances,
        path: "/instances"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_instances_count,
        path: "/instances/$count"
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        path: "/instances/filterByCurrentUser(on='{{id}}')",
        params: on
    );
}

impl AccessReviewsDefinitionsInstancesIdApiClient {
    api_client_link_id!(
        stage,
        ResourceIdentity::AccessReviewsDefinitionsInstancesStages,
        AccessReviewsDefinitionsInstancesStagesIdApiClient
    );
    api_client_link!(
        stages,
        ResourceIdentity::AccessReviewsDefinitionsInstancesStages,
        AccessReviewsDefinitionsInstancesStagesApiClient
    );

    delete!(
        doc: "Delete navigation property instances for identityGovernance",
        name: delete_instances,
        path: "/instances/{{RID}}"
    );
    get!(
        doc: "Get instances from identityGovernance",
        name: get_instances,
        path: "/instances/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property instances in identityGovernance",
        name: update_instances,
        path: "/instances/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action acceptRecommendations",
        name: accept_recommendations,
        path: "/instances/{{RID}}/acceptRecommendations"
    );
    post!(
        doc: "Invoke action applyDecisions",
        name: apply_decisions,
        path: "/instances/{{RID}}/applyDecisions"
    );
    post!(
        doc: "Invoke action batchRecordDecisions",
        name: batch_record_decisions,
        path: "/instances/{{RID}}/batchRecordDecisions",
        body: true
    );
    post!(
        doc: "Create new navigation property to contactedReviewers for identityGovernance",
        name: create_contacted_reviewers,
        path: "/instances/{{RID}}/contactedReviewers",
        body: true
    );
    get!(
        doc: "List contactedReviewers",
        name: list_contacted_reviewers,
        path: "/instances/{{RID}}/contactedReviewers"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_contacted_reviewers_count,
        path: "/instances/{{RID}}/contactedReviewers/$count"
    );
    delete!(
        doc: "Delete navigation property contactedReviewers for identityGovernance",
        name: delete_contacted_reviewers,
        path: "/instances/{{RID}}/contactedReviewers/{{id}}",
        params: access_review_reviewer_id
    );
    get!(
        doc: "Get contactedReviewers from identityGovernance",
        name: get_contacted_reviewers,
        path: "/instances/{{RID}}/contactedReviewers/{{id}}",
        params: access_review_reviewer_id
    );
    patch!(
        doc: "Update the navigation property contactedReviewers in identityGovernance",
        name: update_contacted_reviewers,
        path: "/instances/{{RID}}/contactedReviewers/{{id}}",
        body: true,
        params: access_review_reviewer_id
    );
    post!(
        doc: "Create new navigation property to decisions for identityGovernance",
        name: create_decisions,
        path: "/instances/{{RID}}/decisions",
        body: true
    );
    get!(
        doc: "List decisions",
        name: list_decisions,
        path: "/instances/{{RID}}/decisions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_decisions_count,
        path: "/instances/{{RID}}/decisions/$count"
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        path: "/instances/{{RID}}/decisions/filterByCurrentUser(on='{{id}}')",
        params: on
    );
    delete!(
        doc: "Delete navigation property decisions for identityGovernance",
        name: delete_decisions,
        path: "/instances/{{RID}}/decisions/{{id}}",
        params: access_review_instance_decision_item_id
    );
    get!(
        doc: "Get decisions from identityGovernance",
        name: get_decisions,
        path: "/instances/{{RID}}/decisions/{{id}}",
        params: access_review_instance_decision_item_id
    );
    patch!(
        doc: "Update the navigation property decisions in identityGovernance",
        name: update_decisions,
        path: "/instances/{{RID}}/decisions/{{id}}",
        body: true,
        params: access_review_instance_decision_item_id
    );
    post!(
        doc: "Invoke action resetDecisions",
        name: reset_decisions,
        path: "/instances/{{RID}}/resetDecisions"
    );
    post!(
        doc: "Invoke action sendReminder",
        name: send_reminder,
        path: "/instances/{{RID}}/sendReminder"
    );
    post!(
        doc: "Invoke action stop",
        name: stop,
        path: "/instances/{{RID}}/stop"
    );
}
