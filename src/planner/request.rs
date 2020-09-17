use crate::client::Graph;
use graph_http::types::{Collection, Content};
use graph_http::{GraphResponse, IntoResponse};
use reqwest::Method;

register_client!(PlannerRequest,);
register_client!(PlannerPlansRequest,);
register_client!(PlannerTasksRequest,);
register_client!(PlannerBucketsRequest,);

impl<'a, Client> PlannerRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( get_planner, serde_json::Value => "planner" );
    patch!( [ update_planner, serde_json::Value => "planner" ] );

    pub fn buckets(&self) -> PlannerBucketsRequest<'a, Client> {
        PlannerBucketsRequest::new(&self.client)
    }

    pub fn tasks(&self) -> PlannerTasksRequest<'a, Client> {
        PlannerTasksRequest::new(&self.client)
    }

    pub fn plans(&self) -> PlannerPlansRequest<'a, Client> {
        PlannerPlansRequest::new(&self.client)
    }
}

impl<'a, Client> PlannerPlansRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( list_plans, Collection<serde_json::Value> => "planner/plans" );
    post!( [ create_plans, serde_json::Value => "planner/plans" ] );
    get!( | get_plans, serde_json::Value => "planner/plans/{{id}}" );
    patch!( [| update_plans, serde_json::Value => "planner/plans/{{id}}" ] );
    get!( | list_buckets, Collection<serde_json::Value> => "planner/plans/{{id}}/buckets" );
    get!( || get_buckets, serde_json::Value => "planner/plans/{{id}}/buckets/{{id2}}" );
    get!( | get_details, serde_json::Value => "planner/plans/{{id}}/details" );
    patch!( [| update_details, serde_json::Value => "planner/plans/{{id}}/details" ] );
    get!( | list_tasks, Collection<serde_json::Value> => "planner/plans/{{id}}/tasks" );
    get!( || get_tasks, serde_json::Value => "planner/plans/{{id}}/tasks/{{id2}}" );
}

impl<'a, Client> PlannerTasksRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( list_tasks, Collection<serde_json::Value> => "planner/tasks" );
    post!( [ create_tasks, serde_json::Value => "planner/tasks" ] );
    get!( | get_tasks, Collection<serde_json::Value> => "planner/tasks/{{id}}" );
    patch!( [| update_tasks, serde_json::Value => "planner/tasks/{{id}}" ] );
    delete!( | delete_tasks, GraphResponse<Content> => "planner/tasks/{id}" );
    get!( | get_assigned_to_task_board_format, serde_json::Value => "planner/tasks/{{id}}/assignedToTaskBoardFormat" );
    patch!( [| update_assigned_to_task_board_format, serde_json::Value => "planner/tasks/{{id}}/assignedToTaskBoardFormat" ] );
    get!( | get_bucket_task_board_format, serde_json::Value => "planner/tasks/{{id}}/bucketTaskBoardFormat" );
    patch!( [| update_bucket_task_board_format, serde_json::Value => "planner/tasks/{{id}}/bucketTaskBoardFormat" ] );
    get!( | get_progress_task_board_format, serde_json::Value => "planner/tasks/{{id}}/progressTaskBoardFormat" );
    patch!( [| update_progress_task_board_format, serde_json::Value => "planner/tasks/{{id}}/progressTaskBoardFormat" ] );
    get!( | get_details, serde_json::Value => "planner/tasks/{{id}}/details" );
    patch!( [| update_details, serde_json::Value => "planner/tasks/{{id}}/details" ] );
}

impl<'a, Client> PlannerBucketsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( list_buckets, Collection<serde_json::Value> => "planner/buckets" );
    post!( [ create_buckets, serde_json::Value => "planner/buckets" ] );
    get!( | get_buckets, serde_json::Value => "planner/buckets/{{id}}" );
    patch!( [| update_buckets, serde_json::Value => "planner/buckets/{{id}}" ] );
    delete!( | delete_buckets, GraphResponse<Content> => "planner/buckets/{id}" );
    get!( | list_tasks, Collection<serde_json::Value> => "planner/buckets/{{id}}/tasks" );
    get!( || get_tasks, serde_json::Value => "planner/buckets/{{id}}/tasks/{{id2}}" );
}
