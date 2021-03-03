// GENERATED CODE

use crate::attachments::{AttachmentRequest, AttachmentsRequest};
use crate::calendar::CalendarRequest;
use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::extended_properties::ExtendedPropertiesRequest;
use crate::instances::{InstanceRequest, InstancesRequest};
use graph_http::types::DeltaPhantom;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(CalendarViewRequest, ());
register_client!(CalendarViewsRequest,);

impl<'a, Client> CalendarViewRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
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
    post!({
        doc: "# Invoke action accept",
        name: accept,
        response: NoContent,
        path: "/calendarView/{{RID}}/accept",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get attachments from me",
        name: list_attachments,
        response: serde_json::Value,
        path: "/calendarView/{{RID}}/attachments",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to attachments for me",
        name: create_attachments,
        response: serde_json::Value,
        path: "/calendarView/{{RID}}/attachments",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get calendar from me",
        name: get_calendar,
        response: serde_json::Value,
        path: "/calendarView/{{RID}}/calendar",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendar in me",
        name: update_calendar,
        response: NoContent,
        path: "/calendarView/{{RID}}/calendar",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action decline",
        name: decline,
        response: NoContent,
        path: "/calendarView/{{RID}}/decline",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action dismissReminder",
        name: dismiss_reminder,
        response: NoContent,
        path: "/calendarView/{{RID}}/dismissReminder",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get extensions from me",
        name: list_extensions,
        response: serde_json::Value,
        path: "/calendarView/{{RID}}/extensions",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to extensions for me",
        name: create_extensions,
        response: serde_json::Value,
        path: "/calendarView/{{RID}}/extensions",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get extensions from me",
        name: get_extensions,
        response: serde_json::Value,
        path: "/calendarView/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property extensions in me",
        name: update_extensions,
        response: NoContent,
        path: "/calendarView/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action snoozeReminder",
        name: snooze_reminder,
        response: NoContent,
        path: "/calendarView/{{RID}}/snoozeReminder",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action tentativelyAccept",
        name: tentatively_accept,
        response: NoContent,
        path: "/calendarView/{{RID}}/tentativelyAccept",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> CalendarViewsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> CalendarViewRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::CalendarView);
        CalendarViewRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get calendarView from me",
        name: list_calendar_view,
        response: serde_json::Value,
        path: "/calendarView",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to calendarView for me",
        name: create_calendar_view,
        response: serde_json::Value,
        path: "/calendarView",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/calendarView/delta()",
        params: 0,
        has_body: false
    });
}
