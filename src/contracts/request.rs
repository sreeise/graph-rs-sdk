// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    ContractsApiClient,
    ContractsIdApiClient,
    ResourceIdentity::Contracts
);

impl ContractsApiClient {
    post!(
        doc: "Add new entity to contracts",
        name: create_contract,
        path: "/contracts",
        body: true
    );
    get!(
        doc: "List contracts",
        name: list_contract,
        path: "/contracts"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_contracts_count,
        path: "/contracts/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/contracts/delta()"
    );
    post!(
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        path: "/contracts/getAvailableExtensionProperties",
        body: true
    );
    post!(
        doc: "Invoke action getByIds",
        name: get_by_ids,
        path: "/contracts/getByIds",
        body: true
    );
    post!(
        doc: "Invoke action validateProperties",
        name: validate_properties,
        path: "/contracts/validateProperties",
        body: true
    );
}

impl ContractsIdApiClient {
    delete!(
        doc: "Delete entity from contracts",
        name: delete_contract,
        path: "/contracts/{{RID}}"
    );
    get!(
        doc: "Get Contract",
        name: get_contract,
        path: "/contracts/{{RID}}"
    );
    patch!(
        doc: "Update entity in contracts",
        name: update_contract,
        path: "/contracts/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberGroups",
        name: check_member_groups,
        path: "/contracts/{{RID}}/checkMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberObjects",
        name: check_member_objects,
        path: "/contracts/{{RID}}/checkMemberObjects",
        body: true
    );
    post!(
        doc: "Invoke action getMemberGroups",
        name: get_member_groups,
        path: "/contracts/{{RID}}/getMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action getMemberObjects",
        name: get_member_objects,
        path: "/contracts/{{RID}}/getMemberObjects",
        body: true
    );
    post!(
        doc: "Invoke action restore",
        name: restore,
        path: "/contracts/{{RID}}/restore"
    );
}
