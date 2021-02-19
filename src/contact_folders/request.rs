// GENERATED CODE

use crate::child_folders::{ChildFolderRequest, ChildFoldersRequest};
use crate::client::Graph;
use crate::contacts::{ContactRequest, ContactsRequest};
use crate::core::ResourceIdentity;
use crate::extended_properties::ExtendedPropertiesRequest;
use graph_http::types::Collection;
use graph_http::types::DeltaPhantom;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(ContactFolderRequest,);
register_client!(ContactFoldersRequest, ());

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
    pub fn child_folders(&self) -> ChildFolderRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        ChildFolderRequest::new(self.client)
    }
    pub fn child_folder<ID: AsRef<str>>(&self, id: ID) -> ChildFoldersRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::ChildFolders);
        ChildFoldersRequest::new(id.as_ref(), self.client)
    }
    pub fn contacts(&self) -> ContactRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        ContactRequest::new(self.client)
    }
    pub fn contact<ID: AsRef<str>>(&self, id: ID) -> ContactsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Contacts);
        ContactsRequest::new(id.as_ref(), self.client)
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
        response: NoContent,
        path: "/contactFolders/{{RID}}",
        params: 0,
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
}
