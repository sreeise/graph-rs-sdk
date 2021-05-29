// GENERATED CODE

use crate::{client::Graph, core::ResourceIdentity};
use graph_error::GraphFailure;
use graph_http::{types::NoContent, IntoResponse, UploadSessionClient};
use handlebars::*;
use reqwest::Method;
use std::path::Path;

register_client!(AttachmentRequest,);
register_client!(AttachmentsRequest, ());

impl<'a, Client> AttachmentRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get attachments from groups",
        name: list_attachments,
        response: serde_json::Value,
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

    pub fn id<ID: AsRef<str>>(&self, id: ID) -> AttachmentsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Attachments);
        AttachmentsRequest::new(id.as_ref(), self.client)
    }
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
        response: NoContent,
        path: "/attachments/{{RID}}",
        params: 0,
        has_body: true
    });

    delete!({
        doc: "# Delete navigation property attachments for groups",
        name: delete_attachments,
        response: NoContent,
        path: "/attachments/{{RID}}",
        params: 2,
        has_body: false
    });

    get!({
        name: get_content,
        response: NoContent,
        path: "/attachments/{{RID}}/$value",
        params: 0,
        has_body: false
    });
}
