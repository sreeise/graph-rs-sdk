// GENERATED CODE

use crate::api_default_imports::*;
use graph_error::GraphFailure;
use graph_http::types::NoContent;
use graph_http::UploadSessionClient;
use std::path::Path;

register_client!(AttachmentsRequest,);
register_client!(AttachmentsIdRequest, ());

impl<'a, Client> AttachmentsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, attachments_id: ID) -> AttachmentsIdRequest<'a, Client> {
        AttachmentsIdRequest::new(attachments_id.as_ref(), self.client)
    }

    get!({
        doc: "Get attachments from me",
        name: list_attachments,
        response: serde_json::Value,
        path: "attachments",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to attachments for me",
        name: create_attachments,
        response: serde_json::Value,
        path: "attachments",
        has_body: true
    });
    post!({
        doc: "Invoke action createUploadSession",
        name: create_upload_session,
        path: "attachments/createUploadSession",
        has_body: true,
        upload_session: true
    });
}

impl<'a, Client> AttachmentsIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    delete!({
        doc: "Delete navigation property attachments for me",
        name: delete_attachments,
        response: NoContent,
        path: "attachments/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property attachments in me",
        name: update_attachments,
        response: NoContent,
        path: "attachments/{{RID}}",
        has_body: true
    });
    get!({
        doc: "Get attachments from me",
        name: get_attachments,
        response: serde_json::Value,
        path: "attachments/{{RID}}",
        has_body: false
    });
}
