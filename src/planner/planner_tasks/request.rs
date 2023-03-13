// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    PlannerTasksApiClient,
    PlannerTasksIdApiClient,
    ResourceIdentity::PlannerTasks
);

impl PlannerTasksApiClient {
    post!(
        doc: "Create plannerTask",
        name: create_tasks,
        path: "/tasks",
        body: true
    );
    get!(
        doc: "List tasks",
        name: list_tasks,
        path: "/tasks"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_tasks_count,
        path: "/tasks/$count"
    );
}

impl PlannerTasksIdApiClient {
    delete!(
        doc: "Delete navigation property tasks for planner",
        name: delete_tasks,
        path: "/tasks/{{RID}}"
    );
    get!(
        doc: "Get tasks from planner",
        name: get_tasks,
        path: "/tasks/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property tasks in planner",
        name: update_tasks,
        path: "/tasks/{{RID}}",
        body: true
    );
    delete!(
        doc: "Delete navigation property assignedToTaskBoardFormat for planner",
        name: delete_assigned_to_task_board_format,
        path: "/tasks/{{RID}}/assignedToTaskBoardFormat"
    );
    get!(
        doc: "Get plannerAssignedToTaskBoardTaskFormat",
        name: get_assigned_to_task_board_format,
        path: "/tasks/{{RID}}/assignedToTaskBoardFormat"
    );
    patch!(
        doc: "Update the navigation property assignedToTaskBoardFormat in planner",
        name: update_assigned_to_task_board_format,
        path: "/tasks/{{RID}}/assignedToTaskBoardFormat",
        body: true
    );
    delete!(
        doc: "Delete navigation property bucketTaskBoardFormat for planner",
        name: delete_bucket_task_board_format,
        path: "/tasks/{{RID}}/bucketTaskBoardFormat"
    );
    get!(
        doc: "Get plannerBucketTaskBoardTaskFormat",
        name: get_bucket_task_board_format,
        path: "/tasks/{{RID}}/bucketTaskBoardFormat"
    );
    patch!(
        doc: "Update the navigation property bucketTaskBoardFormat in planner",
        name: update_bucket_task_board_format,
        path: "/tasks/{{RID}}/bucketTaskBoardFormat",
        body: true
    );
    delete!(
        doc: "Delete navigation property details for planner",
        name: delete_details,
        path: "/tasks/{{RID}}/details"
    );
    get!(
        doc: "Get plannerTaskDetails",
        name: get_details,
        path: "/tasks/{{RID}}/details"
    );
    patch!(
        doc: "Update the navigation property details in planner",
        name: update_details,
        path: "/tasks/{{RID}}/details",
        body: true
    );
    delete!(
        doc: "Delete navigation property progressTaskBoardFormat for planner",
        name: delete_progress_task_board_format,
        path: "/tasks/{{RID}}/progressTaskBoardFormat"
    );
    get!(
        doc: "Get plannerProgressTaskBoardTaskFormat",
        name: get_progress_task_board_format,
        path: "/tasks/{{RID}}/progressTaskBoardFormat"
    );
    patch!(
        doc: "Update the navigation property progressTaskBoardFormat in planner",
        name: update_progress_task_board_format,
        path: "/tasks/{{RID}}/progressTaskBoardFormat",
        body: true
    );
}
