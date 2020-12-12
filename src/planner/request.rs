// GENERATED CODE

use crate::buckets::{BucketRequest, BucketsRequest};
use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::plans::{PlanRequest, PlansRequest};
use crate::tasks::{TaskRequest, TasksRequest};
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(PlannerRequest,);

impl<'a, Client> PlannerRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn buckets(&self) -> BucketRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        BucketRequest::new(self.client)
    }
    pub fn bucket<ID: AsRef<str>>(&self, id: ID) -> BucketsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Buckets);
        BucketsRequest::new(id.as_ref(), self.client)
    }
    pub fn plans(&self) -> PlanRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        PlanRequest::new(self.client)
    }
    pub fn plan<ID: AsRef<str>>(&self, id: ID) -> PlansRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Plans);
        PlansRequest::new(id.as_ref(), self.client)
    }
    pub fn tasks(&self) -> TaskRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        TaskRequest::new(self.client)
    }
    pub fn task<ID: AsRef<str>>(&self, id: ID) -> TasksRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Tasks);
        TasksRequest::new(id.as_ref(), self.client)
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
