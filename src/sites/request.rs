use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::{GraphResponse, IntoResponse};
use reqwest::Method;

register_client!(SitesRequest,);
register_client!(SiteRequest,);
register_client!(VersionsRequest,);
register_client!(ItemsRequest,);
register_client!(ListsRequest,);

#[allow(dead_code)]
impl<'a, Client> SitesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn lists(&self) -> ListsRequest<'a, Client> {
        ListsRequest::new(&self.client)
    }
    get!({
        doc: "# Get lists from sites",
        name: list_lists,
        response: Collection<serde_json::Value>,
        path: "/sites/{{RID}}/lists",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to lists for sites",
        name: create_lists,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists",
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
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/columns/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property columns for sites",
        name: delete_columns,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/columns/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get contentTypes from sites",
        name: list_content_types,
        response: Collection<serde_json::Value>,
        path: "/sites/{{RID}}/contentTypes",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to contentTypes for sites",
        name: create_content_types,
        response: serde_json::Value,
        path: "/sites/{{RID}}/contentTypes",
        params: 0,
        has_body: true
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
        doc: "# Get ref of analytics from sites",
        name: get_ref_analytics,
        response: serde_json::Value,
        path: "/sites/{{RID}}/analytics/$ref",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the ref of navigation property analytics in sites",
        name: update_ref_analytics,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/analytics/$ref",
        params: 0,
        has_body: true
    });
    delete!({
        doc: "# Delete ref of navigation property analytics for sites",
        name: delete_ref_analytics,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/analytics/$ref",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Invoke function getByPath",
        name: get_by_path,
        response: serde_json::Value,
        path: "/sites/{{RID}}/microsoft.graph.getByPath(path={path})",
        params: 1,
        has_body: false
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
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/items/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property items for sites",
        name: delete_items,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/items/{{id}}",
        params: 1,
        has_body: false
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
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/drives/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property drives for sites",
        name: delete_drives,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/drives/{{id}}",
        params: 1,
        has_body: false
    });
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
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}",
        params: 0,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from sites",
        name: delete_site,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}",
        params: 0,
        has_body: false
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
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/drive",
        params: 0,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property drive for sites",
        name: delete_drive,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/drive",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get columnLinks from sites",
        name: list_column_links,
        response: Collection<serde_json::Value>,
        path: "/sites/{{RID}}/contentTypes/{{id}}/columnLinks",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to columnLinks for sites",
        name: create_column_links,
        response: serde_json::Value,
        path: "/sites/{{RID}}/contentTypes/{{id}}/columnLinks",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get columnLinks from sites",
        name: get_column_links,
        response: serde_json::Value,
        path: "/sites/{{RID}}/contentTypes/{{id}}/columnLinks/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property columnLinks in sites",
        name: update_column_links,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/contentTypes/{{id}}/columnLinks/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property columnLinks for sites",
        name: delete_column_links,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/contentTypes/{{id}}/columnLinks/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get columns from sites",
        name: list_columns,
        response: Collection<serde_json::Value>,
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
        doc: "# Get lists from sites",
        name: get_lists,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property lists in sites",
        name: update_lists,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property lists for sites",
        name: delete_lists,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get items from sites",
        name: list_items,
        response: Collection<serde_json::Value>,
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
        doc: "# Get sites from sites",
        name: list_sites,
        response: Collection<serde_json::Value>,
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
        doc: "# Get contentTypes from sites",
        name: get_content_types,
        response: serde_json::Value,
        path: "/sites/{{RID}}/contentTypes/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property contentTypes in sites",
        name: update_content_types,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/contentTypes/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property contentTypes for sites",
        name: delete_content_types,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/contentTypes/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get drives from sites",
        name: list_drives,
        response: Collection<serde_json::Value>,
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
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/sites/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property sites for sites",
        name: delete_sites,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/sites/{{id}}",
        params: 1,
        has_body: false
    });
}

#[allow(dead_code)]
impl<'a, Client> ItemsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn versions(&self) -> VersionsRequest<'a, Client> {
        VersionsRequest::new(&self.client)
    }
    get!({
        doc: "# Get ref of analytics from sites",
        name: get_ref_analytics,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}/analytics/$ref",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the ref of navigation property analytics in sites",
        name: update_ref_analytics,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}/analytics/$ref",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete ref of navigation property analytics for sites",
        name: delete_ref_analytics,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}/analytics/$ref",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get versions from sites",
        name: get_versions,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}/versions/{{id3}}",
        params: 3,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property versions in sites",
        name: update_versions,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}/versions/{{id3}}",
        params: 3,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property versions for sites",
        name: delete_versions,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}/versions/{{id3}}",
        params: 3,
        has_body: false
    });
    get!({
        doc: "# Get versions from sites",
        name: list_versions,
        response: Collection<serde_json::Value>,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}/versions",
        params: 2,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to versions for sites",
        name: create_versions,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}/versions",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get analytics from sites",
        name: get_analytics,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}/analytics",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get driveItem from sites",
        name: get_drive_item,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}/driveItem",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property driveItem in sites",
        name: update_drive_item,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}/driveItem",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property driveItem for sites",
        name: delete_drive_item,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}/driveItem",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get fields from sites",
        name: get_fields,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}/fields",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property fields in sites",
        name: update_fields,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}/fields",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property fields for sites",
        name: delete_fields,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}/fields",
        params: 2,
        has_body: false
    });
}

