use crate::client::Graph;
use graph_http::{types::NoContent, IntoResponse};
use handlebars::*;
use reqwest::Method;

register_client!(ContentTypeRequest,);
register_client!(ContentTypesRequest, ());

impl<'a, Client> ContentTypeRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get contentTypes from sites",
        name: list_content_types,
        response: serde_json::Value,
        path: "/contentTypes",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to contentTypes for sites",
        name: create_content_types,
        response: serde_json::Value,
        path: "/contentTypes",
        params: 0,
        has_body: true
    });

    pub fn id<ID: AsRef<str>>(&self, id: ID) -> ContentTypesRequest<'a, Client> {
        ContentTypesRequest::new(id.as_ref(), self.client)
    }
}

impl<'a, Client> ContentTypesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get contentTypes from sites",
        name: get_content_types,
        response: serde_json::Value,
        path: "/contentTypes/{{RID}}",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property contentTypes in sites",
        name: update_content_types,
        response: NoContent,
        path: "/contentTypes/{{RID}}",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get columnLinks from sites",
        name: list_column_links,
        response: serde_json::Value,
        path: "/contentTypes/{{RID}}/columnLinks",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to columnLinks for sites",
        name: create_column_links,
        response: serde_json::Value,
        path: "/contentTypes/{{RID}}/columnLinks",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get columnLinks from sites",
        name: get_column_links,
        response: serde_json::Value,
        path: "/contentTypes/{{RID}}/columnLinks/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property columnLinks in sites",
        name: update_column_links,
        response: NoContent,
        path: "/contentTypes/{{RID}}/columnLinks/{{id}}",
        params: 1,
        has_body: true
    });
}
