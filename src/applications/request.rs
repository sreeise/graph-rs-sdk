use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::types::DeltaPhantom;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(ApplicationsRequest,);
register_client!(CommunicationsRequest,);

impl<'a, Client> ApplicationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entity from applications by key",
        name: get_application,
        response: serde_json::Value,
        path: "/applications/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update entity in applications",
        name: update_application,
        response: GraphResponse<Content>,
        path: "/applications/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from applications",
        name: delete_application,
        response: GraphResponse<Content>,
        path: "/applications/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get tokenIssuancePolicies from applications",
        name: get_token_issuance_policies,
        response: serde_json::Value,
        path: "/applications/{{id}}/tokenIssuancePolicies/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get extensionProperties from applications",
        name: get_extension_properties,
        response: serde_json::Value,
        path: "/applications/{{id}}/extensionProperties/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property extensionProperties in applications",
        name: update_extension_properties,
        response: GraphResponse<Content>,
        path: "/applications/{{id}}/extensionProperties/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get homeRealmDiscoveryPolicies from applications",
        name: get_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/applications/{{id}}/homeRealmDiscoveryPolicies/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get owners from applications",
        name: get_owners,
        response: serde_json::Value,
        path: "/applications/{{id}}/owners/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get createdOnBehalfOf from applications",
        name: get_created_on_behalf_of,
        response: serde_json::Value,
        path: "/applications/{{id}}/createdOnBehalfOf",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get tokenIssuancePolicies from applications",
        name: list_token_issuance_policies,
        response: Collection<serde_json::Value>,
        path: "/applications/{{id}}/tokenIssuancePolicies",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action removePassword",
        name: remove_password,
        response: GraphResponse<Content>,
        path: "/applications/{{id}}/removePassword",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get entities from applications",
        name: list_application,
        response: Collection<serde_json::Value>,
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
    post!({
        doc: "# Invoke action addPassword",
        name: add_password,
        response: serde_json::Value,
        path: "/applications/{{id}}/addPassword",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get tokenLifetimePolicies from applications",
        name: get_token_lifetime_policies,
        response: serde_json::Value,
        path: "/applications/{{id}}/tokenLifetimePolicies/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get tokenLifetimePolicies from applications",
        name: list_token_lifetime_policies,
        response: Collection<serde_json::Value>,
        path: "/applications/{{id}}/tokenLifetimePolicies",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get extensionProperties from applications",
        name: list_extension_properties,
        response: Collection<serde_json::Value>,
        path: "/applications/{{id}}/extensionProperties",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to extensionProperties for applications",
        name: create_extension_properties,
        response: serde_json::Value,
        path: "/applications/{{id}}/extensionProperties",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action addKey",
        name: add_key,
        response: serde_json::Value,
        path: "/applications/{{id}}/addKey",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get homeRealmDiscoveryPolicies from applications",
        name: list_home_realm_discovery_policies,
        response: Collection<serde_json::Value>,
        path: "/applications/{{id}}/homeRealmDiscoveryPolicies",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get owners from applications",
        name: list_owners,
        response: Collection<serde_json::Value>,
        path: "/applications/{{id}}/owners",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action removeKey",
        name: remove_key,
        response: GraphResponse<Content>,
        path: "/applications/{{id}}/removeKey",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> CommunicationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<Collection<serde_json::Value>>,
        path: "/applications/delta()",
        params: 0,
        has_body: false
    });
}
