use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::extended_properties::ExtendedPropertiesRequest;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::types::DeltaPhantom;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(ChildFoldersRequest,);
register_client!(ContactFolderRequest,);
register_client!(ContactFoldersRequest, ());
register_client!(ContactsRequest,);

impl<'a, Client> ChildFoldersRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/contactFolders/{{RID}}/childFolders/delta()",
        params: 0,
        has_body: false
    });
}

impl<'a, Client> ContactFolderRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> ContactFoldersRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::ContactFolders);
        ContactFoldersRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get contactFolders from me",
        name: list_contact_folders,
        response: Collection<serde_json::Value>,
        path: "/contactFolders",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to contactFolders for me",
        name: create_contact_folders,
        response: serde_json::Value,
        path: "/contactFolders",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/contactFolders/delta()",
        params: 0,
        has_body: false
    });
}

impl<'a, Client> ContactFoldersRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn child_folders(&self) -> ChildFoldersRequest<'a, Client> {
        ChildFoldersRequest::new(self.client)
    }
    pub fn contacts(&self) -> ContactsRequest<'a, Client> {
        ContactsRequest::new(self.client)
    }
    pub fn extended_properties(&self) -> ExtendedPropertiesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::ExtendedProperties);
        ExtendedPropertiesRequest::new(self.client)
    }
    get!({
        doc: "# Get contactFolders from me",
        name: get_contact_folders,
        response: serde_json::Value,
        path: "/contactFolders/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property contactFolders in me",
        name: update_contact_folders,
        response: GraphResponse<Content>,
        path: "/contactFolders/{{RID}}",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get childFolders from me",
        name: list_child_folders,
        response: Collection<serde_json::Value>,
        path: "/contactFolders/{{RID}}/childFolders",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to childFolders for me",
        name: create_child_folders,
        response: serde_json::Value,
        path: "/contactFolders/{{RID}}/childFolders",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get childFolders from me",
        name: get_child_folders,
        response: serde_json::Value,
        path: "/contactFolders/{{RID}}/childFolders/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property childFolders in me",
        name: update_child_folders,
        response: GraphResponse<Content>,
        path: "/contactFolders/{{RID}}/childFolders/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get contacts from me",
        name: list_contacts,
        response: Collection<serde_json::Value>,
        path: "/contactFolders/{{RID}}/contacts",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to contacts for me",
        name: create_contacts,
        response: serde_json::Value,
        path: "/contactFolders/{{RID}}/contacts",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get contacts from me",
        name: get_contacts,
        response: serde_json::Value,
        path: "/contactFolders/{{RID}}/contacts/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property contacts in me",
        name: update_contacts,
        response: GraphResponse<Content>,
        path: "/contactFolders/{{RID}}/contacts/{{id}}",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> ContactsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/contactFolders/{{RID}}/contacts/delta()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get extensions from me",
        name: list_extensions,
        response: Collection<serde_json::Value>,
        path: "/contactFolders/{{RID}}/contacts/{{id}}/extensions",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to extensions for me",
        name: create_extensions,
        response: serde_json::Value,
        path: "/contactFolders/{{RID}}/contacts/{{id}}/extensions",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get extensions from me",
        name: get_extensions,
        response: serde_json::Value,
        path: "/contactFolders/{{RID}}/contacts/{{id}}/extensions/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property extensions in me",
        name: update_extensions,
        response: GraphResponse<Content>,
        path: "/contactFolders/{{RID}}/contacts/{{id}}/extensions/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get photo from me",
        name: get_photo,
        response: serde_json::Value,
        path: "/contactFolders/{{RID}}/contacts/{{id}}/photo",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property photo in me",
        name: update_photo,
        response: GraphResponse<Content>,
        path: "/contactFolders/{{RID}}/contacts/{{id}}/photo",
        params: 1,
        has_body: true
    });
}
