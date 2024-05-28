// GENERATED CODE

use crate::api_default_imports::*;

api_client!(IdentityApiClient, ResourceIdentity::Identity);

impl IdentityApiClient {
    get!(
        doc: "Get identity",
        name: get_identity_container,
        path: "/identity"
    );
    patch!(
        doc: "Update identity",
        name: update_identity_container,
        path: "/identity",
        body: true
    );
    post!(
        doc: "Create identityApiConnector",
        name: create_api_connectors,
        path: "/identity/apiConnectors",
        body: true
    );
    get!(
        doc: "List identityApiConnectors",
        name: list_api_connectors,
        path: "/identity/apiConnectors"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_api_connectors_count,
        path: "/identity/apiConnectors/$count"
    );
    delete!(
        doc: "Delete navigation property apiConnectors for identity",
        name: delete_api_connectors,
        path: "/identity/apiConnectors/{{id}}",
        params: identity_api_connector_id
    );
    get!(
        doc: "Get apiConnectors from identity",
        name: get_api_connectors,
        path: "/identity/apiConnectors/{{id}}",
        params: identity_api_connector_id
    );
    patch!(
        doc: "Update the navigation property apiConnectors in identity",
        name: update_api_connectors,
        path: "/identity/apiConnectors/{{id}}",
        body: true,
        params: identity_api_connector_id
    );
    post!(
        doc: "Invoke action uploadClientCertificate",
        name: upload_client_certificate,
        path: "/identity/apiConnectors/{{id}}/uploadClientCertificate",
        body: true,
        params: identity_api_connector_id
    );
    post!(
        doc: "Create b2xIdentityUserFlow",
        name: identity_create_b_2x_user_flows,
        path: "/identity/b2xUserFlows",
        body: true
    );
    get!(
        doc: "List b2xIdentityUserFlows",
        name: identity_list_b_2x_user_flows,
        path: "/identity/b2xUserFlows"
    );
    get!(
        doc: "Get the number of the resource",
        name: count,
        path: "/identity/b2xUserFlows/$count"
    );
    delete!(
        doc: "Delete navigation property b2xUserFlows for identity",
        name: identity_delete_b_2x_user_flows,
        path: "/identity/b2xUserFlows/{{id}}",
        params: b_2x_identity_user_flow_id
    );
    get!(
        doc: "Get b2xUserFlows from identity",
        name: identity_get_b_2x_user_flows,
        path: "/identity/b2xUserFlows/{{id}}",
        params: b_2x_identity_user_flow_id
    );
    patch!(
        doc: "Update the navigation property b2xUserFlows in identity",
        name: identity_update_b_2x_user_flows,
        path: "/identity/b2xUserFlows/{{id}}",
        body: true,
        params: b_2x_identity_user_flow_id
    );
    post!(
        doc: "Create new navigation property to languages for identity",
        name: create_languages,
        path: "/identity/b2xUserFlows/{{id}}/languages",
        body: true,
        params: b_2x_identity_user_flow_id
    );
    get!(
        doc: "List languages",
        name: list_languages,
        path: "/identity/b2xUserFlows/{{id}}/languages",
        params: b_2x_identity_user_flow_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_languages_count,
        path: "/identity/b2xUserFlows/{{id}}/languages/$count",
        params: b_2x_identity_user_flow_id
    );
    delete!(
        doc: "Delete navigation property languages for identity",
        name: delete_languages,
        path: "/identity/b2xUserFlows/{{id}}/languages/{{id2}}",
        params: b_2x_identity_user_flow_id, user_flow_language_configuration_id
    );
    get!(
        doc: "Get languages from identity",
        name: get_languages,
        path: "/identity/b2xUserFlows/{{id}}/languages/{{id2}}",
        params: b_2x_identity_user_flow_id, user_flow_language_configuration_id
    );
    patch!(
        doc: "Update the navigation property languages in identity",
        name: update_languages,
        path: "/identity/b2xUserFlows/{{id}}/languages/{{id2}}",
        body: true,
        params: b_2x_identity_user_flow_id, user_flow_language_configuration_id
    );
    post!(
        doc: "Create new navigation property to defaultPages for identity",
        name: create_default_pages,
        path: "/identity/b2xUserFlows/{{id}}/languages/{{id2}}/defaultPages",
        body: true,
        params: b_2x_identity_user_flow_id, user_flow_language_configuration_id
    );
    get!(
        doc: "Get defaultPages from identity",
        name: list_default_pages,
        path: "/identity/b2xUserFlows/{{id}}/languages/{{id2}}/defaultPages",
        params: b_2x_identity_user_flow_id, user_flow_language_configuration_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_default_pages_count,
        path: "/identity/b2xUserFlows/{{id}}/languages/{{id2}}/defaultPages/$count",
        params: b_2x_identity_user_flow_id, user_flow_language_configuration_id
    );
    delete!(
        doc: "Delete navigation property defaultPages for identity",
        name: delete_default_pages,
        path: "/identity/b2xUserFlows/{{id}}/languages/{{id2}}/defaultPages/{{id3}}",
        params: b_2x_identity_user_flow_id, user_flow_language_configuration_id, user_flow_language_page_id
    );
    get!(
        doc: "Get defaultPages from identity",
        name: get_default_pages,
        path: "/identity/b2xUserFlows/{{id}}/languages/{{id2}}/defaultPages/{{id3}}",
        params: b_2x_identity_user_flow_id, user_flow_language_configuration_id, user_flow_language_page_id
    );
    patch!(
        doc: "Update the navigation property defaultPages in identity",
        name: update_default_pages,
        path: "/identity/b2xUserFlows/{{id}}/languages/{{id2}}/defaultPages/{{id3}}",
        body: true,
        params: b_2x_identity_user_flow_id, user_flow_language_configuration_id, user_flow_language_page_id
    );
    get!(
        doc: "Get media content for the navigation property defaultPages from identity",
        name: get_default_pages_content,
        path: "/identity/b2xUserFlows/{{id}}/languages/{{id2}}/defaultPages/{{id3}}/$value",
        params: b_2x_identity_user_flow_id, user_flow_language_configuration_id, user_flow_language_page_id
    );
    put!(
        doc: "Update media content for the navigation property defaultPages in identity",
        name: update_default_pages_content,
        path: "/identity/b2xUserFlows/{{id}}/languages/{{id2}}/defaultPages/{{id3}}/$value",
        body: true,
        params: b_2x_identity_user_flow_id, user_flow_language_configuration_id, user_flow_language_page_id
    );
    post!(
        doc: "Create new navigation property to overridesPages for identity",
        name: create_overrides_pages,
        path: "/identity/b2xUserFlows/{{id}}/languages/{{id2}}/overridesPages",
        body: true,
        params: b_2x_identity_user_flow_id, user_flow_language_configuration_id
    );
    get!(
        doc: "List overridesPages",
        name: list_overrides_pages,
        path: "/identity/b2xUserFlows/{{id}}/languages/{{id2}}/overridesPages",
        params: b_2x_identity_user_flow_id, user_flow_language_configuration_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_overrides_pages_count,
        path: "/identity/b2xUserFlows/{{id}}/languages/{{id2}}/overridesPages/$count",
        params: b_2x_identity_user_flow_id, user_flow_language_configuration_id
    );
    delete!(
        doc: "Delete navigation property overridesPages for identity",
        name: delete_overrides_pages,
        path: "/identity/b2xUserFlows/{{id}}/languages/{{id2}}/overridesPages/{{id3}}",
        params: b_2x_identity_user_flow_id, user_flow_language_configuration_id, user_flow_language_page_id
    );
    get!(
        doc: "Get overridesPages from identity",
        name: get_overrides_pages,
        path: "/identity/b2xUserFlows/{{id}}/languages/{{id2}}/overridesPages/{{id3}}",
        params: b_2x_identity_user_flow_id, user_flow_language_configuration_id, user_flow_language_page_id
    );
    patch!(
        doc: "Update the navigation property overridesPages in identity",
        name: update_overrides_pages,
        path: "/identity/b2xUserFlows/{{id}}/languages/{{id2}}/overridesPages/{{id3}}",
        body: true,
        params: b_2x_identity_user_flow_id, user_flow_language_configuration_id, user_flow_language_page_id
    );
    get!(
        doc: "Get media content for the navigation property overridesPages from identity",
        name: get_overrides_pages_content,
        path: "/identity/b2xUserFlows/{{id}}/languages/{{id2}}/overridesPages/{{id3}}/$value",
        params: b_2x_identity_user_flow_id, user_flow_language_configuration_id, user_flow_language_page_id
    );
    put!(
        doc: "Update media content for the navigation property overridesPages in identity",
        name: update_overrides_pages_content,
        path: "/identity/b2xUserFlows/{{id}}/languages/{{id2}}/overridesPages/{{id3}}/$value",
        body: true,
        params: b_2x_identity_user_flow_id, user_flow_language_configuration_id, user_flow_language_page_id
    );
    post!(
        doc: "Create userAttributeAssignments",
        name: create_user_attribute_assignments,
        path: "/identity/b2xUserFlows/{{id}}/userAttributeAssignments",
        body: true,
        params: b_2x_identity_user_flow_id
    );
    get!(
        doc: "List userAttributeAssignments",
        name: list_user_attribute_assignments,
        path: "/identity/b2xUserFlows/{{id}}/userAttributeAssignments",
        params: b_2x_identity_user_flow_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_user_attribute_assignments_count,
        path: "/identity/b2xUserFlows/{{id}}/userAttributeAssignments/$count",
        params: b_2x_identity_user_flow_id
    );
    get!(
        doc: "Invoke function getOrder",
        name: get_order,
        path: "/identity/b2xUserFlows/{{id}}/userAttributeAssignments/getOrder()",
        params: b_2x_identity_user_flow_id
    );
    post!(
        doc: "Invoke action setOrder",
        name: set_order,
        path: "/identity/b2xUserFlows/{{id}}/userAttributeAssignments/setOrder",
        body: true,
        params: b_2x_identity_user_flow_id
    );
    delete!(
        doc: "Delete navigation property userAttributeAssignments for identity",
        name: delete_user_attribute_assignments,
        path: "/identity/b2xUserFlows/{{id}}/userAttributeAssignments/{{id2}}",
        params: b_2x_identity_user_flow_id, identity_user_flow_attribute_assignment_id
    );
    get!(
        doc: "Get userAttributeAssignments from identity",
        name: get_user_attribute_assignments,
        path: "/identity/b2xUserFlows/{{id}}/userAttributeAssignments/{{id2}}",
        params: b_2x_identity_user_flow_id, identity_user_flow_attribute_assignment_id
    );
    patch!(
        doc: "Update the navigation property userAttributeAssignments in identity",
        name: update_user_attribute_assignments,
        path: "/identity/b2xUserFlows/{{id}}/userAttributeAssignments/{{id2}}",
        body: true,
        params: b_2x_identity_user_flow_id, identity_user_flow_attribute_assignment_id
    );
    get!(
        doc: "Get userAttribute from identity",
        name: get_user_attribute,
        path: "/identity/b2xUserFlows/{{id}}/userAttributeAssignments/{{id2}}/userAttribute",
        params: b_2x_identity_user_flow_id, identity_user_flow_attribute_assignment_id
    );
    get!(
        doc: "Get userFlowIdentityProviders from identity",
        name: list_user_flow_identity_providers,
        path: "/identity/b2xUserFlows/{{id}}/userFlowIdentityProviders",
        params: b_2x_identity_user_flow_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_user_flow_identity_providers_count,
        path: "/identity/b2xUserFlows/{{id}}/userFlowIdentityProviders/$count",
        params: b_2x_identity_user_flow_id
    );
    post!(
        doc: "Create new navigation property ref to userFlowIdentityProviders for identity",
        name: create_ref_user_flow_identity_providers,
        path: "/identity/b2xUserFlows/{{id}}/userFlowIdentityProviders/$ref",
        body: true,
        params: b_2x_identity_user_flow_id
    );
    get!(
        doc: "Get ref of userFlowIdentityProviders from identity",
        name: list_ref_user_flow_identity_providers,
        path: "/identity/b2xUserFlows/{{id}}/userFlowIdentityProviders/$ref",
        params: b_2x_identity_user_flow_id
    );
    delete!(
        doc: "Delete ref of navigation property userFlowIdentityProviders for identity",
        name: delete_ref_user_flow_identity_providers,
        path: "/identity/b2xUserFlows/{{id}}/userFlowIdentityProviders/{{id2}}/$ref",
        params: b_2x_identity_user_flow_id, identity_provider_base_id
    );
    delete!(
        doc: "Delete navigation property conditionalAccess for identity",
        name: delete_conditional_access,
        path: "/identity/conditionalAccess"
    );
    get!(
        doc: "Get conditionalAccess from identity",
        name: get_conditional_access,
        path: "/identity/conditionalAccess"
    );
    patch!(
        doc: "Update the navigation property conditionalAccess in identity",
        name: update_conditional_access,
        path: "/identity/conditionalAccess",
        body: true
    );
    post!(
        doc: "Create new navigation property to authenticationContextClassReferences for identity",
        name: create_authentication_context_class_references,
        path: "/identity/conditionalAccess/authenticationContextClassReferences",
        body: true
    );
    get!(
        doc: "List authenticationContextClassReferences",
        name: list_authentication_context_class_references,
        path: "/identity/conditionalAccess/authenticationContextClassReferences"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_authentication_context_class_references_count,
        path: "/identity/conditionalAccess/authenticationContextClassReferences/$count"
    );
    delete!(
        doc: "Delete navigation property authenticationContextClassReferences for identity",
        name: delete_authentication_context_class_references,
        path: "/identity/conditionalAccess/authenticationContextClassReferences/{{id}}",
        params: authentication_context_class_reference_id
    );
    get!(
        doc: "Get authenticationContextClassReferences from identity",
        name: get_authentication_context_class_references,
        path: "/identity/conditionalAccess/authenticationContextClassReferences/{{id}}",
        params: authentication_context_class_reference_id
    );
    patch!(
        doc: "Update the navigation property authenticationContextClassReferences in identity",
        name: update_authentication_context_class_references,
        path: "/identity/conditionalAccess/authenticationContextClassReferences/{{id}}",
        body: true,
        params: authentication_context_class_reference_id
    );
    post!(
        doc: "Create namedLocation",
        name: create_named_locations,
        path: "/identity/conditionalAccess/namedLocations",
        body: true
    );
    get!(
        doc: "List namedLocations",
        name: list_named_locations,
        path: "/identity/conditionalAccess/namedLocations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_named_locations_count,
        path: "/identity/conditionalAccess/namedLocations/$count"
    );
    delete!(
        doc: "Delete navigation property namedLocations for identity",
        name: delete_named_locations,
        path: "/identity/conditionalAccess/namedLocations/{{id}}",
        params: named_location_id
    );
    get!(
        doc: "Get namedLocations from identity",
        name: get_named_locations,
        path: "/identity/conditionalAccess/namedLocations/{{id}}",
        params: named_location_id
    );
    patch!(
        doc: "Update the navigation property namedLocations in identity",
        name: update_named_locations,
        path: "/identity/conditionalAccess/namedLocations/{{id}}",
        body: true,
        params: named_location_id
    );
    post!(
        doc: "Create conditionalAccessPolicy",
        name: create_policies,
        path: "/identity/conditionalAccess/policies",
        body: true
    );
    get!(
        doc: "List policies",
        name: list_policies,
        path: "/identity/conditionalAccess/policies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_policies_count,
        path: "/identity/conditionalAccess/policies/$count"
    );
    delete!(
        doc: "Delete navigation property policies for identity",
        name: delete_policies,
        path: "/identity/conditionalAccess/policies/{{id}}",
        params: conditional_access_policy_id
    );
    get!(
        doc: "Get policies from identity",
        name: get_policies,
        path: "/identity/conditionalAccess/policies/{{id}}",
        params: conditional_access_policy_id
    );
    patch!(
        doc: "Update the navigation property policies in identity",
        name: update_policies,
        path: "/identity/conditionalAccess/policies/{{id}}",
        body: true,
        params: conditional_access_policy_id
    );
    get!(
        doc: "List conditionalAccessTemplates",
        name: list_templates,
        path: "/identity/conditionalAccess/templates"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_templates_count,
        path: "/identity/conditionalAccess/templates/$count"
    );
    get!(
        doc: "Get templates from identity",
        name: get_templates,
        path: "/identity/conditionalAccess/templates/{{id}}",
        params: conditional_access_template_id
    );
    post!(
        doc: "Create identityUserFlowAttribute",
        name: create_user_flow_attributes,
        path: "/identity/userFlowAttributes",
        body: true
    );
    get!(
        doc: "List identityUserFlowAttributes",
        name: list_user_flow_attributes,
        path: "/identity/userFlowAttributes"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_user_flow_attributes_count,
        path: "/identity/userFlowAttributes/$count"
    );
    delete!(
        doc: "Delete navigation property userFlowAttributes for identity",
        name: delete_user_flow_attributes,
        path: "/identity/userFlowAttributes/{{id}}",
        params: identity_user_flow_attribute_id
    );
    get!(
        doc: "Get userFlowAttributes from identity",
        name: get_user_flow_attributes,
        path: "/identity/userFlowAttributes/{{id}}",
        params: identity_user_flow_attribute_id
    );
    patch!(
        doc: "Update the navigation property userFlowAttributes in identity",
        name: update_user_flow_attributes,
        path: "/identity/userFlowAttributes/{{id}}",
        body: true,
        params: identity_user_flow_attribute_id
    );
}
