use crate::client::Graph;
use graph_http::types::DeltaPhantom;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(ApplicationsRequest, ());
register_client!(ApplicationRequest,);

impl<'a, Client> ApplicationRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> ApplicationsRequest<'a, Client> {
        ApplicationsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get entities from applications",
        name: list_application,
        response: serde_json::Value,
        path: "/applications",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Add new entity to applications",
        name: create_application,
        response: serde_json::Value,
        path: "/applications",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/applications/delta()",
        params: 0,
        has_body: false
    });
}

impl<'a, Client> ApplicationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get homeRealmDiscoveryPolicies from applications",
        name: get_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/homeRealmDiscoveryPolicies/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get tokenLifetimePolicies from applications",
        name: list_token_lifetime_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenLifetimePolicies",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get tokenIssuancePolicies from applications",
        name: get_token_issuance_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenIssuancePolicies/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get createdOnBehalfOf from applications",
        name: get_created_on_behalf_of,
        response: serde_json::Value,
        path: "/applications/{{RID}}/createdOnBehalfOf",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get tokenIssuancePolicies from applications",
        name: list_token_issuance_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenIssuancePolicies",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action removePassword",
        name: remove_password,
        response: NoContent,
        path: "/applications/{{RID}}/removePassword",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action addKey",
        name: add_key,
        response: serde_json::Value,
        path: "/applications/{{RID}}/addKey",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get tokenLifetimePolicies from applications",
        name: get_token_lifetime_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenLifetimePolicies/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get extensionProperties from applications",
        name: list_extension_properties,
        response: serde_json::Value,
        path: "/applications/{{RID}}/extensionProperties",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to extensionProperties for applications",
        name: create_extension_properties,
        response: serde_json::Value,
        path: "/applications/{{RID}}/extensionProperties",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action addPassword",
        name: add_password,
        response: serde_json::Value,
        path: "/applications/{{RID}}/addPassword",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get entity from applications by key",
        name: get_application,
        response: serde_json::Value,
        path: "/applications/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update entity in applications",
        name: update_application,
        response: NoContent,
        path: "/applications/{{RID}}",
        params: 0,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from applications",
        name: delete_application,
        response: NoContent,
        path: "/applications/{{RID}}",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action removeKey",
        name: remove_key,
        response: NoContent,
        path: "/applications/{{RID}}/removeKey",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get homeRealmDiscoveryPolicies from applications",
        name: list_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/homeRealmDiscoveryPolicies",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get extensionProperties from applications",
        name: get_extension_properties,
        response: serde_json::Value,
        path: "/applications/{{RID}}/extensionProperties/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property extensionProperties in applications",
        name: update_extension_properties,
        response: NoContent,
        path: "/applications/{{RID}}/extensionProperties/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get owners from applications",
        name: get_owners,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get owners from applications",
        name: list_owners,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners",
        params: 0,
        has_body: false
    });
}
