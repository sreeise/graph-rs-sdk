use crate::client::Graph;

use graph_http::types::NoContent;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(ClassesRequest,);
register_client!(EducationRequest,);
register_client!(EducationRootRequest,);
register_client!(MeRequest,);
register_client!(SchoolsRequest,);
register_client!(UsersRequest,);

impl<'a, Client> ClassesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get group from education",
        name: get_group,
        response: serde_json::Value,
        path: "/education/classes/{{id}}/group",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get members from education",
        name: list_members,
        response: serde_json::Value,
        path: "/education/classes/{{id}}/members",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get members from education",
        name: get_members,
        response: serde_json::Value,
        path: "/education/classes/{{id}}/members/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get schools from education",
        name: list_schools,
        response: serde_json::Value,
        path: "/education/classes/{{id}}/schools",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get schools from education",
        name: get_schools,
        response: serde_json::Value,
        path: "/education/classes/{{id}}/schools/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get teachers from education",
        name: list_teachers,
        response: serde_json::Value,
        path: "/education/classes/{{id}}/teachers",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get teachers from education",
        name: get_teachers,
        response: serde_json::Value,
        path: "/education/classes/{{id}}/teachers/{{id2}}",
        params: 2,
        has_body: false
    });
}

impl<'a, Client> EducationRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn classes(&self) -> ClassesRequest<'a, Client> {
        ClassesRequest::new(self.client)
    }
    pub fn education_root(&self) -> EducationRootRequest<'a, Client> {
        EducationRootRequest::new(self.client)
    }
    pub fn me(&self) -> MeRequest<'a, Client> {
        MeRequest::new(self.client)
    }
    pub fn schools(&self) -> SchoolsRequest<'a, Client> {
        SchoolsRequest::new(self.client)
    }
    pub fn users(&self) -> UsersRequest<'a, Client> {
        UsersRequest::new(self.client)
    }
    get!({
        doc: "# Get classes from education",
        name: list_classes,
        response: serde_json::Value,
        path: "/education/classes",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to classes for education",
        name: create_classes,
        response: serde_json::Value,
        path: "/education/classes",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get classes from education",
        name: get_classes,
        response: serde_json::Value,
        path: "/education/classes/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property classes in education",
        name: update_classes,
        response: NoContent,
        path: "/education/classes/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get me from education",
        name: get_me,
        response: serde_json::Value,
        path: "/education/me",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property me in education",
        name: update_me,
        response: NoContent,
        path: "/education/me",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get schools from education",
        name: list_schools,
        response: serde_json::Value,
        path: "/education/schools",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to schools for education",
        name: create_schools,
        response: serde_json::Value,
        path: "/education/schools",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get schools from education",
        name: get_schools,
        response: serde_json::Value,
        path: "/education/schools/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property schools in education",
        name: update_schools,
        response: NoContent,
        path: "/education/schools/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get users from education",
        name: list_users,
        response: serde_json::Value,
        path: "/education/users",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to users for education",
        name: create_users,
        response: serde_json::Value,
        path: "/education/users",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get users from education",
        name: get_users,
        response: serde_json::Value,
        path: "/education/users/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property users in education",
        name: update_users,
        response: NoContent,
        path: "/education/users/{{id}}",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> EducationRootRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get education",
        name: get_education_root,
        response: serde_json::Value,
        path: "/education",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update education",
        name: update_education_root,
        response: NoContent,
        path: "/education",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> MeRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get classes from education",
        name: list_classes,
        response: serde_json::Value,
        path: "/education/me/classes",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get classes from education",
        name: get_classes,
        response: serde_json::Value,
        path: "/education/me/classes/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get schools from education",
        name: list_schools,
        response: serde_json::Value,
        path: "/education/me/schools",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get schools from education",
        name: get_schools,
        response: serde_json::Value,
        path: "/education/me/schools/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get user from education",
        name: get_user,
        response: serde_json::Value,
        path: "/education/me/user",
        params: 0,
        has_body: false
    });
}

impl<'a, Client> SchoolsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get classes from education",
        name: list_classes,
        response: serde_json::Value,
        path: "/education/schools/{{id}}/classes",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get classes from education",
        name: get_classes,
        response: serde_json::Value,
        path: "/education/schools/{{id}}/classes/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get users from education",
        name: list_users,
        response: serde_json::Value,
        path: "/education/schools/{{id}}/users",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get users from education",
        name: get_users,
        response: serde_json::Value,
        path: "/education/schools/{{id}}/users/{{id2}}",
        params: 2,
        has_body: false
    });
}

impl<'a, Client> UsersRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get classes from education",
        name: list_classes,
        response: serde_json::Value,
        path: "/education/users/{{id}}/classes",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get classes from education",
        name: get_classes,
        response: serde_json::Value,
        path: "/education/users/{{id}}/classes/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get schools from education",
        name: list_schools,
        response: serde_json::Value,
        path: "/education/users/{{id}}/schools",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get schools from education",
        name: get_schools,
        response: serde_json::Value,
        path: "/education/users/{{id}}/schools/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get user from education",
        name: get_user,
        response: serde_json::Value,
        path: "/education/users/{{id}}/user",
        params: 1,
        has_body: false
    });
}
