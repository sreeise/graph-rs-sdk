use crate::client::Graph;
use crate::core::ResourceIdentity;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(TaskRequest,);
register_client!(TasksRequest, ());

impl<'a, Client> TaskRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> TasksRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Tasks);
        TasksRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get tasks from planner",
        name: list_tasks,
        response: Collection<serde_json::Value>,
        path: "/tasks",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to tasks for planner",
        name: create_tasks,
        response: serde_json::Value,
        path: "/tasks",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> TasksRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get tasks from planner",
        name: get_tasks,
        response: serde_json::Value,
        path: "/tasks/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property tasks in planner",
        name: update_tasks,
        response: GraphResponse<Content>,
        path: "/tasks/{{RID}}",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get assignedToTaskBoardFormat from planner",
        name: get_assigned_to_task_board_format,
        response: serde_json::Value,
        path: "/tasks/{{RID}}/assignedToTaskBoardFormat",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property assignedToTaskBoardFormat in planner",
        name: update_assigned_to_task_board_format,
        response: GraphResponse<Content>,
        path: "/tasks/{{RID}}/assignedToTaskBoardFormat",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get bucketTaskBoardFormat from planner",
        name: get_bucket_task_board_format,
        response: serde_json::Value,
        path: "/tasks/{{RID}}/bucketTaskBoardFormat",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property bucketTaskBoardFormat in planner",
        name: update_bucket_task_board_format,
        response: GraphResponse<Content>,
        path: "/tasks/{{RID}}/bucketTaskBoardFormat",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get details from planner",
        name: get_details,
        response: serde_json::Value,
        path: "/tasks/{{RID}}/details",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property details in planner",
        name: update_details,
        response: GraphResponse<Content>,
        path: "/tasks/{{RID}}/details",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get progressTaskBoardFormat from planner",
        name: get_progress_task_board_format,
        response: serde_json::Value,
        path: "/tasks/{{RID}}/progressTaskBoardFormat",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property progressTaskBoardFormat in planner",
        name: update_progress_task_board_format,
        response: GraphResponse<Content>,
        path: "/tasks/{{RID}}/progressTaskBoardFormat",
        params: 0,
        has_body: true
    });
}
