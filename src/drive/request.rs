use crate::api_default_imports::*;
use crate::drives::{DrivesItemsApiClient, DrivesItemsIdApiClient, DrivesListApiClient};

resource_api_client!(DefaultDriveApiClient, ResourceIdentity::Drive);

/*
register_api_client!(DriveRequest);
register_api_client!(
    () DrivesRequest,
    drive_item => "drive/items", "items", ResourceIdentity::Drives,
    drive_root => "drive", "", ResourceIdentity::Drives,
    drive_root_path => "drive/root", "root", ResourceIdentity::Drives,
);

 */

impl DefaultDriveApiClient {
    get!(
       doc: "# Get drive",
       name: get_drive,
       path: "/drive"
    );
    patch!(
        doc: "# Update drive",
        name: update_drive,
        path: "/drive",
        body: true
    );
    get!(
       doc: "# Get following from drive",
       name: list_following,
       path: "/drive/following"
    );
    post!(
       doc: "# Create new navigation property to following for drive",
       name: create_following,
       path: "/drive/following",
       body: true
    );
    get!(
        name: list_root_children,
        path: "/drive/root/children"
    );
    post!(
       name: create_root_folder,
       path: "/drive/root/children",
       body: true
    );
    get!(
       name: list_root_activities,
       path: "/drive/activities"
    );
    get!(
       doc: "# Get following from drive",
       name: get_following,
       path: "/drive/following/{{id}}",
       params: drive_item_id
    );
    get!(
       doc: "# Get list from drive",
       name: get_list,
       path: "/drive/list",
       body: false
    );
    patch!(
       doc: "# Update the navigation property list in drive",
       name: update_list,
       path: "/drive/list",
       body: true
    );

    get!(
        doc: "# Invoke function recent",
        name: recent,
        path: "/drive/recent()"
    );

    get!(
        doc: "# Get root from drive",
        name: get_root,
        path: "/drive/root"
    );

    patch!(
        doc: "# Update the navigation property root in drive",
        name: update_root,
        path: "/drive/root"
    );

    get!(
        doc: "# Invoke function sharedWithMe",
        name: shared_with_me,
        path: "/drive/sharedWithMe()"
    );

    get!(
        doc: "# Get special from drive",
        name: list_special,
        path: "/drive/special"
    );

    post!(
        doc: "# Create new navigation property to special for drive",
        name: create_special,
        path: "/drive/special",
        body: true
    );

    get!(
        doc: "# Get special from drive",
        name: get_special,
        path: "/drive/special/{{id}}",
        params: drive_item_id
    );

    patch!(
        doc: "# Update the navigation property special in drive",
        name: update_special,
        path: "/drive/special/{{id}}",
        body: true,
        params: drive_item_id
    );
    get!(
       name: delta,
       path: "/drive/root/delta"
    );

    api_client_link!(items, DrivesItemsApiClient);
    api_client_link_id!(item, DrivesItemsIdApiClient);
    api_client_link!(lists, DrivesListApiClient);
}

//impl DriveIdApiClient {
/*
   get!({
       doc: "# Get entities from drives",
       name: list_drive,
       response: serde_json::Value,
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
       response: NoContent,
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
       response: serde_json::Value,
       path: "/{{drive_item_id}}/activities",
       params: 1,
       has_body: false
   });

   post!({
       name: check_in_item,
       response: NoContent,
       path: "/{{drive_item_id}}/checkin",
       params: 1,
       has_body: true
   });

   get!({
       name: list_children,
       response: serde_json::Value,
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

   /*
   put!({
       name: upload_replace,
       response: serde_json::Value,
       path: "/{{drive_item_id}}/content",
       params: 1,
       upload: true
   });
   */

   post!({
       name: copy_item,
       response: NoContent,
       path: "/{{drive_item_id}}/copy",
       params: 1,
       has_body: true
   });

   get!({
       name: get_thumbnail_binary,
       response: NoContent,
       path: "/{{drive_item_id}}/thumbnails/{{id2}}/{{id3}}/content",
       params: 3,
       has_body: false
   });

   get!({
       name: get_thumbnail,
       response: NoContent,
       path: "/{{drive_item_id}}/thumbnails/{{id2}}/{{id3}}",
       params: 3,
       has_body: false
   });

   get!({
       name: list_item_versions,
       response: serde_json::Value,
       path: "/{{drive_item_id}}/versions",
       params: 1,
       has_body: false
   });

   post!({
       name: restore_item_versions,
       response: NoContent,
       path: "/{{drive_item_id}}/versions/{{id2}}/restoreVersion",
       params: 2,
       has_body: false
   });

   get!({
       name: list_thumbnails,
       response: NoContent,
       path: "/{{drive_item}}/thumbnails",
       params: 0,
       has_body: false
   });

   get!({
       name: list_root_children,
       response: serde_json::Value,
       path: "/{{drive_root}}/root/children",
       params: 0,
       has_body: false
   });

   post!({
       name: create_root_folder,
       response: serde_json::Value,
       path: "/{{drive_root}}/root/children",
       params: 0,
       has_body: true
   });

   get!({
       name: list_root_activities,
       response: serde_json::Value,
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
       response: NoContent,
       path: "{{resource_drive_path}}/",
       params: 0,
       has_body: true
   });

   delete!({
       doc: "# Delete entity from drives",
       name: delete_drive,
       response: NoContent,
       path: "{{resource_drive_path}}/",
       params: 0,
       has_body: false
   });

   get!({
       doc: "# Get following from drives",
       name: list_following,
       response: serde_json::Value,
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
       response: NoContent,
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
       response: NoContent,
       path: "{{resource_drive_path}}/list",
       params: 0,
       has_body: true
   });

   get!({
       doc: "# Invoke function recent",
       name: recent,
       response: serde_json::Value,
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
       response: NoContent,
       path: "{{resource_drive_path}}/root",
       params: 0,
       has_body: true
   });

   get!({
       doc: "# Invoke function sharedWithMe",
       name: shared_with_me,
       response: serde_json::Value,
       path: "{{resource_drive_path}}/sharedWithMe()",
       params: 0,
       has_body: false
   });

   get!({
       doc: "# Get special from drives",
       name: list_special,
       response: serde_json::Value,
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
       response: NoContent,
       path: "{{resource_drive_path}}/special/{{id}}",
       params: 1,
       has_body: true
   });
*/

/*

api_client_link!(items, ResourceIdentity::Items, ItemsApiClient);

api_client_link!(item, ResourceIdentity::Items, ItemsIdApiClient);

api_client_link!(lists, ResourceIdentity::Lists, ListsApiClient);

api_client_link!(list, ResourceIdentity::Lists, ListsIdApiClient);

   pub fn items(&self) -> ItemRequest {
       self.transfer_identity();
       ItemRequest::new(self.client)
   }

   pub fn item<ID: AsRef<str>>(&self, id: ID) -> ItemsRequest {
       self.transfer_identity();
       self.client.set_ident(ResourceIdentity::Items);
       ItemsRequest::new(id.as_ref(), self.client)
   }

   pub fn lists(&self) -> ListRequest {
       self.transfer_identity();
       self.client.set_ident(ResourceIdentity::List);
       ListRequest::new(self.client)
   }

   pub fn list<ID: AsRef<str>>(&self, id: ID) -> ListsRequest {
       self.transfer_identity();
       self.client.set_ident(ResourceIdentity::Lists);
       ListsRequest::new(id.as_ref(), self.client)
   }
*/
//}
