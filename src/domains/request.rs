use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(DomainsRequest,);

impl<'a, Client> DomainsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entities from domains",
        name: list_domain,
        response: Collection<serde_json::Value>,
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
    get!({
        doc: "# Get entity from domains by key",
        name: get_domain,
        response: serde_json::Value,
        path: "/domains/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update entity in domains",
        name: update_domain,
        response: GraphResponse<Content>,
        path: "/domains/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from domains",
        name: delete_domain,
        response: GraphResponse<Content>,
        path: "/domains/{{id}}",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action forceDelete",
        name: force_delete,
        response: GraphResponse<Content>,
        path: "/domains/{{id}}/forceDelete",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get serviceConfigurationRecords from domains",
        name: list_service_configuration_records,
        response: Collection<serde_json::Value>,
        path: "/domains/{{id}}/serviceConfigurationRecords",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to serviceConfigurationRecords for domains",
        name: create_service_configuration_records,
        response: serde_json::Value,
        path: "/domains/{{id}}/serviceConfigurationRecords",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get verificationDnsRecords from domains",
        name: get_verification_dns_records,
        response: serde_json::Value,
        path: "/domains/{{id}}/verificationDnsRecords/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property verificationDnsRecords in domains",
        name: update_verification_dns_records,
        response: GraphResponse<Content>,
        path: "/domains/{{id}}/verificationDnsRecords/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get verificationDnsRecords from domains",
        name: list_verification_dns_records,
        response: Collection<serde_json::Value>,
        path: "/domains/{{id}}/verificationDnsRecords",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to verificationDnsRecords for domains",
        name: create_verification_dns_records,
        response: serde_json::Value,
        path: "/domains/{{id}}/verificationDnsRecords",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action verify",
        name: verify,
        response: serde_json::Value,
        path: "/domains/{{id}}/verify",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get domainNameReferences from domains",
        name: list_domain_name_references,
        response: Collection<serde_json::Value>,
        path: "/domains/{{id}}/domainNameReferences",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get serviceConfigurationRecords from domains",
        name: get_service_configuration_records,
        response: serde_json::Value,
        path: "/domains/{{id}}/serviceConfigurationRecords/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property serviceConfigurationRecords in domains",
        name: update_service_configuration_records,
        response: GraphResponse<Content>,
        path: "/domains/{{id}}/serviceConfigurationRecords/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get domainNameReferences from domains",
        name: get_domain_name_references,
        response: serde_json::Value,
        path: "/domains/{{id}}/domainNameReferences/{{id2}}",
        params: 2,
        has_body: false
    });
}
