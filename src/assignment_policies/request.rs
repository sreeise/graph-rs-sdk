// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(AssignmentPoliciesRequest,);
register_client!(AssignmentPoliciesIdRequest, ());

impl<'a, Client> AssignmentPoliciesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Create new navigation property to assignmentPolicies for identityGovernance",
        name: create_assignment_policies,
        response: serde_json::Value,
        path: "/assignmentPolicies",
        has_body: true
    });
    get!({
        doc: "Get assignmentPolicies from identityGovernance",
        name: list_assignment_policies,
        response: serde_json::Value,
        path: "/assignmentPolicies",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/assignmentPolicies/$count",
        has_body: false
    });
}

impl<'a, Client> AssignmentPoliciesIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    delete!({
        doc: "Delete navigation property assignmentPolicies for identityGovernance",
        name: delete_assignment_policies,
        response: NoContent,
        path: "/assignmentPolicies/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get assignmentPolicies from identityGovernance",
        name: get_assignment_policies,
        response: serde_json::Value,
        path: "/assignmentPolicies/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property assignmentPolicies in identityGovernance",
        name: update_assignment_policies,
        response: NoContent,
        path: "/assignmentPolicies/{{RID}}",
        has_body: true
    });
    get!({
        doc: "Get accessPackage from identityGovernance",
        name: get_access_package,
        response: serde_json::Value,
        path: "/assignmentPolicies/{{RID}}/accessPackage",
        has_body: false
    });
    get!({
        doc: "Get catalog from identityGovernance",
        name: get_catalog,
        response: serde_json::Value,
        path: "/assignmentPolicies/{{RID}}/catalog",
        has_body: false
    });
}
