// GENERATED CODE

use crate::api_default_imports::*;
use crate::drives::{DrivesItemsApiClient, DrivesItemsIdApiClient, DrivesListApiClient};

resource_api_client!(DrivesApiClient, DrivesIdApiClient, ResourceIdentity::Drives);

impl DrivesApiClient {
    post!(
        doc: "Add new entity to drives",
        name: create_drive,
        path: "/drives",
        body: true
    );
    get!(
        doc: "Get Drive",
        name: list_drive,
        path: "/drives"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_drives_count,
        path: "/drives/$count"
    );
}

impl DrivesIdApiClient {
    api_client_link_id!(item, DrivesItemsIdApiClient);
    api_client_link!(items, DrivesItemsApiClient);
    api_client_link_id!(list, DrivesListApiClient);

    delete!(
        doc: "Delete entity from drives",
        name: delete_drive,
        path: "/drives/{{RID}}"
    );
    get!(
        doc: "Get Drive",
        name: get_drive,
        path: "/drives/{{RID}}"
    );
    patch!(
        doc: "Update entity in drives",
        name: update_drive,
        path: "/drives/{{RID}}",
        body: true
    );
    get!(
        doc: "Get bundles from drives",
        name: list_bundles,
        path: "/drives/{{RID}}/bundles"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_bundles_count,
        path: "/drives/{{RID}}/bundles/$count"
    );
    get!(
        doc: "Get bundles from drives",
        name: get_bundles,
        path: "/drives/{{RID}}/bundles/{{id}}",
        params: drive_item_id
    );
    get!(
        doc: "Get content for the navigation property bundles from drives",
        name: get_bundles_content,
        path: "/drives/{{RID}}/bundles/{{id}}/content",
        params: drive_item_id
    );
    put!(
        doc: "Update content for the navigation property bundles in drives",
        name: update_bundles_content,
        path: "/drives/{{RID}}/bundles/{{id}}/content",
        body: true,
        params: drive_item_id
    );
    get!(
        doc: "List followed items",
        name: list_following,
        path: "/drives/{{RID}}/following"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_following_count,
        path: "/drives/{{RID}}/following/$count"
    );
    get!(
        doc: "Get following from drives",
        name: get_following,
        path: "/drives/{{RID}}/following/{{id}}",
        params: drive_item_id
    );
    get!(
        doc: "Get content for the navigation property following from drives",
        name: get_following_content,
        path: "/drives/{{RID}}/following/{{id}}/content",
        params: drive_item_id
    );
    put!(
        doc: "Update content for the navigation property following in drives",
        name: update_following_content,
        path: "/drives/{{RID}}/following/{{id}}/content",
        body: true,
        params: drive_item_id
    );
    get!(
        doc: "Invoke function recent",
        name: recent,
        path: "/drives/{{RID}}/recent()"
    );
    get!(
        doc: "Get a driveItem resource",
        name: get_root,
        path: "/drives/{{RID}}/root"
    );
    get!(
        doc: "Get content for the navigation property root from drives",
        name: get_root_content,
        path: "/drives/{{RID}}/root/content"
    );
    put!(
        doc: "Update content for the navigation property root in drives",
        name: update_root_content,
        path: "/drives/{{RID}}/root/content",
        body: true
    );
    get!(
        doc: "Invoke function search",
        name: search,
        path: "/drives/{{RID}}/search(q='{{id}}')",
        params: q
    );
    get!(
        doc: "Invoke function sharedWithMe",
        name: shared_with_me,
        path: "/drives/{{RID}}/sharedWithMe()"
    );
    get!(
        doc: "Get special from drives",
        name: list_special,
        path: "/drives/{{RID}}/special"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_special_count,
        path: "/drives/{{RID}}/special/$count"
    );
    get!(
        doc: "Get special from drives",
        name: get_special,
        path: "/drives/{{RID}}/special/{{id}}",
        params: drive_item_id
    );
    get!(
        doc: "Get content for the navigation property special from drives",
        name: get_special_content,
        path: "/drives/{{RID}}/special/{{id}}/content",
        params: drive_item_id
    );
    put!(
        doc: "Update content for the navigation property special in drives",
        name: update_special_content,
        path: "/drives/{{RID}}/special/{{id}}/content",
        body: true,
        params: drive_item_id
    );
}
