// GENERATED CODE

use crate::api_default_imports::*;
use crate::education::*;

resource_api_client!(
    EducationClassesApiClient,
    EducationClassesIdApiClient,
    ResourceIdentity::EducationClasses
);

impl EducationClassesApiClient {
    post!(
        doc: "Create educationClass",
        name: create_classes,
        path: "/classes",
        body: true
    );
    get!(
        doc: "List educationClasses",
        name: list_classes,
        path: "/classes"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_classes_count,
        path: "/classes/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/classes/delta()"
    );
}

impl EducationClassesIdApiClient {
    api_client_link_id!(assignment, EducationAssignmentsIdApiClient);
    api_client_link!(assignments, EducationAssignmentsApiClient);

    delete!(
        doc: "Delete navigation property classes for education",
        name: delete_classes,
        path: "/classes/{{RID}}"
    );
    get!(
        doc: "Get classes from education",
        name: get_classes,
        path: "/classes/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property classes in education",
        name: update_classes,
        path: "/classes/{{RID}}",
        body: true
    );
    post!(
        doc: "Create educationCategory",
        name: create_assignment_categories,
        path: "/classes/{{RID}}/assignmentCategories",
        body: true
    );
    get!(
        doc: "List assignmentCategories",
        name: list_assignment_categories,
        path: "/classes/{{RID}}/assignmentCategories"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_assignment_categories_count,
        path: "/classes/{{RID}}/assignmentCategories/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/classes/{{RID}}/assignmentCategories/delta()"
    );
    delete!(
        doc: "Delete navigation property assignmentCategories for education",
        name: delete_assignment_categories,
        path: "/classes/{{RID}}/assignmentCategories/{{id}}",
        params: education_category_id
    );
    get!(
        doc: "Get assignmentCategories from education",
        name: get_assignment_categories,
        path: "/classes/{{RID}}/assignmentCategories/{{id}}",
        params: education_category_id
    );
    patch!(
        doc: "Update the navigation property assignmentCategories in education",
        name: update_assignment_categories,
        path: "/classes/{{RID}}/assignmentCategories/{{id}}",
        body: true,
        params: education_category_id
    );
    delete!(
        doc: "Delete navigation property assignmentDefaults for education",
        name: delete_assignment_defaults,
        path: "/classes/{{RID}}/assignmentDefaults"
    );
    get!(
        doc: "Get educationAssignmentDefaults",
        name: get_assignment_defaults,
        path: "/classes/{{RID}}/assignmentDefaults"
    );
    patch!(
        doc: "Update educationAssignmentDefaults",
        name: update_assignment_defaults,
        path: "/classes/{{RID}}/assignmentDefaults",
        body: true
    );
    delete!(
        doc: "Delete navigation property assignmentSettings for education",
        name: delete_assignment_settings,
        path: "/classes/{{RID}}/assignmentSettings"
    );
    get!(
        doc: "Get assignmentSettings from education",
        name: get_assignment_settings,
        path: "/classes/{{RID}}/assignmentSettings"
    );
    patch!(
        doc: "Update educationAssignmentSettings",
        name: update_assignment_settings,
        path: "/classes/{{RID}}/assignmentSettings",
        body: true
    );
    get!(
        doc: "Get group from education",
        name: get_group,
        path: "/classes/{{RID}}/group"
    );
    get!(
        doc: "List members of an educationClass",
        name: list_members,
        path: "/classes/{{RID}}/members"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_members_count,
        path: "/classes/{{RID}}/members/$count"
    );
    post!(
        doc: "Create new navigation property ref to members for education",
        name: create_ref_members,
        path: "/classes/{{RID}}/members/$ref",
        body: true
    );
    get!(
        doc: "List members of an educationClass",
        name: list_ref_members,
        path: "/classes/{{RID}}/members/$ref"
    );
    delete!(
        doc: "Delete ref of navigation property members for education",
        name: delete_ref_members,
        path: "/classes/{{RID}}/members/{{id}}/$ref",
        params: education_user_id
    );
    get!(
        doc: "List schools",
        name: list_schools,
        path: "/classes/{{RID}}/schools"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_schools_count,
        path: "/classes/{{RID}}/schools/$count"
    );
    get!(
        doc: "Get schools from education",
        name: get_schools,
        path: "/classes/{{RID}}/schools/{{id}}",
        params: education_school_id
    );
    get!(
        doc: "List teachers",
        name: list_teachers,
        path: "/classes/{{RID}}/teachers"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_teachers_count,
        path: "/classes/{{RID}}/teachers/$count"
    );
    post!(
        doc: "Create new navigation property ref to teachers for education",
        name: create_ref_teachers,
        path: "/classes/{{RID}}/teachers/$ref",
        body: true
    );
    get!(
        doc: "List teachers",
        name: list_ref_teachers,
        path: "/classes/{{RID}}/teachers/$ref"
    );
    delete!(
        doc: "Delete ref of navigation property teachers for education",
        name: delete_ref_teachers,
        path: "/classes/{{RID}}/teachers/{{id}}/$ref",
        params: education_user_id
    );
}
