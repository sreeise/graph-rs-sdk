// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    DataPolicyOperationsApiClient,
    ResourceIdentity::DataPolicyOperations
);

impl DataPolicyOperationsApiClient {
    post!(
        doc: "Add new entity to dataPolicyOperations",
        name: create_data_policy_operation,
        path: "/dataPolicyOperations",
        body: true
    );
    get!(
        doc: "Get dataPolicyOperation",
        name: list_data_policy_operation,
        path: "/dataPolicyOperations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_data_policy_operations_count,
        path: "/dataPolicyOperations/$count"
    );
    delete!(
        doc: "Delete entity from dataPolicyOperations",
        name: delete_data_policy_operation,
        path: "/dataPolicyOperations/{{RID}}"
    );
    get!(
        doc: "Get dataPolicyOperation",
        name: get_data_policy_operation,
        path: "/dataPolicyOperations/{{RID}}"
    );
    patch!(
        doc: "Update entity in dataPolicyOperations",
        name: update_data_policy_operation,
        path: "/dataPolicyOperations/{{RID}}",
        body: true
    );
}
