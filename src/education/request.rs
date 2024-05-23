// GENERATED CODE

use crate::api_default_imports::*;
use crate::education::*;

api_client!(EducationApiClient, ResourceIdentity::Education);

impl EducationApiClient {
    api_client_link!(me, EducationMeApiClient);
    api_client_link_id!(class, EducationClassesIdApiClient);
    api_client_link_id!(school, EducationSchoolsIdApiClient);
    api_client_link!(classes, EducationClassesApiClient);
    api_client_link!(users, EducationUsersApiClient);
    api_client_link_id!(user, EducationUsersIdApiClient);
    api_client_link!(schools, EducationSchoolsApiClient);

    get!(
        doc: "Get education",
        name: get_education_root,
        path: "/education"
    );
    patch!(
        doc: "Update education",
        name: update_education_root,
        path: "/education",
        body: true
    );
}
