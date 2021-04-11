// GENERATED CODE

use crate::client::Graph;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(ActivitiesRequest,);
register_client!(HistoryItemsRequest,);

impl<'a, Client> ActivitiesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn history_items(&self) -> HistoryItemsRequest<'a, Client> {
        HistoryItemsRequest::new(self.client)
    }
    get!({
        doc: "# Get activities from me",
        name: list_activities,
        response: serde_json::Value,
        path: "/activities",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to activities for me",
        name: create_activities,
        response: serde_json::Value,
        path: "/activities",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function recent",
        name: recent,
        response: serde_json::Value,
        path: "/activities/recent",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get activities from me",
        name: get_activities,
        response: serde_json::Value,
        path: "/activities/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property activities in me",
        name: update_activities,
        response: NoContent,
        path: "/activities/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property activities for me",
        name: delete_activities,
        response: NoContent,
        path: "/activities/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get historyItems from me",
        name: list_history_items,
        response: serde_json::Value,
        path: "/activities/{{id}}/historyItems",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to historyItems for me",
        name: create_history_items,
        response: serde_json::Value,
        path: "/activities/{{id}}/historyItems",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get historyItems from me",
        name: get_history_items,
        response: serde_json::Value,
        path: "/activities/{{id}}/historyItems/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property historyItems in me",
        name: update_history_items,
        response: NoContent,
        path: "/activities/{{id}}/historyItems/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property historyItems for me",
        name: delete_history_items,
        response: NoContent,
        path: "/activities/{{id}}/historyItems/{{id2}}",
        params: 2,
        has_body: false
    });
}

impl<'a, Client> HistoryItemsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get activity from me",
        name: get_activity,
        response: serde_json::Value,
        path: "/activities/{{id}}/historyItems/{{id2}}/activity",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get ref of activity from me",
        name: get_ref_activity,
        response: serde_json::Value,
        path: "/activities/{{id}}/historyItems/{{id2}}/activity/$ref",
        params: 2,
        has_body: false
    });
    put!({
        doc: "# Update the ref of navigation property activity in me",
        name: update_ref_activity,
        response: NoContent,
        path: "/activities/{{id}}/historyItems/{{id2}}/activity/$ref",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete ref of navigation property activity for me",
        name: delete_ref_activity,
        response: NoContent,
        path: "/activities/{{id}}/historyItems/{{id2}}/activity/$ref",
        params: 2,
        has_body: false
    });
}
