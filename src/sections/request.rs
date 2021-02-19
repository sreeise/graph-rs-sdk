use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::pages::{PageRequest, PagesRequest};
use crate::parent_notebook::ParentNotebookRequest;
use crate::parent_section_group::ParentSectionGroupRequest;
use graph_http::types::Collection;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(SectionRequest,);
register_client!(SectionsRequest, ());

impl<'a, Client> SectionRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> SectionsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Sections);
        SectionsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get sections from me",
        name: list_sections,
        response: Collection<serde_json::Value>,
        path: "/sections",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to sections for me",
        name: create_sections,
        response: serde_json::Value,
        path: "/sections",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> SectionsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn pages(&self) -> PageRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Pages);
        PageRequest::new(self.client)
    }
    pub fn page<ID: AsRef<str>>(&self, id: ID) -> PagesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Pages);
        PagesRequest::new(id.as_ref(), self.client)
    }
    pub fn parent_notebook(&self) -> ParentNotebookRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::ParentNotebook);
        ParentNotebookRequest::new(self.client)
    }
    pub fn parent_section_group(&self) -> ParentSectionGroupRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::ParentSectionGroup);
        ParentSectionGroupRequest::new(self.client)
    }
    get!({
        doc: "# Get sections from me",
        name: get_sections,
        response: serde_json::Value,
        path: "/sections/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property sections in me",
        name: update_sections,
        response: NoContent,
        path: "/sections/{{RID}}",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action copyToNotebook",
        name: copy_to_notebook,
        response: serde_json::Value,
        path: "/sections/{{RID}}/copyToNotebook",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action copyToSectionGroup",
        name: copy_to_section_group,
        response: serde_json::Value,
        path: "/sections/{{RID}}/copyToSectionGroup",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get pages from me",
        name: list_pages,
        response: Collection<serde_json::Value>,
        path: "/sections/{{RID}}/pages",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to pages for me",
        name: create_pages,
        response: serde_json::Value,
        path: "/sections/{{RID}}/pages",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get parentNotebook from me",
        name: get_parent_notebook,
        response: serde_json::Value,
        path: "/sections/{{RID}}/parentNotebook",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property parentNotebook in me",
        name: update_parent_notebook,
        response: NoContent,
        path: "/sections/{{RID}}/parentNotebook",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get parentSectionGroup from me",
        name: get_parent_section_group,
        response: serde_json::Value,
        path: "/sections/{{RID}}/parentSectionGroup",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property parentSectionGroup in me",
        name: update_parent_section_group,
        response: NoContent,
        path: "/sections/{{RID}}/parentSectionGroup",
        params: 0,
        has_body: true
    });
}
