use crate::{
    client::Graph, core::ResourceIdentity, section_groups::SectionGroupsRequest,
    sections::SectionsRequest,
};
use graph_http::{types::NoContent, IntoResponse};
use reqwest::Method;

register_client!(ParentNotebookRequest,);

impl<'a, Client> ParentNotebookRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get parentNotebook from me",
        name: get_parent_notebook,
        response: serde_json::Value,
        path: "/parentNotebook",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property parentNotebook in me",
        name: update_parent_notebook,
        response: NoContent,
        path: "/parentNotebook",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action copyNotebook",
        name: copy_notebook,
        response: serde_json::Value,
        path: "/parentNotebook/copyNotebook",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get sectionGroups from me",
        name: list_section_groups,
        response: serde_json::Value,
        path: "/parentNotebook/sectionGroups",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to sectionGroups for me",
        name: create_section_groups,
        response: serde_json::Value,
        path: "/parentNotebook/sectionGroups",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get sections from me",
        name: list_sections,
        response: serde_json::Value,
        path: "/parentNotebook/sections",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to sections for me",
        name: create_sections,
        response: serde_json::Value,
        path: "/parentNotebook/sections",
        params: 1,
        has_body: true
    });

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
}
