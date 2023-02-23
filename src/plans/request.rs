// GENERATED CODE

use crate::api_default_imports::*;
use crate::buckets::{BucketsApiClient, BucketsIdApiClient};
use crate::tasks::{TasksApiClient, TasksIdApiClient};

resource_api_client!(PlansApiClient, PlansIdApiClient, ResourceIdentity::Plans);

impl PlansApiClient {
    post!({
        doc: "Create new navigation property to plans",
        name: create_plans,
        response: serde_json::Value,
        path: "plans",
        has_body: true
    });
    get!({
        doc: "Get plans",
        name: list_plans,
        response: serde_json::Value,
        path: "plans",
        has_body: false
    });
}

impl PlansIdApiClient {
    api_client_link!(buckets, ResourceIdentity::Buckets, BucketsApiClient);
    api_client_link_id!(bucket, ResourceIdentity::Buckets, BucketsIdApiClient);

    api_client_link!(tasks, ResourceIdentity::Tasks, TasksApiClient);
    api_client_link_id!(task, ResourceIdentity::Tasks, TasksIdApiClient);

    patch!({
        doc: "Update the navigation property plans",
        name: update_plans,
        response: NoContent,
        path: "plans/{{RID}}",
        has_body: true
    });
    get!({
        doc: "Get plans",
        name: get_plans,
        response: serde_json::Value,
        path: "plans/{{RID}}",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property plans",
        name: delete_plans,
        response: NoContent,
        path: "plans/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get buckets",
        name: list_buckets,
        response: serde_json::Value,
        path: "plans/{{RID}}/buckets",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to buckets",
        name: create_buckets,
        response: serde_json::Value,
        path: "plans/{{RID}}/buckets",
        has_body: true
    });
    get!({
        doc: "Get details",
        name: get_details,
        response: serde_json::Value,
        path: "plans/{{RID}}/details",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property details",
        name: delete_details,
        response: NoContent,
        path: "plans/{{RID}}/details",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property details",
        name: update_details,
        response: NoContent,
        path: "plans/{{RID}}/details",
        has_body: true
    });
    get!({
        doc: "Get tasks",
        name: list_tasks,
        response: serde_json::Value,
        path: "plans/{{RID}}/tasks",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to tasks",
        name: create_tasks,
        response: serde_json::Value,
        path: "plans/{{RID}}/tasks",
        has_body: true
    });
}
