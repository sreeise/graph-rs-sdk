// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::DeltaPhantom;
use graph_http::types::NoContent;

register_client!(ApplicationsRequest,);
register_client!(ApplicationsIdRequest, ());

impl<'a, Client> ApplicationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, applications_id: ID) -> ApplicationsIdRequest<'a, Client> {
        ApplicationsIdRequest::new(applications_id.as_ref(), self.client)
    }

    get!({
        doc: "Get entities from applications",
        name: list_application,
        response: serde_json::Value,
        path: "/applications",
        has_body: false
    });
    post!({
        doc: "Add new entity to applications",
        name: create_application,
        response: serde_json::Value,
        path: "/applications",
        has_body: true
    });
    get!({
        doc: "Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/applications/microsoft.graph.delta()",
        has_body: false
    });
    post!({
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        response: serde_json::Value,
        path: "/applications/microsoft.graph.getAvailableExtensionProperties",
        has_body: true
    });
    post!({
        doc: "Invoke action getByIds",
        name: get_by_ids,
        response: serde_json::Value,
        path: "/applications/microsoft.graph.getByIds",
        has_body: true
    });
    post!({
        doc: "Invoke action validateProperties",
        name: validate_properties,
        response: NoContent,
        path: "/applications/microsoft.graph.validateProperties",
        has_body: true
    });
}

