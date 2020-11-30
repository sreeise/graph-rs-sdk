use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::sections::SectionsRequest;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(ParentNotebookRequest,);
register_client!(SectionGroupRequest,);
register_client!(SectionGroupsRequest, ());

impl<'a, Client> ParentNotebookRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action copyNotebook",
        name: copy_notebook,
        response: serde_json::Value,
        path: "/sectionGroups/{{RID}}/parentNotebook/copyNotebook",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get sectionGroups from me",
        name: list_section_groups,
        response: Collection<serde_json::Value>,
        path: "/sectionGroups/{{RID}}/parentNotebook/sectionGroups",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to sectionGroups for me",
        name: create_section_groups,
        response: serde_json::Value,
        path: "/sectionGroups/{{RID}}/parentNotebook/sectionGroups",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get sectionGroups from me",
        name: get_section_groups,
        response: serde_json::Value,
        path: "/sectionGroups/{{RID}}/parentNotebook/sectionGroups/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property sectionGroups in me",
        name: update_section_groups,
        response: GraphResponse<Content>,
        path: "/sectionGroups/{{RID}}/parentNotebook/sectionGroups/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get sections from me",
        name: list_sections,
        response: Collection<serde_json::Value>,
        path: "/sectionGroups/{{RID}}/parentNotebook/sections",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to sections for me",
        name: create_sections,
        response: serde_json::Value,
        path: "/sectionGroups/{{RID}}/parentNotebook/sections",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> SectionGroupRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> SectionGroupsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::SectionGroups);
        SectionGroupsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get sectionGroups from me",
        name: list_section_groups,
        response: Collection<serde_json::Value>,
        path: "/sectionGroups",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to sectionGroups for me",
        name: create_section_groups,
        response: serde_json::Value,
        path: "/sectionGroups",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> SectionGroupsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn parent_notebook(&self) -> ParentNotebookRequest<'a, Client> {
        ParentNotebookRequest::new(self.client)
    }
    pub fn section<ID: AsRef<str>>(&self, id: ID) -> SectionsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Sections);
        SectionsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get sectionGroups from me",
        name: get_section_groups,
        response: serde_json::Value,
        path: "/sectionGroups/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property sectionGroups in me",
        name: update_section_groups,
        response: GraphResponse<Content>,
        path: "/sectionGroups/{{RID}}",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get parentNotebook from me",
        name: get_parent_notebook,
        response: serde_json::Value,
        path: "/sectionGroups/{{RID}}/parentNotebook",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property parentNotebook in me",
        name: update_parent_notebook,
        response: GraphResponse<Content>,
        path: "/sectionGroups/{{RID}}/parentNotebook",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get parentSectionGroup from me",
        name: get_parent_section_group,
        response: serde_json::Value,
        path: "/sectionGroups/{{RID}}/parentSectionGroup",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property parentSectionGroup in me",
        name: update_parent_section_group,
        response: GraphResponse<Content>,
        path: "/sectionGroups/{{RID}}/parentSectionGroup",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get sectionGroups from me",
        name: list_section_groups,
        response: Collection<serde_json::Value>,
        path: "/sectionGroups/{{RID}}/sectionGroups",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to sectionGroups for me",
        name: create_section_groups,
        response: serde_json::Value,
        path: "/sectionGroups/{{RID}}/sectionGroups",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get sections from me",
        name: list_sections,
        response: Collection<serde_json::Value>,
        path: "/sectionGroups/{{RID}}/sections",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to sections for me",
        name: create_sections,
        response: serde_json::Value,
        path: "/sectionGroups/{{RID}}/sections",
        params: 0,
        has_body: true
    });
}
