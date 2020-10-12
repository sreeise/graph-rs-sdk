use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::{GraphResponse, IntoResponse};
use reqwest::Method;

register_client!(PoliciesRequest,);
register_client!(PolicyRootRequest,);

#[allow(dead_code)]
impl<'a, Client> PolicyRootRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get policies",
        name: get_policy_root,
        response: serde_json::Value,
        path: "/policies",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update policies",
        name: update_policy_root,
        response: GraphResponse<Content>,
        path: "/policies",
        params: 0,
        has_body: true
    });
}

#[allow(dead_code)]
impl<'a, Client> PoliciesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn policy_root(&self) -> PolicyRootRequest<'a, Client> {
        PolicyRootRequest::new(&self.client)
    }
    get!({
        doc: "# Get homeRealmDiscoveryPolicies from policies",
        name: list_home_realm_discovery_policies,
        response: Collection<serde_json::Value>,
        path: "/policies/homeRealmDiscoveryPolicies",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to homeRealmDiscoveryPolicies for policies",
        name: create_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/policies/homeRealmDiscoveryPolicies",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get homeRealmDiscoveryPolicies from policies",
        name: get_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/policies/homeRealmDiscoveryPolicies/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property homeRealmDiscoveryPolicies in policies",
        name: update_home_realm_discovery_policies,
        response: GraphResponse<Content>,
        path: "/policies/homeRealmDiscoveryPolicies/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property homeRealmDiscoveryPolicies for policies",
        name: delete_home_realm_discovery_policies,
        response: GraphResponse<Content>,
        path: "/policies/homeRealmDiscoveryPolicies/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get identitySecurityDefaultsEnforcementPolicy from policies",
        name: get_identity_security_defaults_enforcement_policy,
        response: serde_json::Value,
        path: "/policies/identitySecurityDefaultsEnforcementPolicy",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property identitySecurityDefaultsEnforcementPolicy in policies",
        name: update_identity_security_defaults_enforcement_policy,
        response: GraphResponse<Content>,
        path: "/policies/identitySecurityDefaultsEnforcementPolicy",
        params: 0,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property identitySecurityDefaultsEnforcementPolicy for policies",
        name: delete_identity_security_defaults_enforcement_policy,
        response: GraphResponse<Content>,
        path: "/policies/identitySecurityDefaultsEnforcementPolicy",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get tokenLifetimePolicies from policies",
        name: list_token_lifetime_policies,
        response: Collection<serde_json::Value>,
        path: "/policies/tokenLifetimePolicies",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to tokenLifetimePolicies for policies",
        name: create_token_lifetime_policies,
        response: serde_json::Value,
        path: "/policies/tokenLifetimePolicies",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get tokenLifetimePolicies from policies",
        name: get_token_lifetime_policies,
        response: serde_json::Value,
        path: "/policies/tokenLifetimePolicies/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property tokenLifetimePolicies in policies",
        name: update_token_lifetime_policies,
        response: GraphResponse<Content>,
        path: "/policies/tokenLifetimePolicies/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property tokenLifetimePolicies for policies",
        name: delete_token_lifetime_policies,
        response: GraphResponse<Content>,
        path: "/policies/tokenLifetimePolicies/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get activityBasedTimeoutPolicies from policies",
        name: list_activity_based_timeout_policies,
        response: Collection<serde_json::Value>,
        path: "/policies/activityBasedTimeoutPolicies",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to activityBasedTimeoutPolicies for policies",
        name: create_activity_based_timeout_policies,
        response: serde_json::Value,
        path: "/policies/activityBasedTimeoutPolicies",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get claimsMappingPolicies from policies",
        name: get_claims_mapping_policies,
        response: serde_json::Value,
        path: "/policies/claimsMappingPolicies/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property claimsMappingPolicies in policies",
        name: update_claims_mapping_policies,
        response: GraphResponse<Content>,
        path: "/policies/claimsMappingPolicies/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property claimsMappingPolicies for policies",
        name: delete_claims_mapping_policies,
        response: GraphResponse<Content>,
        path: "/policies/claimsMappingPolicies/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get activityBasedTimeoutPolicies from policies",
        name: get_activity_based_timeout_policies,
        response: serde_json::Value,
        path: "/policies/activityBasedTimeoutPolicies/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property activityBasedTimeoutPolicies in policies",
        name: update_activity_based_timeout_policies,
        response: GraphResponse<Content>,
        path: "/policies/activityBasedTimeoutPolicies/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property activityBasedTimeoutPolicies for policies",
        name: delete_activity_based_timeout_policies,
        response: GraphResponse<Content>,
        path: "/policies/activityBasedTimeoutPolicies/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get conditionalAccessPolicies from policies",
        name: list_conditional_access_policies,
        response: Collection<serde_json::Value>,
        path: "/policies/conditionalAccessPolicies",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to conditionalAccessPolicies for policies",
        name: create_conditional_access_policies,
        response: serde_json::Value,
        path: "/policies/conditionalAccessPolicies",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get claimsMappingPolicies from policies",
        name: list_claims_mapping_policies,
        response: Collection<serde_json::Value>,
        path: "/policies/claimsMappingPolicies",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to claimsMappingPolicies for policies",
        name: create_claims_mapping_policies,
        response: serde_json::Value,
        path: "/policies/claimsMappingPolicies",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get tokenIssuancePolicies from policies",
        name: list_token_issuance_policies,
        response: Collection<serde_json::Value>,
        path: "/policies/tokenIssuancePolicies",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to tokenIssuancePolicies for policies",
        name: create_token_issuance_policies,
        response: serde_json::Value,
        path: "/policies/tokenIssuancePolicies",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get tokenIssuancePolicies from policies",
        name: get_token_issuance_policies,
        response: serde_json::Value,
        path: "/policies/tokenIssuancePolicies/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property tokenIssuancePolicies in policies",
        name: update_token_issuance_policies,
        response: GraphResponse<Content>,
        path: "/policies/tokenIssuancePolicies/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property tokenIssuancePolicies for policies",
        name: delete_token_issuance_policies,
        response: GraphResponse<Content>,
        path: "/policies/tokenIssuancePolicies/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get conditionalAccessPolicies from policies",
        name: get_conditional_access_policies,
        response: serde_json::Value,
        path: "/policies/conditionalAccessPolicies/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property conditionalAccessPolicies in policies",
        name: update_conditional_access_policies,
        response: GraphResponse<Content>,
        path: "/policies/conditionalAccessPolicies/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property conditionalAccessPolicies for policies",
        name: delete_conditional_access_policies,
        response: GraphResponse<Content>,
        path: "/policies/conditionalAccessPolicies/{{id}}",
        params: 1,
        has_body: false
    });
}
