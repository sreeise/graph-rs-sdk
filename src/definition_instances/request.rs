// GENERATED CODE

use crate::api_default_imports::*;
use crate::definition_instance_stages::{
    DefinitionInstanceStagesIdRequest, DefinitionInstanceStagesRequest,
};
use graph_http::types::NoContent;

register_client!(DefinitionInstancesRequest,);
register_client!(DefinitionInstancesIdRequest, ());

impl<'a, Client> DefinitionInstancesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Create new navigation property to instances for identityGovernance",
        name: create_instances,
        response: serde_json::Value,
        path: "/instances",
        has_body: true
    });
    get!({
        doc: "Get instances from identityGovernance",
        name: list_instances,
        response: serde_json::Value,
        path: "/instances",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/instances/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        response: serde_json::Value,
        path: "/instances/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: [ on ],
        has_body: false
    });
}

impl<'a, Client> DefinitionInstancesIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn stages(&self) -> DefinitionInstanceStagesRequest<'a, Client> {
        self.client.request.extend_path(&["instances"]);
        self.client.request.extend_path(&[self.id.as_str()]);
        self.client
            .set_ident(ResourceIdentity::DefinitionInstanceStages);
        DefinitionInstanceStagesRequest::new(self.client)
    }

    pub fn stage<ID: AsRef<str>>(&self, id: ID) -> DefinitionInstanceStagesIdRequest<'a, Client> {
        self.client.request.extend_path(&["instances"]);
        self.client.request.extend_path(&[self.id.as_str()]);
        self.client
            .set_ident(ResourceIdentity::DefinitionInstanceStages);
        DefinitionInstanceStagesIdRequest::new(id.as_ref(), self.client)
    }

    delete!({
        doc: "Delete navigation property instances for identityGovernance",
        name: delete_instances,
        response: NoContent,
        path: "/instances/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get instances from identityGovernance",
        name: get_instances,
        response: serde_json::Value,
        path: "/instances/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property instances in identityGovernance",
        name: update_instances,
        response: NoContent,
        path: "/instances/{{RID}}",
        has_body: true
    });
    post!({
        doc: "Create new navigation property to contactedReviewers for identityGovernance",
        name: create_contacted_reviewers,
        response: serde_json::Value,
        path: "/instances/{{RID}}/contactedReviewers",
        has_body: true
    });
    get!({
        doc: "Get contactedReviewers from identityGovernance",
        name: list_contacted_reviewers,
        response: serde_json::Value,
        path: "/instances/{{RID}}/contactedReviewers",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_contacted_reviewers_count,
        response: serde_json::Value,
        path: "/instances/{{RID}}/contactedReviewers/$count",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property contactedReviewers for identityGovernance",
        name: delete_contacted_reviewers,
        response: NoContent,
        path: "/instances/{{RID}}/contactedReviewers/{{id}}",
        params: [ access_review_reviewer_id ],
        has_body: false
    });
    get!({
        doc: "Get contactedReviewers from identityGovernance",
        name: get_contacted_reviewers,
        response: serde_json::Value,
        path: "/instances/{{RID}}/contactedReviewers/{{id}}",
        params: [ access_review_reviewer_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property contactedReviewers in identityGovernance",
        name: update_contacted_reviewers,
        response: NoContent,
        path: "/instances/{{RID}}/contactedReviewers/{{id}}",
        params: [ access_review_reviewer_id ],
        has_body: true
    });
    post!({
        doc: "Create new navigation property to decisions for identityGovernance",
        name: create_decisions,
        response: serde_json::Value,
        path: "/instances/{{RID}}/decisions",
        has_body: true
    });
    get!({
        doc: "Get decisions from identityGovernance",
        name: list_decisions,
        response: serde_json::Value,
        path: "/instances/{{RID}}/decisions",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_decisions_count,
        response: serde_json::Value,
        path: "/instances/{{RID}}/decisions/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        response: serde_json::Value,
        path: "/instances/{{RID}}/decisions/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: [ on ],
        has_body: false
    });
    delete!({
        doc: "Delete navigation property decisions for identityGovernance",
        name: delete_decisions,
        response: NoContent,
        path: "/instances/{{RID}}/decisions/{{id}}",
        params: [ access_review_instance_decision_item_id ],
        has_body: false
    });
    get!({
        doc: "Get decisions from identityGovernance",
        name: get_decisions,
        response: serde_json::Value,
        path: "/instances/{{RID}}/decisions/{{id}}",
        params: [ access_review_instance_decision_item_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property decisions in identityGovernance",
        name: update_decisions,
        response: NoContent,
        path: "/instances/{{RID}}/decisions/{{id}}",
        params: [ access_review_instance_decision_item_id ],
        has_body: true
    });
    post!({
        doc: "Invoke action acceptRecommendations",
        name: accept_recommendations,
        response: NoContent,
        path: "/instances/{{RID}}/microsoft.graph.acceptRecommendations",
        has_body: false
    });
    post!({
        doc: "Invoke action applyDecisions",
        name: apply_decisions,
        response: NoContent,
        path: "/instances/{{RID}}/microsoft.graph.applyDecisions",
        has_body: false
    });
    post!({
        doc: "Invoke action batchRecordDecisions",
        name: batch_record_decisions,
        response: NoContent,
        path: "/instances/{{RID}}/microsoft.graph.batchRecordDecisions",
        has_body: true
    });
    post!({
        doc: "Invoke action resetDecisions",
        name: reset_decisions,
        response: NoContent,
        path: "/instances/{{RID}}/microsoft.graph.resetDecisions",
        has_body: false
    });
    post!({
        doc: "Invoke action sendReminder",
        name: send_reminder,
        response: NoContent,
        path: "/instances/{{RID}}/microsoft.graph.sendReminder",
        has_body: false
    });
    post!({
        doc: "Invoke action stop",
        name: stop,
        response: NoContent,
        path: "/instances/{{RID}}/microsoft.graph.stop",
        has_body: false
    });
}
