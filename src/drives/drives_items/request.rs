// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    DrivesItemsApiClient,
    DrivesItemsIdApiClient,
    ResourceIdentity::DrivesItems
);

impl DrivesItemsApiClient {
    post!(
        doc: "Create new navigation property to items for drives",
        name: create_items,
        path: "/items",
        body: true
    );
    get!(
        doc: "Get items from drives",
        name: list_items,
        path: "/items"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_items_count,
        path: "/items/$count"
    );
}

impl DrivesItemsIdApiClient {
    delete!(
        doc: "Delete navigation property items for drives",
        name: delete_items,
        path: "/items/{{RID}}"
    );
    get!(
        doc: "Get items from drives",
        name: get_items,
        path: "/items/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property items in drives",
        name: update_items,
        path: "/items/{{RID}}",
        body: true
    );
    delete!(
        doc: "Delete navigation property analytics for drives",
        name: delete_analytics,
        path: "/items/{{RID}}/analytics"
    );
    get!(
        doc: "Get analytics from drives",
        name: get_analytics,
        path: "/items/{{RID}}/analytics"
    );
    patch!(
        doc: "Update the navigation property analytics in drives",
        name: update_analytics,
        path: "/items/{{RID}}/analytics",
        body: true
    );
    get!(
        doc: "Get itemAnalytics",
        name: get_all_time,
        path: "/items/{{RID}}/analytics/allTime"
    );
    post!(
        doc: "Create new navigation property to itemActivityStats for drives",
        name: create_item_activity_stats,
        path: "/items/{{RID}}/analytics/itemActivityStats",
        body: true
    );
    get!(
        doc: "Get itemActivityStats from drives",
        name: list_item_activity_stats,
        path: "/items/{{RID}}/analytics/itemActivityStats"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_item_activity_stats_count,
        path: "/items/{{RID}}/analytics/itemActivityStats/$count"
    );
    delete!(
        doc: "Delete navigation property itemActivityStats for drives",
        name: delete_item_activity_stats,
        path: "/items/{{RID}}/analytics/itemActivityStats/{{id}}",
        params: item_activity_stat_id
    );
    get!(
        doc: "Get itemActivityStats from drives",
        name: get_item_activity_stats,
        path: "/items/{{RID}}/analytics/itemActivityStats/{{id}}",
        params: item_activity_stat_id
    );
    patch!(
        doc: "Update the navigation property itemActivityStats in drives",
        name: update_item_activity_stats,
        path: "/items/{{RID}}/analytics/itemActivityStats/{{id}}",
        body: true,
        params: item_activity_stat_id
    );
    post!(
        doc: "Create new navigation property to activities for drives",
        name: create_activities,
        path: "/items/{{RID}}/analytics/itemActivityStats/{{id}}/activities",
        body: true,
        params: item_activity_stat_id
    );
    get!(
        doc: "Get activities from drives",
        name: list_activities,
        path: "/items/{{RID}}/analytics/itemActivityStats/{{id}}/activities",
        params: item_activity_stat_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_activities_count,
        path: "/items/{{RID}}/analytics/itemActivityStats/{{id}}/activities/$count",
        params: item_activity_stat_id
    );
    delete!(
        doc: "Delete navigation property activities for drives",
        name: delete_activities,
        path: "/items/{{RID}}/analytics/itemActivityStats/{{id}}/activities/{{id2}}",
        params: item_activity_stat_id, item_activity_id
    );
    get!(
        doc: "Get activities from drives",
        name: get_activities,
        path: "/items/{{RID}}/analytics/itemActivityStats/{{id}}/activities/{{id2}}",
        params: item_activity_stat_id, item_activity_id
    );
    patch!(
        doc: "Update the navigation property activities in drives",
        name: update_activities,
        path: "/items/{{RID}}/analytics/itemActivityStats/{{id}}/activities/{{id2}}",
        body: true,
        params: item_activity_stat_id, item_activity_id
    );
    get!(
        doc: "Get driveItem from drives",
        name: get_drive_item,
        path: "/items/{{RID}}/analytics/itemActivityStats/{{id}}/activities/{{id2}}/driveItem",
        params: item_activity_stat_id, item_activity_id
    );
    get!(
        doc: "Get content for the navigation property driveItem from drives",
        name: get_drive_item_content,
        path: "/items/{{RID}}/analytics/itemActivityStats/{{id}}/activities/{{id2}}/driveItem/content",
        params: item_activity_stat_id, item_activity_id
    );
    put!(
        doc: "Update content for the navigation property driveItem in drives",
        name: update_drive_item_content,
        path: "/items/{{RID}}/analytics/itemActivityStats/{{id}}/activities/{{id2}}/driveItem/content",
        body: true,
        params: item_activity_stat_id, item_activity_id
    );
    get!(
        doc: "Get lastSevenDays from drives",
        name: get_last_seven_days,
        path: "/items/{{RID}}/analytics/lastSevenDays"
    );
    post!(
        doc: "Invoke action checkin",
        name: checkin,
        path: "/items/{{RID}}/checkin",
        body: true
    );
    post!(
        doc: "Invoke action checkout",
        name: checkout,
        path: "/items/{{RID}}/checkout"
    );
    post!(
        doc: "Create new navigation property to children for drives",
        name: create_children,
        path: "/items/{{RID}}/children",
        body: true
    );
    get!(
        doc: "List children of a driveItem",
        name: list_children,
        path: "/items/{{RID}}/children"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_children_count,
        path: "/items/{{RID}}/children/$count"
    );
    get!(
        doc: "Get children from drives",
        name: get_children,
        path: "/items/{{RID}}/children/{{id}}",
        params: drive_item_id_1
    );
    get!(
        doc: "Get content for the navigation property children from drives",
        name: get_children_content,
        path: "/items/{{RID}}/children/{{id}}/content",
        params: drive_item_id_1
    );
    put!(
        doc: "Update content for the navigation property children in drives",
        name: update_children_content,
        path: "/items/{{RID}}/children/{{id}}/content",
        body: true,
        params: drive_item_id_1
    );
    get!(
        doc: "Get content for the navigation property items from drives",
        name: get_items_content,
        path: "/items/{{RID}}/content"
    );
    put!(
        doc: "Update content for the navigation property items in drives",
        name: update_items_content,
        path: "/items/{{RID}}/content",
        body: true
    );
    post!(
        doc: "Invoke action copy",
        name: copy,
        path: "/items/{{RID}}/copy",
        body: true
    );
    post!(
        doc: "Invoke action createLink",
        name: create_link,
        path: "/items/{{RID}}/createLink",
        body: true
    );
    post!(
        doc: "Invoke action createUploadSession",
        name: create_upload_session,
        path: "/items/{{RID}}/createUploadSession",
        body: true
    );
    get!(
        doc: "Invoke function delta",
        name: get_drive_item_delta,
        path: "/items/{{RID}}/delta()"
    );
    get!(
        doc: "Invoke function delta",
        name: get_drive_item_delta_token,
        path: "/items/{{RID}}/delta(token='{{id}}')",
        params: token
    );
    post!(
        doc: "Invoke action follow",
        name: follow,
        path: "/items/{{RID}}/follow"
    );
    get!(
        doc: "Invoke function getActivitiesByInterval",
        name: get_drive_item_activities_by_interval,
        path: "/items/{{RID}}/getActivitiesByInterval(startDateTime='{{id}}',endDateTime='{{id2}}',interval='{{id3}}')",
        params: start_date_time, end_date_time, interval
    );
    post!(
        doc: "Invoke action invite",
        name: invite,
        path: "/items/{{RID}}/invite",
        body: true
    );
    get!(
        doc: "Get listItem from drives",
        name: get_list_item,
        path: "/items/{{RID}}/listItem"
    );
    post!(
        doc: "Create new navigation property to permissions for drives",
        name: create_permissions,
        path: "/items/{{RID}}/permissions",
        body: true
    );
    get!(
        doc: "List sharing permissions on a driveItem",
        name: list_permissions,
        path: "/items/{{RID}}/permissions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_permissions_count,
        path: "/items/{{RID}}/permissions/$count"
    );
    delete!(
        doc: "Delete navigation property permissions for drives",
        name: delete_permissions,
        path: "/items/{{RID}}/permissions/{{id}}",
        params: permission_id
    );
    get!(
        doc: "Get permissions from drives",
        name: get_permissions,
        path: "/items/{{RID}}/permissions/{{id}}",
        params: permission_id
    );
    patch!(
        doc: "Update the navigation property permissions in drives",
        name: update_permissions,
        path: "/items/{{RID}}/permissions/{{id}}",
        body: true,
        params: permission_id
    );
    post!(
        doc: "Invoke action grant",
        name: grant,
        path: "/items/{{RID}}/permissions/{{id}}/grant",
        body: true,
        params: permission_id
    );
    post!(
        doc: "Invoke action preview",
        name: preview,
        path: "/items/{{RID}}/preview",
        body: true
    );
    post!(
        doc: "Invoke action restore",
        name: restore,
        path: "/items/{{RID}}/restore",
        body: true
    );
    get!(
        doc: "Invoke function search",
        name: search,
        path: "/items/{{RID}}/search(q='{{id}}')",
        params: q
    );
    post!(
        doc: "Create new navigation property to subscriptions for drives",
        name: create_subscriptions,
        path: "/items/{{RID}}/subscriptions",
        body: true
    );
    get!(
        doc: "Get subscriptions from drives",
        name: list_subscriptions,
        path: "/items/{{RID}}/subscriptions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_subscriptions_count,
        path: "/items/{{RID}}/subscriptions/$count"
    );
    delete!(
        doc: "Delete navigation property subscriptions for drives",
        name: delete_subscriptions,
        path: "/items/{{RID}}/subscriptions/{{id}}",
        params: subscription_id
    );
    get!(
        doc: "Get subscriptions from drives",
        name: get_subscriptions,
        path: "/items/{{RID}}/subscriptions/{{id}}",
        params: subscription_id
    );
    patch!(
        doc: "Update the navigation property subscriptions in drives",
        name: update_subscriptions,
        path: "/items/{{RID}}/subscriptions/{{id}}",
        body: true,
        params: subscription_id
    );
    post!(
        doc: "Invoke action reauthorize",
        name: reauthorize,
        path: "/items/{{RID}}/subscriptions/{{id}}/reauthorize",
        params: subscription_id
    );
    post!(
        doc: "Create new navigation property to thumbnails for drives",
        name: create_thumbnails,
        path: "/items/{{RID}}/thumbnails",
        body: true
    );
    get!(
        doc: "List thumbnails for a DriveItem",
        name: list_thumbnails,
        path: "/items/{{RID}}/thumbnails"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_thumbnails_count,
        path: "/items/{{RID}}/thumbnails/$count"
    );
    delete!(
        doc: "Delete navigation property thumbnails for drives",
        name: delete_thumbnails,
        path: "/items/{{RID}}/thumbnails/{{id}}",
        params: thumbnail_set_id
    );
    get!(
        doc: "Get thumbnails from drives",
        name: get_thumbnails,
        path: "/items/{{RID}}/thumbnails/{{id}}",
        params: thumbnail_set_id
    );
    patch!(
        doc: "Update the navigation property thumbnails in drives",
        name: update_thumbnails,
        path: "/items/{{RID}}/thumbnails/{{id}}",
        body: true,
        params: thumbnail_set_id
    );
    post!(
        doc: "Invoke action unfollow",
        name: unfollow,
        path: "/items/{{RID}}/unfollow"
    );
    post!(
        doc: "Invoke action validatePermission",
        name: validate_permission,
        path: "/items/{{RID}}/validatePermission",
        body: true
    );
    post!(
        doc: "Create new navigation property to versions for drives",
        name: create_versions,
        path: "/items/{{RID}}/versions",
        body: true
    );
    get!(
        doc: "List versions of a driveItem",
        name: list_versions,
        path: "/items/{{RID}}/versions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_versions_count,
        path: "/items/{{RID}}/versions/$count"
    );
    delete!(
        doc: "Delete navigation property versions for drives",
        name: delete_versions,
        path: "/items/{{RID}}/versions/{{id}}",
        params: drive_item_version_id
    );
    get!(
        doc: "Get versions from drives",
        name: get_versions,
        path: "/items/{{RID}}/versions/{{id}}",
        params: drive_item_version_id
    );
    patch!(
        doc: "Update the navigation property versions in drives",
        name: update_versions,
        path: "/items/{{RID}}/versions/{{id}}",
        body: true,
        params: drive_item_version_id
    );
    get!(
        doc: "Get content for the navigation property versions from drives",
        name: get_versions_content,
        path: "/items/{{RID}}/versions/{{id}}/content",
        params: drive_item_version_id
    );
    put!(
        doc: "Update content for the navigation property versions in drives",
        name: update_versions_content,
        path: "/items/{{RID}}/versions/{{id}}/content",
        body: true,
        params: drive_item_version_id
    );
    post!(
        doc: "Invoke action restoreVersion",
        name: restore_version,
        path: "/items/{{RID}}/versions/{{id}}/restoreVersion",
        params: drive_item_version_id
    );
}
