// GENERATED CODE

use crate::api_default_imports::*;
use crate::definition_instances::{DefinitionInstancesIdRequest, DefinitionInstancesRequest};
use graph_http::types::NoContent;

register_client!(AccessReviewDefinitionsRequest,);
register_client!(AccessReviewDefinitionsIdRequest, ());

impl<'a, Client> AccessReviewDefinitionsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Create new navigation property to definitions for identityGovernance",
        name: create_definitions,
        response: serde_json::Value,
        path: "/definitions",
        has_body: true
    });
    get!({
        doc: "Get definitions from identityGovernance",
        name: list_definitions,
        response: serde_json::Value,
        path: "/definitions",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/definitions/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        response: serde_json::Value,
        path: "/definitions/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: [ on ],
        has_body: false
    });
}

impl<'a, Client> AccessReviewDefinitionsIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn instances(&self) -> DefinitionInstancesRequest<'a, Client> {
        self.client.request.extend_path(&["definitions"]);
        self.client.request.extend_path(&[self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::DefinitionInstances);
        DefinitionInstancesRequest::new(self.client)
    }

    pub fn instance<ID: AsRef<str>>(&self, id: ID) -> DefinitionInstancesIdRequest<'a, Client> {
        self.client.request.extend_path(&["definitions"]);
        self.client.request.extend_path(&[self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::DefinitionInstances);
        DefinitionInstancesIdRequest::new(id.as_ref(), self.client)
    }

    delete!({
        doc: "Delete navigation property definitions for identityGovernance",
        name: delete_definitions,
        response: NoContent,
        path: "/definitions/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get definitions from identityGovernance",
        name: get_definitions,
        response: serde_json::Value,
        path: "/definitions/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property definitions in identityGovernance",
        name: update_definitions,
        response: NoContent,
        path: "/definitions/{{RID}}",
        has_body: true
    });
    post!({
        doc: "Invoke action stop",
        name: stop,
        response: NoContent,
        path: "/definitions/{{RID}}/microsoft.graph.stop",
        has_body: false
    });
}
