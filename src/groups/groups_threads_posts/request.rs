// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    GroupsThreadsPostsApiClient,
    GroupsThreadsPostsIdApiClient,
    ResourceIdentity::GroupsThreadsPosts
);

impl GroupsThreadsPostsApiClient {
    get!(
        doc: "Get post",
        name: list_posts,
        path: "/posts"
    );
    get!(
        doc: "Get the number of the resource",
        name: posts_fcff,
        path: "/posts/$count"
    );
}

impl GroupsThreadsPostsIdApiClient {
    get!(
        doc: "Get posts from groups",
        name: get_posts,
        path: "/posts/{{RID}}"
    );
    post!(
        doc: "Create new navigation property to attachments for groups",
        name: create_attachments,
        path: "/posts/{{RID}}/attachments",
        body: true
    );
    get!(
        doc: "List attachments",
        name: list_attachments,
        path: "/posts/{{RID}}/attachments"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_attachments_count,
        path: "/posts/{{RID}}/attachments/$count"
    );
    post!(
        doc: "Invoke action createUploadSession",
        name: create_upload_session,
        path: "/posts/{{RID}}/attachments/createUploadSession",
        body: true
    );
    delete!(
        doc: "Delete navigation property attachments for groups",
        name: delete_attachments,
        path: "/posts/{{RID}}/attachments/{{id}}",
        params: attachment_id
    );
    get!(
        doc: "Get attachments from groups",
        name: get_attachments,
        path: "/posts/{{RID}}/attachments/{{id}}",
        params: attachment_id
    );
    post!(
        doc: "Create new navigation property to extensions for groups",
        name: create_extensions,
        path: "/posts/{{RID}}/extensions",
        body: true
    );
    get!(
        doc: "Get extensions from groups",
        name: list_extensions,
        path: "/posts/{{RID}}/extensions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_extensions_count,
        path: "/posts/{{RID}}/extensions/$count"
    );
    delete!(
        doc: "Delete navigation property extensions for groups",
        name: delete_extensions,
        path: "/posts/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    get!(
        doc: "Get extensions from groups",
        name: get_extensions,
        path: "/posts/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    patch!(
        doc: "Update the navigation property extensions in groups",
        name: update_extensions,
        path: "/posts/{{RID}}/extensions/{{id}}",
        body: true,
        params: extension_id
    );
    post!(
        doc: "Invoke action forward",
        name: forward,
        path: "/posts/{{RID}}/forward",
        body: true
    );
    post!(
        doc: "Invoke action reply",
        name: reply,
        path: "/posts/{{RID}}/reply",
        body: true
    );
}
