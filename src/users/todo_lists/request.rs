// GENERATED CODE

use crate::api_default_imports::*;
use crate::users::*;

api_client!(
    TodoListsApiClient,
    TodoListsIdApiClient,
    ResourceIdentity::TodoLists
);

impl TodoListsApiClient {
    post!(
        doc: "Create todoTaskList",
        name: create_lists,
        path: "/lists",
        body: true
    );
    get!(
        doc: "List lists",
        name: list_lists,
        path: "/lists"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_lists_count,
        path: "/lists/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/lists/delta()"
    );
}

impl TodoListsIdApiClient {
    api_client_link!(tasks, TodoListsTasksApiClient);
    api_client_link_id!(task, TodoListsTasksIdApiClient);

    delete!(
        doc: "Delete navigation property lists for users",
        name: delete_lists,
        path: "/lists/{{RID}}"
    );
    get!(
        doc: "Get lists from users",
        name: get_lists,
        path: "/lists/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property lists in users",
        name: update_lists,
        path: "/lists/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to extensions for users",
        name: create_extensions,
        path: "/lists/{{RID}}/extensions",
        body: true
    );
    get!(
        doc: "Get extensions from users",
        name: list_extensions,
        path: "/lists/{{RID}}/extensions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_extensions_count,
        path: "/lists/{{RID}}/extensions/$count"
    );
    delete!(
        doc: "Delete navigation property extensions for users",
        name: delete_extensions,
        path: "/lists/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    get!(
        doc: "Get extensions from users",
        name: get_extensions,
        path: "/lists/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    patch!(
        doc: "Update the navigation property extensions in users",
        name: update_extensions,
        path: "/lists/{{RID}}/extensions/{{id}}",
        body: true,
        params: extension_id
    );
}
