use crate::client::Graph;
use graph_http::types::{Collection, Content};
use graph_http::{GraphResponse, IntoResponse};
use reqwest::Method;

register_client!(ApplicationsRequest,);

impl<'a, Client> ApplicationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        name: get_ref_created_on_behalf_of,
        response: serde_json::Value,
        path: "/applications/{{id}}/createdOnBehalfOf/$ref",
        params: 1,
        has_body: false
    });
    patch!({
        name: update_ref_created_on_behalf_of,
        response: GraphResponse<Content>,
        path: "/applications/{{id}}/createdOnBehalfOf/$ref",
        params: 1,
        has_body: true
    });
    delete!({
        name: delete_ref_created_on_behalf_of,
        response: GraphResponse<Content>,
        path: "/applications/{{id}}/createdOnBehalfOf/$ref",
        params: 1,
        has_body: false
    });
    get!({
        name: list_owners,
        response: serde_json::Value,
        path: "/applications/{{id}}/owners",
        params: 1,
        has_body: false
    });
    get!({
        name: get_token_lifetime_policies,
        response: serde_json::Value,
        path: "/applications/{{id}}/tokenLifetimePolicies/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        name: get_created_on_behalf_of,
        response: serde_json::Value,
        path: "/applications/{{id}}/createdOnBehalfOf",
        params: 1,
        has_body: false
    });
    get!({
        name: get_token_issuance_policies,
        response: serde_json::Value,
        path: "/applications/{{id}}/tokenIssuancePolicies/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        name: list_ref_token_issuance_policies,
        response: serde_json::Value,
        path: "/applications/{{id}}/tokenIssuancePolicies/$ref",
        params: 1,
        has_body: false
    });
    post!({
        name: create_ref_token_issuance_policies,
        response: serde_json::Value,
        path: "/applications/{{id}}/tokenIssuancePolicies/$ref",
        params: 1,
        has_body: true
    });
    delete!({
        name: delete_ref_token_issuance_policies,
        response: GraphResponse<Content>,
        path: "/applications/{{id}}/tokenIssuancePolicies/$ref",
        params: 1,
        has_body: false
    });
    get!({
        name: get_owners,
        response: serde_json::Value,
        path: "/applications/{{id}}/owners/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        name: list_ref_token_lifetime_policies,
        response: serde_json::Value,
        path: "/applications/{{id}}/tokenLifetimePolicies/$ref",
        params: 1,
        has_body: false
    });
    post!({
        name: create_ref_token_lifetime_policies,
        response: serde_json::Value,
        path: "/applications/{{id}}/tokenLifetimePolicies/$ref",
        params: 1,
        has_body: true
    });
    delete!({
        name: delete_ref_token_lifetime_policies,
        response: GraphResponse<Content>,
        path: "/applications/{{id}}/tokenLifetimePolicies/$ref",
        params: 1,
        has_body: false
    });
    get!({
        name: list_token_lifetime_policies,
        response: serde_json::Value,
        path: "/applications/{{id}}/tokenLifetimePolicies",
        params: 1,
        has_body: false
    });
    get!({
        name: list_extension_properties,
        response: serde_json::Value,
        path: "/applications/{{id}}/extensionProperties",
        params: 1,
        has_body: false
    });
    post!({
        name: create_extension_properties,
        response: serde_json::Value,
        path: "/applications/{{id}}/extensionProperties",
        params: 1,
        has_body: true
    });
    get!({
        name: list_ref_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/applications/{{id}}/homeRealmDiscoveryPolicies/$ref",
        params: 1,
        has_body: false
    });
    post!({
        name: create_ref_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/applications/{{id}}/homeRealmDiscoveryPolicies/$ref",
        params: 1,
        has_body: true
    });
    delete!({
        name: delete_ref_home_realm_discovery_policies,
        response: GraphResponse<Content>,
        path: "/applications/{{id}}/homeRealmDiscoveryPolicies/$ref",
        params: 1,
        has_body: false
    });
    get!({
        name: get_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/applications/{{id}}/homeRealmDiscoveryPolicies/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        name: list_token_issuance_policies,
        response: serde_json::Value,
        path: "/applications/{{id}}/tokenIssuancePolicies",
        params: 1,
        has_body: false
    });
    get!({
        name: list_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/applications/{{id}}/homeRealmDiscoveryPolicies",
        params: 1,
        has_body: false
    });
    get!({
        name: list_ref_owners,
        response: serde_json::Value,
        path: "/applications/{{id}}/owners/$ref",
        params: 1,
        has_body: false
    });
    post!({
        name: create_ref_owners,
        response: serde_json::Value,
        path: "/applications/{{id}}/owners/$ref",
        params: 1,
        has_body: true
    });
    delete!({
        name: delete_ref_owners,
        response: GraphResponse<Content>,
        path: "/applications/{{id}}/owners/$ref",
        params: 1,
        has_body: false
    });
    get!({
        name: get_extension_properties,
        response: serde_json::Value,
        path: "/applications/{{id}}/extensionProperties/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        name: update_extension_properties,
        response: GraphResponse<Content>,
        path: "/applications/{{id}}/extensionProperties/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        name: delete_extension_properties,
        response: GraphResponse<Content>,
        path: "/applications/{{id}}/extensionProperties/{{id2}}",
        params: 2,
        has_body: false
    });
    post!({
        name: add_key,
        response: serde_json::Value,
        path: "/applications/{{id}}/microsoft.graph.addKey",
        params: 1,
        has_body: true
    });
    get!({
        name: get_application,
        response: serde_json::Value,
        path: "/applications/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        name: update_application,
        response: GraphResponse<Content>,
        path: "/applications/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        name: delete_application,
        response: GraphResponse<Content>,
        path: "/applications/{{id}}",
        params: 1,
        has_body: false
    });
    post!({
        name: get_member_objects,
        response: Collection<serde_json::Value>,
        path: "/applications/{{id}}/microsoft.graph.getMemberObjects",
        params: 1,
        has_body: true
    });
    post!({
        name: check_member_objects,
        response: Collection<serde_json::Value>,
        path: "/applications/{{id}}/microsoft.graph.checkMemberObjects",
        params: 1,
        has_body: true
    });
    post!({
        name: get_available_extension_properties,
        response: Collection<serde_json::Value>,
        path: "/applications/microsoft.graph.getAvailableExtensionProperties",
        params: 0,
        has_body: true
    });
    post!({
        name: get_by_ids,
        response: Collection<serde_json::Value>,
        path: "/applications/microsoft.graph.getByIds",
        params: 0,
        has_body: true
    });
    post!({
        name: add_password,
        response: serde_json::Value,
        path: "/applications/{{id}}/microsoft.graph.addPassword",
        params: 1,
        has_body: true
    });
    post!({
        name: restore,
        response: serde_json::Value,
        path: "/applications/{{id}}/microsoft.graph.restore",
        params: 1,
        has_body: false
    });
    post!({
        name: check_member_groups,
        response: Collection<serde_json::Value>,
        path: "/applications/{{id}}/microsoft.graph.checkMemberGroups",
        params: 1,
        has_body: true
    });
    get!({
        name: delta,
        response: Collection<serde_json::Value>,
        path: "/applications/microsoft.graph.delta()",
        params: 0,
        has_body: false
    });
    post!({
        name: get_member_groups,
        response: Collection<serde_json::Value>,
        path: "/applications/{{id}}/microsoft.graph.getMemberGroups",
        params: 1,
        has_body: true
    });
    post!({
        name: validate_properties,
        response: GraphResponse<Content>,
        path: "/applications/microsoft.graph.validateProperties",
        params: 0,
        has_body: true
    });
    post!({
        name: remove_password,
        response: GraphResponse<Content>,
        path: "/applications/{{id}}/microsoft.graph.removePassword",
        params: 1,
        has_body: true
    });
    get!({
        name: list_applications,
        response: serde_json::Value,
        path: "/applications",
        params: 0,
        has_body: false
    });
    post!({
        name: create_application,
        response: serde_json::Value,
        path: "/applications",
        params: 0,
        has_body: true
    });
    post!({
        name: remove_key,
        response: GraphResponse<Content>,
        path: "/applications/{{id}}/microsoft.graph.removeKey",
        params: 1,
        has_body: true
    });
}
