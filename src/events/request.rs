use crate::calendar::CalendarRequest;
use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::instances::{InstanceRequest, InstancesRequest};
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::types::DeltaPhantom;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use graph_http::UploadSessionClient;
use handlebars::*;
use reqwest::Method;

register_client!(AttachmentsRequest,);
register_client!(EventRequest,);
register_client!(EventsRequest, ());

impl<'a, Client> AttachmentsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action createUploadSession",
        name: create_upload_session,
        response: UploadSessionClient<Client>,
        path: "/events/{{RID}}/attachments/createUploadSession",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> EventRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> EventsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Events);
        EventsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get events from users",
        name: list_events,
        response: Collection<serde_json::Value>,
        path: "/events",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to events for users",
        name: create_events,
        response: serde_json::Value,
        path: "/events",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/events/delta()",
        params: 0,
        has_body: false
    });
}

impl<'a, Client> EventsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn attachments(&self) -> AttachmentsRequest<'a, Client> {
        AttachmentsRequest::new(self.client)
    }
    pub fn calendar(&self) -> CalendarRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Calendar);
        CalendarRequest::new(self.client)
    }
    pub fn instances(&self) -> InstanceRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        InstanceRequest::new(self.client)
    }
    pub fn instance<ID: AsRef<str>>(&self, id: ID) -> InstancesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Instances);
        InstancesRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get events from users",
        name: get_events,
        response: serde_json::Value,
        path: "/events/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property events in users",
        name: update_events,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action accept",
        name: accept,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/accept",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get attachments from users",
        name: list_attachments,
        response: Collection<serde_json::Value>,
        path: "/events/{{RID}}/attachments",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to attachments for users",
        name: create_attachments,
        response: serde_json::Value,
        path: "/events/{{RID}}/attachments",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get attachments from users",
        name: get_attachments,
        response: serde_json::Value,
        path: "/events/{{RID}}/attachments/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property attachments in users",
        name: update_attachments,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/attachments/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get calendar from users",
        name: get_calendar,
        response: serde_json::Value,
        path: "/events/{{RID}}/calendar",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendar in users",
        name: update_calendar,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/calendar",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action decline",
        name: decline,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/decline",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action dismissReminder",
        name: dismiss_reminder,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/dismissReminder",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get extensions from users",
        name: list_extensions,
        response: Collection<serde_json::Value>,
        path: "/events/{{RID}}/extensions",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to extensions for users",
        name: create_extensions,
        response: serde_json::Value,
        path: "/events/{{RID}}/extensions",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get extensions from users",
        name: get_extensions,
        response: serde_json::Value,
        path: "/events/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property extensions in users",
        name: update_extensions,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action snoozeReminder",
        name: snooze_reminder,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/snoozeReminder",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action tentativelyAccept",
        name: tentatively_accept,
        response: GraphResponse<Content>,
        path: "/events/{{RID}}/tentativelyAccept",
        params: 0,
        has_body: true
    });
}
