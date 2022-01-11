// GENERATED CODE

use crate::attachments::{AttachmentsIdRequest, AttachmentsRequest};
use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::extended_properties::ExtendedPropertiesRequest;
use graph_http::types::DeltaPhantom;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(MessageRequest,);
register_client!(MessagesRequest, ());

impl<'a, Client> MessageRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get messages from me",
        name: list_messages,
        response: serde_json::Value,
        path: "/messages",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to messages for me",
        name: create_messages,
        response: serde_json::Value,
        path: "/messages",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/messages/delta()",
        params: 0,
        has_body: false
    });

    pub fn id<ID: AsRef<str>>(&self, id: ID) -> MessagesRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Messages);
        MessagesRequest::new(id.as_ref(), self.client)
    }
}

impl<'a, Client> MessagesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get messages from me",
        name: get_messages,
        response: serde_json::Value,
        path: "/messages/{{RID}}",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property messages in me",
        name: update_messages,
        response: NoContent,
        path: "/messages/{{RID}}",
        params: 0,
        has_body: true
    });

    delete!({
        name: delete_messages,
        response: NoContent,
        path: "/messages/{{RID}}",
        params: 0,
        has_body: false
    });

    get!({
        name: get_message_content,
        response: NoContent,
        path: "/messages/{{RID}}/$value",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get attachments from me",
        name: list_attachments,
        response: serde_json::Value,
        path: "/messages/{{RID}}/attachments",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to attachments for me",
        name: create_attachments,
        response: serde_json::Value,
        path: "/messages/{{RID}}/attachments",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action copy",
        name: copy,
        response: serde_json::Value,
        path: "/messages/{{RID}}/copy",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action createForward",
        name: create_forward,
        response: serde_json::Value,
        path: "/messages/{{RID}}/createForward",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action createReply",
        name: create_reply,
        response: serde_json::Value,
        path: "/messages/{{RID}}/createReply",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action createReplyAll",
        name: create_reply_all,
        response: serde_json::Value,
        path: "/messages/{{RID}}/createReplyAll",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get extensions from me",
        name: list_extensions,
        response: serde_json::Value,
        path: "/messages/{{RID}}/extensions",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to extensions for me",
        name: create_extensions,
        response: serde_json::Value,
        path: "/messages/{{RID}}/extensions",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get extensions from me",
        name: get_extensions,
        response: serde_json::Value,
        path: "/messages/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property extensions in me",
        name: update_extensions,
        response: NoContent,
        path: "/messages/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action forward",
        name: forward,
        response: NoContent,
        path: "/messages/{{RID}}/forward",
        params: 0,
        has_body: true
    });

    post!({
        name: move_message,
        response: serde_json::Value,
        path: "/messages/{{RID}}/move",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action reply",
        name: reply,
        response: NoContent,
        path: "/messages/{{RID}}/reply",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action replyAll",
        name: reply_all,
        response: NoContent,
        path: "/messages/{{RID}}/replyAll",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action send",
        name: send,
        response: NoContent,
        path: "/messages/{{RID}}/send",
        params: 0,
        has_body: false
    });

    pub fn attachments(&self) -> AttachmentsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        AttachmentsRequest::new(self.client)
    }

    pub fn attachment<ID: AsRef<str>>(&self, id: ID) -> AttachmentsIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Attachments);
        AttachmentsIdRequest::new(id.as_ref(), self.client)
    }

    pub fn extended_properties(&self) -> ExtendedPropertiesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::ExtendedProperties);
        ExtendedPropertiesRequest::new(self.client)
    }
}
