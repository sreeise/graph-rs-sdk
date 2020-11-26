use crate::client::Graph;
use crate::content_types::{ContentTypeRequest, ContentTypesRequest};
use crate::items::{ItemRequest, ItemsRequest};
use graph_core::resource::ResourceIdentity;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(ListRequest,);
register_client!(ListsRequest, ());

impl<'a, Client> ListRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> ListsRequest<'a, Client> {
        ListsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get lists from sites",
        name: list_lists,
        response: Collection<serde_json::Value>,
        path: "/lists",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to lists for sites",
        name: create_lists,
        response: serde_json::Value,
        path: "/lists",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> ListsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn content_types(&self) -> ContentTypeRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::ContentTypes);
        ContentTypeRequest::new(self.client)
    }
    pub fn content_type<ID: AsRef<str>>(&self, id: ID) -> ContentTypesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::ContentTypes);
        ContentTypesRequest::new(id.as_ref(), self.client)
    }
    pub fn items(&self) -> ItemRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Items);
        ItemRequest::new(self.client)
    }
    pub fn item<ID: AsRef<str>>(&self, id: ID) -> ItemsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Items);
        ItemsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get lists from sites",
        name: get_lists,
        response: serde_json::Value,
        path: "/lists/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property lists in sites",
        name: update_lists,
        response: GraphResponse<Content>,
        path: "/lists/{{RID}}",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get columns from sites",
        name: list_columns,
        response: Collection<serde_json::Value>,
        path: "/lists/{{RID}}/columns",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to columns for sites",
        name: create_columns,
        response: serde_json::Value,
        path: "/lists/{{RID}}/columns",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get columns from sites",
        name: get_columns,
        response: serde_json::Value,
        path: "/lists/{{RID}}/columns/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property columns in sites",
        name: update_columns,
        response: GraphResponse<Content>,
        path: "/lists/{{RID}}/columns/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get drive from sites",
        name: get_drive,
        response: serde_json::Value,
        path: "/lists/{{RID}}/drive",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property drive in sites",
        name: update_drive,
        response: GraphResponse<Content>,
        path: "/lists/{{RID}}/drive",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get subscriptions from sites",
        name: list_subscriptions,
        response: Collection<serde_json::Value>,
        path: "/lists/{{RID}}/subscriptions",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to subscriptions for sites",
        name: create_subscriptions,
        response: serde_json::Value,
        path: "/lists/{{RID}}/subscriptions",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get subscriptions from sites",
        name: get_subscriptions,
        response: serde_json::Value,
        path: "/lists/{{RID}}/subscriptions/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property subscriptions in sites",
        name: update_subscriptions,
        response: GraphResponse<Content>,
        path: "/lists/{{RID}}/subscriptions/{{id}}",
        params: 1,
        has_body: true
    });
}
