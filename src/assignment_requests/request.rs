// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(AssignmentRequestsRequest,);
register_client!(AssignmentRequestsIdRequest, ());

impl<'a, Client> AssignmentRequestsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Create new navigation property to assignmentRequests for identityGovernance",
        name: create_assignment_requests,
        response: serde_json::Value,
        path: "/assignmentRequests",
        has_body: true
    });
    get!({
        doc: "Get assignmentRequests from identityGovernance",
        name: list_assignment_requests,
        response: serde_json::Value,
        path: "/assignmentRequests",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/assignmentRequests/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        response: serde_json::Value,
        path: "/assignmentRequests/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: [ on ],
        has_body: false
    });
}

impl<'a, Client> AssignmentRequestsIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    delete!({
        doc: "Delete navigation property assignmentRequests for identityGovernance",
        name: delete_assignment_requests,
        response: NoContent,
        path: "/assignmentRequests/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get assignmentRequests from identityGovernance",
        name: get_assignment_requests,
        response: serde_json::Value,
        path: "/assignmentRequests/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property assignmentRequests in identityGovernance",
        name: update_assignment_requests,
        response: NoContent,
        path: "/assignmentRequests/{{RID}}",
        has_body: true
    });
    get!({
        doc: "Get accessPackage from identityGovernance",
        name: get_access_package,
        response: serde_json::Value,
        path: "/assignmentRequests/{{RID}}/accessPackage",
        has_body: false
    });
    get!({
        doc: "Get assignment from identityGovernance",
        name: get_assignment,
        response: serde_json::Value,
        path: "/assignmentRequests/{{RID}}/assignment",
        has_body: false
    });
    post!({
        doc: "Invoke action cancel",
        name: cancel,
        response: NoContent,
        path: "/assignmentRequests/{{RID}}/microsoft.graph.cancel",
        has_body: false
    });
    post!({
        doc: "Invoke action reprocess",
        name: reprocess,
        response: NoContent,
        path: "/assignmentRequests/{{RID}}/microsoft.graph.reprocess",
        has_body: false
    });
    get!({
        doc: "Get requestor from identityGovernance",
        name: get_requestor,
        response: serde_json::Value,
        path: "/assignmentRequests/{{RID}}/requestor",
        has_body: false
    });
}
