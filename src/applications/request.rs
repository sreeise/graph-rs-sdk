// GENERATED CODE

use crate::api_default_imports::*;
use crate::service_principals::*;

resource_api_client!(
    ApplicationsApiClient,
    ApplicationsIdApiClient,
    ResourceIdentity::Applications
);

impl ApplicationsApiClient {
    post!(
        doc: "Create application",
        name: create_application,
        path: "/applications",
        body: true
    );
    get!(
        doc: "List applications",
        name: list_application,
        path: "/applications"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_applications_count,
        path: "/applications/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/applications/delta()"
    );
    post!(
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        path: "/applications/getAvailableExtensionProperties",
        body: true
    );
    post!(
        doc: "Invoke action getByIds",
        name: get_by_ids,
        path: "/applications/getByIds",
        body: true
    );
    post!(
        doc: "Invoke action validateProperties",
        name: validate_properties,
        path: "/applications/validateProperties",
        body: true
    );
}

impl ApplicationsIdApiClient {
    api_client_link_id!(owner, ServicePrincipalsOwnersIdApiClient);
    api_client_link!(owners, ServicePrincipalsOwnersApiClient);

    delete!(
        doc: "Delete application",
        name: delete_application,
        path: "/applications/{{RID}}"
    );
    get!(
        doc: "Get application",
        name: get_application,
        path: "/applications/{{RID}}"
    );
    patch!(
        doc: "Update application",
        name: update_application,
        path: "/applications/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action addKey",
        name: add_key,
        path: "/applications/{{RID}}/addKey",
        body: true
    );
    post!(
        doc: "Invoke action addPassword",
        name: add_password,
        path: "/applications/{{RID}}/addPassword",
        body: true
    );
    get!(
        doc: "Get appManagementPolicies from applications",
        name: list_app_management_policies,
        path: "/applications/{{RID}}/appManagementPolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: count,
        path: "/applications/{{RID}}/appManagementPolicies/$count"
    );
    post!(
        doc: "Create new navigation property ref to appManagementPolicies for applications",
        name: create_ref_app_management_policies,
        path: "/applications/{{RID}}/appManagementPolicies/$ref",
        body: true
    );
    get!(
        doc: "Get ref of appManagementPolicies from applications",
        name: list_ref_app_management_policies,
        path: "/applications/{{RID}}/appManagementPolicies/$ref"
    );
    delete!(
        doc: "Delete ref of navigation property appManagementPolicies for applications",
        name: delete_ref_app_management_policies,
        path: "/applications/{{RID}}/appManagementPolicies/{{id}}/$ref",
        params: app_management_policy_id
    );
    post!(
        doc: "Invoke action checkMemberGroups",
        name: check_member_groups,
        path: "/applications/{{RID}}/checkMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberObjects",
        name: check_member_objects,
        path: "/applications/{{RID}}/checkMemberObjects",
        body: true
    );
    get!(
        doc: "Get createdOnBehalfOf from applications",
        name: get_created_on_behalf_of,
        path: "/applications/{{RID}}/createdOnBehalfOf"
    );
    post!(
        doc: "Create extensionProperty (directory extension)",
        name: create_extension_properties,
        path: "/applications/{{RID}}/extensionProperties",
        body: true
    );
    get!(
        doc: "List extensionProperties (directory extensions)",
        name: list_extension_properties,
        path: "/applications/{{RID}}/extensionProperties"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_extension_properties_count,
        path: "/applications/{{RID}}/extensionProperties/$count"
    );
    delete!(
        doc: "Delete navigation property extensionProperties for applications",
        name: delete_extension_properties,
        path: "/applications/{{RID}}/extensionProperties/{{id}}",
        params: extension_property_id
    );
    get!(
        doc: "Get extensionProperties from applications",
        name: get_extension_properties,
        path: "/applications/{{RID}}/extensionProperties/{{id}}",
        params: extension_property_id
    );
    patch!(
        doc: "Update the navigation property extensionProperties in applications",
        name: update_extension_properties,
        path: "/applications/{{RID}}/extensionProperties/{{id}}",
        body: true,
        params: extension_property_id
    );
    post!(
        doc: "Create federatedIdentityCredential",
        name: create_federated_identity_credentials,
        path: "/applications/{{RID}}/federatedIdentityCredentials",
        body: true
    );
    get!(
        doc: "List federatedIdentityCredentials",
        name: list_federated_identity_credentials,
        path: "/applications/{{RID}}/federatedIdentityCredentials"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_federated_identity_credentials_count,
        path: "/applications/{{RID}}/federatedIdentityCredentials/$count"
    );
    delete!(
        doc: "Delete navigation property federatedIdentityCredentials for applications",
        name: delete_federated_identity_credentials,
        path: "/applications/{{RID}}/federatedIdentityCredentials/{{id}}",
        params: federated_identity_credential_id
    );
    get!(
        doc: "Get federatedIdentityCredentials from applications",
        name: get_federated_identity_credentials,
        path: "/applications/{{RID}}/federatedIdentityCredentials/{{id}}",
        params: federated_identity_credential_id
    );
    patch!(
        doc: "Update the navigation property federatedIdentityCredentials in applications",
        name: update_federated_identity_credentials,
        path: "/applications/{{RID}}/federatedIdentityCredentials/{{id}}",
        body: true,
        params: federated_identity_credential_id
    );
    post!(
        doc: "Invoke action getMemberGroups",
        name: get_member_groups,
        path: "/applications/{{RID}}/getMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action getMemberObjects",
        name: get_member_objects,
        path: "/applications/{{RID}}/getMemberObjects",
        body: true
    );
    get!(
        doc: "Get homeRealmDiscoveryPolicies from applications",
        name: list_home_realm_discovery_policies,
        path: "/applications/{{RID}}/homeRealmDiscoveryPolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_home_realm_discovery_policies_count,
        path: "/applications/{{RID}}/homeRealmDiscoveryPolicies/$count"
    );
    get!(
        doc: "Get homeRealmDiscoveryPolicies from applications",
        name: get_home_realm_discovery_policies,
        path: "/applications/{{RID}}/homeRealmDiscoveryPolicies/{{id}}",
        params: home_realm_discovery_policy_id
    );
    get!(
        doc: "Get logo for application from applications",
        name: get_logo,
        path: "/applications/{{RID}}/logo"
    );
    put!(
        doc: "Update logo for application in applications",
        name: update_logo,
        path: "/applications/{{RID}}/logo",
        body: true
    );
    post!(
        doc: "Invoke action removeKey",
        name: remove_key,
        path: "/applications/{{RID}}/removeKey",
        body: true
    );
    post!(
        doc: "Invoke action removePassword",
        name: remove_password,
        path: "/applications/{{RID}}/removePassword",
        body: true
    );
    post!(
        doc: "Invoke action restore",
        name: restore,
        path: "/applications/{{RID}}/restore"
    );
    post!(
        doc: "Invoke action setVerifiedPublisher",
        name: set_verified_publisher,
        path: "/applications/{{RID}}/setVerifiedPublisher",
        body: true
    );
    get!(
        doc: "List assigned tokenIssuancePolicies",
        name: list_token_issuance_policies,
        path: "/applications/{{RID}}/tokenIssuancePolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_token_issuance_policies_count,
        path: "/applications/{{RID}}/tokenIssuancePolicies/$count"
    );
    post!(
        doc: "Create new navigation property ref to tokenIssuancePolicies for applications",
        name: create_ref_token_issuance_policies,
        path: "/applications/{{RID}}/tokenIssuancePolicies/$ref",
        body: true
    );
    get!(
        doc: "List assigned tokenIssuancePolicies",
        name: list_ref_token_issuance_policies,
        path: "/applications/{{RID}}/tokenIssuancePolicies/$ref"
    );
    delete!(
        doc: "Delete ref of navigation property tokenIssuancePolicies for applications",
        name: delete_ref_token_issuance_policies,
        path: "/applications/{{RID}}/tokenIssuancePolicies/{{id}}/$ref",
        params: token_issuance_policy_id
    );
    get!(
        doc: "List assigned tokenLifetimePolicy",
        name: list_token_lifetime_policies,
        path: "/applications/{{RID}}/tokenLifetimePolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_token_lifetime_policies_count,
        path: "/applications/{{RID}}/tokenLifetimePolicies/$count"
    );
    post!(
        doc: "Create new navigation property ref to tokenLifetimePolicies for applications",
        name: create_ref_token_lifetime_policies,
        path: "/applications/{{RID}}/tokenLifetimePolicies/$ref",
        body: true
    );
    get!(
        doc: "List assigned tokenLifetimePolicy",
        name: list_ref_token_lifetime_policies,
        path: "/applications/{{RID}}/tokenLifetimePolicies/$ref"
    );
    delete!(
        doc: "Delete ref of navigation property tokenLifetimePolicies for applications",
        name: delete_ref_token_lifetime_policies,
        path: "/applications/{{RID}}/tokenLifetimePolicies/{{id}}/$ref",
        params: token_lifetime_policy_id
    );
    post!(
        doc: "Invoke action unsetVerifiedPublisher",
        name: unset_verified_publisher,
        path: "/applications/{{RID}}/unsetVerifiedPublisher"
    );
}
