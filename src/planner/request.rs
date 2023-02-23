// GENERATED CODE

use crate::api_default_imports::*;
use crate::buckets::{BucketsApiClient, BucketsIdApiClient};
use crate::plans::{PlansApiClient, PlansIdApiClient};
use crate::tasks::{TasksApiClient, TasksIdApiClient};

resource_api_client!(PlannerApiClient);

impl PlannerApiClient {
    api_client_link!(buckets, ResourceIdentity::Buckets, BucketsApiClient);
    api_client_link_id!(bucket, ResourceIdentity::Buckets, BucketsIdApiClient);

    api_client_link!(plans, ResourceIdentity::Plans, PlansApiClient);
    api_client_link_id!(plan, ResourceIdentity::Plans, PlansIdApiClient);

    api_client_link!(tasks, ResourceIdentity::Tasks, TasksApiClient);
    api_client_link_id!(task, ResourceIdentity::Tasks, TasksIdApiClient);

    get!({
        doc: "Get planner",
        name: get_planner,
        response: serde_json::Value,
        path: "/planner",
        has_body: false
    });
    patch!({
        doc: "Update planner",
        name: update_planner,
        response: NoContent,
        path: "/planner",
        has_body: true
    });
    get!({
        doc: "Get buckets from planner",
        name: list_buckets,
        response: serde_json::Value,
        path: "/planner/buckets",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to buckets for planner",
        name: create_buckets,
        response: serde_json::Value,
        path: "/planner/buckets",
        has_body: true
    });
    post!({
        doc: "Create new navigation property to plans for planner",
        name: create_plans,
        response: serde_json::Value,
        path: "/planner/plans",
        has_body: true
    });
    get!({
        doc: "Get plans from planner",
        name: list_plans,
        response: serde_json::Value,
        path: "/planner/plans",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to tasks for planner",
        name: create_tasks,
        response: serde_json::Value,
        path: "/planner/tasks",
        has_body: true
    });
    get!({
        doc: "Get tasks from planner",
        name: list_tasks,
        response: serde_json::Value,
        path: "/planner/tasks",
        has_body: false
    });
}
