use crate::api_default_imports::*;
use crate::default_drive::*;
use crate::drives::*;

resource_api_client!(DefaultDriveApiClient, ResourceIdentity::Drive);

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
        doc: "Get bundles from drives",
        name: list_bundles,
        path: "/drive/bundles"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_bundles_count,
        path: "/drive/bundles/$count"
    );
    get!(
        doc: "Get bundles from drives",
        name: get_bundles,
        path: "/drive/bundles/{{id}}",
        params: drive_item_id
    );
    get!(
        doc: "Get content for the navigation property bundles from drives",
        name: get_bundles_content,
        path: "/drive/bundles/{{id}}/content",
        params: drive_item_id
    );
    put!(
        doc: "Update content for the navigation property bundles in drives",
        name: update_bundles_content,
        path: "/drive/bundles/{{id}}/content",
        body: true,
        params: drive_item_id
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
        doc: "Get the number of the resource",
        name: get_following_count,
        path: "/drives/{{RID}}/following/$count"
    );
    get!(
        doc: "Get following from drive",
        name: get_following,
        path: "/drive/following/{{id}}",
        params: drive_item_id
    );
    get!(
        doc: "Get content for the navigation property following from drive",
        name: get_following_content,
        path: "/drive/following/{{id}}/content",
        params: drive_item_id
    );
    put!(
        doc: "Update content for the navigation property following in drive",
        name: update_following_content,
        path: "/drive/following/{{id}}/content",
        body: true,
        params: drive_item_id
    );
    get!(
        name: list_root_children,
        path: "/drive/root/children"
    );
    get!(
       name: list_root_activities,
       path: "/drive/activities"
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
        doc: "Get content for the navigation property special from drive",
        name: get_special_content,
        path: "/drive/special/{{id}}/content",
        params: drive_item_id
    );
    put!(
        doc: "Update content for the navigation property special in drive",
        name: update_special_content,
        path: "/drive/special/{{id}}/content",
        body: true,
        params: drive_item_id
    );
    get!(
       name: root_delta,
       path: "/drive/root/delta"
    );
    get!(
        doc: "Invoke function search",
        name: search,
        path: "/drive/search(q='{{id}}')",
        params: q
    );
    get!(
        doc: "Get content for the navigation property root from drives",
        name: get_root_content,
        path: "/drive/root/content"
    );
    put!(
        doc: "Update content for the navigation property root in drives",
        name: update_root_content,
        path: "/drive/root/content",
        body: true
    );
    post!(
        doc: "Create drive item in root of drive",
        name: create_root_folder,
        path: "/drive/root/children",
        body: true
    );

    api_client_link!(items, DrivesItemsApiClient);
    api_client_link_id!(item, DrivesItemsIdApiClient);
    api_client_link_id!(item_by_path, DefaultDrivesItemsPathIdApiClient);
    api_client_link!(lists, DrivesListApiClient);
}
