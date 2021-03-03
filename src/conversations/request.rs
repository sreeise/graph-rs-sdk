use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::threads::{ThreadRequest, ThreadsRequest};
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(ConversationRequest,);
register_client!(ConversationsRequest, ());

impl<'a, Client> ConversationRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> ConversationsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Conversations);
        ConversationsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get conversations from groups",
        name: list_conversations,
        response: serde_json::Value,
        path: "/conversations",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to conversations for groups",
        name: create_conversations,
        response: serde_json::Value,
        path: "/conversations",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> ConversationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn threads(&self) -> ThreadRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        ThreadRequest::new(self.client)
    }
    pub fn thread<ID: AsRef<str>>(&self, id: ID) -> ThreadsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Threads);
        ThreadsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get conversations from groups",
        name: get_conversations,
        response: serde_json::Value,
        path: "/conversations/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property conversations in groups",
        name: update_conversations,
        response: NoContent,
        path: "/conversations/{{RID}}",
        params: 0,
        has_body: true
    });
    delete!({
        name: delete_conversations,
        response: NoContent,
        path: "/conversations/{{RID}}",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get threads from groups",
        name: list_threads,
        response: serde_json::Value,
        path: "/conversations/{{RID}}/threads",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to threads for groups",
        name: create_threads,
        response: serde_json::Value,
        path: "/conversations/{{RID}}/threads",
        params: 0,
        has_body: true
    });
}
