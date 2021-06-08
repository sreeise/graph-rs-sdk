use crate::client::Graph;
use crate::content_types::{ContentTypeRequest, ContentTypesRequest};
use crate::core::ResourceIdentity;
use crate::drive::DrivesRequest;
use crate::lists::{ListRequest, ListsRequest};
use crate::onenote::OnenoteRequest;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(SiteRequest,);
register_client!(SitesRequest, ());

impl<'a, Client> SiteRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entities from sites",
        name: list_site,
        response: serde_json::Value,
        path: "/sites",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Add new entity to sites",
        name: create_site,
        response: serde_json::Value,
        path: "/sites",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action add",
        name: add,
        response: serde_json::Value,
        path: "/sites/add",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action remove",
        name: remove,
        response: serde_json::Value,
        path: "/sites/remove",
        params: 0,
        has_body: true
    });

    pub fn id<ID: AsRef<str>>(&self, id: ID) -> SitesRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Sites);
        SitesRequest::new(id.as_ref(), self.client)
    }
}

impl<'a, Client> SitesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entity from sites by key",
        name: get_site,
        response: serde_json::Value,
        path: "/sites/{{RID}}",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update entity in sites",
        name: update_site,
        response: NoContent,
        path: "/sites/{{RID}}",
        params: 0,
        has_body: true
    });

    delete!({
        doc: "# Delete entity from sites",
        name: delete_site,
        response: NoContent,
        path: "/sites/{{RID}}",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get analytics from sites",
        name: get_analytics,
        response: serde_json::Value,
        path: "/sites/{{RID}}/analytics",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get columns from sites",
        name: list_columns,
        response: serde_json::Value,
        path: "/sites/{{RID}}/columns",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to columns for sites",
        name: create_columns,
        response: serde_json::Value,
        path: "/sites/{{RID}}/columns",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get columns from sites",
        name: get_columns,
        response: serde_json::Value,
        path: "/sites/{{RID}}/columns/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property columns in sites",
        name: update_columns,
        response: NoContent,
        path: "/sites/{{RID}}/columns/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get drive from sites",
        name: get_drive,
        response: serde_json::Value,
        path: "/sites/{{RID}}/drive",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property drive in sites",
        name: update_drive,
        response: NoContent,
        path: "/sites/{{RID}}/drive",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get drives from sites",
        name: list_drives,
        response: serde_json::Value,
        path: "/sites/{{RID}}/drives",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to drives for sites",
        name: create_drives,
        response: serde_json::Value,
        path: "/sites/{{RID}}/drives",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get drives from sites",
        name: get_drives,
        response: serde_json::Value,
        path: "/sites/{{RID}}/drives/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property drives in sites",
        name: update_drives,
        response: NoContent,
        path: "/sites/{{RID}}/drives/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get items from sites",
        name: list_items,
        response: serde_json::Value,
        path: "/sites/{{RID}}/items",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to items for sites",
        name: create_items,
        response: serde_json::Value,
        path: "/sites/{{RID}}/items",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get items from sites",
        name: get_items,
        response: serde_json::Value,
        path: "/sites/{{RID}}/items/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property items in sites",
        name: update_items,
        response: NoContent,
        path: "/sites/{{RID}}/items/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get sites from sites",
        name: list_sites,
        response: serde_json::Value,
        path: "/sites/{{RID}}/sites",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to sites for sites",
        name: create_sites,
        response: serde_json::Value,
        path: "/sites/{{RID}}/sites",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get sites from sites",
        name: get_sites,
        response: serde_json::Value,
        path: "/sites/{{RID}}/sites/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property sites in sites",
        name: update_sites,
        response: NoContent,
        path: "/sites/{{RID}}/sites/{{id}}",
        params: 1,
        has_body: true
    });

    pub fn content_types(&self) -> ContentTypeRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        ContentTypeRequest::new(self.client)
    }

    pub fn content_type<ID: AsRef<str>>(&self, id: ID) -> ContentTypesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::ContentTypes);
        ContentTypesRequest::new(id.as_ref(), self.client)
    }

    pub fn drive(&self) -> DrivesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        DrivesRequest::new("", self.client)
    }

    pub fn lists(&self) -> ListRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        ListRequest::new(self.client)
    }

    pub fn list<ID: AsRef<str>>(&self, id: ID) -> ListsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Lists);
        ListsRequest::new(id.as_ref(), self.client)
    }

    pub fn onenote(&self) -> OnenoteRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Onenote);
        OnenoteRequest::new(self.client)
    }
}
