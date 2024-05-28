// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    AccessReviewsDefinitionsInstancesStagesApiClient,
    AccessReviewsDefinitionsInstancesStagesIdApiClient,
    ResourceIdentity::AccessReviewsDefinitionsInstancesStages
);

impl AccessReviewsDefinitionsInstancesStagesApiClient {
    post!(
        doc: "Create new navigation property to stages for identityGovernance",
        name: create_stages,
        path: "/stages",
        body: true
    );
    get!(
        doc: "List stages (of an access review)",
        name: list_stages,
        path: "/stages"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_stages_count,
        path: "/stages/$count"
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        path: "/stages/filterByCurrentUser(on='{{id}}')",
        params: on
    );
}

impl AccessReviewsDefinitionsInstancesStagesIdApiClient {
    delete!(
        doc: "Delete navigation property stages for identityGovernance",
        name: delete_stages,
        path: "/stages/{{RID}}"
    );
    get!(
        doc: "Get stages from identityGovernance",
        name: get_stages,
        path: "/stages/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property stages in identityGovernance",
        name: update_stages,
        path: "/stages/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to decisions for identityGovernance",
        name: create_decisions,
        path: "/stages/{{RID}}/decisions",
        body: true
    );
    get!(
        doc: "List decisions (from a multi-stage access review)",
        name: list_decisions,
        path: "/stages/{{RID}}/decisions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_decisions_count,
        path: "/stages/{{RID}}/decisions/$count"
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        path: "/stages/{{RID}}/decisions/filterByCurrentUser(on='{{id}}')",
        params: on
    );
    delete!(
        doc: "Delete navigation property decisions for identityGovernance",
        name: delete_decisions,
        path: "/stages/{{RID}}/decisions/{{id}}",
        params: access_review_instance_decision_item_id
    );
    get!(
        doc: "Get decisions from identityGovernance",
        name: get_decisions,
        path: "/stages/{{RID}}/decisions/{{id}}",
        params: access_review_instance_decision_item_id
    );
    patch!(
        doc: "Update the navigation property decisions in identityGovernance",
        name: update_decisions,
        path: "/stages/{{RID}}/decisions/{{id}}",
        body: true,
        params: access_review_instance_decision_item_id
    );
    post!(
        doc: "Invoke action stop",
        name: stop,
        path: "/stages/{{RID}}/stop"
    );
}