impl<'a, Client> ApplicationsIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    patch!({
        doc: "Update entity in applications",
        name: update_application,
        response: NoContent,
        path: "/applications/{{RID}}",
        has_body: true
    });
    delete!({
        doc: "Delete entity from applications",
        name: delete_application,
        response: NoContent,
        path: "/applications/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get entity from applications by key",
        name: get_application,
        response: serde_json::Value,
        path: "/applications/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get createdOnBehalfOf from applications",
        name: get_created_on_behalf_of,
        response: serde_json::Value,
        path: "/applications/{{RID}}/createdOnBehalfOf",
        has_body: false
    });
    get!({
        doc: "Get ref of createdOnBehalfOf from applications",
        name: get_ref_created_on_behalf_of,
        response: serde_json::Value,
        path: "/applications/{{RID}}/createdOnBehalfOf/$ref",
        has_body: false
    });
    put!({
        doc: "Update the ref of navigation property createdOnBehalfOf in applications",
        name: update_ref_created_on_behalf_of,
        response: NoContent,
        path: "/applications/{{RID}}/createdOnBehalfOf/$ref",
        has_body: true
    });
    delete!({
        doc: "Delete ref of navigation property createdOnBehalfOf for applications",
        name: delete_ref_created_on_behalf_of,
        response: NoContent,
        path: "/applications/{{RID}}/createdOnBehalfOf/$ref",
        has_body: false
    });
    get!({
        doc: "Get extensionProperties from applications",
        name: list_extension_properties,
        response: serde_json::Value,
        path: "/applications/{{RID}}/extensionProperties",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to extensionProperties for applications",
        name: create_extension_properties,
        response: serde_json::Value,
        path: "/applications/{{RID}}/extensionProperties",
        has_body: true
    });
    patch!({
        doc: "Update the navigation property extensionProperties in applications",
        name: update_extension_properties,
        response: NoContent,
        path: "/applications/{{RID}}/extensionProperties/{{id}}",
        params: [ extension_property_id ],
        has_body: true
    });
    delete!({
        doc: "Delete navigation property extensionProperties for applications",
        name: delete_extension_properties,
        response: NoContent,
        path: "/applications/{{RID}}/extensionProperties/{{id}}",
        params: [ extension_property_id ],
        has_body: false
    });
    get!({
        doc: "Get extensionProperties from applications",
        name: get_extension_properties,
        response: serde_json::Value,
        path: "/applications/{{RID}}/extensionProperties/{{id}}",
        params: [ extension_property_id ],
        has_body: false
    });
    get!({
        doc: "Get homeRealmDiscoveryPolicies from applications",
        name: list_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/homeRealmDiscoveryPolicies",
        has_body: false
    });
    post!({
        doc: "Create new navigation property ref to homeRealmDiscoveryPolicies for applications",
        name: create_ref_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/homeRealmDiscoveryPolicies/$ref",
        has_body: true
    });
    get!({
        doc: "Get ref of homeRealmDiscoveryPolicies from applications",
        name: list_ref_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/homeRealmDiscoveryPolicies/$ref",
        has_body: false
    });
    get!({
        doc: "Get media content for application from applications",
        name: get_logo,
        response: serde_json::Value,
        path: "/applications/{{RID}}/logo",
        has_body: false
    });
    put!({
        doc: "Update media content for application in applications",
        name: update_logo,
        response: NoContent,
        path: "/applications/{{RID}}/logo",
        has_body: true
    });
    post!({
        doc: "Invoke action addKey",
        name: add_key,
        response: serde_json::Value,
        path: "/applications/{{RID}}/microsoft.graph.addKey",
        has_body: true
    });
    post!({
        doc: "Invoke action addPassword",
        name: add_password,
        response: serde_json::Value,
        path: "/applications/{{RID}}/microsoft.graph.addPassword",
        has_body: true
    });
    post!({
        doc: "Invoke action checkMemberGroups",
        name: check_member_groups,
        response: serde_json::Value,
        path: "/applications/{{RID}}/microsoft.graph.checkMemberGroups",
        has_body: true
    });
    post!({
        doc: "Invoke action checkMemberObjects",
        name: check_member_objects,
        response: serde_json::Value,
        path: "/applications/{{RID}}/microsoft.graph.checkMemberObjects",
        has_body: true
    });
    post!({
        doc: "Invoke action getMemberGroups",
        name: get_member_groups,
        response: serde_json::Value,
        path: "/applications/{{RID}}/microsoft.graph.getMemberGroups",
        has_body: true
    });
    post!({
        doc: "Invoke action getMemberObjects",
        name: get_member_objects,
        response: serde_json::Value,
        path: "/applications/{{RID}}/microsoft.graph.getMemberObjects",
        has_body: true
    });
    post!({
        doc: "Invoke action removeKey",
        name: remove_key,
        response: NoContent,
        path: "/applications/{{RID}}/microsoft.graph.removeKey",
        has_body: true
    });
    post!({
        doc: "Invoke action removePassword",
        name: remove_password,
        response: NoContent,
        path: "/applications/{{RID}}/microsoft.graph.removePassword",
        has_body: true
    });
    post!({
        doc: "Invoke action restore",
        name: restore,
        response: serde_json::Value,
        path: "/applications/{{RID}}/microsoft.graph.restore",
        has_body: false
    });
    post!({
        doc: "Invoke action setVerifiedPublisher",
        name: set_verified_publisher,
        response: NoContent,
        path: "/applications/{{RID}}/microsoft.graph.setVerifiedPublisher",
        has_body: true
    });
    post!({
        doc: "Invoke action unsetVerifiedPublisher",
        name: unset_verified_publisher,
        response: NoContent,
        path: "/applications/{{RID}}/microsoft.graph.unsetVerifiedPublisher",
        has_body: false
    });
    get!({
        doc: "Get owners from applications",
        name: list_owners,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners",
        has_body: false
    });
    get!({
        doc: "Get ref of owners from applications",
        name: list_ref_owners,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners/$ref",
        has_body: false
    });
    post!({
        doc: "Create new navigation property ref to owners for applications",
        name: create_ref_owners,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners/$ref",
        has_body: true
    });
    get!({
        doc: "Get tokenIssuancePolicies from applications",
        name: list_token_issuance_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenIssuancePolicies",
        has_body: false
    });
    post!({
        doc: "Create new navigation property ref to tokenIssuancePolicies for applications",
        name: create_ref_token_issuance_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenIssuancePolicies/$ref",
        has_body: true
    });
    get!({
        doc: "Get ref of tokenIssuancePolicies from applications",
        name: list_ref_token_issuance_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenIssuancePolicies/$ref",
        has_body: false
    });
    get!({
        doc: "Get tokenLifetimePolicies from applications",
        name: list_token_lifetime_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenLifetimePolicies",
        has_body: false
    });
    get!({
        doc: "Get ref of tokenLifetimePolicies from applications",
        name: list_ref_token_lifetime_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenLifetimePolicies/$ref",
        has_body: false
    });
    post!({
        doc: "Create new navigation property ref to tokenLifetimePolicies for applications",
        name: create_ref_token_lifetime_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenLifetimePolicies/$ref",
        has_body: true
    });
}
