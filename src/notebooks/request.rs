use crate::{
    client::Graph, core::ResourceIdentity, section_groups::SectionGroupsRequest,
    sections::SectionsRequest,
};
use graph_http::{types::NoContent, IntoResponse};
use handlebars::*;
use reqwest::Method;

register_client!(NotebookRequest,);
register_client!(NotebooksRequest, ());

impl<'a, Client> NotebookRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get notebooks from me",
        name: list_notebooks,
        response: serde_json::Value,
        path: "/notebooks",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to notebooks for me",
        name: create_notebooks,
        response: serde_json::Value,
        path: "/notebooks",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action getNotebookFromWebUrl",
        name: get_notebook_from_web_url,
        response: serde_json::Value,
        path: "/notebooks/getNotebookFromWebUrl",
        params: 0,
        has_body: true
    });

    pub fn id<ID: AsRef<str>>(&self, id: ID) -> NotebooksRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Notebooks);
        NotebooksRequest::new(id.as_ref(), self.client)
    }
}

impl<'a, Client> NotebooksRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get notebooks from me",
        name: get_notebooks,
        response: serde_json::Value,
        path: "/notebooks/{{RID}}",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property notebooks in me",
        name: update_notebooks,
        response: NoContent,
        path: "/notebooks/{{RID}}",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action copyNotebook",
        name: copy_notebook,
        response: serde_json::Value,
        path: "/notebooks/{{RID}}/copyNotebook",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get sectionGroups from me",
        name: list_section_groups,
        response: serde_json::Value,
        path: "/notebooks/{{RID}}/sectionGroups",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to sectionGroups for me",
        name: create_section_groups,
        response: serde_json::Value,
        path: "/notebooks/{{RID}}/sectionGroups",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get sections from me",
        name: list_sections,
        response: serde_json::Value,
        path: "/notebooks/{{RID}}/sections",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to sections for me",
        name: create_sections,
        response: serde_json::Value,
        path: "/notebooks/{{RID}}/sections",
        params: 0,
        has_body: true
    });

    pub fn section_group<ID: AsRef<str>>(&self, id: ID) -> SectionGroupsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::SectionGroups);
        SectionGroupsRequest::new(id.as_ref(), self.client)
    }

    pub fn section<ID: AsRef<str>>(&self, id: ID) -> SectionsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Sections);
        SectionsRequest::new(id.as_ref(), self.client)
    }
}
