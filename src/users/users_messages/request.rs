// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    UsersMessagesApiClient,
    UsersMessagesIdApiClient,
    ResourceIdentity::UsersMessages
);

impl UsersMessagesApiClient {
    post!(
        doc: "Create open extension",
        name: create_messages,
        path: "/messages",
        body: true
    );
    get!(
        doc: "Get open extension",
        name: list_messages,
        path: "/messages"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_messages_count,
        path: "/messages/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/messages/delta()"
    );
}

impl UsersMessagesIdApiClient {
    delete!(
        doc: "Delete navigation property messages for users",
        name: delete_messages,
        path: "/messages/{{RID}}"
    );
    get!(
        doc: "Get messages from users",
        name: get_messages,
        path: "/messages/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property messages in users",
        name: update_messages,
        path: "/messages/{{RID}}",
        body: true
    );
    get!(
        doc: "Get media content for the navigation property messages from users",
        name: get_messages_content,
        path: "/messages/{{RID}}/$value"
    );
    put!(
        doc: "Update media content for the navigation property messages in users",
        name: update_messages_content,
        path: "/messages/{{RID}}/$value",
        body: true
    );
    post!(
        doc: "Add attachment",
        name: create_attachments,
        path: "/messages/{{RID}}/attachments",
        body: true
    );
    get!(
        doc: "List attachments",
        name: list_attachments,
        path: "/messages/{{RID}}/attachments"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_attachments_count,
        path: "/messages/{{RID}}/attachments/$count"
    );
    post!(
        doc: "Invoke action createUploadSession",
        name: create_upload_session,
        path: "/messages/{{RID}}/attachments/createUploadSession",
        body: true
    );
    delete!(
        doc: "Delete navigation property attachments for users",
        name: delete_attachments,
        path: "/messages/{{RID}}/attachments/{{id}}",
        params: attachment_id
    );
    get!(
        doc: "Get attachments from users",
        name: get_attachments,
        path: "/messages/{{RID}}/attachments/{{id}}",
        params: attachment_id
    );
    post!(
        doc: "Invoke action copy",
        name: copy,
        path: "/messages/{{RID}}/copy",
        body: true
    );
    post!(
        doc: "Invoke action createForward",
        name: create_forward,
        path: "/messages/{{RID}}/createForward",
        body: true
    );
    post!(
        doc: "Invoke action createReply",
        name: create_reply,
        path: "/messages/{{RID}}/createReply",
        body: true
    );
    post!(
        doc: "Invoke action createReplyAll",
        name: create_reply_all,
        path: "/messages/{{RID}}/createReplyAll",
        body: true
    );
    post!(
        doc: "Create open extension",
        name: create_extensions,
        path: "/messages/{{RID}}/extensions",
        body: true
    );
    get!(
        doc: "Get extensions from users",
        name: list_extensions,
        path: "/messages/{{RID}}/extensions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_extensions_count,
        path: "/messages/{{RID}}/extensions/$count"
    );
    delete!(
        doc: "Delete navigation property extensions for users",
        name: delete_extensions,
        path: "/messages/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    get!(
        doc: "Get extensions from users",
        name: get_extensions,
        path: "/messages/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    patch!(
        doc: "Update the navigation property extensions in users",
        name: update_extensions,
        path: "/messages/{{RID}}/extensions/{{id}}",
        body: true,
        params: extension_id
    );
    post!(
        doc: "Invoke action forward",
        name: forward,
        path: "/messages/{{RID}}/forward",
        body: true
    );
    post!(
        doc: "Invoke action move",
        name: move_message,
        path: "/messages/{{RID}}/move",
        body: true
    );
    post!(
        doc: "Invoke action reply",
        name: reply,
        path: "/messages/{{RID}}/reply",
        body: true
    );
    post!(
        doc: "Invoke action replyAll",
        name: reply_all,
        path: "/messages/{{RID}}/replyAll",
        body: true
    );
    post!(
        doc: "Invoke action send",
        name: send,
        path: "/messages/{{RID}}/send"
    );
}
