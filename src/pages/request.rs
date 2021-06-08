use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::parent_notebook::ParentNotebookRequest;
use crate::parent_section::ParentSectionRequest;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use graph_http::{
    AsyncDownload, AsyncHttpClient, BlockingDownload, BlockingHttpClient, RequestClient,
};
use handlebars::*;
use reqwest::Method;
use std::path::Path;

register_client!(PageRequest,);
register_client!(PagesRequest, ());

impl<'a, Client> PageRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get pages from me",
        name: list_pages,
        response: serde_json::Value,
        path: "/pages",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to pages for me",
        name: create_pages,
        response: serde_json::Value,
        path: "/pages",
        params: 0,
        has_body: true
    });

    pub fn id<ID: AsRef<str>>(&self, id: ID) -> PagesRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Pages);
        PagesRequest::new(id.as_ref(), self.client)
    }
}

impl<'a, Client> PagesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get pages from me",
        name: get_pages,
        response: serde_json::Value,
        path: "/pages/{{RID}}",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property pages in me",
        name: update_pages,
        response: NoContent,
        path: "/pages/{{RID}}",
        params: 0,
        has_body: true
    });

    delete!({
        name: delete_pages,
        response: NoContent,
        path: "/pages/{{RID}}",
        params: 0,
        has_body: false
    });

    patch!({
        name: update_page_content,
        response: serde_json::Value,
        path: "/pages/{{RID}}/content",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action copyToSection",
        name: copy_to_section,
        response: serde_json::Value,
        path: "/pages/{{RID}}/copyToSection",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action onenotePatchContent",
        name: onenote_patch_content,
        response: NoContent,
        path: "/pages/{{RID}}/onenotePatchContent",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get parentNotebook from me",
        name: get_parent_notebook,
        response: serde_json::Value,
        path: "/pages/{{RID}}/parentNotebook",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property parentNotebook in me",
        name: update_parent_notebook,
        response: NoContent,
        path: "/pages/{{RID}}/parentNotebook",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get parentSection from me",
        name: get_parent_section,
        response: serde_json::Value,
        path: "/pages/{{RID}}/parentSection",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property parentSection in me",
        name: update_parent_section,
        response: NoContent,
        path: "/pages/{{RID}}/parentSection",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Invoke function preview",
        name: preview,
        response: serde_json::Value,
        path: "/pages/{{RID}}/preview()",
        params: 0,
        has_body: false
    });

    pub fn parent_notebook(&self) -> ParentNotebookRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::ParentNotebook);
        ParentNotebookRequest::new(self.client)
    }

    pub fn parent_section(&self) -> ParentSectionRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::ParentSection);
        ParentSectionRequest::new(self.client)
    }
}

impl<'a> PagesRequest<'a, BlockingHttpClient> {
    download!({
        name: download_page,
        response: BlockingDownload,
        path: "/pages/{{RID}}/content",
        params: 0
    });
}

impl<'a> PagesRequest<'a, AsyncHttpClient> {
    async_download!({
        name: download_page,
        response: AsyncDownload,
        path: "/pages/{{RID}}/content",
        params: 0
    });
}
