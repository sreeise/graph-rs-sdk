// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    TermsAndConditionsApiClient,
    TermsAndConditionsIdApiClient,
    ResourceIdentity::TermsAndConditions
);

impl TermsAndConditionsApiClient {
    post!(
        doc: "Create new navigation property to termsAndConditions for deviceManagement",
        name: create_terms_and_conditions,
        path: "/termsAndConditions",
        body: true
    );
    get!(
        doc: "Get termsAndConditions from deviceManagement",
        name: list_terms_and_conditions,
        path: "/termsAndConditions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_terms_and_conditions_count,
        path: "/termsAndConditions/$count"
    );
}

impl TermsAndConditionsIdApiClient {
    delete!(
        doc: "Delete navigation property termsAndConditions for deviceManagement",
        name: delete_terms_and_conditions,
        path: "/termsAndConditions/{{RID}}"
    );
    get!(
        doc: "Get termsAndConditions from deviceManagement",
        name: get_terms_and_conditions,
        path: "/termsAndConditions/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property termsAndConditions in deviceManagement",
        name: update_terms_and_conditions,
        path: "/termsAndConditions/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to acceptanceStatuses for deviceManagement",
        name: create_acceptance_statuses,
        path: "/termsAndConditions/{{RID}}/acceptanceStatuses",
        body: true
    );
    get!(
        doc: "Get acceptanceStatuses from deviceManagement",
        name: list_acceptance_statuses,
        path: "/termsAndConditions/{{RID}}/acceptanceStatuses"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_acceptance_statuses_count,
        path: "/termsAndConditions/{{RID}}/acceptanceStatuses/$count"
    );
    delete!(
        doc: "Delete navigation property acceptanceStatuses for deviceManagement",
        name: delete_acceptance_statuses,
        path: "/termsAndConditions/{{RID}}/acceptanceStatuses/{{id}}",
        params: terms_and_conditions_acceptance_status_id
    );
    get!(
        doc: "Get acceptanceStatuses from deviceManagement",
        name: get_acceptance_statuses,
        path: "/termsAndConditions/{{RID}}/acceptanceStatuses/{{id}}",
        params: terms_and_conditions_acceptance_status_id
    );
    patch!(
        doc: "Update the navigation property acceptanceStatuses in deviceManagement",
        name: update_acceptance_statuses,
        path: "/termsAndConditions/{{RID}}/acceptanceStatuses/{{id}}",
        body: true,
        params: terms_and_conditions_acceptance_status_id
    );
    get!(
        doc: "Get termsAndConditions from deviceManagement",
        name: get_acceptance_statuses_terms_and_conditions,
        path: "/termsAndConditions/{{RID}}/acceptanceStatuses/{{id}}/termsAndConditions",
        params: terms_and_conditions_acceptance_status_id
    );
    post!(
        doc: "Create new navigation property to assignments for deviceManagement",
        name: create_assignments,
        path: "/termsAndConditions/{{RID}}/assignments",
        body: true
    );
    get!(
        doc: "Get assignments from deviceManagement",
        name: list_assignments,
        path: "/termsAndConditions/{{RID}}/assignments"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_assignments_count,
        path: "/termsAndConditions/{{RID}}/assignments/$count"
    );
    delete!(
        doc: "Delete navigation property assignments for deviceManagement",
        name: delete_assignments,
        path: "/termsAndConditions/{{RID}}/assignments/{{id}}",
        params: terms_and_conditions_assignment_id
    );
    get!(
        doc: "Get assignments from deviceManagement",
        name: get_assignments,
        path: "/termsAndConditions/{{RID}}/assignments/{{id}}",
        params: terms_and_conditions_assignment_id
    );
    patch!(
        doc: "Update the navigation property assignments in deviceManagement",
        name: update_assignments,
        path: "/termsAndConditions/{{RID}}/assignments/{{id}}",
        body: true,
        params: terms_and_conditions_assignment_id
    );
}
