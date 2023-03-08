// GENERATED CODE

use crate::api_default_imports::*;
use crate::identity_governance::{
    AccessReviewsDefinitionsApiClient, AccessReviewsDefinitionsIdApiClient,
};

resource_api_client!(AccessReviewsApiClient, ResourceIdentity::AccessReviews);

impl AccessReviewsApiClient {
    api_client_link_id!(definition, AccessReviewsDefinitionsIdApiClient);
    api_client_link!(definitions, AccessReviewsDefinitionsApiClient);

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
        doc: "Create definitions",
        name: create_definitions,
        path: "/accessReviews/definitions",
        body: true
    );
    get!(
        doc: "List definitions",
        name: list_definitions,
        path: "/accessReviews/definitions"
    );
    get!(
        doc: "Get the number of the resource",
        name: count,
        path: "/accessReviews/definitions/$count"
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        path: "/accessReviews/definitions/filterByCurrentUser(on='{{id}}')",
        params: on
    );
    delete!(
        doc: "Delete navigation property definitions for identityGovernance",
        name: delete_definitions,
        path: "/accessReviews/definitions/{{id}}",
        params: access_review_schedule_definition_id
    );
    get!(
        doc: "Get definitions from identityGovernance",
        name: get_definitions,
        path: "/accessReviews/definitions/{{id}}",
        params: access_review_schedule_definition_id
    );
    patch!(
        doc: "Update the navigation property definitions in identityGovernance",
        name: update_definitions,
        path: "/accessReviews/definitions/{{id}}",
        body: true,
        params: access_review_schedule_definition_id
    );
    post!(
        doc: "Create new navigation property to instances for identityGovernance",
        name: create_instances,
        path: "/accessReviews/definitions/{{id}}/instances",
        body: true,
        params: access_review_schedule_definition_id
    );
    get!(
        doc: "List instances",
        name: list_instances,
        path: "/accessReviews/definitions/{{id}}/instances",
        params: access_review_schedule_definition_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_instances_count,
        path: "/accessReviews/definitions/{{id}}/instances/$count",
        params: access_review_schedule_definition_id
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        path: "/accessReviews/definitions/{{id}}/instances/filterByCurrentUser(on='{{id2}}')",
        params: on
    );
    delete!(
        doc: "Delete navigation property instances for identityGovernance",
        name: delete_instances,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}",
        params: access_review_schedule_definition_id, access_review_instance_id
    );
    get!(
        doc: "Get instances from identityGovernance",
        name: get_instances,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}",
        params: access_review_schedule_definition_id, access_review_instance_id
    );
    patch!(
        doc: "Update the navigation property instances in identityGovernance",
        name: update_instances,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}",
        body: true,
        params: access_review_schedule_definition_id, access_review_instance_id
    );
    post!(
        doc: "Invoke action acceptRecommendations",
        name: accept_recommendations,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/acceptRecommendations",
        params: access_review_schedule_definition_id, access_review_instance_id
    );
    post!(
        doc: "Invoke action applyDecisions",
        name: apply_decisions,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/applyDecisions",
        params: access_review_schedule_definition_id, access_review_instance_id
    );
    post!(
        doc: "Invoke action batchRecordDecisions",
        name: batch_record_decisions,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/batchRecordDecisions",
        body: true,
        params: access_review_schedule_definition_id, access_review_instance_id
    );
    post!(
        doc: "Create new navigation property to contactedReviewers for identityGovernance",
        name: create_contacted_reviewers,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/contactedReviewers",
        body: true,
        params: access_review_schedule_definition_id, access_review_instance_id
    );
    get!(
        doc: "List contactedReviewers",
        name: list_contacted_reviewers,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/contactedReviewers",
        params: access_review_schedule_definition_id, access_review_instance_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_contacted_reviewers_count,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/contactedReviewers/$count",
        params: access_review_schedule_definition_id, access_review_instance_id
    );
    delete!(
        doc: "Delete navigation property contactedReviewers for identityGovernance",
        name: delete_contacted_reviewers,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/contactedReviewers/{{id3}}",
        params: access_review_schedule_definition_id, access_review_instance_id, access_review_reviewer_id
    );
    get!(
        doc: "Get contactedReviewers from identityGovernance",
        name: get_contacted_reviewers,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/contactedReviewers/{{id3}}",
        params: access_review_schedule_definition_id, access_review_instance_id, access_review_reviewer_id
    );
    patch!(
        doc: "Update the navigation property contactedReviewers in identityGovernance",
        name: update_contacted_reviewers,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/contactedReviewers/{{id3}}",
        body: true,
        params: access_review_schedule_definition_id, access_review_instance_id, access_review_reviewer_id
    );
    post!(
        doc: "Create new navigation property to decisions for identityGovernance",
        name: create_decisions,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/decisions",
        body: true,
        params: access_review_schedule_definition_id, access_review_instance_id
    );
    get!(
        doc: "List decisions",
        name: list_decisions,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/decisions",
        params: access_review_schedule_definition_id, access_review_instance_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_decisions_count,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/decisions/$count",
        params: access_review_schedule_definition_id, access_review_instance_id
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/decisions/filterByCurrentUser(on='{{id3}}')",
        params: on
    );
    delete!(
        doc: "Delete navigation property decisions for identityGovernance",
        name: delete_decisions,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/decisions/{{id3}}",
        params: access_review_schedule_definition_id, access_review_instance_id, access_review_instance_decision_item_id
    );
    get!(
        doc: "Get decisions from identityGovernance",
        name: get_decisions,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/decisions/{{id3}}",
        params: access_review_schedule_definition_id, access_review_instance_id, access_review_instance_decision_item_id
    );
    patch!(
        doc: "Update the navigation property decisions in identityGovernance",
        name: update_decisions,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/decisions/{{id3}}",
        body: true,
        params: access_review_schedule_definition_id, access_review_instance_id, access_review_instance_decision_item_id
    );
    post!(
        doc: "Invoke action resetDecisions",
        name: reset_decisions,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/resetDecisions",
        params: access_review_schedule_definition_id, access_review_instance_id
    );
    post!(
        doc: "Invoke action sendReminder",
        name: send_reminder,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/sendReminder",
        params: access_review_schedule_definition_id, access_review_instance_id
    );
    post!(
        doc: "Create new navigation property to stages for identityGovernance",
        name: create_stages,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/stages",
        body: true,
        params: access_review_schedule_definition_id, access_review_instance_id
    );
    get!(
        doc: "List stages (of an access review)",
        name: list_stages,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/stages",
        params: access_review_schedule_definition_id, access_review_instance_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_stages_count,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/stages/$count",
        params: access_review_schedule_definition_id, access_review_instance_id
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/stages/filterByCurrentUser(on='{{id3}}')",
        params: on
    );
    delete!(
        doc: "Delete navigation property stages for identityGovernance",
        name: delete_stages,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/stages/{{id3}}",
        params: access_review_schedule_definition_id, access_review_instance_id, access_review_stage_id
    );
    get!(
        doc: "Get stages from identityGovernance",
        name: get_stages,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/stages/{{id3}}",
        params: access_review_schedule_definition_id, access_review_instance_id, access_review_stage_id
    );
    patch!(
        doc: "Update the navigation property stages in identityGovernance",
        name: update_stages,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/stages/{{id3}}",
        body: true,
        params: access_review_schedule_definition_id, access_review_instance_id, access_review_stage_id
    );
    post!(
        doc: "Create new navigation property to decisions for identityGovernance",
        name: create_decisions,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/stages/{{id3}}/decisions",
        body: true,
        params: access_review_schedule_definition_id, access_review_instance_id, access_review_stage_id
    );
    get!(
        doc: "List decisions (from a multi-stage access review)",
        name: list_decisions,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/stages/{{id3}}/decisions",
        params: access_review_schedule_definition_id, access_review_instance_id, access_review_stage_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_decisions_count,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/stages/{{id3}}/decisions/$count",
        params: access_review_schedule_definition_id, access_review_instance_id, access_review_stage_id
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/stages/{{id3}}/decisions/filterByCurrentUser(on='{{id4}}')",
        params: on
    );
    delete!(
        doc: "Delete navigation property decisions for identityGovernance",
        name: delete_decisions,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/stages/{{id3}}/decisions/{{id4}}",
        params: access_review_schedule_definition_id, access_review_instance_id, access_review_stage_id, access_review_instance_decision_item_id
    );
    get!(
        doc: "Get decisions from identityGovernance",
        name: get_decisions,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/stages/{{id3}}/decisions/{{id4}}",
        params: access_review_schedule_definition_id, access_review_instance_id, access_review_stage_id, access_review_instance_decision_item_id
    );
    patch!(
        doc: "Update the navigation property decisions in identityGovernance",
        name: update_decisions,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/stages/{{id3}}/decisions/{{id4}}",
        body: true,
        params: access_review_schedule_definition_id, access_review_instance_id, access_review_stage_id, access_review_instance_decision_item_id
    );
    post!(
        doc: "Invoke action stop",
        name: stop,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/stages/{{id3}}/stop",
        params: access_review_schedule_definition_id, access_review_instance_id, access_review_stage_id
    );
    post!(
        doc: "Invoke action stop",
        name: stop,
        path: "/accessReviews/definitions/{{id}}/instances/{{id2}}/stop",
        params: access_review_schedule_definition_id, access_review_instance_id
    );
    post!(
        doc: "Invoke action stop",
        name: stop,
        path: "/accessReviews/definitions/{{id}}/stop",
        params: access_review_schedule_definition_id
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
        path: "/accessReviews/historyDefinitions/{{id}}/instances/{{id2}}/generateDownloadUri",
        params: access_review_history_definition_id, access_review_history_instance_id
    );
}
