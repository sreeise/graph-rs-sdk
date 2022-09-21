// GENERATED CODE

use crate::administrative_units::{AdministrativeUnitsIdRequest, AdministrativeUnitsRequest};
use crate::api_default_imports::*;
use crate::directory_deleted_items::{
    DirectoryDeletedItemsIdRequest, DirectoryDeletedItemsRequest,
};
use graph_http::types::NoContent;

register_client!(DirectoryRequest,);

impl<'a, Client> DirectoryRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn administrative_units(&self) -> AdministrativeUnitsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::AdministrativeUnits);
        AdministrativeUnitsRequest::new(self.client)
    }

    pub fn administrative_unit<ID: AsRef<str>>(
        &self,
        id: ID,
    ) -> AdministrativeUnitsIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::AdministrativeUnits);
        AdministrativeUnitsIdRequest::new(id.as_ref(), self.client)
    }

    pub fn deleted_items(&self) -> DirectoryDeletedItemsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::AdministrativeUnits);
        DirectoryDeletedItemsRequest::new(self.client)
    }

    pub fn deleted_item<ID: AsRef<str>>(
        &self,
        id: ID,
    ) -> DirectoryDeletedItemsIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::AdministrativeUnits);
        DirectoryDeletedItemsIdRequest::new(id.as_ref(), self.client)
    }

    get!({
        doc: "Get directory",
        name: get_directory,
        response: serde_json::Value,
        path: "/directory",
        has_body: false
    });
    patch!({
        doc: "Update directory",
        name: update_directory,
        response: NoContent,
        path: "/directory",
        has_body: true
    });
    post!({
        doc: "Create new navigation property to federationConfigurations for directory",
        name: create_federation_configurations,
        response: serde_json::Value,
        path: "/directory/federationConfigurations",
        has_body: true
    });
    get!({
        doc: "Get federationConfigurations from directory",
        name: list_federation_configurations,
        response: serde_json::Value,
        path: "/directory/federationConfigurations",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_federation_configurations_count,
        response: serde_json::Value,
        path: "/directory/federationConfigurations/$count",
        has_body: false
    });
    get!({
        doc: "Invoke function availableProviderTypes",
        name: get_federation_configurations_available_provider_types,
        response: serde_json::Value,
        path: "/directory/federationConfigurations/microsoft.graph.availableProviderTypes()",
        has_body: false
    });
    delete!({
        doc: "Delete navigation property federationConfigurations for directory",
        name: delete_federation_configurations,
        response: NoContent,
        path: "/directory/federationConfigurations/{{id}}",
        params: [ identity_provider_base_id ],
        has_body: false
    });
    get!({
        doc: "Get federationConfigurations from directory",
        name: get_federation_configurations,
        response: serde_json::Value,
        path: "/directory/federationConfigurations/{{id}}",
        params: [ identity_provider_base_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property federationConfigurations in directory",
        name: update_federation_configurations,
        response: NoContent,
        path: "/directory/federationConfigurations/{{id}}",
        params: [ identity_provider_base_id ],
        has_body: true
    });
}
