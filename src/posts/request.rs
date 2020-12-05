use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::extended_properties::ExtendedPropertiesRequest;
use graph_error::GraphFailure;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use graph_http::UploadSessionClient;
use handlebars::*;
use reqwest::Method;
use std::path::Path;

register_client!(AttachmentsRequest,);
register_client!(InReplyToRequest,);
register_client!(PostRequest,);
register_client!(PostsRequest, ());

impl<'a, Client> AttachmentsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action createUploadSession",
        name: create_upload_session,
        path: "/posts/{{RID}}/attachments/createUploadSession",
        params: 0,
        has_body: true,
        upload_session: true
    });
}

impl<'a, Client> InReplyToRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action forward",
        name: forward,
        response: GraphResponse<Content>,
        path: "/posts/{{RID}}/inReplyTo/forward",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action reply",
        name: reply,
        response: GraphResponse<Content>,
        path: "/posts/{{RID}}/inReplyTo/reply",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> PostRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> PostsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Posts);
        PostsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get posts from groups",
        name: list_posts,
        response: Collection<serde_json::Value>,
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
}

impl<'a, Client> PostsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn attachments(&self) -> AttachmentsRequest<'a, Client> {
        AttachmentsRequest::new(self.client)
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
        response: GraphResponse<Content>,
        path: "/posts/{{RID}}",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get attachments from groups",
        name: list_attachments,
        response: Collection<serde_json::Value>,
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
        doc: "# Get attachments from groups",
        name: get_attachments,
        response: serde_json::Value,
        path: "/posts/{{RID}}/attachments/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property attachments in groups",
        name: update_attachments,
        response: GraphResponse<Content>,
        path: "/posts/{{RID}}/attachments/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get extensions from groups",
        name: list_extensions,
        response: Collection<serde_json::Value>,
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
        response: GraphResponse<Content>,
        path: "/posts/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action forward",
        name: forward,
        response: GraphResponse<Content>,
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
        response: GraphResponse<Content>,
        path: "/posts/{{RID}}/inReplyTo",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action reply",
        name: reply,
        response: GraphResponse<Content>,
        path: "/posts/{{RID}}/reply",
        params: 0,
        has_body: true
    });
}
