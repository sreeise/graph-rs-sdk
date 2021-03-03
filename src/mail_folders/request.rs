// GENERATED CODE

use crate::child_folders::{ChildFolderRequest, ChildFoldersRequest};
use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::extended_properties::ExtendedPropertiesRequest;
use crate::messages::{MessageRequest, MessagesRequest};
use graph_http::types::DeltaPhantom;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(MailFolderRequest,);
register_client!(MailFoldersRequest, ());

impl<'a, Client> MailFolderRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> MailFoldersRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::MailFolders);
        MailFoldersRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get mailFolders from me",
        name: list_mail_folders,
        response: serde_json::Value,
        path: "/mailFolders",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to mailFolders for me",
        name: create_mail_folders,
        response: serde_json::Value,
        path: "/mailFolders",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/mailFolders/delta()",
        params: 0,
        has_body: false
    });
}

impl<'a, Client> MailFoldersRequest<'a, Client>
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
    pub fn extended_properties(&self) -> ExtendedPropertiesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::ExtendedProperties);
        ExtendedPropertiesRequest::new(self.client)
    }
    pub fn messages(&self) -> MessageRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        MessageRequest::new(self.client)
    }
    pub fn message<ID: AsRef<str>>(&self, id: ID) -> MessagesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Messages);
        MessagesRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get mailFolders from me",
        name: get_mail_folders,
        response: serde_json::Value,
        path: "/mailFolders/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property mailFolders in me",
        name: update_mail_folders,
        response: NoContent,
        path: "/mailFolders/{{RID}}",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action copy",
        name: copy,
        response: serde_json::Value,
        path: "/mailFolders/{{RID}}/copy",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get messageRules from me",
        name: list_message_rules,
        response: serde_json::Value,
        path: "/mailFolders/{{RID}}/messageRules",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to messageRules for me",
        name: create_message_rules,
        response: serde_json::Value,
        path: "/mailFolders/{{RID}}/messageRules",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get messageRules from me",
        name: get_message_rules,
        response: serde_json::Value,
        path: "/mailFolders/{{RID}}/messageRules/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property messageRules in me",
        name: update_message_rules,
        response: NoContent,
        path: "/mailFolders/{{RID}}/messageRules/{{id}}",
        params: 1,
        has_body: true
    });
}
