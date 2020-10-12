use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::{GraphResponse, IntoResponse};
use reqwest::Method;

register_client!(ServicePrincipalsRequest,);

#[allow(dead_code)]
impl<'a, Client> ServicePrincipalsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/microsoft.graph.delta()",
        params: 0,
        has_body: false
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
        name: service_principals_get_oauth_2_permission_grants,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/oauth2PermissionGrants/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get endpoints from servicePrincipals",
        name: list_endpoints,
        response: Collection<serde_json::Value>,
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
    post!({
        doc: "# Invoke action addPassword",
        name: add_password,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/microsoft.graph.addPassword",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get ref of transitiveMemberOf from servicePrincipals",
        name: list_ref_transitive_member_of,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/transitiveMemberOf/$ref",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property ref to transitiveMemberOf for servicePrincipals",
        name: create_ref_transitive_member_of,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/transitiveMemberOf/$ref",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete ref of navigation property transitiveMemberOf for servicePrincipals",
        name: delete_ref_transitive_member_of,
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}/transitiveMemberOf/$ref",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get ref of claimsMappingPolicies from servicePrincipals",
        name: list_ref_claims_mapping_policies,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/claimsMappingPolicies/$ref",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property ref to claimsMappingPolicies for servicePrincipals",
        name: create_ref_claims_mapping_policies,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/claimsMappingPolicies/$ref",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete ref of navigation property claimsMappingPolicies for servicePrincipals",
        name: delete_ref_claims_mapping_policies,
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}/claimsMappingPolicies/$ref",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get ref of ownedObjects from servicePrincipals",
        name: list_ref_owned_objects,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/ownedObjects/$ref",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property ref to ownedObjects for servicePrincipals",
        name: create_ref_owned_objects,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/ownedObjects/$ref",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete ref of navigation property ownedObjects for servicePrincipals",
        name: delete_ref_owned_objects,
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}/ownedObjects/$ref",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get createdObjects from servicePrincipals",
        name: list_created_objects,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/createdObjects",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action getMemberGroups",
        name: get_member_groups,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/microsoft.graph.getMemberGroups",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action getMemberObjects",
        name: get_member_objects,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/microsoft.graph.getMemberObjects",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action addKey",
        name: add_key,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/microsoft.graph.addKey",
        params: 1,
        has_body: true
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
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}/appRoleAssignments/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property appRoleAssignments for servicePrincipals",
        name: delete_app_role_assignments,
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}/appRoleAssignments/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get homeRealmDiscoveryPolicies from servicePrincipals",
        name: list_home_realm_discovery_policies,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/homeRealmDiscoveryPolicies",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get claimsMappingPolicies from servicePrincipals",
        name: list_claims_mapping_policies,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/claimsMappingPolicies",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get ownedObjects from servicePrincipals",
        name: list_owned_objects,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/ownedObjects",
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
        doc: "# Get tokenIssuancePolicies from servicePrincipals",
        name: list_token_issuance_policies,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/tokenIssuancePolicies",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get ref of createdObjects from servicePrincipals",
        name: list_ref_created_objects,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/createdObjects/$ref",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property ref to createdObjects for servicePrincipals",
        name: create_ref_created_objects,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/createdObjects/$ref",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete ref of navigation property createdObjects for servicePrincipals",
        name: delete_ref_created_objects,
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}/createdObjects/$ref",
        params: 1,
        has_body: false
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
        doc: "# Get tokenIssuancePolicies from servicePrincipals",
        name: get_token_issuance_policies,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/tokenIssuancePolicies/{{id2}}",
        params: 2,
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
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from servicePrincipals",
        name: delete_service_principal,
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}",
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
    get!({
        doc: "# Get entities from servicePrincipals",
        name: list_service_principal,
        response: Collection<serde_json::Value>,
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
        doc: "# Get memberOf from servicePrincipals",
        name: list_member_of,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/memberOf",
        params: 1,
        has_body: false
    });
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
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}/appRoleAssignedTo/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property appRoleAssignedTo for servicePrincipals",
        name: delete_app_role_assigned_to,
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}/appRoleAssignedTo/{{id2}}",
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
        doc: "# Get claimsMappingPolicies from servicePrincipals",
        name: get_claims_mapping_policies,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/claimsMappingPolicies/{{id2}}",
        params: 2,
        has_body: false
    });
    post!({
        doc: "# Invoke action removePassword",
        name: remove_password,
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}/microsoft.graph.removePassword",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get owners from servicePrincipals",
        name: list_owners,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/owners",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get appRoleAssignedTo from servicePrincipals",
        name: list_app_role_assigned_to,
        response: Collection<serde_json::Value>,
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
    post!({
        doc: "# Invoke action removeKey",
        name: remove_key,
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}/microsoft.graph.removeKey",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get ref of tokenLifetimePolicies from servicePrincipals",
        name: list_ref_token_lifetime_policies,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/tokenLifetimePolicies/$ref",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property ref to tokenLifetimePolicies for servicePrincipals",
        name: create_ref_token_lifetime_policies,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/tokenLifetimePolicies/$ref",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete ref of navigation property tokenLifetimePolicies for servicePrincipals",
        name: delete_ref_token_lifetime_policies,
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}/tokenLifetimePolicies/$ref",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action validateProperties",
        name: validate_properties,
        response: GraphResponse<Content>,
        path: "/servicePrincipals/microsoft.graph.validateProperties",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action checkMemberGroups",
        name: check_member_groups,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/microsoft.graph.checkMemberGroups",
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
        doc: "# Get ref of memberOf from servicePrincipals",
        name: list_ref_member_of,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/memberOf/$ref",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property ref to memberOf for servicePrincipals",
        name: create_ref_member_of,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/memberOf/$ref",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete ref of navigation property memberOf for servicePrincipals",
        name: delete_ref_member_of,
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}/memberOf/$ref",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action restore",
        name: restore,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/microsoft.graph.restore",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get appRoleAssignments from servicePrincipals",
        name: list_app_role_assignments,
        response: Collection<serde_json::Value>,
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
        doc: "# Get ownedObjects from servicePrincipals",
        name: get_owned_objects,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/ownedObjects/{{id2}}",
        params: 2,
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
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}/endpoints/{{id2}}",
        params: 2,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property endpoints for servicePrincipals",
        name: delete_endpoints,
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}/endpoints/{{id2}}",
        params: 2,
        has_body: false
    });
    get!({
        doc: "# Get ref of homeRealmDiscoveryPolicies from servicePrincipals",
        name: list_ref_home_realm_discovery_policies,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/homeRealmDiscoveryPolicies/$ref",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property ref to homeRealmDiscoveryPolicies for servicePrincipals",
        name: create_ref_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/homeRealmDiscoveryPolicies/$ref",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete ref of navigation property homeRealmDiscoveryPolicies for servicePrincipals",
        name: delete_ref_home_realm_discovery_policies,
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}/homeRealmDiscoveryPolicies/$ref",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get tokenLifetimePolicies from servicePrincipals",
        name: list_token_lifetime_policies,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/tokenLifetimePolicies",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get ref of tokenIssuancePolicies from servicePrincipals",
        name: list_ref_token_issuance_policies,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/tokenIssuancePolicies/$ref",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property ref to tokenIssuancePolicies for servicePrincipals",
        name: create_ref_token_issuance_policies,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/tokenIssuancePolicies/$ref",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete ref of navigation property tokenIssuancePolicies for servicePrincipals",
        name: delete_ref_token_issuance_policies,
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}/tokenIssuancePolicies/$ref",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get transitiveMemberOf from servicePrincipals",
        name: list_transitive_member_of,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/transitiveMemberOf",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get ref of owners from servicePrincipals",
        name: list_ref_owners,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/owners/$ref",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property ref to owners for servicePrincipals",
        name: create_ref_owners,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/owners/$ref",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete ref of navigation property owners for servicePrincipals",
        name: delete_ref_owners,
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}/owners/$ref",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get ref of oauth2PermissionGrants from servicePrincipals",
        name: service_principals_list_ref_oauth_2_permission_grants,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/oauth2PermissionGrants/$ref",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property ref to oauth2PermissionGrants for servicePrincipals",
        name: service_principals_create_ref_oauth_2_permission_grants,
        response: serde_json::Value,
        path: "/servicePrincipals/{{id}}/oauth2PermissionGrants/$ref",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete ref of navigation property oauth2PermissionGrants for servicePrincipals",
        name: service_principals_delete_ref_oauth_2_permission_grants,
        response: GraphResponse<Content>,
        path: "/servicePrincipals/{{id}}/oauth2PermissionGrants/$ref",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/microsoft.graph.getAvailableExtensionProperties",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action checkMemberObjects",
        name: check_member_objects,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/{{id}}/microsoft.graph.checkMemberObjects",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action getByIds",
        name: get_by_ids,
        response: Collection<serde_json::Value>,
        path: "/servicePrincipals/microsoft.graph.getByIds",
        params: 0,
        has_body: true
    });
}
