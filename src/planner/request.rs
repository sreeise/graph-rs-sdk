use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(BucketsRequest,);
register_client!(PlannerRequest,);
register_client!(PlansRequest,);
register_client!(TasksRequest,);

impl<'a, Client> BucketsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn tasks(&self) -> TasksRequest<'a, Client> {
        TasksRequest::new(self.client)
    }
    get!({
        doc: "# Get tasks from planner",
        name: list_tasks,
        response: Collection<serde_json::Value>,
        path: "/planner/plans/{{id}}/buckets/{{id2}}/tasks",
        params: 2,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to tasks for planner",
        name: create_tasks,
        response: serde_json::Value,
        path: "/planner/plans/{{id}}/buckets/{{id2}}/tasks",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get tasks from planner",
        name: get_tasks,
        response: serde_json::Value,
        path: "/planner/plans/{{id}}/buckets/{{id2}}/tasks/{{id3}}",
        params: 3,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property tasks in planner",
        name: update_tasks,
        response: GraphResponse<Content>,
        path: "/planner/plans/{{id}}/buckets/{{id2}}/tasks/{{id3}}",
        params: 3,
        has_body: true
    });
}

impl<'a, Client> PlannerRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn buckets(&self) -> BucketsRequest<'a, Client> {
        BucketsRequest::new(self.client)
    }
    pub fn plans(&self) -> PlansRequest<'a, Client> {
        PlansRequest::new(self.client)
    }
    pub fn tasks(&self) -> TasksRequest<'a, Client> {
        TasksRequest::new(self.client)
    }
    get!({
        doc: "# Get planner",
        name: get_planner,
        response: serde_json::Value,
        path: "/planner",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update planner",
        name: update_planner,
        response: GraphResponse<Content>,
        path: "/planner",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get buckets from planner",
        name: list_buckets,
        response: Collection<serde_json::Value>,
        path: "/planner/buckets",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to buckets for planner",
        name: create_buckets,
        response: serde_json::Value,
        path: "/planner/buckets",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get buckets from planner",
        name: get_buckets,
        response: serde_json::Value,
        path: "/planner/buckets/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property buckets in planner",
        name: update_buckets,
        response: GraphResponse<Content>,
        path: "/planner/buckets/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get plans from planner",
        name: list_plans,
        response: Collection<serde_json::Value>,
        path: "/planner/plans",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to plans for planner",
        name: create_plans,
        response: serde_json::Value,
        path: "/planner/plans",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get plans from planner",
        name: get_plans,
        response: serde_json::Value,
        path: "/planner/plans/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property plans in planner",
        name: update_plans,
        response: GraphResponse<Content>,
        path: "/planner/plans/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get tasks from planner",
        name: list_tasks,
        response: Collection<serde_json::Value>,
        path: "/planner/tasks",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to tasks for planner",
        name: create_tasks,
        response: serde_json::Value,
        path: "/planner/tasks",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get tasks from planner",
        name: get_tasks,
        response: serde_json::Value,
        path: "/planner/tasks/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property tasks in planner",
        name: update_tasks,
        response: GraphResponse<Content>,
        path: "/planner/tasks/{{id}}",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> PlansRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn buckets(&self) -> BucketsRequest<'a, Client> {
        BucketsRequest::new(self.client)
    }
    pub fn tasks(&self) -> TasksRequest<'a, Client> {
        TasksRequest::new(self.client)
    }
    get!({
        doc: "# Get buckets from planner",
        name: list_buckets,
        response: Collection<serde_json::Value>,
        path: "/planner/plans/{{id}}/buckets",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to buckets for planner",
        name: create_buckets,
        response: serde_json::Value,
        path: "/planner/plans/{{id}}/buckets",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get buckets from planner",
        name: get_buckets,
        response: serde_json::Value,
        path: "/planner/plans/{{id}}/buckets/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property buckets in planner",
        name: update_buckets,
        response: GraphResponse<Content>,
        path: "/planner/plans/{{id}}/buckets/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get details from planner",
        name: get_details,
        response: serde_json::Value,
        path: "/planner/plans/{{id}}/details",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property details in planner",
        name: update_details,
        response: GraphResponse<Content>,
        path: "/planner/plans/{{id}}/details",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get tasks from planner",
        name: list_tasks,
        response: Collection<serde_json::Value>,
        path: "/planner/plans/{{id}}/tasks",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to tasks for planner",
        name: create_tasks,
        response: serde_json::Value,
        path: "/planner/plans/{{id}}/tasks",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get tasks from planner",
        name: get_tasks,
        response: serde_json::Value,
        path: "/planner/plans/{{id}}/tasks/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property tasks in planner",
        name: update_tasks,
        response: GraphResponse<Content>,
        path: "/planner/plans/{{id}}/tasks/{{id2}}",
        params: 2,
        has_body: true
    });
}

impl<'a, Client> TasksRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get assignedToTaskBoardFormat from planner",
        name: get_assigned_to_task_board_format,
        response: serde_json::Value,
        path: "/planner/plans/{{id}}/buckets/{{id2}}/tasks/{{id3}}/assignedToTaskBoardFormat",
        params: 3,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property assignedToTaskBoardFormat in planner",
        name: update_assigned_to_task_board_format,
        response: GraphResponse<Content>,
        path: "/planner/plans/{{id}}/buckets/{{id2}}/tasks/{{id3}}/assignedToTaskBoardFormat",
        params: 3,
        has_body: true
    });
    get!({
        doc: "# Get bucketTaskBoardFormat from planner",
        name: get_bucket_task_board_format,
        response: serde_json::Value,
        path: "/planner/plans/{{id}}/buckets/{{id2}}/tasks/{{id3}}/bucketTaskBoardFormat",
        params: 3,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property bucketTaskBoardFormat in planner",
        name: update_bucket_task_board_format,
        response: GraphResponse<Content>,
        path: "/planner/plans/{{id}}/buckets/{{id2}}/tasks/{{id3}}/bucketTaskBoardFormat",
        params: 3,
        has_body: true
    });
    get!({
        doc: "# Get details from planner",
        name: get_details,
        response: serde_json::Value,
        path: "/planner/plans/{{id}}/buckets/{{id2}}/tasks/{{id3}}/details",
        params: 3,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property details in planner",
        name: update_details,
        response: GraphResponse<Content>,
        path: "/planner/plans/{{id}}/buckets/{{id2}}/tasks/{{id3}}/details",
        params: 3,
        has_body: true
    });
    get!({
        doc: "# Get progressTaskBoardFormat from planner",
        name: get_progress_task_board_format,
        response: serde_json::Value,
        path: "/planner/plans/{{id}}/buckets/{{id2}}/tasks/{{id3}}/progressTaskBoardFormat",
        params: 3,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property progressTaskBoardFormat in planner",
        name: update_progress_task_board_format,
        response: GraphResponse<Content>,
        path: "/planner/plans/{{id}}/buckets/{{id2}}/tasks/{{id3}}/progressTaskBoardFormat",
        params: 3,
        has_body: true
    });
}
