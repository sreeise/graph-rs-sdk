// GENERATED CODE

use crate::{
    attachments::{AttachmentRequest, AttachmentsRequest},
    calendar::CalendarRequest,
    client::Graph,
    core::ResourceIdentity,
    extended_properties::ExtendedPropertiesRequest,
    instances::{InstanceRequest, InstancesRequest},
};

use graph_http::{
    types::{DeltaPhantom, NoContent},
    IntoResponse,
};
use handlebars::*;
use reqwest::Method;

register_client!(EventRequest,);
register_client!(EventsRequest, ());

impl<'a, Client> EventRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get events from users",
        name: list_events,
        response: serde_json::Value,
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

    pub fn id<ID: AsRef<str>>(&self, id: ID) -> EventsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Events);
        EventsRequest::new(id.as_ref(), self.client)
    }
}

impl<'a, Client> EventsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
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
        response: NoContent,
        path: "/events/{{RID}}",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action accept",
        name: accept,
        response: NoContent,
        path: "/events/{{RID}}/accept",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get attachments from users",
        name: list_attachments,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/events/{{RID}}/calendar",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action decline",
        name: decline,
        response: NoContent,
        path: "/events/{{RID}}/decline",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action dismissReminder",
        name: dismiss_reminder,
        response: NoContent,
        path: "/events/{{RID}}/dismissReminder",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get extensions from users",
        name: list_extensions,
        response: serde_json::Value,
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
        response: NoContent,
        path: "/events/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action snoozeReminder",
        name: snooze_reminder,
        response: NoContent,
        path: "/events/{{RID}}/snoozeReminder",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action tentativelyAccept",
        name: tentatively_accept,
        response: NoContent,
        path: "/events/{{RID}}/tentativelyAccept",
        params: 0,
        has_body: true
    });

    pub fn attachments(&self) -> AttachmentRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        AttachmentRequest::new(self.client)
    }

    pub fn attachment<ID: AsRef<str>>(&self, id: ID) -> AttachmentsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Attachments);
        AttachmentsRequest::new(id.as_ref(), self.client)
    }

    pub fn calendar(&self) -> CalendarRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Calendar);
        CalendarRequest::new(self.client)
    }

    pub fn extended_properties(&self) -> ExtendedPropertiesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::ExtendedProperties);
        ExtendedPropertiesRequest::new(self.client)
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
}
