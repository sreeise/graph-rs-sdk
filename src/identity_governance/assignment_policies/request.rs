// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    AssignmentPoliciesApiClient,
    AssignmentPoliciesIdApiClient,
    ResourceIdentity::AssignmentPolicies
);

impl AssignmentPoliciesApiClient {
    post!(
        doc: "Create assignmentPolicies",
        name: create_assignment_policies,
        path: "/assignmentPolicies",
        body: true
    );
    get!(
        doc: "List assignmentPolicies",
        name: list_assignment_policies,
        path: "/assignmentPolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_assignment_policies_count,
        path: "/assignmentPolicies/$count"
    );
}

impl AssignmentPoliciesIdApiClient {
    delete!(
        doc: "Delete navigation property assignmentPolicies for identityGovernance",
        name: delete_assignment_policies,
        path: "/assignmentPolicies/{{RID}}"
    );
    get!(
        doc: "Get assignmentPolicies from identityGovernance",
        name: get_assignment_policies,
        path: "/assignmentPolicies/{{RID}}"
    );
    put!(
        doc: "Update the navigation property assignmentPolicies in identityGovernance",
        name: update_assignment_policies,
        path: "/assignmentPolicies/{{RID}}",
        body: true
    );
    get!(
        doc: "Get accessPackage from identityGovernance",
        name: get_access_package,
        path: "/assignmentPolicies/{{RID}}/accessPackage"
    );
    get!(
        doc: "Get catalog from identityGovernance",
        name: get_catalog,
        path: "/assignmentPolicies/{{RID}}/catalog"
    );
    post!(
        doc: "Create new navigation property to questions for identityGovernance",
        name: create_questions,
        path: "/assignmentPolicies/{{RID}}/questions",
        body: true
    );
    get!(
        doc: "Get questions from identityGovernance",
        name: list_questions,
        path: "/assignmentPolicies/{{RID}}/questions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_questions_count,
        path: "/assignmentPolicies/{{RID}}/questions/$count"
    );
    delete!(
        doc: "Delete navigation property questions for identityGovernance",
        name: delete_questions,
        path: "/assignmentPolicies/{{RID}}/questions/{{id}}",
        params: access_package_question_id
    );
    get!(
        doc: "Get questions from identityGovernance",
        name: get_questions,
        path: "/assignmentPolicies/{{RID}}/questions/{{id}}",
        params: access_package_question_id
    );
    patch!(
        doc: "Update the navigation property questions in identityGovernance",
        name: update_questions,
        path: "/assignmentPolicies/{{RID}}/questions/{{id}}",
        body: true,
        params: access_package_question_id
    );
}
