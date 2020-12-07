// GENERATED CODE

use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::sessions::{SessionRequest, SessionsRequest};
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(CallRecordRequest,);
register_client!(CallRecordsRequest, ());

impl<'a, Client> CallRecordRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> CallRecordsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::CallRecords);
        CallRecordsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get callRecords from communications",
        name: list_call_records,
        response: Collection<serde_json::Value>,
        path: "/callRecords",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to callRecords for communications",
        name: create_call_records,
        response: serde_json::Value,
        path: "/callRecords",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> CallRecordsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn sessions(&self) -> SessionRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        SessionRequest::new(self.client)
    }
    pub fn session<ID: AsRef<str>>(&self, id: ID) -> SessionsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Sessions);
        SessionsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get callRecords from communications",
        name: get_call_records,
        response: serde_json::Value,
        path: "/callRecords/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property callRecords in communications",
        name: update_call_records,
        response: GraphResponse<Content>,
        path: "/callRecords/{{RID}}",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get sessions from communications",
        name: list_sessions,
        response: Collection<serde_json::Value>,
        path: "/callRecords/{{RID}}/sessions",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to sessions for communications",
        name: create_sessions,
        response: serde_json::Value,
        path: "/callRecords/{{RID}}/sessions",
        params: 0,
        has_body: true
    });
}
