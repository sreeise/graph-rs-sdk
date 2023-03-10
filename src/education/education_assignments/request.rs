// GENERATED CODE

use crate::api_default_imports::*;
use crate::education::*;

resource_api_client!(
    EducationAssignmentsApiClient,
    EducationAssignmentsIdApiClient,
    ResourceIdentity::EducationAssignments
);

impl EducationAssignmentsApiClient {
    post!(
        doc: "Create new navigation property to assignments for education",
        name: create_assignments,
        path: "/assignments",
        body: true
    );
    get!(
        doc: "List assignments of a user",
        name: list_assignments,
        path: "/assignments"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_assignments_count,
        path: "/assignments/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/assignments/delta()"
    );
}

impl EducationAssignmentsIdApiClient {
    api_client_link!(submissions, EducationAssignmentsSubmissionsApiClient);
    api_client_link_id!(submission, EducationAssignmentsSubmissionsIdApiClient);

    delete!(
        doc: "Delete navigation property assignments for education",
        name: delete_assignments,
        path: "/assignments/{{RID}}"
    );
    get!(
        doc: "Get assignments from education",
        name: get_assignments,
        path: "/assignments/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property assignments in education",
        name: update_assignments,
        path: "/assignments/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to categories for education",
        name: create_categories,
        path: "/assignments/{{RID}}/categories",
        body: true
    );
    get!(
        doc: "List categories",
        name: list_categories,
        path: "/assignments/{{RID}}/categories"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_categories_count,
        path: "/assignments/{{RID}}/categories/$count"
    );
    post!(
        doc: "Create new navigation property ref to categories for education",
        name: create_ref_categories,
        path: "/assignments/{{RID}}/categories/$ref",
        body: true
    );
    get!(
        doc: "List categories",
        name: list_ref_categories,
        path: "/assignments/{{RID}}/categories/$ref"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/assignments/{{RID}}/categories/delta()"
    );
    delete!(
        doc: "Delete ref of navigation property categories for education",
        name: delete_ref_categories,
        path: "/assignments/{{RID}}/categories/{{id}}/$ref",
        params: education_category_id
    );
    post!(
        doc: "Invoke action publish",
        name: publish,
        path: "/assignments/{{RID}}/publish"
    );
    post!(
        doc: "Create educationAssignmentResource",
        name: create_resources,
        path: "/assignments/{{RID}}/resources",
        body: true
    );
    get!(
        doc: "List assignment resources",
        name: list_resources,
        path: "/assignments/{{RID}}/resources"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_resources_count,
        path: "/assignments/{{RID}}/resources/$count"
    );
    delete!(
        doc: "Delete navigation property resources for education",
        name: delete_resources,
        path: "/assignments/{{RID}}/resources/{{id}}",
        params: education_assignment_resource_id
    );
    get!(
        doc: "Get resources from education",
        name: get_resources,
        path: "/assignments/{{RID}}/resources/{{id}}",
        params: education_assignment_resource_id
    );
    patch!(
        doc: "Update the navigation property resources in education",
        name: update_resources,
        path: "/assignments/{{RID}}/resources/{{id}}",
        body: true,
        params: education_assignment_resource_id
    );
    delete!(
        doc: "Delete navigation property rubric for education",
        name: delete_rubric,
        path: "/assignments/{{RID}}/rubric"
    );
    get!(
        doc: "Get educationRubric attached to educationAssignment",
        name: get_rubric,
        path: "/assignments/{{RID}}/rubric"
    );
    patch!(
        doc: "Update the navigation property rubric in education",
        name: update_rubric,
        path: "/assignments/{{RID}}/rubric",
        body: true
    );
    delete!(
        doc: "Delete ref of navigation property rubric for education",
        name: delete_ref_rubric,
        path: "/assignments/{{RID}}/rubric/$ref"
    );
    get!(
        doc: "Get educationRubric attached to educationAssignment",
        name: get_ref_rubric,
        path: "/assignments/{{RID}}/rubric/$ref"
    );
    put!(
        doc: "Update the ref of navigation property rubric in education",
        name: update_ref_rubric,
        path: "/assignments/{{RID}}/rubric/$ref",
        body: true
    );
    post!(
        doc: "Invoke action setUpFeedbackResourcesFolder",
        name: set_up_feedback_resources_folder,
        path: "/assignments/{{RID}}/setUpFeedbackResourcesFolder"
    );
    post!(
        doc: "Invoke action setUpResourcesFolder",
        name: set_up_resources_folder,
        path: "/assignments/{{RID}}/setUpResourcesFolder"
    );
}
