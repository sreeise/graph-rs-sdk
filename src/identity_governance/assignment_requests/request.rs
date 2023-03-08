// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    AssignmentRequestsApiClient,
    AssignmentRequestsIdApiClient,
    ResourceIdentity::AssignmentRequests
);

impl AssignmentRequestsApiClient {
    post!(
        doc: "Create accessPackageAssignmentRequest",
        name: create_assignment_requests,
        path: "/assignmentRequests",
        body: true
    );
    get!(
        doc: "List assignmentRequests",
        name: list_assignment_requests,
        path: "/assignmentRequests"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_assignment_requests_count,
        path: "/assignmentRequests/$count"
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        path: "/assignmentRequests/filterByCurrentUser(on='{{id}}')",
        params: on
    );
}

impl AssignmentRequestsIdApiClient {
    delete!(
        doc: "Delete navigation property assignmentRequests for identityGovernance",
        name: delete_assignment_requests,
        path: "/assignmentRequests/{{RID}}"
    );
    get!(
        doc: "Get assignmentRequests from identityGovernance",
        name: get_assignment_requests,
        path: "/assignmentRequests/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property assignmentRequests in identityGovernance",
        name: update_assignment_requests,
        path: "/assignmentRequests/{{RID}}",
        body: true
    );
    get!(
        doc: "Get accessPackage from identityGovernance",
        name: get_access_package,
        path: "/assignmentRequests/{{RID}}/accessPackage"
    );
    get!(
        doc: "Get assignment from identityGovernance",
        name: get_assignment,
        path: "/assignmentRequests/{{RID}}/assignment"
    );
    post!(
        doc: "Invoke action cancel",
        name: cancel,
        path: "/assignmentRequests/{{RID}}/cancel"
    );
    post!(
        doc: "Invoke action reprocess",
        name: reprocess,
        path: "/assignmentRequests/{{RID}}/reprocess"
    );
    get!(
        doc: "Get requestor from identityGovernance",
        name: get_requestor,
        path: "/assignmentRequests/{{RID}}/requestor"
    );
}
