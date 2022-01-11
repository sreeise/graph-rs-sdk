// GENERATED CODE

use crate::api_default_imports::*;
use crate::buckets::{BucketsIdRequest, BucketsRequest};
use crate::tasks::{TasksIdRequest, TasksRequest};
use graph_http::types::NoContent;

register_client!(PlansRequest,);
register_client!(PlansIdRequest, ());

impl<'a, Client> PlansRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
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

impl<'a, Client> PlansIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn buckets(&self) -> BucketsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Buckets);
        BucketsRequest::new(self.client)
    }

    pub fn bucket<ID: AsRef<str>>(&self, id: ID) -> BucketsIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Buckets);
        BucketsIdRequest::new(id.as_ref(), self.client)
    }

    pub fn tasks(&self) -> TasksRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Tasks);
        TasksRequest::new(self.client)
    }

    pub fn task<ID: AsRef<str>>(&self, id: ID) -> TasksIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Tasks);
        TasksIdRequest::new(id.as_ref(), self.client)
    }

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
