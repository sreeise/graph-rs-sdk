use crate::client::Graph;
use crate::core::ResourceIdentity;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::types::DeltaPhantom;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(ChildFoldersRequest,);
register_client!(MailFolderRequest,);
register_client!(MailFoldersRequest, ());

impl<'a, Client> ChildFoldersRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/mailFolders/{{RID}}/childFolders/delta()",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action copy",
        name: copy,
        response: serde_json::Value,
        path: "/mailFolders/{{RID}}/childFolders/{{id}}/copy",
        params: 1,
        has_body: true
    });
}

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
        response: Collection<serde_json::Value>,
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
    pub fn child_folders(&self) -> ChildFoldersRequest<'a, Client> {
        ChildFoldersRequest::new(self.client)
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
        response: GraphResponse<Content>,
        path: "/mailFolders/{{RID}}",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get childFolders from me",
        name: list_child_folders,
        response: Collection<serde_json::Value>,
        path: "/mailFolders/{{RID}}/childFolders",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to childFolders for me",
        name: create_child_folders,
        response: serde_json::Value,
        path: "/mailFolders/{{RID}}/childFolders",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get childFolders from me",
        name: get_child_folders,
        response: serde_json::Value,
        path: "/mailFolders/{{RID}}/childFolders/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property childFolders in me",
        name: update_child_folders,
        response: GraphResponse<Content>,
        path: "/mailFolders/{{RID}}/childFolders/{{id}}",
        params: 1,
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
        response: Collection<serde_json::Value>,
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
        response: GraphResponse<Content>,
        path: "/mailFolders/{{RID}}/messageRules/{{id}}",
        params: 1,
        has_body: true
    });
}
