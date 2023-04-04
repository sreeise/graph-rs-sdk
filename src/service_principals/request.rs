// GENERATED CODE

use crate::api_default_imports::*;
use crate::service_principals::*;
use crate::users::*;

resource_api_client!(
    ServicePrincipalsApiClient,
    ServicePrincipalsIdApiClient,
    ResourceIdentity::ServicePrincipals
);

impl ServicePrincipalsApiClient {
    post!(
        doc: "Create servicePrincipal",
        name: create_service_principal,
        path: "/servicePrincipals",
        body: true
    );
    get!(
        doc: "List servicePrincipals",
        name: list_service_principal,
        path: "/servicePrincipals"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_service_principals_count,
        path: "/servicePrincipals/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/servicePrincipals/delta()"
    );
    post!(
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        path: "/servicePrincipals/getAvailableExtensionProperties",
        body: true
    );
    post!(
        doc: "Invoke action getByIds",
        name: get_by_ids,
        path: "/servicePrincipals/getByIds",
        body: true
    );
    post!(
        doc: "Invoke action validateProperties",
        name: validate_properties,
        path: "/servicePrincipals/validateProperties",
        body: true
    );
}

impl ServicePrincipalsIdApiClient {
    api_client_link_id!(owner, ServicePrincipalsOwnersIdApiClient);
    api_client_link!(member_of, MemberOfApiClient);
    api_client_link!(transitive_member_of, TransitiveMemberOfApiClient);
    api_client_link!(owners, ServicePrincipalsOwnersApiClient);

