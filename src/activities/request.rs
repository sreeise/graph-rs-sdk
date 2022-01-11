// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(ActivitiesRequest,);
register_client!(ActivitiesIdRequest, ());

impl<'a, Client> ActivitiesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "Get activities from me",
        name: list_activities,
        response: serde_json::Value,
        path: "activities",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to activities for me",
        name: create_activities,
        response: serde_json::Value,
        path: "activities",
        has_body: true
    });
    get!({
        doc: "Invoke function recent",
        name: recent,
        response: serde_json::Value,
        path: "activities/microsoft.graph.recent()",
        has_body: false
    });
}

impl<'a, Client> ActivitiesIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    delete!({
        doc: "Delete navigation property activities for me",
        name: delete_activities,
        response: NoContent,
        path: "activities/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get activities from me",
        name: get_activities,
        response: serde_json::Value,
        path: "activities/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property activities in me",
        name: update_activities,
        response: NoContent,
        path: "activities/{{RID}}",
        has_body: true
    });
    get!({
        doc: "Get historyItems from me",
        name: list_history_items,
        response: serde_json::Value,
        path: "activities/{{RID}}/historyItems",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to historyItems for me",
        name: create_history_items,
        response: serde_json::Value,
        path: "activities/{{RID}}/historyItems",
        has_body: true
    });
    get!({
        doc: "Get historyItems from me",
        name: get_history_items,
        response: serde_json::Value,
        path: "activities/{{RID}}/historyItems/{{id}}",
        params: [ activity_history_item_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property historyItems in me",
        name: update_history_items,
        response: NoContent,
        path: "activities/{{RID}}/historyItems/{{id}}",
        params: [ activity_history_item_id ],
        has_body: true
    });
    delete!({
        doc: "Delete navigation property historyItems for me",
        name: delete_history_items,
        response: NoContent,
        path: "activities/{{RID}}/historyItems/{{id}}",
        params: [ activity_history_item_id ],
        has_body: false
    });
    get!({
        doc: "Get activity from me",
        name: get_history_items_activity,
        response: serde_json::Value,
        path: "activities/{{RID}}/historyItems/{{id}}/activity",
        params: [ activity_history_item_id ],
        has_body: false
    });
    get!({
        doc: "Get ref of activity from me",
        name: get_ref_activity,
        response: serde_json::Value,
        path: "activities/{{RID}}/historyItems/{{id}}/activity/$ref",
        params: [ activity_history_item_id ],
        has_body: false
    });
    put!({
        doc: "Update the ref of navigation property activity in me",
        name: update_ref_activity,
        response: NoContent,
        path: "activities/{{RID}}/historyItems/{{id}}/activity/$ref",
        params: [ activity_history_item_id ],
        has_body: true
    });
    delete!({
        doc: "Delete ref of navigation property activity for me",
        name: delete_ref_activity,
        response: NoContent,
        path: "activities/{{RID}}/historyItems/{{id}}/activity/$ref",
        params: [ activity_history_item_id ],
        has_body: false
    });
}
