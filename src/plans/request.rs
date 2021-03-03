use crate::buckets::{BucketRequest, BucketsRequest};
use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::tasks::{TaskRequest, TasksRequest};
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(PlanRequest,);
register_client!(PlansRequest, ());

impl<'a, Client> PlanRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> PlansRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Plans);
        PlansRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get plans from planner",
        name: list_plans,
        response: serde_json::Value,
        path: "/plans",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to plans for planner",
        name: create_plans,
        response: serde_json::Value,
        path: "/plans",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> PlansRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn buckets(&self) -> BucketRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        BucketRequest::new(self.client)
    }
    pub fn bucket<ID: AsRef<str>>(&self, id: ID) -> BucketsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Buckets);
        BucketsRequest::new(id.as_ref(), self.client)
    }
    pub fn tasks(&self) -> TaskRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        TaskRequest::new(self.client)
    }
    pub fn task<ID: AsRef<str>>(&self, id: ID) -> TasksRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Tasks);
        TasksRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get plans from planner",
        name: get_plans,
        response: serde_json::Value,
        path: "/plans/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property plans in planner",
        name: update_plans,
        response: NoContent,
        path: "/plans/{{RID}}",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get buckets from planner",
        name: list_buckets,
        response: serde_json::Value,
        path: "/plans/{{RID}}/buckets",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to buckets for planner",
        name: create_buckets,
        response: serde_json::Value,
        path: "/plans/{{RID}}/buckets",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get details from planner",
        name: get_details,
        response: serde_json::Value,
        path: "/plans/{{RID}}/details",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property details in planner",
        name: update_details,
        response: NoContent,
        path: "/plans/{{RID}}/details",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get tasks from planner",
        name: list_tasks,
        response: serde_json::Value,
        path: "/plans/{{RID}}/tasks",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to tasks for planner",
        name: create_tasks,
        response: serde_json::Value,
        path: "/plans/{{RID}}/tasks",
        params: 0,
        has_body: true
    });
}
