// GENERATED CODE

use crate::{client::Graph, core::ResourceIdentity};
use graph_http::{
    types::{DeltaPhantom, NoContent},
    IntoResponse,
};
use handlebars::*;
use reqwest::Method;

register_client!(ApplicationRequest,);
register_client!(ApplicationsRequest, ());

impl<'a, Client> ApplicationRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entities from applications",
        name: list_application,
        response: serde_json::Value,
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

    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/applications/delta",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        response: serde_json::Value,
        path: "/applications/getAvailableExtensionProperties",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action getByIds",
        name: get_by_ids,
        response: serde_json::Value,
        path: "/applications/getByIds",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action validateProperties",
        name: validate_properties,
        response: NoContent,
        path: "/applications/validateProperties",
        params: 0,
        has_body: true
    });

    pub fn id<ID: AsRef<str>>(&self, id: ID) -> ApplicationsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Applications);
        ApplicationsRequest::new(id.as_ref(), self.client)
    }
}

impl<'a, Client> ApplicationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entity from applications by key",
        name: get_application,
        response: serde_json::Value,
        path: "/applications/{{RID}}",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update entity in applications",
        name: update_application,
        response: NoContent,
        path: "/applications/{{RID}}",
        params: 0,
        has_body: true
    });

    delete!({
        doc: "# Delete entity from applications",
        name: delete_application,
        response: NoContent,
        path: "/applications/{{RID}}",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Invoke action addKey",
        name: add_key,
        response: serde_json::Value,
        path: "/applications/{{RID}}/addKey",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action addPassword",
        name: add_password,
        response: serde_json::Value,
        path: "/applications/{{RID}}/addPassword",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action checkMemberGroups",
        name: check_member_groups,
        response: serde_json::Value,
        path: "/applications/{{RID}}/checkMemberGroups",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action checkMemberObjects",
        name: check_member_objects,
        response: serde_json::Value,
        path: "/applications/{{RID}}/checkMemberObjects",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get createdOnBehalfOf from applications",
        name: get_created_on_behalf_of,
        response: serde_json::Value,
        path: "/applications/{{RID}}/createdOnBehalfOf",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get ref of createdOnBehalfOf from applications",
        name: get_ref_created_on_behalf_of,
        response: serde_json::Value,
        path: "/applications/{{RID}}/createdOnBehalfOf/$ref",
        params: 0,
        has_body: false
    });

    put!({
        doc: "# Update the ref of navigation property createdOnBehalfOf in applications",
        name: update_ref_created_on_behalf_of,
        response: NoContent,
        path: "/applications/{{RID}}/createdOnBehalfOf/$ref",
        params: 0,
        has_body: true
    });

    delete!({
        doc: "# Delete ref of navigation property createdOnBehalfOf for applications",
        name: delete_ref_created_on_behalf_of,
        response: NoContent,
        path: "/applications/{{RID}}/createdOnBehalfOf/$ref",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get extensionProperties from applications",
        name: list_extension_properties,
        response: serde_json::Value,
        path: "/applications/{{RID}}/extensionProperties",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to extensionProperties for applications",
        name: create_extension_properties,
        response: serde_json::Value,
        path: "/applications/{{RID}}/extensionProperties",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get extensionProperties from applications",
        name: get_extension_properties,
        response: serde_json::Value,
        path: "/applications/{{RID}}/extensionProperties/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property extensionProperties in applications",
        name: update_extension_properties,
        response: NoContent,
        path: "/applications/{{RID}}/extensionProperties/{{id}}",
        params: 1,
        has_body: true
    });

    delete!({
        doc: "# Delete navigation property extensionProperties for applications",
        name: delete_extension_properties,
        response: NoContent,
        path: "/applications/{{RID}}/extensionProperties/{{id}}",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action getMemberGroups",
        name: get_member_groups,
        response: serde_json::Value,
        path: "/applications/{{RID}}/getMemberGroups",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action getMemberObjects",
        name: get_member_objects,
        response: serde_json::Value,
        path: "/applications/{{RID}}/getMemberObjects",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get homeRealmDiscoveryPolicies from applications",
        name: list_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/homeRealmDiscoveryPolicies",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get ref of homeRealmDiscoveryPolicies from applications",
        name: list_ref_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/homeRealmDiscoveryPolicies/$ref",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property ref to homeRealmDiscoveryPolicies for applications",
        name: create_ref_home_realm_discovery_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/homeRealmDiscoveryPolicies/$ref",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get media content for application from applications",
        name: get_logo,
        response: serde_json::Value,
        path: "/applications/{{RID}}/logo",
        params: 0,
        has_body: false
    });

    put!({
        doc: "# Update media content for application in applications",
        name: update_logo,
        response: NoContent,
        path: "/applications/{{RID}}/logo",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get owners from applications",
        name: list_owners,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get ref of owners from applications",
        name: list_ref_owners,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners/$ref",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property ref to owners for applications",
        name: create_ref_owners,
        response: serde_json::Value,
        path: "/applications/{{RID}}/owners/$ref",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action removeKey",
        name: remove_key,
        response: NoContent,
        path: "/applications/{{RID}}/removeKey",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action removePassword",
        name: remove_password,
        response: NoContent,
        path: "/applications/{{RID}}/removePassword",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action restore",
        name: restore,
        response: serde_json::Value,
        path: "/applications/{{RID}}/restore",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get tokenIssuancePolicies from applications",
        name: list_token_issuance_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenIssuancePolicies",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get ref of tokenIssuancePolicies from applications",
        name: list_ref_token_issuance_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenIssuancePolicies/$ref",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property ref to tokenIssuancePolicies for applications",
        name: create_ref_token_issuance_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenIssuancePolicies/$ref",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get tokenLifetimePolicies from applications",
        name: list_token_lifetime_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenLifetimePolicies",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get ref of tokenLifetimePolicies from applications",
        name: list_ref_token_lifetime_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenLifetimePolicies/$ref",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property ref to tokenLifetimePolicies for applications",
        name: create_ref_token_lifetime_policies,
        response: serde_json::Value,
        path: "/applications/{{RID}}/tokenLifetimePolicies/$ref",
        params: 0,
        has_body: true
    });
}
