use crate::client::Graph;
use graph_http::types::DeltaPhantom;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(ServicePrincipalsRequest,);

impl<'a, Client> ServicePrincipalsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get appRoleAssignedTo from servicePrincipals",
        name: get_app_role_assigned_to,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/appRoleAssignedTo/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property appRoleAssignedTo in servicePrincipals",
        name: update_app_role_assigned_to,
        response: NoContent,
        path: "/servicePrincipals/{{id}}/appRoleAssignedTo/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get claimsMappingPolicies from servicePrincipals",
        name: get_claims_mapping_policies,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/claimsMappingPolicies/{{id2}}",
        params: 2,
        has_body: false
    });

    get!({
        doc: "# Get appRoleAssignments from servicePrincipals",
        name: list_app_role_assignments,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/appRoleAssignments",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to appRoleAssignments for servicePrincipals",
        name: create_app_role_assignments,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/appRoleAssignments",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get endpoints from servicePrincipals",
        name: get_endpoints,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/endpoints/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property endpoints in servicePrincipals",
        name: update_endpoints,
        response: NoContent,
        path: "/servicePrincipals/{{id}}/endpoints/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get createdObjects from servicePrincipals",
        name: list_created_objects,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/createdObjects",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get entity from servicePrincipals by key",
        name: get_service_principal,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update entity in servicePrincipals",
        name: update_service_principal,
        response: NoContent,
        path: "/servicePrincipals/{{id}}",
        params: 1,
        has_body: true
    });

    delete!({
        doc: "# Delete entity from servicePrincipals",
        name: delete_service_principal,
        response: NoContent,
        path: "/servicePrincipals/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get endpoints from servicePrincipals",
        name: list_endpoints,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/endpoints",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to endpoints for servicePrincipals",
        name: create_endpoints,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/endpoints",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get homeRealmDiscoveryPolicies from servicePrincipals",
        name: get_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/homeRealmDiscoveryPolicies/{{id2}}",
        params: 2,
        has_body: false
    });

    get!({
        doc: "# Get ownedObjects from servicePrincipals",
        name: get_owned_objects,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/ownedObjects/{{id2}}",
        params: 2,
        has_body: false
    });

    post!({
        doc: "# Invoke action addPassword",
        name: add_password,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/addPassword",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get transitiveMemberOf from servicePrincipals",
        name: get_transitive_member_of,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/transitiveMemberOf/{{id2}}",
        params: 2,
        has_body: false
    });

    get!({
        doc: "# Get owners from servicePrincipals",
        name: list_owners,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/owners",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get entities from servicePrincipals",
        name: list_service_principal,
        response: serde_json::Value,
        path: "/servicePrincipals",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Add new entity to servicePrincipals",
        name: create_service_principal,
        response: serde_json::Value,
        path: "/servicePrincipals",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get appRoleAssignedTo from servicePrincipals",
        name: list_app_role_assigned_to,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/appRoleAssignedTo",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to appRoleAssignedTo for servicePrincipals",
        name: create_app_role_assigned_to,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/appRoleAssignedTo",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get memberOf from servicePrincipals",
        name: list_member_of,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/memberOf",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/servicePrincipals/delta()",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Invoke action removePassword",
        name: remove_password,
        response: NoContent,
        path: "/servicePrincipals/{{id}}/removePassword",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get memberOf from servicePrincipals",
        name: get_member_of,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/memberOf/{{id2}}",
        params: 2,
        has_body: false
    });

    get!({
        doc: "# Get oauth2PermissionGrants from servicePrincipals",
        name: service_principals_list_oauth_2_permission_grants,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/oauth2PermissionGrants",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get tokenIssuancePolicies from servicePrincipals",
        name: list_token_issuance_policies,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/tokenIssuancePolicies",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get owners from servicePrincipals",
        name: get_owners,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/owners/{{id2}}",
        params: 2,
        has_body: false
    });

    get!({
        doc: "# Get tokenLifetimePolicies from servicePrincipals",
        name: list_token_lifetime_policies,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/tokenLifetimePolicies",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get appRoleAssignments from servicePrincipals",
        name: get_app_role_assignments,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/appRoleAssignments/{{id2}}",
        params: 2,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property appRoleAssignments in servicePrincipals",
        name: update_app_role_assignments,
        response: NoContent,
        path: "/servicePrincipals/{{id}}/appRoleAssignments/{{id2}}",
        params: 2,
        has_body: true
    });

    get!({
        doc: "# Get oauth2PermissionGrants from servicePrincipals",
        name: service_principals_get_oauth_2_permission_grants,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/oauth2PermissionGrants/{{id2}}",
        params: 2,
        has_body: false
    });

    get!({
        doc: "# Get transitiveMemberOf from servicePrincipals",
        name: list_transitive_member_of,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/transitiveMemberOf",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get tokenIssuancePolicies from servicePrincipals",
        name: get_token_issuance_policies,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/tokenIssuancePolicies/{{id2}}",
        params: 2,
        has_body: false
    });

    get!({
        doc: "# Get ownedObjects from servicePrincipals",
        name: list_owned_objects,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/ownedObjects",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get createdObjects from servicePrincipals",
        name: get_created_objects,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/createdObjects/{{id2}}",
        params: 2,
        has_body: false
    });

    get!({
        doc: "# Get homeRealmDiscoveryPolicies from servicePrincipals",
        name: list_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/homeRealmDiscoveryPolicies",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get claimsMappingPolicies from servicePrincipals",
        name: list_claims_mapping_policies,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/claimsMappingPolicies",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get tokenLifetimePolicies from servicePrincipals",
        name: get_token_lifetime_policies,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/tokenLifetimePolicies/{{id2}}",
        params: 2,
        has_body: false
    });

    post!({
        doc: "# Invoke action addKey",
        name: add_key,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/addKey",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action removeKey",
        name: remove_key,
        response: NoContent,
        path: "/servicePrincipals/{{id}}/removeKey",
        params: 1,
        has_body: true
    });
}
