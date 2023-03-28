// GENERATED CODE

use crate::api_default_imports::*;
use crate::sites::*;

resource_api_client!(
    SitesItemsApiClient,
    SitesItemsIdApiClient,
    ResourceIdentity::SitesItems
);

impl SitesItemsApiClient {
    post!(
        doc: "Create a new item in a list",
        name: create_items,
        path: "/items",
        body: true
    );
    get!(
        doc: "Enumerate items in a list",
        name: list_items,
        path: "/items"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_items_count,
        path: "/items/$count"
    );
}

impl SitesItemsIdApiClient {
    api_client_link!(versions, SitesItemsVersionsApiClient);
    api_client_link_id!(version, SitesItemsVersionsIdApiClient);

    delete!(
        doc: "Delete navigation property items for sites",
        name: delete_items,
        path: "/items/{{RID}}"
    );
    get!(
        doc: "Get items from sites",
        name: get_items,
        path: "/items/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property items in sites",
        name: update_items,
        path: "/items/{{RID}}",
        body: true
    );
    get!(
        doc: "Get analytics from sites",
        name: get_analytics,
        path: "/items/{{RID}}/analytics"
    );
    post!(
        doc: "Create documentSetVersion",
        name: create_document_set_versions,
        path: "/items/{{RID}}/documentSetVersions",
        body: true
    );
    get!(
        doc: "List documentSetVersions",
        name: list_document_set_versions,
        path: "/items/{{RID}}/documentSetVersions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_document_set_versions_count,
        path: "/items/{{RID}}/documentSetVersions/$count"
    );
    delete!(
        doc: "Delete navigation property documentSetVersions for sites",
        name: delete_document_set_versions,
        path: "/items/{{RID}}/documentSetVersions/{{id}}",
        params: document_set_version_id
    );
    get!(
        doc: "Get documentSetVersions from sites",
        name: get_document_set_versions,
        path: "/items/{{RID}}/documentSetVersions/{{id}}",
        params: document_set_version_id
    );
    patch!(
        doc: "Update the navigation property documentSetVersions in sites",
        name: update_document_set_versions,
        path: "/items/{{RID}}/documentSetVersions/{{id}}",
        body: true,
        params: document_set_version_id
    );
    post!(
        doc: "Invoke action restore",
        name: restore,
        path: "/items/{{RID}}/documentSetVersions/{{id}}/restore",
        params: document_set_version_id
    );
    get!(
        doc: "Get driveItem from sites",
        name: get_drive_item,
        path: "/items/{{RID}}/driveItem"
    );
    get!(
        doc: "Get content for the navigation property driveItem from sites",
        name: get_drive_item_content,
        path: "/items/{{RID}}/driveItem/content"
    );
    put!(
        doc: "Update content for the navigation property driveItem in sites",
        name: update_drive_item_content,
        path: "/items/{{RID}}/driveItem/content",
        body: true
    );
    delete!(
        doc: "Delete navigation property fields for sites",
        name: delete_fields,
        path: "/items/{{RID}}/fields"
    );
    get!(
        doc: "Get fields from sites",
        name: get_fields,
        path: "/items/{{RID}}/fields"
    );
    patch!(
        doc: "Update listItem",
        name: update_fields,
        path: "/items/{{RID}}/fields",
        body: true
    );
    get!(
        doc: "Invoke function getActivitiesByInterval",
        name: get_activities_by_interval,
        path: "/items/{{RID}}/getActivitiesByInterval(startDateTime='{{id}}',endDateTime='{{id2}}',interval='{{id3}}')",
        params: start_date_time, end_date_time, interval
    );
}
