// GENERATED CODE

use crate::api_default_imports::*;
use crate::tasks::{TasksIdRequest, TasksRequest};
use graph_http::types::NoContent;

register_client!(BucketsRequest,);
register_client!(BucketsIdRequest, ());

impl<'a, Client> BucketsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, buckets_id: ID) -> BucketsIdRequest<'a, Client> {
        BucketsIdRequest::new(buckets_id.as_ref(), self.client)
    }

    get!({
        doc: "Get buckets from planner",
        name: list_buckets,
        response: serde_json::Value,
        path: "/buckets",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to buckets for planner",
        name: create_buckets,
        response: serde_json::Value,
        path: "/buckets",
        has_body: true
    });
}

impl<'a, Client> BucketsIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn tasks(&self) -> TasksRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Buckets);
        TasksRequest::new(self.client)
    }

    pub fn task<ID: AsRef<str>>(&self, id: ID) -> TasksIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Buckets);
        TasksIdRequest::new(id.as_ref(), self.client)
    }

    patch!({
        doc: "Update the navigation property buckets in planner",
        name: update_buckets,
        response: NoContent,
        path: "/buckets/{{RID}}",
        has_body: true
    });
    get!({
        doc: "Get buckets from planner",
        name: get_buckets,
        response: serde_json::Value,
        path: "/buckets/{{RID}}",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property buckets for planner",
        name: delete_buckets,
        response: NoContent,
        path: "/buckets/{{RID}}",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to tasks for planner",
        name: create_tasks,
        response: serde_json::Value,
        path: "/buckets/{{RID}}/tasks",
        has_body: true
    });
    get!({
        doc: "Get tasks from planner",
        name: list_tasks,
        response: serde_json::Value,
        path: "/buckets/{{RID}}/tasks",
        has_body: false
    });
}
