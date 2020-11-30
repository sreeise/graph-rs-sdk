use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::pages::PagesRequest;
use crate::parent_notebook::ParentNotebookRequest;
use crate::parent_section_group::ParentSectionGroupRequest;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(ParentSectionRequest,);

impl<'a, Client> ParentSectionRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn page<ID: AsRef<str>>(&self, id: ID) -> PagesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Pages);
        PagesRequest::new(id.as_ref(), self.client)
    }
    pub fn parent_notebook(&self) -> ParentNotebookRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::ParentNotebook);
        ParentNotebookRequest::new(self.client)
    }
    pub fn parent_section_group(&self) -> ParentSectionGroupRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::ParentSectionGroup);
        ParentSectionGroupRequest::new(self.client)
    }
    get!({
        doc: "# Get parentSection from me",
        name: get_parent_section,
        response: serde_json::Value,
        path: "/parentSection",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property parentSection in me",
        name: update_parent_section,
        response: GraphResponse<Content>,
        path: "/parentSection",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action copyToNotebook",
        name: copy_to_notebook,
        response: serde_json::Value,
        path: "/parentSection/copyToNotebook",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action copyToSectionGroup",
        name: copy_to_section_group,
        response: serde_json::Value,
        path: "/parentSection/copyToSectionGroup",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get pages from me",
        name: list_pages,
        response: Collection<serde_json::Value>,
        path: "/parentSection/pages",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to pages for me",
        name: create_pages,
        response: serde_json::Value,
        path: "/parentSection/pages",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get parentNotebook from me",
        name: get_parent_notebook,
        response: serde_json::Value,
        path: "/parentSection/parentNotebook",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property parentNotebook in me",
        name: update_parent_notebook,
        response: GraphResponse<Content>,
        path: "/parentSection/parentNotebook",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get parentSectionGroup from me",
        name: get_parent_section_group,
        response: serde_json::Value,
        path: "/parentSection/parentSectionGroup",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property parentSectionGroup in me",
        name: update_parent_section_group,
        response: GraphResponse<Content>,
        path: "/parentSection/parentSectionGroup",
        params: 0,
        has_body: true
    });
}
