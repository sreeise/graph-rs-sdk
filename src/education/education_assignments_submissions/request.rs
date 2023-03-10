// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    EducationAssignmentsSubmissionsApiClient,
    EducationAssignmentsSubmissionsIdApiClient,
    ResourceIdentity::EducationAssignmentsSubmissions
);

impl EducationAssignmentsSubmissionsApiClient {
    post!(
        doc: "Create new navigation property to submissions for education",
        name: create_submissions,
        path: "/submissions",
        body: true
    );
    get!(
        doc: "List submissions",
        name: list_submissions,
        path: "/submissions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_submissions_count,
        path: "/submissions/$count"
    );
}

impl EducationAssignmentsSubmissionsIdApiClient {
    delete!(
        doc: "Delete navigation property submissions for education",
        name: delete_submissions,
        path: "/submissions/{{RID}}"
    );
    get!(
        doc: "Get submissions from education",
        name: get_submissions,
        path: "/submissions/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property submissions in education",
        name: update_submissions,
        path: "/submissions/{{RID}}",
        body: true
    );
    post!(
        doc: "Create educationFeedbackResourceOutcome",
        name: create_outcomes,
        path: "/submissions/{{RID}}/outcomes",
        body: true
    );
    get!(
        doc: "List outcomes",
        name: list_outcomes,
        path: "/submissions/{{RID}}/outcomes"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_outcomes_count,
        path: "/submissions/{{RID}}/outcomes/$count"
    );
    delete!(
        doc: "Delete navigation property outcomes for education",
        name: delete_outcomes,
        path: "/submissions/{{RID}}/outcomes/{{id}}",
        params: education_outcome_id
    );
    get!(
        doc: "Get outcomes from education",
        name: get_outcomes,
        path: "/submissions/{{RID}}/outcomes/{{id}}",
        params: education_outcome_id
    );
    patch!(
        doc: "Update the navigation property outcomes in education",
        name: update_outcomes,
        path: "/submissions/{{RID}}/outcomes/{{id}}",
        body: true,
        params: education_outcome_id
    );
    post!(
        doc: "Invoke action reassign",
        name: reassign,
        path: "/submissions/{{RID}}/reassign"
    );
    post!(
        doc: "Create educationSubmissionResource",
        name: create_resources,
        path: "/submissions/{{RID}}/resources",
        body: true
    );
    get!(
        doc: "List submission resources",
        name: list_resources,
        path: "/submissions/{{RID}}/resources"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_resources_count,
        path: "/submissions/{{RID}}/resources/$count"
    );
    delete!(
        doc: "Delete navigation property resources for education",
        name: delete_resources,
        path: "/submissions/{{RID}}/resources/{{id}}",
        params: education_submission_resource_id
    );
    get!(
        doc: "Get resources from education",
        name: get_resources,
        path: "/submissions/{{RID}}/resources/{{id}}",
        params: education_submission_resource_id
    );
    patch!(
        doc: "Update the navigation property resources in education",
        name: update_resources,
        path: "/submissions/{{RID}}/resources/{{id}}",
        body: true,
        params: education_submission_resource_id
    );
    post!(
        doc: "Invoke action return",
        name: submissions_return,
        path: "/submissions/{{RID}}/return"
    );
    post!(
        doc: "Invoke action setUpResourcesFolder",
        name: set_up_resources_folder,
        path: "/submissions/{{RID}}/setUpResourcesFolder"
    );
    post!(
        doc: "Invoke action submit",
        name: submit,
        path: "/submissions/{{RID}}/submit"
    );
    post!(
        doc: "Create new navigation property to submittedResources for education",
        name: create_submitted_resources,
        path: "/submissions/{{RID}}/submittedResources",
        body: true
    );
    get!(
        doc: "List submittedResources",
        name: list_submitted_resources,
        path: "/submissions/{{RID}}/submittedResources"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_submitted_resources_count,
        path: "/submissions/{{RID}}/submittedResources/$count"
    );
    delete!(
        doc: "Delete navigation property submittedResources for education",
        name: delete_submitted_resources,
        path: "/submissions/{{RID}}/submittedResources/{{id}}",
        params: education_submission_resource_id
    );
    get!(
        doc: "Get submittedResources from education",
        name: get_submitted_resources,
        path: "/submissions/{{RID}}/submittedResources/{{id}}",
        params: education_submission_resource_id
    );
    patch!(
        doc: "Update the navigation property submittedResources in education",
        name: update_submitted_resources,
        path: "/submissions/{{RID}}/submittedResources/{{id}}",
        body: true,
        params: education_submission_resource_id
    );
    post!(
        doc: "Invoke action unsubmit",
        name: unsubmit,
        path: "/submissions/{{RID}}/unsubmit"
    );
}
