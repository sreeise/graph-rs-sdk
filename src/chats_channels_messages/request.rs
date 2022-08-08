// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::DeltaPhantom;
use graph_http::types::NoContent;

register_client!(ChatsAndChannelsMessagesRequest,);
register_client!(ChatsAndChannelsMessagesIdRequest, ());

impl<'a, Client> ChatsAndChannelsMessagesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Create new navigation property to messages for me",
        name: create_messages,
        response: serde_json::Value,
        path: "/messages",
        has_body: true
    });
    get!({
        doc: "Get messages from me",
        name: list_messages,
        response: serde_json::Value,
        path: "/messages",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/messages/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/messages/microsoft.graph.delta()",
        has_body: false
    });
}

impl<'a, Client> ChatsAndChannelsMessagesIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    patch!({
        doc: "Update the navigation property messages in me",
        name: update_messages,
        response: NoContent,
        path: "/messages/{{RID}}",
        has_body: true
    });
    get!({
        doc: "Get messages from me",
        name: get_messages,
        response: serde_json::Value,
        path: "/messages/{{RID}}",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property messages for me",
        name: delete_messages,
        response: NoContent,
        path: "/messages/{{RID}}",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to hostedContents for me",
        name: create_hosted_contents,
        response: serde_json::Value,
        path: "/messages/{{RID}}/hostedContents",
        has_body: true
    });
    get!({
        doc: "Get hostedContents from me",
        name: list_hosted_contents,
        response: serde_json::Value,
        path: "/messages/{{RID}}/hostedContents",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_hosted_contents_count,
        response: serde_json::Value,
        path: "/messages/{{RID}}/hostedContents/$count",
        has_body: false
    });
    get!({
        doc: "Get hostedContents from me",
        name: get_hosted_contents,
        response: serde_json::Value,
        path: "/messages/{{RID}}/hostedContents/{{id}}",
        params: [ chat_message_hosted_content_id ],
        has_body: false
    });
    delete!({
        doc: "Delete navigation property hostedContents for me",
        name: delete_hosted_contents,
        response: NoContent,
        path: "/messages/{{RID}}/hostedContents/{{id}}",
        params: [ chat_message_hosted_content_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property hostedContents in me",
        name: update_hosted_contents,
        response: NoContent,
        path: "/messages/{{RID}}/hostedContents/{{id}}",
        params: [ chat_message_hosted_content_id ],
        has_body: true
    });
    post!({
        doc: "Create new navigation property to replies for me",
        name: create_replies,
        response: serde_json::Value,
        path: "/messages/{{RID}}/replies",
        has_body: true
    });
    get!({
        doc: "Get replies from me",
        name: list_replies,
        response: serde_json::Value,
        path: "/messages/{{RID}}/replies",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_replies_count,
        response: serde_json::Value,
        path: "/messages/{{RID}}/replies/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function delta",
        name: replies_delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/messages/{{RID}}/replies/microsoft.graph.delta()",
        has_body: false
    });
    get!({
        doc: "Get replies from me",
        name: get_replies,
        response: serde_json::Value,
        path: "/messages/{{RID}}/replies/{{id}}",
        params: [ chat_message_id_1 ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property replies in me",
        name: update_replies,
        response: NoContent,
        path: "/messages/{{RID}}/replies/{{id}}",
        params: [ chat_message_id_1 ],
        has_body: true
    });
    delete!({
        doc: "Delete navigation property replies for me",
        name: delete_replies,
        response: NoContent,
        path: "/messages/{{RID}}/replies/{{id}}",
        params: [ chat_message_id_1 ],
        has_body: false
    });
    post!({
        doc: "Create new navigation property to hostedContents for me",
        name: create_replies_hosted_contents,
        response: serde_json::Value,
        path: "/messages/{{RID}}/replies/{{id}}/hostedContents",
        params: [ chat_message_id_1 ],
        has_body: true
    });
    get!({
        doc: "Get hostedContents from me",
        name: list_replies_hosted_contents,
        response: serde_json::Value,
        path: "/messages/{{RID}}/replies/{{id}}/hostedContents",
        params: [ chat_message_id_1 ],
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_replies_hosted_contents_count,
        response: serde_json::Value,
        path: "/messages/{{RID}}/replies/{{id}}/hostedContents/$count",
        params: [ chat_message_id_1 ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property hostedContents in me",
        name: update_replies_hosted_contents,
        response: NoContent,
        path: "/messages/{{RID}}/replies/{{id}}/hostedContents/{{id2}}",
        params: [ chat_message_id_1  chat_message_hosted_content_id ],
        has_body: true
    });
    delete!({
        doc: "Delete navigation property hostedContents for me",
        name: delete_replies_hosted_contents,
        response: NoContent,
        path: "/messages/{{RID}}/replies/{{id}}/hostedContents/{{id2}}",
        params: [ chat_message_id_1  chat_message_hosted_content_id ],
        has_body: false
    });
    get!({
        doc: "Get hostedContents from me",
        name: get_replies_hosted_contents,
        response: serde_json::Value,
        path: "/messages/{{RID}}/replies/{{id}}/hostedContents/{{id2}}",
        params: [ chat_message_id_1  chat_message_hosted_content_id ],
        has_body: false
    });
}
