// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(PoliciesApiClient, ResourceIdentity::Policies);

impl PoliciesApiClient {
    get!(
        doc: "Get policies",
        name: get_policy_root,
        path: "/policies"
    );
    patch!(
        doc: "Update policies",
        name: update_policy_root,
        path: "/policies",
        body: true
    );
    post!(
        doc: "Create activityBasedTimeoutPolicy",
        name: create_activity_based_timeout_policies,
        path: "/policies/activityBasedTimeoutPolicies",
        body: true
    );
    get!(
        doc: "List activityBasedTimeoutPolicies",
        name: list_activity_based_timeout_policies,
        path: "/policies/activityBasedTimeoutPolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_activity_based_timeout_policies_count,
        path: "/policies/activityBasedTimeoutPolicies/$count"
    );
    delete!(
        doc: "Delete navigation property activityBasedTimeoutPolicies for policies",
        name: delete_activity_based_timeout_policies,
        path: "/policies/activityBasedTimeoutPolicies/{{id}}",
        params: activity_based_timeout_policy_id
    );
    get!(
        doc: "Get activityBasedTimeoutPolicies from policies",
        name: get_activity_based_timeout_policies,
        path: "/policies/activityBasedTimeoutPolicies/{{id}}",
        params: activity_based_timeout_policy_id
    );
    patch!(
        doc: "Update the navigation property activityBasedTimeoutPolicies in policies",
        name: update_activity_based_timeout_policies,
        path: "/policies/activityBasedTimeoutPolicies/{{id}}",
        body: true,
        params: activity_based_timeout_policy_id
    );
    delete!(
        doc: "Delete navigation property adminConsentRequestPolicy for policies",
        name: delete_admin_consent_request_policy,
        path: "/policies/adminConsentRequestPolicy"
    );
    get!(
        doc: "Get adminConsentRequestPolicy",
        name: get_admin_consent_request_policy,
        path: "/policies/adminConsentRequestPolicy"
    );
    patch!(
        doc: "Update adminConsentRequestPolicy",
        name: update_admin_consent_request_policy,
        path: "/policies/adminConsentRequestPolicy",
        body: true
    );
    post!(
        doc: "Create new navigation property to appManagementPolicies for policies",
        name: create_app_management_policies,
        path: "/policies/appManagementPolicies",
        body: true
    );
    get!(
        doc: "Get appManagementPolicies from policies",
        name: list_app_management_policies,
        path: "/policies/appManagementPolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_app_management_policies_count,
        path: "/policies/appManagementPolicies/$count"
    );
    delete!(
        doc: "Delete navigation property appManagementPolicies for policies",
        name: delete_app_management_policies,
        path: "/policies/appManagementPolicies/{{id}}",
        params: app_management_policy_id
    );
    get!(
        doc: "Get appManagementPolicies from policies",
        name: get_app_management_policies,
        path: "/policies/appManagementPolicies/{{id}}",
        params: app_management_policy_id
    );
    patch!(
        doc: "Update the navigation property appManagementPolicies in policies",
        name: update_app_management_policies,
        path: "/policies/appManagementPolicies/{{id}}",
        body: true,
        params: app_management_policy_id
    );
    get!(
        doc: "Get appliesTo from policies",
        name: list_applies_to,
        path: "/policies/appManagementPolicies/{{id}}/appliesTo",
        params: app_management_policy_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_applies_to_count,
        path: "/policies/appManagementPolicies/{{id}}/appliesTo/$count",
        params: app_management_policy_id
    );
    get!(
        doc: "Get appliesTo from policies",
        name: get_applies_to,
        path: "/policies/appManagementPolicies/{{id}}/appliesTo/{{id2}}",
        params: app_management_policy_id, directory_object_id
    );
    delete!(
        doc: "Delete navigation property authenticationFlowsPolicy for policies",
        name: delete_authentication_flows_policy,
        path: "/policies/authenticationFlowsPolicy"
    );
    get!(
        doc: "Get authenticationFlowsPolicy",
        name: get_authentication_flows_policy,
        path: "/policies/authenticationFlowsPolicy"
    );
    patch!(
        doc: "Update authenticationFlowsPolicy",
        name: update_authentication_flows_policy,
        path: "/policies/authenticationFlowsPolicy",
        body: true
    );
    delete!(
        doc: "Delete navigation property authenticationMethodsPolicy for policies",
        name: delete_authentication_methods_policy,
        path: "/policies/authenticationMethodsPolicy"
    );
    get!(
        doc: "Get authenticationMethodsPolicy",
        name: get_authentication_methods_policy,
        path: "/policies/authenticationMethodsPolicy"
    );
    patch!(
        doc: "Update authenticationMethodsPolicy",
        name: update_authentication_methods_policy,
        path: "/policies/authenticationMethodsPolicy",
        body: true
    );
    post!(
        doc: "Create new navigation property to authenticationMethodConfigurations for policies",
        name: create_authentication_method_configurations,
        path: "/policies/authenticationMethodsPolicy/authenticationMethodConfigurations",
        body: true
    );
    get!(
        doc: "Get authenticationMethodConfigurations from policies",
        name: list_authentication_method_configurations,
        path: "/policies/authenticationMethodsPolicy/authenticationMethodConfigurations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_authentication_method_configurations_count,
        path: "/policies/authenticationMethodsPolicy/authenticationMethodConfigurations/$count"
    );
    delete!(
        doc: "Delete navigation property authenticationMethodConfigurations for policies",
        name: delete_authentication_method_configurations,
        path: "/policies/authenticationMethodsPolicy/authenticationMethodConfigurations/{{id}}",
        params: authentication_method_configuration_id
    );
    get!(
        doc: "Get authenticationMethodConfigurations from policies",
        name: get_authentication_method_configurations,
        path: "/policies/authenticationMethodsPolicy/authenticationMethodConfigurations/{{id}}",
        params: authentication_method_configuration_id
    );
    patch!(
        doc: "Update the navigation property authenticationMethodConfigurations in policies",
        name: update_authentication_method_configurations,
        path: "/policies/authenticationMethodsPolicy/authenticationMethodConfigurations/{{id}}",
        body: true,
        params: authentication_method_configuration_id
    );
    delete!(
        doc: "Delete navigation property authorizationPolicy for policies",
        name: delete_authorization_policy,
        path: "/policies/authorizationPolicy"
    );
    get!(
        doc: "Get authorizationPolicy from policies",
        name: get_authorization_policy,
        path: "/policies/authorizationPolicy"
    );
    patch!(
        doc: "Update authorizationPolicy",
        name: update_authorization_policy,
        path: "/policies/authorizationPolicy",
        body: true
    );
    post!(
        doc: "Create claimsMappingPolicy",
        name: create_claims_mapping_policies,
        path: "/policies/claimsMappingPolicies",
        body: true
    );
    get!(
        doc: "List claimsMappingPolicies",
        name: list_claims_mapping_policies,
        path: "/policies/claimsMappingPolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_claims_mapping_policies_count,
        path: "/policies/claimsMappingPolicies/$count"
    );
    delete!(
        doc: "Delete navigation property claimsMappingPolicies for policies",
        name: delete_claims_mapping_policies,
        path: "/policies/claimsMappingPolicies/{{id}}",
        params: claims_mapping_policy_id
    );
    get!(
        doc: "Get claimsMappingPolicies from policies",
        name: get_claims_mapping_policies,
        path: "/policies/claimsMappingPolicies/{{id}}",
        params: claims_mapping_policy_id
    );
    patch!(
        doc: "Update the navigation property claimsMappingPolicies in policies",
        name: update_claims_mapping_policies,
        path: "/policies/claimsMappingPolicies/{{id}}",
        body: true,
        params: claims_mapping_policy_id
    );
    post!(
        doc: "Create new navigation property to conditionalAccessPolicies for policies",
        name: create_conditional_access_policies,
        path: "/policies/conditionalAccessPolicies",
        body: true
    );
    get!(
        doc: "Get conditionalAccessPolicies from policies",
        name: list_conditional_access_policies,
        path: "/policies/conditionalAccessPolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_conditional_access_policies_count,
        path: "/policies/conditionalAccessPolicies/$count"
    );
    delete!(
        doc: "Delete navigation property conditionalAccessPolicies for policies",
        name: delete_conditional_access_policies,
        path: "/policies/conditionalAccessPolicies/{{id}}",
        params: conditional_access_policy_id
    );
    get!(
        doc: "Get conditionalAccessPolicies from policies",
        name: get_conditional_access_policies,
        path: "/policies/conditionalAccessPolicies/{{id}}",
        params: conditional_access_policy_id
    );
    patch!(
        doc: "Update the navigation property conditionalAccessPolicies in policies",
        name: update_conditional_access_policies,
        path: "/policies/conditionalAccessPolicies/{{id}}",
        body: true,
        params: conditional_access_policy_id
    );
    delete!(
        doc: "Delete navigation property crossTenantAccessPolicy for policies",
        name: delete_cross_tenant_access_policy,
        path: "/policies/crossTenantAccessPolicy"
    );
    get!(
        doc: "Get crossTenantAccessPolicy",
        name: get_cross_tenant_access_policy,
        path: "/policies/crossTenantAccessPolicy"
    );
    patch!(
        doc: "Update crossTenantAccessPolicy",
        name: update_cross_tenant_access_policy,
        path: "/policies/crossTenantAccessPolicy",
        body: true
    );
    delete!(
        doc: "Delete navigation property default for policies",
        name: delete_default,
        path: "/policies/crossTenantAccessPolicy/default"
    );
    get!(
        doc: "Get crossTenantAccessPolicyConfigurationDefault",
        name: get_default,
        path: "/policies/crossTenantAccessPolicy/default"
    );
    patch!(
        doc: "Update crossTenantAccessPolicyConfigurationDefault",
        name: update_default,
        path: "/policies/crossTenantAccessPolicy/default",
        body: true
    );
    post!(
        doc: "Invoke action resetToSystemDefault",
        name: reset_to_system_default,
        path: "/policies/crossTenantAccessPolicy/default/resetToSystemDefault"
    );
    post!(
        doc: "Create crossTenantAccessPolicyConfigurationPartner",
        name: create_partners,
        path: "/policies/crossTenantAccessPolicy/partners",
        body: true
    );
    get!(
        doc: "List partners",
        name: list_partners,
        path: "/policies/crossTenantAccessPolicy/partners"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_partners_count,
        path: "/policies/crossTenantAccessPolicy/partners/$count"
    );
    delete!(
        doc: "Delete navigation property partners for policies",
        name: delete_partners,
        path: "/policies/crossTenantAccessPolicy/partners/{{id}}",
        params: cross_tenant_access_policy_configuration_partner_tenant_id
    );
    get!(
        doc: "Get partners from policies",
        name: get_partners,
        path: "/policies/crossTenantAccessPolicy/partners/{{id}}",
        params: cross_tenant_access_policy_configuration_partner_tenant_id
    );
    patch!(
        doc: "Update the navigation property partners in policies",
        name: update_partners,
        path: "/policies/crossTenantAccessPolicy/partners/{{id}}",
        body: true,
        params: cross_tenant_access_policy_configuration_partner_tenant_id
    );
    delete!(
        doc: "Delete navigation property defaultAppManagementPolicy for policies",
        name: delete_default_app_management_policy,
        path: "/policies/defaultAppManagementPolicy"
    );
    get!(
        doc: "Get defaultAppManagementPolicy from policies",
        name: get_default_app_management_policy,
        path: "/policies/defaultAppManagementPolicy"
    );
    patch!(
        doc: "Update the navigation property defaultAppManagementPolicy in policies",
        name: update_default_app_management_policy,
        path: "/policies/defaultAppManagementPolicy",
        body: true
    );
    post!(
        doc: "Create featureRolloutPolicy",
        name: create_feature_rollout_policies,
        path: "/policies/featureRolloutPolicies",
        body: true
    );
    get!(
        doc: "List featureRolloutPolicies",
        name: list_feature_rollout_policies,
        path: "/policies/featureRolloutPolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_feature_rollout_policies_count,
        path: "/policies/featureRolloutPolicies/$count"
    );
    delete!(
        doc: "Delete navigation property featureRolloutPolicies for policies",
        name: delete_feature_rollout_policies,
        path: "/policies/featureRolloutPolicies/{{id}}",
        params: feature_rollout_policy_id
    );
    get!(
        doc: "Get featureRolloutPolicies from policies",
        name: get_feature_rollout_policies,
        path: "/policies/featureRolloutPolicies/{{id}}",
        params: feature_rollout_policy_id
    );
    patch!(
        doc: "Update the navigation property featureRolloutPolicies in policies",
        name: update_feature_rollout_policies,
        path: "/policies/featureRolloutPolicies/{{id}}",
        body: true,
        params: feature_rollout_policy_id
    );
    post!(
        doc: "Create new navigation property to appliesTo for policies",
        name: create_applies_to,
        path: "/policies/featureRolloutPolicies/{{id}}/appliesTo",
        body: true,
        params: feature_rollout_policy_id
    );
    get!(
        doc: "Get appliesTo from policies",
        name: list_feature_roll_out_applies_to,
        path: "/policies/featureRolloutPolicies/{{id}}/appliesTo",
        params: feature_rollout_policy_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_feature_rollout_applies_to_count,
        path: "/policies/featureRolloutPolicies/{{id}}/appliesTo/$count",
        params: feature_rollout_policy_id
    );
    post!(
        doc: "Create new navigation property ref to appliesTo for policies",
        name: create_ref_applies_to,
        path: "/policies/featureRolloutPolicies/{{id}}/appliesTo/$ref",
        body: true,
        params: feature_rollout_policy_id
    );
    get!(
        doc: "Get ref of appliesTo from policies",
        name: list_ref_applies_to,
        path: "/policies/featureRolloutPolicies/{{id}}/appliesTo/$ref",
        params: feature_rollout_policy_id
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/policies/featureRolloutPolicies/{{id}}/appliesTo/delta()",
        params: feature_rollout_policy_id
    );
    post!(
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        path: "/policies/featureRolloutPolicies/{{id}}/appliesTo/getAvailableExtensionProperties",
        body: true,
        params: feature_rollout_policy_id
    );
    post!(
        doc: "Invoke action getByIds",
        name: get_by_ids,
        path: "/policies/featureRolloutPolicies/{{id}}/appliesTo/getByIds",
        body: true,
        params: feature_rollout_policy_id
    );
    post!(
        doc: "Invoke action validateProperties",
        name: validate_properties,
        path: "/policies/featureRolloutPolicies/{{id}}/appliesTo/validateProperties",
        body: true,
        params: feature_rollout_policy_id
    );
    delete!(
        doc: "Delete ref of navigation property appliesTo for policies",
        name: delete_ref_applies_to,
        path: "/policies/featureRolloutPolicies/{{id}}/appliesTo/{{id2}}/$ref",
        params: feature_rollout_policy_id, directory_object_id
    );
    post!(
        doc: "Create homeRealmDiscoveryPolicy",
        name: create_home_realm_discovery_policies,
        path: "/policies/homeRealmDiscoveryPolicies",
        body: true
    );
    get!(
        doc: "List homeRealmDiscoveryPolicies",
        name: list_home_realm_discovery_policies,
        path: "/policies/homeRealmDiscoveryPolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_home_realm_discovery_policies_count,
        path: "/policies/homeRealmDiscoveryPolicies/$count"
    );
    delete!(
        doc: "Delete navigation property homeRealmDiscoveryPolicies for policies",
        name: delete_home_realm_discovery_policies,
        path: "/policies/homeRealmDiscoveryPolicies/{{id}}",
        params: home_realm_discovery_policy_id
    );
    get!(
        doc: "Get homeRealmDiscoveryPolicies from policies",
        name: get_home_realm_discovery_policies,
        path: "/policies/homeRealmDiscoveryPolicies/{{id}}",
        params: home_realm_discovery_policy_id
    );
    patch!(
        doc: "Update the navigation property homeRealmDiscoveryPolicies in policies",
        name: update_home_realm_discovery_policies,
        path: "/policies/homeRealmDiscoveryPolicies/{{id}}",
        body: true,
        params: home_realm_discovery_policy_id
    );
    delete!(
        doc: "Delete navigation property identitySecurityDefaultsEnforcementPolicy for policies",
        name: delete_identity_security_defaults_enforcement_policy,
        path: "/policies/identitySecurityDefaultsEnforcementPolicy"
    );
    get!(
        doc: "Get identitySecurityDefaultsEnforcementPolicy",
        name: get_identity_security_defaults_enforcement_policy,
        path: "/policies/identitySecurityDefaultsEnforcementPolicy"
    );
    patch!(
        doc: "Update identitySecurityDefaultsEnforcementPolicy",
        name: update_identity_security_defaults_enforcement_policy,
        path: "/policies/identitySecurityDefaultsEnforcementPolicy",
        body: true
    );
    post!(
        doc: "Create permissionGrantPolicy",
        name: create_permission_grant_policies,
        path: "/policies/permissionGrantPolicies",
        body: true
    );
    get!(
        doc: "List permissionGrantPolicies",
        name: list_permission_grant_policies,
        path: "/policies/permissionGrantPolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_permission_grant_policies_count,
        path: "/policies/permissionGrantPolicies/$count"
    );
    delete!(
        doc: "Delete navigation property permissionGrantPolicies for policies",
        name: delete_permission_grant_policies,
        path: "/policies/permissionGrantPolicies/{{id}}",
        params: permission_grant_policy_id
    );
    get!(
        doc: "Get permissionGrantPolicies from policies",
        name: get_permission_grant_policies,
        path: "/policies/permissionGrantPolicies/{{id}}",
        params: permission_grant_policy_id
    );
    patch!(
        doc: "Update the navigation property permissionGrantPolicies in policies",
        name: update_permission_grant_policies,
        path: "/policies/permissionGrantPolicies/{{id}}",
        body: true,
        params: permission_grant_policy_id
    );
    post!(
        doc: "Create permissionGrantConditionSet in excludes collection of permissionGrantPolicy",
        name: create_excludes,
        path: "/policies/permissionGrantPolicies/{{id}}/excludes",
        body: true,
        params: permission_grant_policy_id
    );
    get!(
        doc: "List excludes collection of permissionGrantPolicy",
        name: list_excludes,
        path: "/policies/permissionGrantPolicies/{{id}}/excludes",
        params: permission_grant_policy_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_excludes_count,
        path: "/policies/permissionGrantPolicies/{{id}}/excludes/$count",
        params: permission_grant_policy_id
    );
    delete!(
        doc: "Delete navigation property excludes for policies",
        name: delete_excludes,
        path: "/policies/permissionGrantPolicies/{{id}}/excludes/{{id2}}",
        params: permission_grant_policy_id, permission_grant_condition_set_id
    );
    get!(
        doc: "Get excludes from policies",
        name: get_excludes,
        path: "/policies/permissionGrantPolicies/{{id}}/excludes/{{id2}}",
        params: permission_grant_policy_id, permission_grant_condition_set_id
    );
    patch!(
        doc: "Update the navigation property excludes in policies",
        name: update_excludes,
        path: "/policies/permissionGrantPolicies/{{id}}/excludes/{{id2}}",
        body: true,
        params: permission_grant_policy_id, permission_grant_condition_set_id
    );
    post!(
        doc: "Create permissionGrantConditionSet in includes collection of permissionGrantPolicy",
        name: create_includes,
        path: "/policies/permissionGrantPolicies/{{id}}/includes",
        body: true,
        params: permission_grant_policy_id
    );
    get!(
        doc: "List includes collection of permissionGrantPolicy",
        name: list_includes,
        path: "/policies/permissionGrantPolicies/{{id}}/includes",
        params: permission_grant_policy_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_includes_count,
        path: "/policies/permissionGrantPolicies/{{id}}/includes/$count",
        params: permission_grant_policy_id
    );
    delete!(
        doc: "Delete navigation property includes for policies",
        name: delete_includes,
        path: "/policies/permissionGrantPolicies/{{id}}/includes/{{id2}}",
        params: permission_grant_policy_id, permission_grant_condition_set_id
    );
    get!(
        doc: "Get includes from policies",
        name: get_includes,
        path: "/policies/permissionGrantPolicies/{{id}}/includes/{{id2}}",
        params: permission_grant_policy_id, permission_grant_condition_set_id
    );
    patch!(
        doc: "Update the navigation property includes in policies",
        name: update_includes,
        path: "/policies/permissionGrantPolicies/{{id}}/includes/{{id2}}",
        body: true,
        params: permission_grant_policy_id, permission_grant_condition_set_id
    );
    post!(
        doc: "Create new navigation property to roleManagementPolicies for policies",
        name: create_role_management_policies,
        path: "/policies/roleManagementPolicies",
        body: true
    );
    get!(
        doc: "List roleManagementPolicies",
        name: list_role_management_policies,
        path: "/policies/roleManagementPolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: role_management_policies_cdca,
        path: "/policies/roleManagementPolicies/$count"
    );
    delete!(
        doc: "Delete navigation property roleManagementPolicies for policies",
        name: delete_role_management_policies,
        path: "/policies/roleManagementPolicies/{{id}}",
        params: unified_role_management_policy_id
    );
    get!(
        doc: "Get roleManagementPolicies from policies",
        name: get_role_management_policies,
        path: "/policies/roleManagementPolicies/{{id}}",
        params: unified_role_management_policy_id
    );
    patch!(
        doc: "Update the navigation property roleManagementPolicies in policies",
        name: update_role_management_policies,
        path: "/policies/roleManagementPolicies/{{id}}",
        body: true,
        params: unified_role_management_policy_id
    );
    post!(
        doc: "Create new navigation property to effectiveRules for policies",
        name: create_effective_rules,
        path: "/policies/roleManagementPolicies/{{id}}/effectiveRules",
        body: true,
        params: unified_role_management_policy_id
    );
    get!(
        doc: "Get effectiveRules from policies",
        name: list_effective_rules,
        path: "/policies/roleManagementPolicies/{{id}}/effectiveRules",
        params: unified_role_management_policy_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_effective_rules_count,
        path: "/policies/roleManagementPolicies/{{id}}/effectiveRules/$count",
        params: unified_role_management_policy_id
    );
    delete!(
        doc: "Delete navigation property effectiveRules for policies",
        name: delete_effective_rules,
        path: "/policies/roleManagementPolicies/{{id}}/effectiveRules/{{id2}}",
        params: unified_role_management_policy_id, unified_role_management_policy_rule_id
    );
    get!(
        doc: "Get effectiveRules from policies",
        name: get_effective_rules,
        path: "/policies/roleManagementPolicies/{{id}}/effectiveRules/{{id2}}",
        params: unified_role_management_policy_id, unified_role_management_policy_rule_id
    );
    patch!(
        doc: "Update the navigation property effectiveRules in policies",
        name: update_effective_rules,
        path: "/policies/roleManagementPolicies/{{id}}/effectiveRules/{{id2}}",
        body: true,
        params: unified_role_management_policy_id, unified_role_management_policy_rule_id
    );
    post!(
        doc: "Create new navigation property to rules for policies",
        name: create_rules,
        path: "/policies/roleManagementPolicies/{{id}}/rules",
        body: true,
        params: unified_role_management_policy_id
    );
    get!(
        doc: "List rules (for a role management policy)",
        name: list_rules,
        path: "/policies/roleManagementPolicies/{{id}}/rules",
        params: unified_role_management_policy_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_rules_count,
        path: "/policies/roleManagementPolicies/{{id}}/rules/$count",
        params: unified_role_management_policy_id
    );
    delete!(
        doc: "Delete navigation property rules for policies",
        name: delete_rules,
        path: "/policies/roleManagementPolicies/{{id}}/rules/{{id2}}",
        params: unified_role_management_policy_id, unified_role_management_policy_rule_id
    );
    get!(
        doc: "Get rules from policies",
        name: get_rules,
        path: "/policies/roleManagementPolicies/{{id}}/rules/{{id2}}",
        params: unified_role_management_policy_id, unified_role_management_policy_rule_id
    );
    patch!(
        doc: "Update the navigation property rules in policies",
        name: update_rules,
        path: "/policies/roleManagementPolicies/{{id}}/rules/{{id2}}",
        body: true,
        params: unified_role_management_policy_id, unified_role_management_policy_rule_id
    );
    post!(
        doc: "Create new navigation property to roleManagementPolicyAssignments for policies",
        name: create_role_management_policy_assignments,
        path: "/policies/roleManagementPolicyAssignments",
        body: true
    );
    get!(
        doc: "List roleManagementPolicyAssignments",
        name: list_role_management_policy_assignments,
        path: "/policies/roleManagementPolicyAssignments"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_role_management_policy_assignments_count,
        path: "/policies/roleManagementPolicyAssignments/$count"
    );
    delete!(
        doc: "Delete navigation property roleManagementPolicyAssignments for policies",
        name: delete_role_management_policy_assignments,
        path: "/policies/roleManagementPolicyAssignments/{{id}}",
        params: unified_role_management_policy_assignment_id
    );
    get!(
        doc: "Get roleManagementPolicyAssignments from policies",
        name: get_role_management_policy_assignments,
        path: "/policies/roleManagementPolicyAssignments/{{id}}",
        params: unified_role_management_policy_assignment_id
    );
    patch!(
        doc: "Update the navigation property roleManagementPolicyAssignments in policies",
        name: update_role_management_policy_assignments,
        path: "/policies/roleManagementPolicyAssignments/{{id}}",
        body: true,
        params: unified_role_management_policy_assignment_id
    );
    get!(
        doc: "Get policy from policies",
        name: get_policy,
        path: "/policies/roleManagementPolicyAssignments/{{id}}/policy",
        params: unified_role_management_policy_assignment_id
    );
    post!(
        doc: "Create tokenIssuancePolicy",
        name: create_token_issuance_policies,
        path: "/policies/tokenIssuancePolicies",
        body: true
    );
    get!(
        doc: "List tokenIssuancePolicy",
        name: list_token_issuance_policies,
        path: "/policies/tokenIssuancePolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_token_issuance_policies_count,
        path: "/policies/tokenIssuancePolicies/$count"
    );
    delete!(
        doc: "Delete navigation property tokenIssuancePolicies for policies",
        name: delete_token_issuance_policies,
        path: "/policies/tokenIssuancePolicies/{{id}}",
        params: token_issuance_policy_id
    );
    get!(
        doc: "Get tokenIssuancePolicies from policies",
        name: get_token_issuance_policies,
        path: "/policies/tokenIssuancePolicies/{{id}}",
        params: token_issuance_policy_id
    );
    patch!(
        doc: "Update the navigation property tokenIssuancePolicies in policies",
        name: update_token_issuance_policies,
        path: "/policies/tokenIssuancePolicies/{{id}}",
        body: true,
        params: token_issuance_policy_id
    );
    post!(
        doc: "Create tokenLifetimePolicy",
        name: create_token_lifetime_policies,
        path: "/policies/tokenLifetimePolicies",
        body: true
    );
    get!(
        doc: "List tokenLifetimePolicies",
        name: list_token_lifetime_policies,
        path: "/policies/tokenLifetimePolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_token_lifetime_policies_count,
        path: "/policies/tokenLifetimePolicies/$count"
    );
    delete!(
        doc: "Delete navigation property tokenLifetimePolicies for policies",
        name: delete_token_lifetime_policies,
        path: "/policies/tokenLifetimePolicies/{{id}}",
        params: token_lifetime_policy_id
    );
    get!(
        doc: "Get tokenLifetimePolicies from policies",
        name: get_token_lifetime_policies,
        path: "/policies/tokenLifetimePolicies/{{id}}",
        params: token_lifetime_policy_id
    );
    patch!(
        doc: "Update the navigation property tokenLifetimePolicies in policies",
        name: update_token_lifetime_policies,
        path: "/policies/tokenLifetimePolicies/{{id}}",
        body: true,
        params: token_lifetime_policy_id
    );
}
