use crate::client::Graph;
use crate::http::{GraphResponse, IntoResponse};
use crate::types::{collection::Collection, content::Content};
use handlebars::*;
use reqwest::Method;

register_client!(EducationRequest,);
register_client!(EducationClassesRequest,
    edc => "education/classes",
);
register_client!(EducationUsersRequest,);
register_client!(EducationMeRequest,);
register_client!(EducationSchoolsRequest,
    eds => "education/schools",
);

impl<'a, Client> EducationRequest<'a, Client>
where
    Client: crate::http::RequestClient,
{
    get!( get_education_root, serde_json::Value => "/education");
    patch!( [ update_education_root, serde_json::Value => "/education" ] );
    get!( list_users, Collection<serde_json::Value> => "education/users" );
    get!( list_schools, Collection<serde_json::Value> => "{{eds}}" );
    get!( list_classes, Collection<serde_json::Value> => "{{edc}}");

    pub fn classes(&self) -> EducationClassesRequest<'a, Client> {
        EducationClassesRequest::new(&self.client)
    }

    pub fn schools(&self) -> EducationSchoolsRequest<'a, Client> {
        EducationSchoolsRequest::new(&self.client)
    }
}

impl<'a, Client> EducationClassesRequest<'a, Client>
where
    Client: crate::http::RequestClient,
{
    get!( list_classes, Collection<serde_json::Value> => "{{edc}}");
    post!( [ create_class, serde_json::Value => "{{edc}}" ] );
    get!( | get_class, serde_json::Value => "{{edc}}/{{id}}" );
    patch!( [| update_class, serde_json::Value => "{{edc}}/{{id}}" ] );
    delete!( | delete_class, GraphResponse<Content> => "{{edc}}/{{id}}" );
    get!( | get_group, serde_json::Value => "{{edc}}/{{id}}/group" );
    get!( | list_members, Collection<serde_json::Value> => "{{edc}}/{{id}}/members" );
    get!( || get_member, serde_json::Value => "{{edc}}/{{id}}/members/{{id2}}" );
    post!( [| add_member, serde_json::Value => "{{edc}}/{{id}}/members/$ref" ] );
    delete!( || remove_member, GraphResponse<Content> => "{{edc}}/{{id}}/members/{{id2}}/$ref" );
    get!( | list_schools, Collection<serde_json::Value> => "{{edc}}/{{id}}/schools" );
    get!( || get_school, serde_json::Value => "{{edc}}/{{id}}/schools/{{id2}}" );
    get!( | list_teachers, Collection<serde_json::Value> => "{{edc}}/{{id}}/teachers" );
    get!( || get_teacher, serde_json::Value => "{{edc}}/{{id}}/teachers/{{id2}}" );
    post!( [| add_teacher, serde_json::Value => "{{edc}}/{{id}}/teachers/$ref" ] );
    delete!( || remove_teacher, GraphResponse<Content> => "{{edc}}/{{id}}/teachers/{{id2}}/$ref" );
}

impl<'a, Client> EducationSchoolsRequest<'a, Client>
where
    Client: crate::http::RequestClient,
{
    get!( | list_classes, Collection<serde_json::Value> => "{{eds}}/{{id}}/classes" );
    get!( || get_class, serde_json::Value => "{{eds}}/{{id}}/classes/{{id2}}" );
    delete!( || remove_class, GraphResponse<Content> => "{{eds}}/{{id}}/classes/{{id2}}/$ref" );
    get!( | list_users, Collection<serde_json::Value> => "{{eds}}/{{id}}/users" );
    get!( || get_user, serde_json::Value => "{{eds}}/{{id}}/users/{{id2}}" );
    post!( [| create_user, serde_json::Value => "{{eds}}/{{id}}/users/$ref" ] );
    get!( list_schools, Collection<serde_json::Value> => "{{eds}}" );
    post!( [ create_school, serde_json::Value => "{{eds}}" ] );
    get!( | get_school, serde_json::Value => "{{eds}}/{{id}}" );
    patch!( [| update_school, serde_json::Value => "{{eds}}/{{id}}" ] );
    delete!( | delete_school, GraphResponse<Content> => "{{eds}}/{{id}}" );
}

impl<'a, Client> EducationUsersRequest<'a, Client>
where
    Client: crate::http::RequestClient,
{
    get!( list_classes, Collection<serde_json::Value> => "education/users/{{RID}}/classes" );
    get!( | get_class, serde_json::Value => "education/users/{{RID}}/classes/{{id2}}" );
    get!( list_schools, Collection<serde_json::Value> => "education/users/{{RID}}/schools" );
    get!( | get_school, serde_json::Value => "education/users/{{RID}}/schools/{{id2}}" );
    get!( get_user, serde_json::Value => "education/users/{{RID}/user" );
    get!( list_users, Collection<serde_json::Value> => "education/users" );
    post!( [ create_user, serde_json::Value => "education/users" ] );
    get!( get_users, serde_json::Value => "education/users/{{RID}}" );
    patch!( [ update_user, serde_json::Value => "education/users/{{RID}}" ] );
}

impl<'a, Client> EducationMeRequest<'a, Client>
where
    Client: crate::http::RequestClient,
{
    get!( get_me, serde_json::Value => "education/me" );
    patch!( [ update_me, serde_json::Value => "education/me" ] );
    get!( list_classes, Collection<serde_json::Value> => "education/me/classes" );
    get!( | get_class, serde_json::Value => "education/me/classes/{{id}}" );
    get!( list_schools, Collection<serde_json::Value> => "education/me/schools" );
    get!( | get_school, serde_json::Value => "education/me/schools/{{id}}" );
    get!( get_user, serde_json::Value => "education/me/user" );
}
