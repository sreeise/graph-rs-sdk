// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    DomainDnsRecordsApiClient,
    DomainDnsRecordsIdApiClient,
    ResourceIdentity::DomainDnsRecords
);

impl DomainDnsRecordsApiClient {
    post!(
        doc: "Add new entity to domainDnsRecords",
        name: create_domain_dns_record,
        path: "/domainDnsRecords",
        body: true
    );
    get!(
        doc: "Get entities from domainDnsRecords",
        name: list_domain_dns_record,
        path: "/domainDnsRecords"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_domain_dns_records_count,
        path: "/domainDnsRecords/$count"
    );
}

impl DomainDnsRecordsIdApiClient {
    delete!(
        doc: "Delete entity from domainDnsRecords",
        name: delete_domain_dns_record,
        path: "/domainDnsRecords/{{RID}}"
    );
    get!(
        doc: "Get entity from domainDnsRecords by key",
        name: get_domain_dns_record,
        path: "/domainDnsRecords/{{RID}}"
    );
    patch!(
        doc: "Update entity in domainDnsRecords",
        name: update_domain_dns_record,
        path: "/domainDnsRecords/{{RID}}",
        body: true
    );
}
