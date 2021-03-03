// GENERATED CODE

use crate::call_records::{CallRecordRequest, CallRecordsRequest};
use crate::calls::{CallRequest, CallsRequest};
use crate::client::Graph;
use crate::core::ResourceIdentity;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(CloudCommunicationsRequest,);
register_client!(CommunicationsRequest,);
register_client!(OnlineMeetingsRequest,);

impl<'a, Client> CloudCommunicationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get communications",
        name: get_cloud_communications,
        response: serde_json::Value,
        path: "/communications",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update communications",
        name: update_cloud_communications,
        response: NoContent,
        path: "/communications",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> CommunicationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn calls(&self) -> CallRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        CallRequest::new(self.client)
    }
    pub fn call_records(&self) -> CallRecordRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        CallRecordRequest::new(self.client)
    }
    pub fn call_record<ID: AsRef<str>>(&self, id: ID) -> CallRecordsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::CallRecords);
        CallRecordsRequest::new(id.as_ref(), self.client)
    }
    pub fn call<ID: AsRef<str>>(&self, id: ID) -> CallsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Calls);
        CallsRequest::new(id.as_ref(), self.client)
    }
    pub fn cloud_communications(&self) -> CloudCommunicationsRequest<'a, Client> {
        CloudCommunicationsRequest::new(self.client)
    }
    pub fn online_meetings(&self) -> OnlineMeetingsRequest<'a, Client> {
        OnlineMeetingsRequest::new(self.client)
    }
    get!({
        doc: "# Get callRecords from communications",
        name: list_call_records,
        response: serde_json::Value,
        path: "/communications/callRecords",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to callRecords for communications",
        name: create_call_records,
        response: serde_json::Value,
        path: "/communications/callRecords",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get callRecords from communications",
        name: get_call_records,
        response: serde_json::Value,
        path: "/communications/callRecords/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property callRecords in communications",
        name: update_call_records,
        response: NoContent,
        path: "/communications/callRecords/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get calls from communications",
        name: list_calls,
        response: serde_json::Value,
        path: "/communications/calls",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to calls for communications",
        name: create_calls,
        response: serde_json::Value,
        path: "/communications/calls",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get calls from communications",
        name: get_calls,
        response: serde_json::Value,
        path: "/communications/calls/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calls in communications",
        name: update_calls,
        response: NoContent,
        path: "/communications/calls/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get onlineMeetings from communications",
        name: list_online_meetings,
        response: serde_json::Value,
        path: "/communications/onlineMeetings",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to onlineMeetings for communications",
        name: create_online_meetings,
        response: serde_json::Value,
        path: "/communications/onlineMeetings",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get onlineMeetings from communications",
        name: get_online_meetings,
        response: serde_json::Value,
        path: "/communications/onlineMeetings/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property onlineMeetings in communications",
        name: update_online_meetings,
        response: NoContent,
        path: "/communications/onlineMeetings/{{id}}",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> OnlineMeetingsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action createOrGet",
        name: create_or_get,
        response: serde_json::Value,
        path: "/communications/onlineMeetings/createOrGet",
        params: 0,
        has_body: true
    });
}
