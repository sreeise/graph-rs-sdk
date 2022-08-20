// GENERATED CODE

use crate::access_review_definitions::{
    AccessReviewDefinitionsIdRequest, AccessReviewDefinitionsRequest,
};
use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(AccessReviewsRequest,);

impl<'a, Client> AccessReviewsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn definitions(&self) -> AccessReviewDefinitionsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client
            .set_ident(ResourceIdentity::AccessReviewDefinitions);
        AccessReviewDefinitionsRequest::new(self.client)
    }

    pub fn definition<ID: AsRef<str>>(
        &self,
        id: ID,
    ) -> AccessReviewDefinitionsIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client
            .set_ident(ResourceIdentity::AccessReviewDefinitions);
        AccessReviewDefinitionsIdRequest::new(id.as_ref(), self.client)
    }

    delete!({
        doc: "Delete navigation property accessReviews for identityGovernance",
        name: delete_access_reviews,
        response: NoContent,
        path: "/accessReviews",
        has_body: false
    });
    get!({
        doc: "Get accessReviews from identityGovernance",
        name: get_access_reviews,
        response: serde_json::Value,
        path: "/accessReviews",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property accessReviews in identityGovernance",
        name: update_access_reviews,
        response: NoContent,
        path: "/accessReviews",
        has_body: true
    });
    post!({
        doc: "Create new navigation property to historyDefinitions for identityGovernance",
        name: create_history_definitions,
        response: serde_json::Value,
        path: "/accessReviews/historyDefinitions",
        has_body: true
    });
    get!({
        doc: "Get historyDefinitions from identityGovernance",
        name: list_history_definitions,
        response: serde_json::Value,
        path: "/accessReviews/historyDefinitions",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_history_definitions_count,
        response: serde_json::Value,
        path: "/accessReviews/historyDefinitions/$count",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property historyDefinitions for identityGovernance",
        name: delete_history_definitions,
        response: NoContent,
        path: "/accessReviews/historyDefinitions/{{id}}",
        params: [ access_review_history_definition_id ],
        has_body: false
    });
    get!({
        doc: "Get historyDefinitions from identityGovernance",
        name: get_history_definitions,
        response: serde_json::Value,
        path: "/accessReviews/historyDefinitions/{{id}}",
        params: [ access_review_history_definition_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property historyDefinitions in identityGovernance",
        name: update_history_definitions,
        response: NoContent,
        path: "/accessReviews/historyDefinitions/{{id}}",
        params: [ access_review_history_definition_id ],
        has_body: true
    });
    post!({
        doc: "Create new navigation property to instances for identityGovernance",
        name: create_instances,
        response: serde_json::Value,
        path: "/accessReviews/historyDefinitions/{{id}}/instances",
        params: [ access_review_history_definition_id ],
        has_body: true
    });
    get!({
        doc: "Get instances from identityGovernance",
        name: list_instances,
        response: serde_json::Value,
        path: "/accessReviews/historyDefinitions/{{id}}/instances",
        params: [ access_review_history_definition_id ],
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_history_definitions_instances_count,
        response: serde_json::Value,
        path: "/accessReviews/historyDefinitions/{{id}}/instances/$count",
        params: [ access_review_history_definition_id ],
        has_body: false
    });
    delete!({
        doc: "Delete navigation property instances for identityGovernance",
        name: delete_instances,
        response: NoContent,
        path: "/accessReviews/historyDefinitions/{{id}}/instances/{{id2}}",
        params: [ access_review_history_definition_id  access_review_history_instance_id ],
        has_body: false
    });
    get!({
        doc: "Get instances from identityGovernance",
        name: get_instances,
        response: serde_json::Value,
        path: "/accessReviews/historyDefinitions/{{id}}/instances/{{id2}}",
        params: [ access_review_history_definition_id  access_review_history_instance_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property instances in identityGovernance",
        name: update_instances,
        response: NoContent,
        path: "/accessReviews/historyDefinitions/{{id}}/instances/{{id2}}",
        params: [ access_review_history_definition_id  access_review_history_instance_id ],
        has_body: true
    });
    post!({
        doc: "Invoke action generateDownloadUri",
        name: generate_download_uri,
        response: serde_json::Value,
        path: "/accessReviews/historyDefinitions/{{id}}/instances/{{id2}}/microsoft.graph.generateDownloadUri",
        params: [ access_review_history_definition_id  access_review_history_instance_id ],
        has_body: false
    });
}
