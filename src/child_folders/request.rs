// GENERATED CODE

use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::messages::{MessageRequest, MessagesRequest};
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::types::DeltaPhantom;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(ChildFolderRequest,);
register_client!(ChildFoldersRequest, ());

impl<'a, Client> ChildFolderRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> ChildFoldersRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::ChildFolders);
        ChildFoldersRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get childFolders from me",
        name: list_child_folders,
        response: Collection<serde_json::Value>,
        path: "/childFolders",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to childFolders for me",
        name: create_child_folders,
        response: serde_json::Value,
        path: "/childFolders",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/childFolders/delta()",
        params: 0,
        has_body: false
    });
}

impl<'a, Client> ChildFoldersRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
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
        doc: "# Get childFolders from me",
        name: get_child_folders,
        response: serde_json::Value,
        path: "/childFolders/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property childFolders in me",
        name: update_child_folders,
        response: GraphResponse<Content>,
        path: "/childFolders/{{RID}}",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action copy",
        name: copy,
        response: serde_json::Value,
        path: "/childFolders/{{RID}}/copy",
        params: 0,
        has_body: true
    });
    post!({
        name: move_child_folders,
        response: serde_json::Value,
        path: "/childFolders/{{RID}}/move",
        params: 0,
        has_body: true
    });
}
