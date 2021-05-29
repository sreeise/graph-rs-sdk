// GENERATED CODE

use crate::{
    client::Graph, core::ResourceIdentity, extended_properties::ExtendedPropertiesRequest,
};
use graph_http::{
    types::{DeltaPhantom, NoContent},
    IntoResponse,
};
use handlebars::*;
use reqwest::Method;

register_client!(ContactRequest,);
register_client!(ContactsRequest, ());

impl<'a, Client> ContactRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get contacts from me",
        name: list_contacts,
        response: serde_json::Value,
        path: "/contacts",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to contacts for me",
        name: create_contacts,
        response: serde_json::Value,
        path: "/contacts",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/contacts/delta()",
        params: 0,
        has_body: false
    });

    pub fn id<ID: AsRef<str>>(&self, id: ID) -> ContactsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Contacts);
        ContactsRequest::new(id.as_ref(), self.client)
    }
}

impl<'a, Client> ContactsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get contacts from me",
        name: get_contacts,
        response: serde_json::Value,
        path: "/contacts/{{RID}}",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property contacts in me",
        name: update_contacts,
        response: NoContent,
        path: "/contacts/{{RID}}",
        params: 0,
        has_body: true
    });

    delete!({
        name: delete_contacts,
        response: NoContent,
        path: "/contacts/{{RID}}",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get extensions from me",
        name: list_extensions,
        response: serde_json::Value,
        path: "/contacts/{{RID}}/extensions",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to extensions for me",
        name: create_extensions,
        response: serde_json::Value,
        path: "/contacts/{{RID}}/extensions",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get extensions from me",
        name: get_extensions,
        response: serde_json::Value,
        path: "/contacts/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property extensions in me",
        name: update_extensions,
        response: NoContent,
        path: "/contacts/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get photo from me",
        name: get_photo,
        response: serde_json::Value,
        path: "/contacts/{{RID}}/photo",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property photo in me",
        name: update_photo,
        response: NoContent,
        path: "/contacts/{{RID}}/photo",
        params: 0,
        has_body: true
    });

    pub fn extended_properties(&self) -> ExtendedPropertiesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::ExtendedProperties);
        ExtendedPropertiesRequest::new(self.client)
    }
}
