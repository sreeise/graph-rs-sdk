use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::DeltaPhantom;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(InstanceRequest,);
register_client!(InstancesRequest, ());

impl<'a, Client> InstanceRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> InstancesRequest<'a, Client> {
        InstancesRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get instances from me",
        name: list_instances,
        response: Collection<serde_json::Value>,
        path: "/instances",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to instances for me",
        name: create_instances,
        response: serde_json::Value,
        path: "/instances",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/instances/delta()",
        params: 0,
        has_body: false
    });
}

impl<'a, Client> InstancesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get instances from me",
        name: get_instances,
        response: serde_json::Value,
        path: "/instances/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property instances in me",
        name: update_instances,
        response: NoContent,
        path: "/instances/{{RID}}",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action accept",
        name: accept,
        response: NoContent,
        path: "/instances/{{RID}}/accept",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action decline",
        name: decline,
        response: NoContent,
        path: "/instances/{{RID}}/decline",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action dismissReminder",
        name: dismiss_reminder,
        response: NoContent,
        path: "/instances/{{RID}}/dismissReminder",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action snoozeReminder",
        name: snooze_reminder,
        response: NoContent,
        path: "/instances/{{RID}}/snoozeReminder",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action tentativelyAccept",
        name: tentatively_accept,
        response: NoContent,
        path: "/instances/{{RID}}/tentativelyAccept",
        params: 0,
        has_body: true
    });
}
