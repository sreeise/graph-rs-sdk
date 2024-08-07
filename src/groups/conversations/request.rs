// GENERATED CODE

use crate::api_default_imports::*;
use crate::groups::*;

api_client!(
    ConversationsApiClient,
    ConversationsIdApiClient,
    ResourceIdentity::Conversations
);

impl ConversationsApiClient {
    post!(
        doc: "Create conversation",
        name: create_conversations,
        path: "/conversations",
        body: true
    );
    get!(
        doc: "List conversations",
        name: list_conversations,
        path: "/conversations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_conversations_count,
        path: "/conversations/$count"
    );
}

impl ConversationsIdApiClient {
    api_client_link!(threads, ThreadsApiClient);
    api_client_link_id!(thread, ThreadsIdApiClient);

    delete!(
        doc: "Delete navigation property conversations for groups",
        name: delete_conversations,
        path: "/conversations/{{RID}}"
    );
    get!(
        doc: "Get conversations from groups",
        name: get_conversations,
        path: "/conversations/{{RID}}"
    );
}
