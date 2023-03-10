// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    MessagesRepliesApiClient,
    MessagesRepliesIdApiClient,
    ResourceIdentity::MessagesReplies
);

impl MessagesRepliesApiClient {
    get!(
        doc: "List replies",
        name: list_replies,
        path: "/replies"
    );
    post!(
        doc: "Reply to a message in a channel",
        name: create_replies,
        path: "/replies",
        body: true
    );
    get!(
        doc: "Get the number of the resource",
        name: get_replies_count,
        path: "/replies/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/replies/delta()"
    );
}

impl MessagesRepliesIdApiClient {
    delete!(
        doc: "Delete navigation property replies for chats",
        name: delete_replies,
        path: "/replies/{{RID}}"
    );
    get!(
        doc: "Get replies from chats",
        name: get_replies,
        path: "/replies/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property replies in chats",
        name: update_replies,
        path: "/replies/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to hostedContents for chats",
        name: create_hosted_contents,
        path: "/replies/{{RID}}/hostedContents",
        body: true
    );
    get!(
        doc: "List hostedContents",
        name: list_hosted_contents,
        path: "/replies/{{RID}}/hostedContents"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_hosted_contents_count,
        path: "/replies/{{RID}}/hostedContents/$count"
    );
    delete!(
        doc: "Delete navigation property hostedContents for chats",
        name: delete_hosted_contents,
        path: "/replies/{{RID}}/hostedContents/{{id}}",
        params: chat_message_hosted_content_id
    );
    get!(
        doc: "Get hostedContents from chats",
        name: get_hosted_contents,
        path: "/replies/{{RID}}/hostedContents/{{id}}",
        params: chat_message_hosted_content_id
    );
    patch!(
        doc: "Update the navigation property hostedContents in chats",
        name: update_hosted_contents,
        path: "/replies/{{RID}}/hostedContents/{{id}}",
        body: true,
        params: chat_message_hosted_content_id
    );
    post!(
        doc: "Invoke action softDelete",
        name: soft_delete,
        path: "/replies/{{RID}}/softDelete"
    );
    post!(
        doc: "Invoke action undoSoftDelete",
        name: undo_soft_delete,
        path: "/replies/{{RID}}/undoSoftDelete"
    );
}