    delete!(
        doc: "Delete servicePrincipal",
        name: delete_service_principal,
        path: "/servicePrincipals/{{RID}}"
    );
    get!(
        doc: "Get servicePrincipal",
        name: get_service_principal,
        path: "/servicePrincipals/{{RID}}"
    );
    patch!(
        doc: "Update entity in servicePrincipals",
        name: update_service_principal,
        path: "/servicePrincipals/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action addKey",
        name: add_key,
        path: "/servicePrincipals/{{RID}}/addKey",
        body: true
    );
    post!(
        doc: "Invoke action addPassword",
        name: add_password,
        path: "/servicePrincipals/{{RID}}/addPassword",
        body: true
    );
    post!(
        doc: "Invoke action addTokenSigningCertificate",
        name: add_token_signing_certificate,
        path: "/servicePrincipals/{{RID}}/addTokenSigningCertificate",
        body: true
    );
    get!(
        doc: "Get appManagementPolicies from servicePrincipals",
        name: list_app_management_policies,
        path: "/servicePrincipals/{{RID}}/appManagementPolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: count,
        path: "/servicePrincipals/{{RID}}/appManagementPolicies/$count"
    );
    get!(
        doc: "Get appManagementPolicies from servicePrincipals",
        name: get_app_management_policies,
        path: "/servicePrincipals/{{RID}}/appManagementPolicies/{{id}}",
        params: app_management_policy_id
    );
    post!(
        doc: "Grant an appRoleAssignment for a service principal",
        name: create_app_role_assigned_to,
        path: "/servicePrincipals/{{RID}}/appRoleAssignedTo",
        body: true
    );
    get!(
        doc: "List appRoleAssignments granted for a service principal",
        name: list_app_role_assigned_to,
        path: "/servicePrincipals/{{RID}}/appRoleAssignedTo"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_app_role_assigned_to_count,
        path: "/servicePrincipals/{{RID}}/appRoleAssignedTo/$count"
    );
    delete!(
        doc: "Delete navigation property appRoleAssignedTo for servicePrincipals",
        name: delete_app_role_assigned_to,
        path: "/servicePrincipals/{{RID}}/appRoleAssignedTo/{{id}}",
        params: app_role_assignment_id
    );
    get!(
        doc: "Get appRoleAssignedTo from servicePrincipals",
        name: get_app_role_assigned_to,
        path: "/servicePrincipals/{{RID}}/appRoleAssignedTo/{{id}}",
        params: app_role_assignment_id
    );
    patch!(
        doc: "Update the navigation property appRoleAssignedTo in servicePrincipals",
        name: update_app_role_assigned_to,
        path: "/servicePrincipals/{{RID}}/appRoleAssignedTo/{{id}}",
        body: true,
        params: app_role_assignment_id
    );
    get!(
        doc: "Get appRoleAssignments from servicePrincipals",
        name: list_app_role_assignments,
        path: "/servicePrincipals/{{RID}}/appRoleAssignments"
    );
    post!(
        doc: "Grant an appRoleAssignment to a service principal",
        name: create_app_role_assignments,
        path: "/servicePrincipals/{{RID}}/appRoleAssignments",
        body: true
    );
    get!(
        doc: "Get the number of the resource",
        name: get_app_role_assignments_count,
        path: "/servicePrincipals/{{RID}}/appRoleAssignments/$count"
    );
    delete!(
        doc: "Delete navigation property appRoleAssignments for servicePrincipals",
        name: delete_app_role_assignments,
        path: "/servicePrincipals/{{RID}}/appRoleAssignments/{{id}}",
        params: app_role_assignment_id
    );
    get!(
        doc: "Get appRoleAssignments from servicePrincipals",
        name: get_app_role_assignments,
        path: "/servicePrincipals/{{RID}}/appRoleAssignments/{{id}}",
        params: app_role_assignment_id
    );
    patch!(
        doc: "Update the navigation property appRoleAssignments in servicePrincipals",
        name: update_app_role_assignments,
        path: "/servicePrincipals/{{RID}}/appRoleAssignments/{{id}}",
        body: true,
        params: app_role_assignment_id
    );
    post!(
        doc: "Invoke action checkMemberGroups",
        name: check_member_groups,
        path: "/servicePrincipals/{{RID}}/checkMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberObjects",
        name: check_member_objects,
        path: "/servicePrincipals/{{RID}}/checkMemberObjects",
        body: true
    );
    get!(
        doc: "List assigned claimsMappingPolicy",
        name: list_claims_mapping_policies,
        path: "/servicePrincipals/{{RID}}/claimsMappingPolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_claims_mapping_policies_count,
        path: "/servicePrincipals/{{RID}}/claimsMappingPolicies/$count"
    );
    post!(
        doc: "Create new navigation property ref to claimsMappingPolicies for servicePrincipals",
        name: create_ref_claims_mapping_policies,
        path: "/servicePrincipals/{{RID}}/claimsMappingPolicies/$ref",
        body: true
    );
    get!(
        doc: "List assigned claimsMappingPolicy",
        name: list_ref_claims_mapping_policies,
        path: "/servicePrincipals/{{RID}}/claimsMappingPolicies/$ref"
    );
    delete!(
        doc: "Delete ref of navigation property claimsMappingPolicies for servicePrincipals",
        name: delete_ref_claims_mapping_policies,
        path: "/servicePrincipals/{{RID}}/claimsMappingPolicies/{{id}}/$ref",
        params: claims_mapping_policy_id
    );
    post!(
        doc: "Create delegatedPermissionClassification",
        name: create_delegated_permission_classifications,
        path: "/servicePrincipals/{{RID}}/delegatedPermissionClassifications",
        body: true
    );
    get!(
        doc: "List delegatedPermissionClassifications collection of servicePrincipal",
        name: list_delegated_permission_classifications,
        path: "/servicePrincipals/{{RID}}/delegatedPermissionClassifications"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_delegated_permission_classifications_count,
        path: "/servicePrincipals/{{RID}}/delegatedPermissionClassifications/$count"
    );
    delete!(
        doc: "Delete navigation property delegatedPermissionClassifications for servicePrincipals",
        name: delete_delegated_permission_classifications,
        path: "/servicePrincipals/{{RID}}/delegatedPermissionClassifications/{{id}}",
        params: delegated_permission_classification_id
    );
    get!(
        doc: "Get delegatedPermissionClassifications from servicePrincipals",
        name: get_delegated_permission_classifications,
        path: "/servicePrincipals/{{RID}}/delegatedPermissionClassifications/{{id}}",
        params: delegated_permission_classification_id
    );
    patch!(
        doc: "Update the navigation property delegatedPermissionClassifications in servicePrincipals",
        name: update_delegated_permission_classifications,
        path: "/servicePrincipals/{{RID}}/delegatedPermissionClassifications/{{id}}",
        body: true,
        params: delegated_permission_classification_id
    );
    post!(
        doc: "Create new navigation property to endpoints for servicePrincipals",
        name: create_endpoints,
        path: "/servicePrincipals/{{RID}}/endpoints",
        body: true
    );
    get!(
        doc: "Get endpoints from servicePrincipals",
        name: list_endpoints,
        path: "/servicePrincipals/{{RID}}/endpoints"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_endpoints_count,
        path: "/servicePrincipals/{{RID}}/endpoints/$count"
    );
    delete!(
        doc: "Delete navigation property endpoints for servicePrincipals",
        name: delete_endpoints,
        path: "/servicePrincipals/{{RID}}/endpoints/{{id}}",
        params: endpoint_id
    );
    get!(
        doc: "Get endpoints from servicePrincipals",
        name: get_endpoints,
        path: "/servicePrincipals/{{RID}}/endpoints/{{id}}",
        params: endpoint_id
    );
    patch!(
        doc: "Update the navigation property endpoints in servicePrincipals",
        name: update_endpoints,
        path: "/servicePrincipals/{{RID}}/endpoints/{{id}}",
        body: true,
        params: endpoint_id
    );
    post!(
        doc: "Create new navigation property to federatedIdentityCredentials for servicePrincipals",
        name: create_federated_identity_credentials,
        path: "/servicePrincipals/{{RID}}/federatedIdentityCredentials",
        body: true
    );
    get!(
        doc: "Get federatedIdentityCredentials from servicePrincipals",
        name: list_federated_identity_credentials,
        path: "/servicePrincipals/{{RID}}/federatedIdentityCredentials"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_federated_identity_credentials_count,
        path: "/servicePrincipals/{{RID}}/federatedIdentityCredentials/$count"
    );
    delete!(
        doc: "Delete navigation property federatedIdentityCredentials for servicePrincipals",
        name: delete_federated_identity_credentials,
        path: "/servicePrincipals/{{RID}}/federatedIdentityCredentials/{{id}}",
        params: federated_identity_credential_id
    );
    get!(
        doc: "Get federatedIdentityCredentials from servicePrincipals",
        name: get_federated_identity_credentials,
        path: "/servicePrincipals/{{RID}}/federatedIdentityCredentials/{{id}}",
        params: federated_identity_credential_id
    );
    patch!(
        doc: "Update the navigation property federatedIdentityCredentials in servicePrincipals",
        name: update_federated_identity_credentials,
        path: "/servicePrincipals/{{RID}}/federatedIdentityCredentials/{{id}}",
        body: true,
        params: federated_identity_credential_id
    );
    post!(
        doc: "Invoke action getMemberGroups",
        name: get_member_groups,
        path: "/servicePrincipals/{{RID}}/getMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action getMemberObjects",
        name: get_member_objects,
        path: "/servicePrincipals/{{RID}}/getMemberObjects",
        body: true
    );
    get!(
        doc: "List assigned homeRealmDiscoveryPolicy",
        name: list_home_realm_discovery_policies,
        path: "/servicePrincipals/{{RID}}/homeRealmDiscoveryPolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_home_realm_discovery_policies_count,
        path: "/servicePrincipals/{{RID}}/homeRealmDiscoveryPolicies/$count"
    );
    post!(
        doc: "Create new navigation property ref to homeRealmDiscoveryPolicies for servicePrincipals",
        name: create_ref_home_realm_discovery_policies,
        path: "/servicePrincipals/{{RID}}/homeRealmDiscoveryPolicies/$ref",
        body: true
    );
    get!(
        doc: "List assigned homeRealmDiscoveryPolicy",
        name: list_ref_home_realm_discovery_policies,
        path: "/servicePrincipals/{{RID}}/homeRealmDiscoveryPolicies/$ref"
    );
    delete!(
        doc: "Delete ref of navigation property homeRealmDiscoveryPolicies for servicePrincipals",
        name: delete_ref_home_realm_discovery_policies,
        path: "/servicePrincipals/{{RID}}/homeRealmDiscoveryPolicies/{{id}}/$ref",
        params: home_realm_discovery_policy_id
    );
    post!(
        doc: "Invoke action removeKey",
        name: remove_key,
        path: "/servicePrincipals/{{RID}}/removeKey",
        body: true
    );
    post!(
        doc: "Invoke action removePassword",
        name: remove_password,
        path: "/servicePrincipals/{{RID}}/removePassword",
        body: true
    );
    post!(
        doc: "Invoke action restore",
        name: restore,
        path: "/servicePrincipals/{{RID}}/restore"
    );
    get!(
        doc: "Get tokenIssuancePolicies from servicePrincipals",
        name: list_token_issuance_policies,
        path: "/servicePrincipals/{{RID}}/tokenIssuancePolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_token_issuance_policies_count,
        path: "/servicePrincipals/{{RID}}/tokenIssuancePolicies/$count"
    );
    get!(
        doc: "Get tokenIssuancePolicies from servicePrincipals",
        name: get_token_issuance_policies,
        path: "/servicePrincipals/{{RID}}/tokenIssuancePolicies/{{id}}",
        params: token_issuance_policy_id
    );
    get!(
        doc: "Get tokenLifetimePolicies from servicePrincipals",
        name: list_token_lifetime_policies,
        path: "/servicePrincipals/{{RID}}/tokenLifetimePolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_token_lifetime_policies_count,
        path: "/servicePrincipals/{{RID}}/tokenLifetimePolicies/$count"
    );
    get!(
        doc: "Get tokenLifetimePolicies from servicePrincipals",
        name: get_token_lifetime_policies,
        path: "/servicePrincipals/{{RID}}/tokenLifetimePolicies/{{id}}",
        params: token_lifetime_policy_id
    );
}
