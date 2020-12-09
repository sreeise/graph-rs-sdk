// GENERATED CODE

use crate::client::Graph;
use crate::core::ResourceIdentity;
use graph_error::GraphFailure;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use graph_http::UploadSessionClient;
use handlebars::*;
use reqwest::Method;
use std::path::Path;

register_client!(AttachmentRequest,);
register_client!(AttachmentsRequest, ());

impl<'a, Client> AttachmentRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> AttachmentsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Attachments);
        AttachmentsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get attachments from groups",
        name: list_attachments,
        response: Collection<serde_json::Value>,
        path: "/attachments",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to attachments for groups",
        name: create_attachments,
        response: serde_json::Value,
        path: "/attachments",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action createUploadSession",
        name: create_upload_session,
        path: "/attachments/createUploadSession",
        params: 0,
        has_body: true,
        upload_session: true
    });
}

impl<'a, Client> AttachmentsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get attachments from groups",
        name: get_attachments,
        response: serde_json::Value,
        path: "/attachments/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property attachments in groups",
        name: update_attachments,
        response: GraphResponse<Content>,
        path: "/attachments/{{RID}}",
        params: 0,
        has_body: true
    });
}
