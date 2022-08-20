// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(DefinitionInstanceStagesRequest,);
register_client!(DefinitionInstanceStagesIdRequest, ());

impl<'a, Client> DefinitionInstanceStagesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Create new navigation property to stages for identityGovernance",
        name: create_stages,
        response: serde_json::Value,
        path: "/stages",
        has_body: true
    });
    get!({
        doc: "Get stages from identityGovernance",
        name: list_stages,
        response: serde_json::Value,
        path: "/stages",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/stages/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        response: serde_json::Value,
        path: "/stages/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: [ on ],
        has_body: false
    });
}

impl<'a, Client> DefinitionInstanceStagesIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    delete!({
        doc: "Delete navigation property stages for identityGovernance",
        name: delete_stages,
        response: NoContent,
        path: "/stages/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get stages from identityGovernance",
        name: get_stages,
        response: serde_json::Value,
        path: "/stages/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property stages in identityGovernance",
        name: update_stages,
        response: NoContent,
        path: "/stages/{{RID}}",
        has_body: true
    });
    post!({
        doc: "Create new navigation property to decisions for identityGovernance",
        name: create_decisions,
        response: serde_json::Value,
        path: "/stages/{{RID}}/decisions",
        has_body: true
    });
    get!({
        doc: "Get decisions from identityGovernance",
        name: list_decisions,
        response: serde_json::Value,
        path: "/stages/{{RID}}/decisions",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/stages/{{RID}}/decisions/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        response: serde_json::Value,
        path: "/stages/{{RID}}/decisions/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: [ on ],
        has_body: false
    });
    delete!({
        doc: "Delete navigation property decisions for identityGovernance",
        name: delete_decisions,
        response: NoContent,
        path: "/stages/{{RID}}/decisions/{{id}}",
        params: [ access_review_instance_decision_item_id ],
        has_body: false
    });
    get!({
        doc: "Get decisions from identityGovernance",
        name: get_decisions,
        response: serde_json::Value,
        path: "/stages/{{RID}}/decisions/{{id}}",
        params: [ access_review_instance_decision_item_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property decisions in identityGovernance",
        name: update_decisions,
        response: NoContent,
        path: "/stages/{{RID}}/decisions/{{id}}",
        params: [ access_review_instance_decision_item_id ],
        has_body: true
    });
    post!({
        doc: "Invoke action stop",
        name: stop,
        response: NoContent,
        path: "/stages/{{RID}}/microsoft.graph.stop",
        has_body: false
    });
}
