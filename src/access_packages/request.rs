// GENERATED CODE

use crate::api_default_imports::*;
use crate::assignment_policies::{AssignmentPoliciesIdRequest, AssignmentPoliciesRequest};
use graph_http::types::NoContent;

register_client!(AccessPackagesRequest,);
register_client!(AccessPackagesIdRequest, ());

impl<'a, Client> AccessPackagesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Create new navigation property to accessPackages for identityGovernance",
        name: create_access_packages,
        response: serde_json::Value,
        path: "/accessPackages",
        has_body: true
    });
    get!({
        doc: "Get accessPackages from identityGovernance",
        name: list_access_packages,
        response: serde_json::Value,
        path: "/accessPackages",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/accessPackages/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        response: serde_json::Value,
        path: "/accessPackages/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: [ on ],
        has_body: false
    });
}

impl<'a, Client> AccessPackagesIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn assignment_policies(&self) -> AssignmentPoliciesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::AssignmentPolicies);
        AssignmentPoliciesRequest::new(self.client)
    }

    pub fn assignment_policy<ID: AsRef<str>>(
        &self,
        id: ID,
    ) -> AssignmentPoliciesIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::AssignmentPolicies);
        AssignmentPoliciesIdRequest::new(id.as_ref(), self.client)
    }

    delete!({
        doc: "Delete navigation property accessPackages for identityGovernance",
        name: delete_access_packages,
        response: NoContent,
        path: "/accessPackages/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get accessPackages from identityGovernance",
        name: get_access_packages,
        response: serde_json::Value,
        path: "/accessPackages/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property accessPackages in identityGovernance",
        name: update_access_packages,
        response: NoContent,
        path: "/accessPackages/{{RID}}",
        has_body: true
    });
    get!({
        doc: "Get catalog from identityGovernance",
        name: get_catalog,
        response: serde_json::Value,
        path: "/accessPackages/{{RID}}/catalog",
        has_body: false
    });
    post!({
        doc: "Invoke action getApplicablePolicyRequirements",
        name: get_applicable_policy_requirements,
        response: serde_json::Value,
        path: "/accessPackages/{{RID}}/microsoft.graph.getApplicablePolicyRequirements",
        has_body: false
    });
}
