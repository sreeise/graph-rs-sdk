use crate::client::Graph;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(DomainDnsRecordsRequest,);

impl<'a, Client> DomainDnsRecordsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entity from domainDnsRecords by key",
        name: get_domain_dns_record,
        response: serde_json::Value,
        path: "/domainDnsRecords/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update entity in domainDnsRecords",
        name: update_domain_dns_record,
        response: NoContent,
        path: "/domainDnsRecords/{{id}}",
        params: 1,
        has_body: true
    });

    delete!({
        doc: "# Delete entity from domainDnsRecords",
        name: delete_domain_dns_record,
        response: NoContent,
        path: "/domainDnsRecords/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get entities from domainDnsRecords",
        name: list_domain_dns_record,
        response: serde_json::Value,
        path: "/domainDnsRecords",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Add new entity to domainDnsRecords",
        name: create_domain_dns_record,
        response: serde_json::Value,
        path: "/domainDnsRecords",
        params: 0,
        has_body: true
    });
}
