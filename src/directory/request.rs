// GENERATED CODE

use crate::api_default_imports::*;
use crate::directory::*;

resource_api_client!(DirectoryApiClient, ResourceIdentity::Directory);

impl DirectoryApiClient {
    api_client_link!(deleted_items, DeletedItemsApiClient);
    api_client_link_id!(administrative_unit, AdministrativeUnitsIdApiClient);
    api_client_link!(administrative_units, AdministrativeUnitsApiClient);
    api_client_link_id!(deleted_item, DeletedItemsIdApiClient);

    get!(
        doc: "Get directory",
        name: get_directory,
        path: "/directory"
    );
    patch!(
        doc: "Update directory",
        name: update_directory,
        path: "/directory",
        body: true
    );
    post!(
        doc: "Create new navigation property to federationConfigurations for directory",
        name: create_federation_configurations,
        path: "/directory/federationConfigurations",
        body: true
    );
    get!(
        doc: "Get federationConfigurations from directory",
        name: list_federation_configurations,
        path: "/directory/federationConfigurations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_federation_configurations_count,
        path: "/directory/federationConfigurations/$count"
    );
    get!(
        doc: "Invoke function availableProviderTypes",
        name: available_provider_types,
        path: "/directory/federationConfigurations/availableProviderTypes()"
    );
    delete!(
        doc: "Delete navigation property federationConfigurations for directory",
        name: delete_federation_configurations,
        path: "/directory/federationConfigurations/{{id}}",
        params: identity_provider_base_id
    );
    get!(
        doc: "Get federationConfigurations from directory",
        name: get_federation_configurations,
        path: "/directory/federationConfigurations/{{id}}",
        params: identity_provider_base_id
    );
    patch!(
        doc: "Update the navigation property federationConfigurations in directory",
        name: update_federation_configurations,
        path: "/directory/federationConfigurations/{{id}}",
        body: true,
        params: identity_provider_base_id
    );
    post!(
        doc: "Create new navigation property to onPremisesSynchronization for directory",
        name: create_on_premises_synchronization,
        path: "/directory/onPremisesSynchronization",
        body: true
    );
    get!(
        doc: "Get onPremisesSynchronization from directory",
        name: list_on_premises_synchronization,
        path: "/directory/onPremisesSynchronization"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_on_premises_synchronization_count,
        path: "/directory/onPremisesSynchronization/$count"
    );
    delete!(
        doc: "Delete navigation property onPremisesSynchronization for directory",
        name: delete_on_premises_synchronization,
        path: "/directory/onPremisesSynchronization/{{id}}",
        params: on_premises_directory_synchronization_id
    );
    get!(
        doc: "Get onPremisesSynchronization from directory",
        name: get_on_premises_synchronization,
        path: "/directory/onPremisesSynchronization/{{id}}",
        params: on_premises_directory_synchronization_id
    );
    patch!(
        doc: "Update the navigation property onPremisesSynchronization in directory",
        name: update_on_premises_synchronization,
        path: "/directory/onPremisesSynchronization/{{id}}",
        body: true,
        params: on_premises_directory_synchronization_id
    );
}
