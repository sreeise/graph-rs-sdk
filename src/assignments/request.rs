// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(AssignmentsRequest,);
register_client!(AssignmentsIdRequest, ());

impl<'a, Client> AssignmentsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Create new navigation property to assignments for identityGovernance",
        name: create_assignments,
        response: serde_json::Value,
        path: "/assignments",
        has_body: true
    });
    get!({
        doc: "Get assignments from identityGovernance",
        name: list_assignments,
        response: serde_json::Value,
        path: "/assignments",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/assignments/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        response: serde_json::Value,
        path: "/assignments/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: [ on ],
        has_body: false
    });
}

impl<'a, Client> AssignmentsIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    delete!({
        doc: "Delete navigation property assignments for identityGovernance",
        name: delete_assignments,
        response: NoContent,
        path: "/assignments/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get assignments from identityGovernance",
        name: get_assignments,
        response: serde_json::Value,
        path: "/assignments/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property assignments in identityGovernance",
        name: update_assignments,
        response: NoContent,
        path: "/assignments/{{RID}}",
        has_body: true
    });
    get!({
        doc: "Get accessPackage from identityGovernance",
        name: get_access_package,
        response: serde_json::Value,
        path: "/assignments/{{RID}}/accessPackage",
        has_body: false
    });
    get!({
        doc: "Get assignmentPolicy from identityGovernance",
        name: get_assignment_policy,
        response: serde_json::Value,
        path: "/assignments/{{RID}}/assignmentPolicy",
        has_body: false
    });
    post!({
        doc: "Invoke action reprocess",
        name: reprocess,
        response: NoContent,
        path: "/assignments/{{RID}}/microsoft.graph.reprocess",
        has_body: false
    });
    get!({
        doc: "Get target from identityGovernance",
        name: get_target,
        response: serde_json::Value,
        path: "/assignments/{{RID}}/target",
        has_body: false
    });
}
