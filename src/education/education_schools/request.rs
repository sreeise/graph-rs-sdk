// GENERATED CODE

use crate::api_default_imports::*;
use crate::education::*;

resource_api_client!(
    EducationSchoolsApiClient,
    EducationSchoolsIdApiClient,
    ResourceIdentity::EducationSchools
);

impl EducationSchoolsApiClient {
    post!(
        doc: "Create educationSchool",
        name: create_schools,
        path: "/schools",
        body: true
    );
    get!(
        doc: "List educationSchools",
        name: list_schools,
        path: "/schools"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_schools_count,
        path: "/schools/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/schools/delta()"
    );
}

impl EducationSchoolsIdApiClient {
    api_client_link!(assignments, EducationAssignmentsApiClient);
    api_client_link_id!(assignment, EducationAssignmentsIdApiClient);

    delete!(
        doc: "Delete navigation property schools for education",
        name: delete_schools,
        path: "/schools/{{RID}}"
    );
    get!(
        doc: "Get schools from education",
        name: get_schools,
        path: "/schools/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property schools in education",
        name: update_schools,
        path: "/schools/{{RID}}",
        body: true
    );
    get!(
        doc: "List administrativeUnit an educationSchool",
        name: get_administrative_unit,
        path: "/schools/{{RID}}/administrativeUnit"
    );
    patch!(
        doc: "Update the navigation property administrativeUnit in education",
        name: update_administrative_unit,
        path: "/schools/{{RID}}/administrativeUnit",
        body: true
    );
    get!(
        doc: "List classes of an educationSchool",
        name: list_classes,
        path: "/schools/{{RID}}/classes"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_classes_count,
        path: "/schools/{{RID}}/classes/$count"
    );
    post!(
        doc: "Create new navigation property ref to classes for education",
        name: create_ref_classes,
        path: "/schools/{{RID}}/classes/$ref",
        body: true
    );
    get!(
        doc: "List classes of an educationSchool",
        name: list_ref_classes,
        path: "/schools/{{RID}}/classes/$ref"
    );
    delete!(
        doc: "Delete ref of navigation property classes for education",
        name: delete_ref_classes,
        path: "/schools/{{RID}}/classes/{{id}}/$ref",
        params: education_class_id
    );
    get!(
        doc: "List users of an educationSchool",
        name: list_users,
        path: "/schools/{{RID}}/users"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_users_count,
        path: "/schools/{{RID}}/users/$count"
    );
    post!(
        doc: "Create new navigation property ref to users for education",
        name: create_ref_users,
        path: "/schools/{{RID}}/users/$ref",
        body: true
    );
    get!(
        doc: "List users of an educationSchool",
        name: list_ref_users,
        path: "/schools/{{RID}}/users/$ref"
    );
    delete!(
        doc: "Delete ref of navigation property users for education",
        name: delete_ref_users,
        path: "/schools/{{RID}}/users/{{id}}/$ref",
        params: education_user_id
    );
}
