// GENERATED CODE

use crate::api_default_imports::*;
use crate::chats::*;

api_client!(
    ChatsMessagesApiClient,
    ChatsMessagesIdApiClient,
    ResourceIdentity::ChatsMessages
);

impl ChatsMessagesApiClient {
    get!(
        doc: "List messages in a chat",
        name: list_messages,
        path: "/messages"
    );
    post!(
        doc: "Send message in a chat",
        name: create_messages,
        path: "/messages",
        body: true
    );
    get!(
        doc: "Get the number of the resource",
        name: count,
        path: "/messages/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/messages/delta()"
    );
}

impl ChatsMessagesIdApiClient {
    api_client_link_id!(reply, ChatsMessagesRepliesIdApiClient);
    api_client_link!(replies, ChatsMessagesRepliesApiClient);

    delete!(
        doc: "Delete navigation property messages for chats",
        name: delete_messages,
        path: "/messages/{{RID}}"
    );
    get!(
        doc: "Get messages from chats",
        name: get_messages,
        path: "/messages/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property messages in chats",
        name: update_messages,
        path: "/messages/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to hostedContents for chats",
        name: create_hosted_contents,
        path: "/messages/{{RID}}/hostedContents",
        body: true
    );
    get!(
        doc: "List hostedContents",
        name: list_hosted_contents,
        path: "/messages/{{RID}}/hostedContents"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_hosted_contents_count,
        path: "/messages/{{RID}}/hostedContents/$count"
    );
    delete!(
        doc: "Delete navigation property hostedContents for chats",
        name: delete_hosted_contents,
        path: "/messages/{{RID}}/hostedContents/{{id}}",
        params: chat_message_hosted_content_id
    );
    get!(
        doc: "Get hostedContents from chats",
        name: get_hosted_contents,
        path: "/messages/{{RID}}/hostedContents/{{id}}",
        params: chat_message_hosted_content_id
    );
    patch!(
        doc: "Update the navigation property hostedContents in chats",
        name: update_hosted_contents,
        path: "/messages/{{RID}}/hostedContents/{{id}}",
        body: true,
        params: chat_message_hosted_content_id
    );
    post!(
        doc: "Invoke action softDelete",
        name: soft_delete,
        path: "/messages/{{RID}}/softDelete"
    );
    post!(
        doc: "Invoke action undoSoftDelete",
        name: undo_soft_delete,
        path: "/messages/{{RID}}/undoSoftDelete"
    );
}
