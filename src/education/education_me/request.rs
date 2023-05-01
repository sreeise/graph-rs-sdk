// GENERATED CODE

use crate::api_default_imports::*;
use crate::education::*;

api_client!(EducationMeApiClient, ResourceIdentity::EducationMe);

impl EducationMeApiClient {
    api_client_link!(assignments, EducationAssignmentsApiClient);
    api_client_link_id!(assignment, EducationAssignmentsIdApiClient);

    delete!(
        doc: "Delete navigation property me for education",
        name: delete_me,
        path: "/me"
    );
    get!(
        doc: "Get me from education",
        name: get_me,
        path: "/me"
    );
    patch!(
        doc: "Update the navigation property me in education",
        name: update_me,
        path: "/me",
        body: true
    );
    get!(
        doc: "List classes of an educationUser",
        name: list_classes,
        path: "/me/classes"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_classes_count,
        path: "/me/classes/$count"
    );
    get!(
        doc: "Get classes from education",
        name: get_classes,
        path: "/me/classes/{{id}}",
        params: education_class_id
    );
    post!(
        doc: "Create educationRubric",
        name: create_rubrics,
        path: "/me/rubrics",
        body: true
    );
    get!(
        doc: "List rubrics",
        name: list_rubrics,
        path: "/me/rubrics"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_rubrics_count,
        path: "/me/rubrics/$count"
    );
    delete!(
        doc: "Delete navigation property rubrics for education",
        name: delete_rubrics,
        path: "/me/rubrics/{{id}}",
        params: education_rubric_id
    );
    get!(
        doc: "Get rubrics from education",
        name: get_rubrics,
        path: "/me/rubrics/{{id}}",
        params: education_rubric_id
    );
    patch!(
        doc: "Update the navigation property rubrics in education",
        name: update_rubrics,
        path: "/me/rubrics/{{id}}",
        body: true,
        params: education_rubric_id
    );
    get!(
        doc: "List schools of an educationUser",
        name: list_schools,
        path: "/me/schools"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_schools_count,
        path: "/me/schools/$count"
    );
    get!(
        doc: "Get schools from education",
        name: get_schools,
        path: "/me/schools/{{id}}",
        params: education_school_id
    );
    get!(
        doc: "List taughtClasses",
        name: list_taught_classes,
        path: "/me/taughtClasses"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_taught_classes_count,
        path: "/me/taughtClasses/$count"
    );
    get!(
        doc: "Get taughtClasses from education",
        name: get_taught_classes,
        path: "/me/taughtClasses/{{id}}",
        params: education_class_id
    );
    get!(
        doc: "Get educationUser",
        name: get_user,
        path: "/me/user"
    );
}
