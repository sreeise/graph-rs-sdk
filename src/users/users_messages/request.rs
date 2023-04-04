// GENERATED CODE

use crate::api_default_imports::*;
use crate::users::*;

resource_api_client!(
    UsersMessagesApiClient,
    UsersMessagesIdApiClient,
    ResourceIdentity::UsersMessages
);

impl UsersMessagesApiClient {
    post!(
        doc: "Create message",
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
    api_client_link!(attachments, UsersAttachmentsApiClient);
    api_client_link_id!(attachment, UsersAttachmentsIdApiClient);

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
        doc: "Create new navigation property to multiValueExtendedProperties for users",
        name: create_multi_value_extended_properties,
        path: "/messages/{{RID}}/multiValueExtendedProperties",
        body: true
    );
    get!(
        doc: "Get multiValueExtendedProperties from users",
        name: list_multi_value_extended_properties,
        path: "/messages/{{RID}}/multiValueExtendedProperties"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_multi_value_extended_properties_count,
        path: "/messages/{{RID}}/multiValueExtendedProperties/$count"
    );
    delete!(
        doc: "Delete navigation property multiValueExtendedProperties for users",
        name: delete_multi_value_extended_properties,
        path: "/messages/{{RID}}/multiValueExtendedProperties/{{id}}",
        params: multi_value_legacy_extended_property_id
    );
    get!(
        doc: "Get multiValueExtendedProperties from users",
        name: get_multi_value_extended_properties,
        path: "/messages/{{RID}}/multiValueExtendedProperties/{{id}}",
        params: multi_value_legacy_extended_property_id
    );
    patch!(
        doc: "Update the navigation property multiValueExtendedProperties in users",
        name: update_multi_value_extended_properties,
        path: "/messages/{{RID}}/multiValueExtendedProperties/{{id}}",
        body: true,
        params: multi_value_legacy_extended_property_id
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
    post!(
        doc: "Create new navigation property to singleValueExtendedProperties for users",
        name: create_single_value_extended_properties,
        path: "/messages/{{RID}}/singleValueExtendedProperties",
        body: true
    );
    get!(
        doc: "Get singleValueExtendedProperties from users",
        name: list_single_value_extended_properties,
        path: "/messages/{{RID}}/singleValueExtendedProperties"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_single_value_extended_properties_count,
        path: "/messages/{{RID}}/singleValueExtendedProperties/$count"
    );
    delete!(
        doc: "Delete navigation property singleValueExtendedProperties for users",
        name: delete_single_value_extended_properties,
        path: "/messages/{{RID}}/singleValueExtendedProperties/{{id}}",
        params: single_value_legacy_extended_property_id
    );
    get!(
        doc: "Get singleValueExtendedProperties from users",
        name: get_single_value_extended_properties,
        path: "/messages/{{RID}}/singleValueExtendedProperties/{{id}}",
        params: single_value_legacy_extended_property_id
    );
    patch!(
        doc: "Update the navigation property singleValueExtendedProperties in users",
        name: update_single_value_extended_properties,
        path: "/messages/{{RID}}/singleValueExtendedProperties/{{id}}",
        body: true,
        params: single_value_legacy_extended_property_id
    );
}
