// GENERATED CODE

use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::tasks::{TaskRequest, TasksRequest};
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(BucketRequest,);
register_client!(BucketsRequest, ());

impl<'a, Client> BucketRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get buckets from planner",
        name: list_buckets,
        response: serde_json::Value,
        path: "/buckets",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to buckets for planner",
        name: create_buckets,
        response: serde_json::Value,
        path: "/buckets",
        params: 0,
        has_body: true
    });

    pub fn id<ID: AsRef<str>>(&self, id: ID) -> BucketsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Buckets);
        BucketsRequest::new(id.as_ref(), self.client)
    }
}

impl<'a, Client> BucketsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get buckets from planner",
        name: get_buckets,
        response: serde_json::Value,
        path: "/buckets/{{RID}}",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property buckets in planner",
        name: update_buckets,
        response: NoContent,
        path: "/buckets/{{RID}}",
        params: 0,
        has_body: true
    });

    delete!({
        doc: "# Delete navigation property buckets for planner",
        name: delete_buckets,
        response: NoContent,
        path: "/buckets/{{RID}}",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get tasks from planner",
        name: list_tasks,
        response: serde_json::Value,
        path: "/buckets/{{RID}}/tasks",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to tasks for planner",
        name: create_tasks,
        response: serde_json::Value,
        path: "/buckets/{{RID}}/tasks",
        params: 0,
        has_body: true
    });

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
}
