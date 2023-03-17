// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    UsersAttachmentsApiClient,
    UsersAttachmentsIdApiClient,
    ResourceIdentity::UsersAttachments
);

impl UsersAttachmentsApiClient {
    post!(
        doc: "Create taskFileAttachment",
        name: create_attachments,
        path: "/attachments",
        body: true
    );
    get!(
        doc: "List taskFileAttachments",
        name: list_attachments,
        path: "/attachments"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_attachments_count,
        path: "/attachments/$count"
    );
    post!(
        doc: "Invoke action createUploadSession",
        name: create_upload_session,
        path: "/attachments/createUploadSession",
        body: true
    );
}

impl UsersAttachmentsIdApiClient {
    delete!(
        doc: "Delete navigation property attachments for users",
        name: delete_attachments,
        path: "/attachments/{{RID}}"
    );
    get!(
        doc: "Get attachments from users",
        name: get_attachments,
        path: "/attachments/{{RID}}"
    );
    get!(
        doc: "Get media content for the navigation property attachments from users",
        name: get_attachments_content,
        path: "/attachments/{{RID}}/$value"
    );
    put!(
        doc: "Update media content for the navigation property attachments in users",
        name: update_attachments_content,
        path: "/attachments/{{RID}}/$value",
        body: true
    );
}