#[allow(dead_code)]
impl<'a, Client> ListsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn items(&self) -> ItemsRequest<'a, Client> {
        ItemsRequest::new(&self.client)
    }
    get!({
        doc: "# Get subscriptions from sites",
        name: get_subscriptions,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists/{{id}}/subscriptions/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property subscriptions in sites",
        name: update_subscriptions,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/subscriptions/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property subscriptions for sites",
        name: delete_subscriptions,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/subscriptions/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get columnLinks from sites",
        name: get_column_links,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists/{{id}}/contentTypes/{{id2}}/columnLinks/{{id3}}",
        params: 3,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property columnLinks in sites",
        name: update_column_links,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/contentTypes/{{id2}}/columnLinks/{{id3}}",
        params: 3,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property columnLinks for sites",
        name: delete_column_links,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/contentTypes/{{id2}}/columnLinks/{{id3}}",
        params: 3,
        has_body: false
    });
    get!({
        doc: "# Get columnLinks from sites",
        name: list_column_links,
        response: Collection<serde_json::Value>,
        path: "/sites/{{RID}}/lists/{{id}}/contentTypes/{{id2}}/columnLinks",
        params: 2,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to columnLinks for sites",
        name: create_column_links,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists/{{id}}/contentTypes/{{id2}}/columnLinks",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get drive from sites",
        name: get_drive,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists/{{id}}/drive",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property drive in sites",
        name: update_drive,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/drive",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property drive for sites",
        name: delete_drive,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/drive",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get items from sites",
        name: get_items,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property items in sites",
        name: update_items,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property items for sites",
        name: delete_items,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get columns from sites",
        name: get_columns,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists/{{id}}/columns/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property columns in sites",
        name: update_columns,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/columns/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property columns for sites",
        name: delete_columns,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/columns/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get items from sites",
        name: list_items,
        response: Collection<serde_json::Value>,
        path: "/sites/{{RID}}/lists/{{id}}/items",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to items for sites",
        name: create_items,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists/{{id}}/items",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get contentTypes from sites",
        name: list_content_types,
        response: Collection<serde_json::Value>,
        path: "/sites/{{RID}}/lists/{{id}}/contentTypes",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to contentTypes for sites",
        name: create_content_types,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists/{{id}}/contentTypes",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get columns from sites",
        name: list_columns,
        response: Collection<serde_json::Value>,
        path: "/sites/{{RID}}/lists/{{id}}/columns",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to columns for sites",
        name: create_columns,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists/{{id}}/columns",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get contentTypes from sites",
        name: get_content_types,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists/{{id}}/contentTypes/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property contentTypes in sites",
        name: update_content_types,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/contentTypes/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property contentTypes for sites",
        name: delete_content_types,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/contentTypes/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get subscriptions from sites",
        name: list_subscriptions,
        response: Collection<serde_json::Value>,
        path: "/sites/{{RID}}/lists/{{id}}/subscriptions",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to subscriptions for sites",
        name: create_subscriptions,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists/{{id}}/subscriptions",
        params: 1,
        has_body: true
    });
}

#[allow(dead_code)]
impl<'a, Client> VersionsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get fields from sites",
        name: get_fields,
        response: serde_json::Value,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}/versions/{{id3}}/fields",
        params: 3,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property fields in sites",
        name: update_fields,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}/versions/{{id3}}/fields",
        params: 3,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property fields for sites",
        name: delete_fields,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}/versions/{{id3}}/fields",
        params: 3,
        has_body: false
    });
    post!({
        doc: "# Invoke action restoreVersion",
        name: restore_version,
        response: GraphResponse<Content>,
        path: "/sites/{{RID}}/lists/{{id}}/items/{{id2}}/versions/{{id3}}/microsoft.graph.restoreVersion",
        params: 3,
        has_body: false
    });
}

#[allow(dead_code)]
impl<'a, Client> SiteRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action remove",
        name: remove,
        response: Collection<serde_json::Value>,
        path: "/sites/microsoft.graph.remove",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action add",
        name: add,
        response: Collection<serde_json::Value>,
        path: "/sites/microsoft.graph.add",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get entities from sites",
        name: list_site,
        response: Collection<serde_json::Value>,
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
}
