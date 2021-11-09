// GENERATED CODE

use crate::api_default_imports::*;
use crate::buckets::{BucketsIdRequest, BucketsRequest};
use crate::plans::{PlansIdRequest, PlansRequest};
use crate::tasks::{TasksIdRequest, TasksRequest};
use graph_http::types::NoContent;

register_client!(PlannerRequest,);

impl<'a, Client> PlannerRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn buckets(&self) -> BucketsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Buckets);
        BucketsRequest::new(self.client)
    }

    pub fn bucket<ID: AsRef<str>>(&self, id: ID) -> BucketsIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Buckets);
        BucketsIdRequest::new(id.as_ref(), self.client)
    }

    pub fn plans(&self) -> PlansRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Plans);
        PlansRequest::new(self.client)
    }

    pub fn plan<ID: AsRef<str>>(&self, id: ID) -> PlansIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Plans);
        PlansIdRequest::new(id.as_ref(), self.client)
    }

    pub fn tasks(&self) -> TasksRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Tasks);
        TasksRequest::new(self.client)
    }

    pub fn task<ID: AsRef<str>>(&self, id: ID) -> TasksIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Tasks);
        TasksIdRequest::new(id.as_ref(), self.client)
    }

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
