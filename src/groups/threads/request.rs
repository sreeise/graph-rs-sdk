// GENERATED CODE

use crate::api_default_imports::*;
use crate::groups::*;

resource_api_client!(
    ThreadsApiClient,
    ThreadsIdApiClient,
    ResourceIdentity::Threads
);

impl ThreadsApiClient {
    post!(
        doc: "Create conversation thread",
        name: create_threads,
        path: "/threads",
        body: true
    );
    get!(
        doc: "List threads",
        name: list_threads,
        path: "/threads"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_threads_count,
        path: "/threads/$count"
    );
}

impl ThreadsIdApiClient {
    api_client_link!(post, ThreadsPostsIdApiClient);
    api_client_link!(posts, ThreadsPostsApiClient);

    delete!(
        doc: "Delete navigation property threads for groups",
        name: delete_threads,
        path: "/threads/{{RID}}"
    );
    get!(
        doc: "Get threads from groups",
        name: get_threads,
        path: "/threads/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property threads in groups",
        name: update_threads,
        path: "/threads/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action reply",
        name: reply,
        path: "/threads/{{RID}}/reply",
        body: true
    );
}
