use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::notebooks::{NotebookRequest, NotebooksRequest};
use crate::pages::{PageRequest, PagesRequest};
use crate::section_groups::{SectionGroupRequest, SectionGroupsRequest};
use crate::sections::{SectionRequest, SectionsRequest};
use graph_http::types::Collection;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(OnenoteRequest,);

impl<'a, Client> OnenoteRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn notebooks(&self) -> NotebookRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        NotebookRequest::new(self.client)
    }
    pub fn notebook<ID: AsRef<str>>(&self, id: ID) -> NotebooksRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Notebooks);
        NotebooksRequest::new(id.as_ref(), self.client)
    }
    pub fn pages(&self) -> PageRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        PageRequest::new(self.client)
    }
    pub fn page<ID: AsRef<str>>(&self, id: ID) -> PagesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Pages);
        PagesRequest::new(id.as_ref(), self.client)
    }
    pub fn sections(&self) -> SectionRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        SectionRequest::new(self.client)
    }
    pub fn section_groups(&self) -> SectionGroupRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        SectionGroupRequest::new(self.client)
    }
    pub fn section_group<ID: AsRef<str>>(&self, id: ID) -> SectionGroupsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::SectionGroups);
        SectionGroupsRequest::new(id.as_ref(), self.client)
    }
    pub fn section<ID: AsRef<str>>(&self, id: ID) -> SectionsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Sections);
        SectionsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get notebooks from me",
        name: list_notebooks,
        response: Collection<serde_json::Value>,
        path: "/onenote/notebooks",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to notebooks for me",
        name: create_notebooks,
        response: serde_json::Value,
        path: "/onenote/notebooks",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get operations from me",
        name: list_operations,
        response: Collection<serde_json::Value>,
        path: "/onenote/operations",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to operations for me",
        name: create_operations,
        response: serde_json::Value,
        path: "/onenote/operations",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get operations from me",
        name: get_operations,
        response: serde_json::Value,
        path: "/onenote/operations/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property operations in me",
        name: update_operations,
        response: NoContent,
        path: "/onenote/operations/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get pages from me",
        name: list_pages,
        response: Collection<serde_json::Value>,
        path: "/onenote/pages",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to pages for me",
        name: create_pages,
        response: serde_json::Value,
        path: "/onenote/pages",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get resources from me",
        name: list_resources,
        response: Collection<serde_json::Value>,
        path: "/onenote/resources",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to resources for me",
        name: create_resources,
        response: serde_json::Value,
        path: "/onenote/resources",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get resources from me",
        name: get_resources,
        response: serde_json::Value,
        path: "/onenote/resources/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property resources in me",
        name: update_resources,
        response: NoContent,
        path: "/onenote/resources/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get sectionGroups from me",
        name: list_section_groups,
        response: Collection<serde_json::Value>,
        path: "/onenote/sectionGroups",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to sectionGroups for me",
        name: create_section_groups,
        response: serde_json::Value,
        path: "/onenote/sectionGroups",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get sections from me",
        name: list_sections,
        response: Collection<serde_json::Value>,
        path: "/onenote/sections",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to sections for me",
        name: create_sections,
        response: serde_json::Value,
        path: "/onenote/sections",
        params: 0,
        has_body: true
    });
}
