// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(AccessPackageAssignmentApprovalsRequest,);
register_client!(AccessPackageAssignmentApprovalsIdRequest, ());

impl<'a, Client> AccessPackageAssignmentApprovalsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Create new navigation property to accessPackageAssignmentApprovals for identityGovernance",
        name: create_access_package_assignment_approvals,
        response: serde_json::Value,
        path: "/accessPackageAssignmentApprovals",
        has_body: true
    });
    get!({
        doc: "Get accessPackageAssignmentApprovals from identityGovernance",
        name: list_access_package_assignment_approvals,
        response: serde_json::Value,
        path: "/accessPackageAssignmentApprovals",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/accessPackageAssignmentApprovals/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        response: serde_json::Value,
        path: "/accessPackageAssignmentApprovals/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: [ on ],
        has_body: false
    });
}

impl<'a, Client> AccessPackageAssignmentApprovalsIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    delete!({
        doc: "Delete navigation property accessPackageAssignmentApprovals for identityGovernance",
        name: delete_access_package_assignment_approvals,
        response: NoContent,
        path: "/accessPackageAssignmentApprovals/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get accessPackageAssignmentApprovals from identityGovernance",
        name: get_access_package_assignment_approvals,
        response: serde_json::Value,
        path: "/accessPackageAssignmentApprovals/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property accessPackageAssignmentApprovals in identityGovernance",
        name: update_access_package_assignment_approvals,
        response: NoContent,
        path: "/accessPackageAssignmentApprovals/{{RID}}",
        has_body: true
    });
    post!({
        doc: "Create new navigation property to stages for identityGovernance",
        name: create_stages,
        response: serde_json::Value,
        path: "/accessPackageAssignmentApprovals/{{RID}}/stages",
        has_body: true
    });
    get!({
        doc: "Get stages from identityGovernance",
        name: list_stages,
        response: serde_json::Value,
        path: "/accessPackageAssignmentApprovals/{{RID}}/stages",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/accessPackageAssignmentApprovals/{{RID}}/stages/$count",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property stages for identityGovernance",
        name: delete_stages,
        response: NoContent,
        path: "/accessPackageAssignmentApprovals/{{RID}}/stages/{{id}}",
        params: [ approval_stage_id ],
        has_body: false
    });
    get!({
        doc: "Get stages from identityGovernance",
        name: get_stages,
        response: serde_json::Value,
        path: "/accessPackageAssignmentApprovals/{{RID}}/stages/{{id}}",
        params: [ approval_stage_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property stages in identityGovernance",
        name: update_stages,
        response: NoContent,
        path: "/accessPackageAssignmentApprovals/{{RID}}/stages/{{id}}",
        params: [ approval_stage_id ],
        has_body: true
    });
}
