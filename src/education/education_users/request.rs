// GENERATED CODE

use crate::api_default_imports::*;
use crate::education::*;

resource_api_client!(
    EducationUsersApiClient,
    EducationUsersIdApiClient,
    ResourceIdentity::EducationUsers
);

impl EducationUsersApiClient {
    post!(
        doc: "Create educationUser",
        name: create_users,
        path: "/users",
        body: true
    );
    get!(
        doc: "List educationUsers",
        name: list_users,
        path: "/users"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_users_count,
        path: "/users/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/users/delta()"
    );
}

impl EducationUsersIdApiClient {
    api_client_link_id!(assignment, EducationAssignmentsIdApiClient);
    api_client_link!(assignments, EducationAssignmentsApiClient);

    delete!(
        doc: "Delete navigation property users for education",
        name: delete_users,
        path: "/users/{{RID}}"
    );
    get!(
        doc: "Get users from education",
        name: get_users,
        path: "/users/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property users in education",
        name: update_users,
        path: "/users/{{RID}}",
        body: true
    );
    get!(
        doc: "List classes of an educationUser",
        name: list_classes,
        path: "/users/{{RID}}/classes"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_classes_count,
        path: "/users/{{RID}}/classes/$count"
    );
    get!(
        doc: "Get classes from education",
        name: get_classes,
        path: "/users/{{RID}}/classes/{{id}}",
        params: education_class_id
    );
    post!(
        doc: "Create educationRubric",
        name: create_rubrics,
        path: "/users/{{RID}}/rubrics",
        body: true
    );
    get!(
        doc: "List rubrics",
        name: list_rubrics,
        path: "/users/{{RID}}/rubrics"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_rubrics_count,
        path: "/users/{{RID}}/rubrics/$count"
    );
    delete!(
        doc: "Delete navigation property rubrics for education",
        name: delete_rubrics,
        path: "/users/{{RID}}/rubrics/{{id}}",
        params: education_rubric_id
    );
    get!(
        doc: "Get rubrics from education",
        name: get_rubrics,
        path: "/users/{{RID}}/rubrics/{{id}}",
        params: education_rubric_id
    );
    patch!(
        doc: "Update the navigation property rubrics in education",
        name: update_rubrics,
        path: "/users/{{RID}}/rubrics/{{id}}",
        body: true,
        params: education_rubric_id
    );
    get!(
        doc: "List schools of an educationUser",
        name: list_schools,
        path: "/users/{{RID}}/schools"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_schools_count,
        path: "/users/{{RID}}/schools/$count"
    );
    get!(
        doc: "Get schools from education",
        name: get_schools,
        path: "/users/{{RID}}/schools/{{id}}",
        params: education_school_id
    );
    get!(
        doc: "List taughtClasses",
        name: list_taught_classes,
        path: "/users/{{RID}}/taughtClasses"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_taught_classes_count,
        path: "/users/{{RID}}/taughtClasses/$count"
    );
    get!(
        doc: "Get taughtClasses from education",
        name: get_taught_classes,
        path: "/users/{{RID}}/taughtClasses/{{id}}",
        params: education_class_id
    );
    get!(
        doc: "Get educationUser",
        name: get_user,
        path: "/users/{{RID}}/user"
    );
}
