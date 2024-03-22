// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    DomainsApiClient,
    DomainsIdApiClient,
    ResourceIdentity::Domains
);

impl DomainsApiClient {
    post!(
        doc: "Create domain",
        name: create_domain,
        path: "/domains",
        body: true
    );
    get!(
        doc: "List domains",
        name: list_domain,
        path: "/domains"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_domains_count,
        path: "/domains/$count"
    );
}

impl DomainsIdApiClient {
    delete!(
        doc: "Delete domain",
        name: delete_domain,
        path: "/domains/{{RID}}"
    );
    get!(
        doc: "Get domain",
        name: get_domain,
        path: "/domains/{{RID}}"
    );
    patch!(
        doc: "Update domain",
        name: update_domain,
        path: "/domains/{{RID}}",
        body: true
    );
    get!(
        doc: "List domainNameReferences",
        name: list_domain_name_references,
        path: "/domains/{{RID}}/domainNameReferences"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_domain_name_references_count,
        path: "/domains/{{RID}}/domainNameReferences/$count"
    );
    get!(
        doc: "Get domainNameReferences from domains",
        name: get_domain_name_references,
        path: "/domains/{{RID}}/domainNameReferences/{{id}}",
        params: directory_object_id
    );
    post!(
        doc: "Create federationConfiguration",
        name: create_federation_configuration,
        path: "/domains/{{RID}}/federationConfiguration",
        body: true
    );
    get!(
        doc: "Get federationConfiguration from domains",
        name: list_federation_configuration,
        path: "/domains/{{RID}}/federationConfiguration"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_federation_configuration_count,
        path: "/domains/{{RID}}/federationConfiguration/$count"
    );
    delete!(
        doc: "Delete navigation property federationConfiguration for domains",
        name: delete_federation_configuration,
        path: "/domains/{{RID}}/federationConfiguration/{{id}}",
        params: internal_domain_federation_id
    );
    get!(
        doc: "Get federationConfiguration from domains",
        name: get_federation_configuration,
        path: "/domains/{{RID}}/federationConfiguration/{{id}}",
        params: internal_domain_federation_id
    );
    patch!(
        doc: "Update the navigation property federationConfiguration in domains",
        name: update_federation_configuration,
        path: "/domains/{{RID}}/federationConfiguration/{{id}}",
        body: true,
        params: internal_domain_federation_id
    );
    post!(
        doc: "Invoke action forceDelete",
        name: force_delete,
        path: "/domains/{{RID}}/forceDelete",
        body: true
    );
    post!(
        doc: "Invoke action promote",
        name: promote,
        path: "/domains/{{RID}}/promote"
    );
    post!(
        doc: "Create new navigation property to serviceConfigurationRecords for domains",
        name: create_service_configuration_records,
        path: "/domains/{{RID}}/serviceConfigurationRecords",
        body: true
    );
    get!(
        doc: "List serviceConfigurationRecords",
        name: list_service_configuration_records,
        path: "/domains/{{RID}}/serviceConfigurationRecords"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_service_configuration_records_count,
        path: "/domains/{{RID}}/serviceConfigurationRecords/$count"
    );
    delete!(
        doc: "Delete navigation property serviceConfigurationRecords for domains",
        name: delete_service_configuration_records,
        path: "/domains/{{RID}}/serviceConfigurationRecords/{{id}}",
        params: domain_dns_record_id
    );
    get!(
        doc: "Get serviceConfigurationRecords from domains",
        name: get_service_configuration_records,
        path: "/domains/{{RID}}/serviceConfigurationRecords/{{id}}",
        params: domain_dns_record_id
    );
    patch!(
        doc: "Update the navigation property serviceConfigurationRecords in domains",
        name: update_service_configuration_records,
        path: "/domains/{{RID}}/serviceConfigurationRecords/{{id}}",
        body: true,
        params: domain_dns_record_id
    );
    post!(
        doc: "Create new navigation property to verificationDnsRecords for domains",
        name: create_verification_dns_records,
        path: "/domains/{{RID}}/verificationDnsRecords",
        body: true
    );
    get!(
        doc: "List verificationDnsRecords",
        name: list_verification_dns_records,
        path: "/domains/{{RID}}/verificationDnsRecords"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_verification_dns_records_count,
        path: "/domains/{{RID}}/verificationDnsRecords/$count"
    );
    delete!(
        doc: "Delete navigation property verificationDnsRecords for domains",
        name: delete_verification_dns_records,
        path: "/domains/{{RID}}/verificationDnsRecords/{{id}}",
        params: domain_dns_record_id
    );
    get!(
        doc: "Get verificationDnsRecords from domains",
        name: get_verification_dns_records,
        path: "/domains/{{RID}}/verificationDnsRecords/{{id}}",
        params: domain_dns_record_id
    );
    patch!(
        doc: "Update the navigation property verificationDnsRecords in domains",
        name: update_verification_dns_records,
        path: "/domains/{{RID}}/verificationDnsRecords/{{id}}",
        body: true,
        params: domain_dns_record_id
    );
    post!(
        doc: "Invoke action verify",
        name: verify,
        path: "/domains/{{RID}}/verify"
    );
}
