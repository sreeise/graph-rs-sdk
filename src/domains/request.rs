// GENERATED CODE

use crate::client::Graph;
use crate::core::ResourceIdentity;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(DomainRequest,);
register_client!(DomainsRequest, ());

impl<'a, Client> DomainRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entities from domains",
        name: list_domain,
        response: serde_json::Value,
        path: "/domains",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Add new entity to domains",
        name: create_domain,
        response: serde_json::Value,
        path: "/domains",
        params: 0,
        has_body: true
    });

    pub fn id<ID: AsRef<str>>(&self, id: ID) -> DomainsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Domains);
        DomainsRequest::new(id.as_ref(), self.client)
    }
}

impl<'a, Client> DomainsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entity from domains by key",
        name: get_domain,
        response: serde_json::Value,
        path: "/domains/{{RID}}",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update entity in domains",
        name: update_domain,
        response: NoContent,
        path: "/domains/{{RID}}",
        params: 0,
        has_body: true
    });

    delete!({
        doc: "# Delete entity from domains",
        name: delete_domain,
        response: NoContent,
        path: "/domains/{{RID}}",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get domainNameReferences from domains",
        name: list_domain_name_references,
        response: serde_json::Value,
        path: "/domains/{{RID}}/domainNameReferences",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get domainNameReferences from domains",
        name: get_domain_name_references,
        response: serde_json::Value,
        path: "/domains/{{RID}}/domainNameReferences/{{id}}",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action forceDelete",
        name: force_delete,
        response: NoContent,
        path: "/domains/{{RID}}/forceDelete",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get serviceConfigurationRecords from domains",
        name: list_service_configuration_records,
        response: serde_json::Value,
        path: "/domains/{{RID}}/serviceConfigurationRecords",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to serviceConfigurationRecords for domains",
        name: create_service_configuration_records,
        response: serde_json::Value,
        path: "/domains/{{RID}}/serviceConfigurationRecords",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get serviceConfigurationRecords from domains",
        name: get_service_configuration_records,
        response: serde_json::Value,
        path: "/domains/{{RID}}/serviceConfigurationRecords/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property serviceConfigurationRecords in domains",
        name: update_service_configuration_records,
        response: NoContent,
        path: "/domains/{{RID}}/serviceConfigurationRecords/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get verificationDnsRecords from domains",
        name: list_verification_dns_records,
        response: serde_json::Value,
        path: "/domains/{{RID}}/verificationDnsRecords",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to verificationDnsRecords for domains",
        name: create_verification_dns_records,
        response: serde_json::Value,
        path: "/domains/{{RID}}/verificationDnsRecords",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get verificationDnsRecords from domains",
        name: get_verification_dns_records,
        response: serde_json::Value,
        path: "/domains/{{RID}}/verificationDnsRecords/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property verificationDnsRecords in domains",
        name: update_verification_dns_records,
        response: NoContent,
        path: "/domains/{{RID}}/verificationDnsRecords/{{id}}",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action verify",
        name: verify,
        response: serde_json::Value,
        path: "/domains/{{RID}}/verify",
        params: 0,
        has_body: false
    });
}
