use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::items::{ItemRequest, ItemsRequest};
use crate::lists::{ListRequest, ListsRequest};
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::types::DeltaPhantom;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;
use std::path::Path;

register_client!(DriveRequest,);
register_client!(
    () DrivesRequest,
    drive_item => "drive/items", "items", ResourceIdentity::Drives,
    drive_root => "drive", "", ResourceIdentity::Drives,
    drive_root_path => "drive/root", "root", ResourceIdentity::Drives,
);
impl<'a, Client> DriveRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn items(&self) -> ItemRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        ItemRequest::new(self.client)
    }
    pub fn lists(&self) -> ListRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        ListRequest::new(self.client)
    }
    get!({
        doc: "# Get drive",
        name: get_drive,
        response: serde_json::Value,
        path: "/drive",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update drive",
        name: update_drive,
        response: GraphResponse<Content>,
        path: "/drive",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get following from drive",
        name: list_following,
        response: Collection<serde_json::Value>,
        path: "/drive/following",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to following for drive",
        name: create_following,
        response: serde_json::Value,
        path: "/drive/following",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get following from drive",
        name: get_following,
        response: serde_json::Value,
        path: "/drive/following/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property following in drive",
        name: update_following,
        response: GraphResponse<Content>,
        path: "/drive/following/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get list from drive",
        name: get_list,
        response: serde_json::Value,
        path: "/drive/list",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property list in drive",
        name: update_list,
        response: GraphResponse<Content>,
        path: "/drive/list",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function recent",
        name: recent,
        response: Collection<serde_json::Value>,
        path: "/drive/recent()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get root from drive",
        name: get_root,
        response: serde_json::Value,
        path: "/drive/root",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property root in drive",
        name: update_root,
        response: GraphResponse<Content>,
        path: "/drive/root",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function sharedWithMe",
        name: shared_with_me,
        response: Collection<serde_json::Value>,
        path: "/drive/sharedWithMe()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get special from drive",
        name: list_special,
        response: Collection<serde_json::Value>,
        path: "/drive/special",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to special for drive",
        name: create_special,
        response: serde_json::Value,
        path: "/drive/special",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get special from drive",
        name: get_special,
        response: serde_json::Value,
        path: "/drive/special/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property special in drive",
        name: update_special,
        response: GraphResponse<Content>,
        path: "/drive/special/{{id}}",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> DrivesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn items(&self) -> ItemRequest<'a, Client> {
        self.transfer_identity();
        ItemRequest::new(self.client)
    }
    pub fn item<ID: AsRef<str>>(&self, id: ID) -> ItemsRequest<'a, Client> {
        self.transfer_identity();
        self.client.set_ident(ResourceIdentity::Items);
        ItemsRequest::new(id.as_ref(), self.client)
    }
    pub fn lists(&self) -> ListRequest<'a, Client> {
        self.transfer_identity();
        self.client.set_ident(ResourceIdentity::List);
        ListRequest::new(self.client)
    }
    pub fn list<ID: AsRef<str>>(&self, id: ID) -> ListsRequest<'a, Client> {
        self.transfer_identity();
        self.client.set_ident(ResourceIdentity::Lists);
        ListsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get entities from drives",
        name: list_drive,
        response: Collection<serde_json::Value>,
        path: "/drives",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Add new entity to drives",
        name: create_drive,
        response: serde_json::Value,
        path: "/drives",
        params: 0,
        has_body: true
    });
    get!({
        name: get_items,
        response: serde_json::Value,
        path: "/{{drive_item_id}}",
        params: 1,
        has_body: false
    });
    post!({
        name: move_items,
        response: serde_json::Value,
        path: "/{{drive_item_id}}",
        params: 1,
        has_body: true
    });
    delete!({
        name: delete_items,
        response: GraphResponse<Content>,
        path: "/{{drive_item_id}}",
        params: 1,
        has_body: false
    });
    patch!({
        name: update_items,
        response: serde_json::Value,
        path: "/{{drive_item_id}}",
        params: 1,
        has_body: true
    });
    get!({
        name: get_item_activities,
        response: Collection<serde_json::Value>,
        path: "/{{drive_item_id}}/activities",
        params: 1,
        has_body: false
    });
    post!({
        name: check_in_item,
        response: GraphResponse<Content>,
        path: "/{{drive_item_id}}/checkin",
        params: 1,
        has_body: true
    });
    get!({
        name: list_children,
        response: Collection<serde_json::Value>,
        path: "/{{drive_item_id}}/children",
        params: 1,
        has_body: false
    });
    post!({
        name: create_folder,
        response: serde_json::Value,
        path: "/{{drive_item_id}}/children",
        params: 1,
        has_body: true
    });
    get!({
        name: get_item_content,
        response: serde_json::Value,
        path: "/{{drive_item_id}}/content",
        params: 1,
        has_body: false
    });
    put!({
        name: upload_replace,
        response: serde_json::Value,
        path: "/{{drive_item_id}}/content",
        params: 1,
        upload: true
    });
    post!({
        name: copy_item,
        response: GraphResponse<Content>,
        path: "/{{drive_item_id}}/copy",
        params: 1,
        has_body: true
    });
    get!({
        name: get_thumbnail_binary,
        response: GraphResponse<Content>,
        path: "/{{drive_item_id}}/thumbnails/{{id2}}/{{id3}}/content",
        params: 3,
        has_body: false
    });
    get!({
        name: get_thumbnail,
        response: GraphResponse<Content>,
        path: "/{{drive_item_id}}/thumbnails/{{id}}/{{id2}}",
        params: 3,
        has_body: false
    });
    get!({
        name: list_item_versions,
        response: Collection<serde_json::Value>,
        path: "/{{drive_item_id}}/versions",
        params: 1,
        has_body: false
    });
    post!({
        name: restore_item_versions,
        response: GraphResponse<Content>,
        path: "/{{drive_item_id}}/versions/{{id2}}/restoreVersion",
        params: 2,
        has_body: false
    });
    get!({
        name: list_thumbnails,
        response: GraphResponse<Content>,
        path: "/{{drive_item}}/thumbnails",
        params: 0,
        has_body: false
    });
    get!({
        name: list_root_children,
        response: Collection<serde_json::Value>,
        path: "/{{drive_root}}/root/children",
        params: 0,
        has_body: false
    });
    get!({
        name: list_root_activities,
        response: Collection<serde_json::Value>,
        path: "{{drive_root}}/activities",
        params: 0,
        has_body: false
    });
    get!({
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "{{drive_root}}/root/delta",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get entity from drives by key",
        name: get_drive,
        response: serde_json::Value,
        path: "{{resource_drive_path}}/",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update entity in drives",
        name: update_drive,
        response: GraphResponse<Content>,
        path: "{{resource_drive_path}}/",
        params: 0,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from drives",
        name: delete_drive,
        response: GraphResponse<Content>,
        path: "{{resource_drive_path}}/",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get following from drives",
        name: list_following,
        response: Collection<serde_json::Value>,
        path: "{{resource_drive_path}}/following",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to following for drives",
        name: create_following,
        response: serde_json::Value,
        path: "{{resource_drive_path}}/following",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get following from drives",
        name: get_following,
        response: serde_json::Value,
        path: "{{resource_drive_path}}/following/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property following in drives",
        name: update_following,
        response: GraphResponse<Content>,
        path: "{{resource_drive_path}}/following/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get list from drives",
        name: get_list,
        response: serde_json::Value,
        path: "{{resource_drive_path}}/list",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property list in drives",
        name: update_list,
        response: GraphResponse<Content>,
        path: "{{resource_drive_path}}/list",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function recent",
        name: recent,
        response: Collection<serde_json::Value>,
        path: "{{resource_drive_path}}/recent()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get root from drives",
        name: get_root,
        response: serde_json::Value,
        path: "{{resource_drive_path}}/root",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property root in drives",
        name: update_root,
        response: GraphResponse<Content>,
        path: "{{resource_drive_path}}/root",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function sharedWithMe",
        name: shared_with_me,
        response: Collection<serde_json::Value>,
        path: "{{resource_drive_path}}/sharedWithMe()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get special from drives",
        name: list_special,
        response: Collection<serde_json::Value>,
        path: "{{resource_drive_path}}/special",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to special for drives",
        name: create_special,
        response: serde_json::Value,
        path: "{{resource_drive_path}}/special",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get special from drives",
        name: get_special,
        response: serde_json::Value,
        path: "{{resource_drive_path}}/special/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property special in drives",
        name: update_special,
        response: GraphResponse<Content>,
        path: "{{resource_drive_path}}/special/{{id}}",
        params: 1,
        has_body: true
    });
}
