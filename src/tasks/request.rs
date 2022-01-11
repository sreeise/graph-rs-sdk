// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(TasksRequest,);
register_client!(TasksIdRequest, ());

impl<'a, Client> TasksRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Create new navigation property to tasks",
        name: create_tasks,
        response: serde_json::Value,
        path: "tasks",
        has_body: true
    });
    get!({
        doc: "Get tasks",
        name: list_tasks,
        response: serde_json::Value,
        path: "tasks",
        has_body: false
    });
}

impl<'a, Client> TasksIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    delete!({
        doc: "Delete navigation property tasks",
        name: delete_tasks,
        response: NoContent,
        path: "tasks/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property tasks",
        name: update_tasks,
        response: NoContent,
        path: "tasks/{{RID}}",
        has_body: true
    });
    get!({
        doc: "Get tasks",
        name: get_tasks,
        response: serde_json::Value,
        path: "tasks/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property assignedToTaskBoardFormat",
        name: update_assigned_to_task_board_format,
        response: NoContent,
        path: "tasks/{{RID}}/assignedToTaskBoardFormat",
        has_body: true
    });
    delete!({
        doc: "Delete navigation property assignedToTaskBoardFormat",
        name: delete_assigned_to_task_board_format,
        response: NoContent,
        path: "tasks/{{RID}}/assignedToTaskBoardFormat",
        has_body: false
    });
    get!({
        doc: "Get assignedToTaskBoardFormat",
        name: get_assigned_to_task_board_format,
        response: serde_json::Value,
        path: "tasks/{{RID}}/assignedToTaskBoardFormat",
        has_body: false
    });
    get!({
        doc: "Get bucketTaskBoardFormat",
        name: get_bucket_task_board_format,
        response: serde_json::Value,
        path: "tasks/{{RID}}/bucketTaskBoardFormat",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property bucketTaskBoardFormat",
        name: update_bucket_task_board_format,
        response: NoContent,
        path: "tasks/{{RID}}/bucketTaskBoardFormat",
        has_body: true
    });
    delete!({
        doc: "Delete navigation property bucketTaskBoardFormat",
        name: delete_bucket_task_board_format,
        response: NoContent,
        path: "tasks/{{RID}}/bucketTaskBoardFormat",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property details",
        name: update_details,
        response: NoContent,
        path: "tasks/{{RID}}/details",
        has_body: true
    });
    get!({
        doc: "Get details",
        name: get_details,
        response: serde_json::Value,
        path: "tasks/{{RID}}/details",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property details",
        name: delete_details,
        response: NoContent,
        path: "tasks/{{RID}}/details",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property progressTaskBoardFormat",
        name: delete_progress_task_board_format,
        response: NoContent,
        path: "tasks/{{RID}}/progressTaskBoardFormat",
        has_body: false
    });
    get!({
        doc: "Get progressTaskBoardFormat",
        name: get_progress_task_board_format,
        response: serde_json::Value,
        path: "tasks/{{RID}}/progressTaskBoardFormat",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property progressTaskBoardFormat",
        name: update_progress_task_board_format,
        response: NoContent,
        path: "tasks/{{RID}}/progressTaskBoardFormat",
        has_body: true
    });
}
