use crate::client::Graph;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(ItemRequest,);
register_client!(ItemsRequest, ());
register_client!(VersionsRequest,);

impl<'a, Client> ItemRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get items from sites",
        name: list_items,
        response: serde_json::Value,
        path: "/items",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to items for sites",
        name: create_items,
        response: serde_json::Value,
        path: "/items",
        params: 0,
        has_body: true
    });

    pub fn id<ID: AsRef<str>>(&self, id: ID) -> ItemsRequest<'a, Client> {
        ItemsRequest::new(id.as_ref(), self.client)
    }
}

impl<'a, Client> ItemsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get items from sites",
        name: get_items,
        response: serde_json::Value,
        path: "/items/{{RID}}",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property items in sites",
        name: update_items,
        response: NoContent,
        path: "/items/{{RID}}",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get analytics from sites",
        name: get_analytics,
        response: serde_json::Value,
        path: "/items/{{RID}}/analytics",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get driveItem from sites",
        name: get_drive_item,
        response: serde_json::Value,
        path: "/items/{{RID}}/driveItem",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property driveItem in sites",
        name: update_drive_item,
        response: NoContent,
        path: "/items/{{RID}}/driveItem",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get fields from sites",
        name: get_fields,
        response: serde_json::Value,
        path: "/items/{{RID}}/fields",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property fields in sites",
        name: update_fields,
        response: NoContent,
        path: "/items/{{RID}}/fields",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get versions from sites",
        name: list_versions,
        response: serde_json::Value,
        path: "/items/{{RID}}/versions",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to versions for sites",
        name: create_versions,
        response: serde_json::Value,
        path: "/items/{{RID}}/versions",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get versions from sites",
        name: get_versions,
        response: serde_json::Value,
        path: "/items/{{RID}}/versions/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property versions in sites",
        name: update_versions,
        response: NoContent,
        path: "/items/{{RID}}/versions/{{id}}",
        params: 1,
        has_body: true
    });

    pub fn versions(&self) -> VersionsRequest<'a, Client> {
        VersionsRequest::new(self.client)
    }
}

impl<'a, Client> VersionsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get fields from sites",
        name: get_fields,
        response: serde_json::Value,
        path: "/items/{{RID}}/versions/{{id}}/fields",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property fields in sites",
        name: update_fields,
        response: NoContent,
        path: "/items/{{RID}}/versions/{{id}}/fields",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action restoreVersion",
        name: restore_version,
        response: NoContent,
        path: "/items/{{RID}}/versions/{{id}}/restoreVersion",
        params: 1,
        has_body: false
    });
}
