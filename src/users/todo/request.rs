// GENERATED CODE

use crate::api_default_imports::*;
use crate::users::*;

resource_api_client!(TodoApiClient, ResourceIdentity::Todo);

impl TodoApiClient {
    api_client_link!(lists, TodoListsApiClient);
    api_client_link_id!(list, TodoListsIdApiClient);

    delete!(
        doc: "Delete navigation property todo for users",
        name: delete_todo,
        path: "/todo"
    );
    get!(
        doc: "Get todo from users",
        name: get_todo,
        path: "/todo"
    );
    patch!(
        doc: "Update the navigation property todo in users",
        name: update_todo,
        path: "/todo",
        body: true
    );
}
