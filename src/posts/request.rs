// GENERATED CODE

use crate::attachments::{AttachmentRequest, AttachmentsRequest};
use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::extended_properties::ExtendedPropertiesRequest;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(InReplyToRequest,);
register_client!(PostRequest,);
register_client!(PostsRequest, ());

impl<'a, Client> InReplyToRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action forward",
        name: forward,
        response: NoContent,
        path: "/posts/{{RID}}/inReplyTo/forward",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action reply",
        name: reply,
        response: NoContent,
        path: "/posts/{{RID}}/inReplyTo/reply",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> PostRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get posts from groups",
        name: list_posts,
        response: serde_json::Value,
        path: "/posts",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to posts for groups",
        name: create_posts,
        response: serde_json::Value,
        path: "/posts",
        params: 0,
        has_body: true
    });

    pub fn id<ID: AsRef<str>>(&self, id: ID) -> PostsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Posts);
        PostsRequest::new(id.as_ref(), self.client)
    }
}

impl<'a, Client> PostsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get posts from groups",
        name: get_posts,
        response: serde_json::Value,
        path: "/posts/{{RID}}",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property posts in groups",
        name: update_posts,
        response: NoContent,
        path: "/posts/{{RID}}",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get attachments from groups",
        name: list_attachments,
        response: serde_json::Value,
        path: "/posts/{{RID}}/attachments",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to attachments for groups",
        name: create_attachments,
        response: serde_json::Value,
        path: "/posts/{{RID}}/attachments",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get extensions from groups",
        name: list_extensions,
        response: serde_json::Value,
        path: "/posts/{{RID}}/extensions",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to extensions for groups",
        name: create_extensions,
        response: serde_json::Value,
        path: "/posts/{{RID}}/extensions",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get extensions from groups",
        name: get_extensions,
        response: serde_json::Value,
        path: "/posts/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property extensions in groups",
        name: update_extensions,
        response: NoContent,
        path: "/posts/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action forward",
        name: forward,
        response: NoContent,
        path: "/posts/{{RID}}/forward",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get inReplyTo from groups",
        name: get_in_reply_to,
        response: serde_json::Value,
        path: "/posts/{{RID}}/inReplyTo",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property inReplyTo in groups",
        name: update_in_reply_to,
        response: NoContent,
        path: "/posts/{{RID}}/inReplyTo",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action reply",
        name: reply,
        response: NoContent,
        path: "/posts/{{RID}}/reply",
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

    pub fn extended_properties(&self) -> ExtendedPropertiesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::ExtendedProperties);
        ExtendedPropertiesRequest::new(self.client)
    }

    pub fn in_reply_to(&self) -> InReplyToRequest<'a, Client> {
        InReplyToRequest::new(self.client)
    }
}
