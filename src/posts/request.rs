// GENERATED CODE

use crate::api_default_imports::*;
use crate::attachments::{AttachmentsApiClient, AttachmentsIdApiClient};
use crate::extended_properties::ExtendedPropertiesApiClient;

resource_api_client!(PostsApiClient, PostsIdApiClient, ResourceIdentity::Posts);

impl PostsApiClient {
    get!({
        doc: "# Get posts from groups",
        name: list_posts,
        response: serde_json::Value,
        path: "/posts",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to posts for groups",
        name: create_posts,
        response: serde_json::Value,
        path: "/posts",
        params: 0,
        has_body: true
    });
}

impl PostsIdApiClient {
    api_client_link!(
        attachments,
        ResourceIdentity::Attachments,
        AttachmentsApiClient
    );
    api_client_link_id!(
        attachment,
        ResourceIdentity::Attachments,
        AttachmentsIdApiClient
    );

    api_client_link!(
        extended_properties,
        ResourceIdentity::ExtendedProperties,
        ExtendedPropertiesApiClient
    );

    get!({
        doc: "# Get posts from groups",
        name: get_posts,
        response: serde_json::Value,
        path: "/posts/{{RID}}",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property posts in groups",
        name: update_posts,
        response: NoContent,
        path: "/posts/{{RID}}",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get attachments from groups",
        name: list_attachments,
        response: serde_json::Value,
        path: "/posts/{{RID}}/attachments",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to attachments for groups",
        name: create_attachments,
        response: serde_json::Value,
        path: "/posts/{{RID}}/attachments",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get extensions from groups",
        name: list_extensions,
        response: serde_json::Value,
        path: "/posts/{{RID}}/extensions",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to extensions for groups",
        name: create_extensions,
        response: serde_json::Value,
        path: "/posts/{{RID}}/extensions",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get extensions from groups",
        name: get_extensions,
        response: serde_json::Value,
        path: "/posts/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property extensions in groups",
        name: update_extensions,
        response: NoContent,
        path: "/posts/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action forward",
        name: forward,
        response: NoContent,
        path: "/posts/{{RID}}/forward",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get inReplyTo from groups",
        name: get_in_reply_to,
        response: serde_json::Value,
        path: "/posts/{{RID}}/inReplyTo",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property inReplyTo in groups",
        name: update_in_reply_to,
        response: NoContent,
        path: "/posts/{{RID}}/inReplyTo",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action reply",
        name: reply,
        response: NoContent,
        path: "/posts/{{RID}}/reply",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action forward",
        name: forward_in_reply_to,
        response: NoContent,
        path: "/posts/{{RID}}/inReplyTo/forward",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action reply",
        name: reply_in_reply_to,
        response: NoContent,
        path: "/posts/{{RID}}/inReplyTo/reply",
        params: 0,
        has_body: true
    });
}
