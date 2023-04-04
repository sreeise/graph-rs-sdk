// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    OnenotePagesApiClient,
    OnenotePagesIdApiClient,
    ResourceIdentity::OnenotePages
);

impl OnenotePagesApiClient {
    post!(
        doc: "Create new navigation property to pages for users",
        name: create_pages,
        path: "/pages",
        body: true
    );
    get!(
        doc: "Get pages from users",
        name: list_pages,
        path: "/pages"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_pages_count,
        path: "/pages/$count"
    );
}

impl OnenotePagesIdApiClient {
    delete!(
        doc: "Delete navigation property pages for users",
        name: delete_pages,
        path: "/pages/{{RID}}"
    );
    get!(
        doc: "Get pages from users",
        name: get_pages,
        path: "/pages/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property pages in users",
        name: update_pages,
        path: "/pages/{{RID}}",
        body: true
    );
    get!(
        doc: "Get content for the navigation property pages from users",
        name: get_pages_content,
        path: "/pages/{{RID}}/content"
    );
    put!(
        doc: "Update content for the navigation property pages in users",
        name: update_pages_content,
        path: "/pages/{{RID}}/content",
        body: true
    );
    post!(
        doc: "Invoke action copyToSection",
        name: copy_to_section,
        path: "/pages/{{RID}}/copyToSection",
        body: true
    );
    post!(
        doc: "Invoke action onenotePatchContent",
        name: onenote_patch_content,
        path: "/pages/{{RID}}/onenotePatchContent",
        body: true
    );
    get!(
        doc: "Get parentNotebook from users",
        name: get_parent_notebook,
        path: "/pages/{{RID}}/parentNotebook"
    );
    get!(
        doc: "Get parentSection from users",
        name: get_parent_section,
        path: "/pages/{{RID}}/parentSection"
    );
    get!(
        doc: "Invoke function preview",
        name: preview,
        path: "/pages/{{RID}}/preview()"
    );
}
