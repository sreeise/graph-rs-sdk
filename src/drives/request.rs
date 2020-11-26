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
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> DrivesRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Drives);
        DrivesRequest::new(id.as_ref(), self.client)
    }
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
}

impl<'a, Client> DrivesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn items(&self) -> ItemRequest<'a, Client> {
        self.transfer_identity();
        self.client.set_ident(ResourceIdentity::Items);
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
        doc: "# Get following from drives",
        name: list_following,
        response: Collection<serde_json::Value>,
        path: "/following",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to following for drives",
        name: create_following,
        response: serde_json::Value,
        path: "/following",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get following from drives",
        name: get_following,
        response: serde_json::Value,
        path: "/following/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property following in drives",
        name: update_following,
        response: GraphResponse<Content>,
        path: "/following/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get list from drives",
        name: get_list,
        response: serde_json::Value,
        path: "/list",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property list in drives",
        name: update_list,
        response: GraphResponse<Content>,
        path: "/list",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function recent",
        name: recent,
        response: Collection<serde_json::Value>,
        path: "/recent()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get root from drives",
        name: get_root,
        response: serde_json::Value,
        path: "/root",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property root in drives",
        name: update_root,
        response: GraphResponse<Content>,
        path: "/root",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function sharedWithMe",
        name: shared_with_me,
        response: Collection<serde_json::Value>,
        path: "/sharedWithMe()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get special from drives",
        name: list_special,
        response: Collection<serde_json::Value>,
        path: "/special",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to special for drives",
        name: create_special,
        response: serde_json::Value,
        path: "/special",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get special from drives",
        name: get_special,
        response: serde_json::Value,
        path: "/special/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property special in drives",
        name: update_special,
        response: GraphResponse<Content>,
        path: "/special/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        name: list_thumbnails,
        response: GraphResponse<Content>,
        path: "/{{drive_item}}/thumbnails",
        params: 0,
        has_body: false
    });
    get!({
        name: get_items,
        response: serde_json::Value,
        path: "/{{drive_item}}/{{id}}",
        params: 1,
        has_body: false
    });
    post!({
        name: move_items,
        response: serde_json::Value,
        path: "/{{drive_item}}/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        name: delete_items,
        response: GraphResponse<Content>,
        path: "/{{drive_item}}/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        name: update_items,
        response: serde_json::Value,
        path: "/{{drive_item}}/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        name: get_activities,
        response: Collection<serde_json::Value>,
        path: "/{{drive_item}}/{{id}}/activities",
        params: 1,
        has_body: false
    });
    post!({
        name: check_in_item,
        response: GraphResponse<Content>,
        path: "/{{drive_item}}/{{id}}/checkin",
        params: 1,
        has_body: true
    });
    get!({
        name: list_children,
        response: Collection<serde_json::Value>,
        path: "/{{drive_item}}/{{id}}/children",
        params: 1,
        has_body: false
    });
    post!({
        name: create_folder,
        response: GraphResponse<Content>,
        path: "/{{drive_item}}/{{id}}/children",
        params: 1,
        has_body: false
    });
    get!({
        name: get_item_content,
        response: serde_json::Value,
        path: "/{{drive_item}}/{{id}}/content",
        params: 1,
        has_body: false
    });
    put!({
        name: upload_replace,
        response: serde_json::Value,
        path: "/{{drive_item}}/{{id}}/content",
        params: 1,
        upload: true
    });
    post!({
        name: copy_item,
        response: GraphResponse<Content>,
        path: "/{{drive_item}}/{{id}}/copy",
        params: 1,
        has_body: true
    });
    get!({
        name: get_thumbnail,
        response: GraphResponse<Content>,
        path: "/{{drive_item}}/{{id}}/thumbnails/{{id2]}/{{id3}}",
        params: 3,
        has_body: false
    });
    get!({
        name: get_thumbnail_binary,
        response: GraphResponse<Content>,
        path: "/{{drive_item}}/{{id}}/thumbnails/{{id2]}/{{id3}}/content",
        params: 3,
        has_body: false
    });
    get!({
        name: list_item_versions,
        response: Collection<serde_json::Value>,
        path: "/{{drive_item}}/{{id}}/versions",
        params: 1,
        has_body: true
    });
    post!({
        name: restore_item_versions,
        response: GraphResponse<Content>,
        path: "/{{drive_item}}/{{id}}/versions/{{id2}}/restoreVersion",
        params: 2,
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
        doc: "# Get entity from drives by key",
        name: get_drive,
        response: serde_json::Value,
        path: "{{drive_root}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update entity in drives",
        name: update_drive,
        response: GraphResponse<Content>,
        path: "{{drive_root}}",
        params: 0,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from drives",
        name: delete_drive,
        response: GraphResponse<Content>,
        path: "{{drive_root}}",
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
}
