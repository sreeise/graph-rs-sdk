// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    ActivitiesApiClient,
    ActivitiesIdApiClient,
    ResourceIdentity::Activities
);

impl ActivitiesApiClient {
    post!(
        doc: "Create new navigation property to activities for users",
        name: create_activities,
        path: "/activities",
        body: true
    );
    get!(
        doc: "Get activities from users",
        name: list_activities,
        path: "/activities"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_activities_count,
        path: "/activities/$count"
    );
    get!(
        doc: "Invoke function recent",
        name: recent,
        path: "/activities/recent()"
    );
}

impl ActivitiesIdApiClient {
    delete!(
        doc: "Delete navigation property activities for users",
        name: delete_activities,
        path: "/activities/{{RID}}"
    );
    get!(
        doc: "Get activities from users",
        name: get_activities,
        path: "/activities/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property activities in users",
        name: update_activities,
        path: "/activities/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to historyItems for users",
        name: create_history_items,
        path: "/activities/{{RID}}/historyItems",
        body: true
    );
    get!(
        doc: "Get historyItems from users",
        name: list_history_items,
        path: "/activities/{{RID}}/historyItems"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_history_items_count,
        path: "/activities/{{RID}}/historyItems/$count"
    );
    delete!(
        doc: "Delete navigation property historyItems for users",
        name: delete_history_items,
        path: "/activities/{{RID}}/historyItems/{{id}}",
        params: activity_history_item_id
    );
    get!(
        doc: "Get historyItems from users",
        name: get_history_items,
        path: "/activities/{{RID}}/historyItems/{{id}}",
        params: activity_history_item_id
    );
    patch!(
        doc: "Update the navigation property historyItems in users",
        name: update_history_items,
        path: "/activities/{{RID}}/historyItems/{{id}}",
        body: true,
        params: activity_history_item_id
    );
    get!(
        doc: "Get activity from users",
        name: get_activity,
        path: "/activities/{{RID}}/historyItems/{{id}}/activity",
        params: activity_history_item_id
    );
}
