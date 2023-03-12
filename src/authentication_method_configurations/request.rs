// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    AuthenticationMethodConfigurationsApiClient,
    AuthenticationMethodConfigurationsIdApiClient,
    ResourceIdentity::AuthenticationMethodConfigurations
);

impl AuthenticationMethodConfigurationsApiClient {
    post!(
        doc: "Add new entity to authenticationMethodConfigurations",
        name: create_authentication_method_configuration,
        path: "/authenticationMethodConfigurations",
        body: true
    );
    get!(
        doc: "Get entities from authenticationMethodConfigurations",
        name: list_authentication_method_configuration,
        path: "/authenticationMethodConfigurations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_authentication_method_configurations_count,
        path: "/authenticationMethodConfigurations/$count"
    );
}

impl AuthenticationMethodConfigurationsIdApiClient {
    delete!(
        doc: "Delete entity from authenticationMethodConfigurations",
        name: delete_authentication_method_configuration,
        path: "/authenticationMethodConfigurations/{{RID}}"
    );
    get!(
        doc: "Get entity from authenticationMethodConfigurations by key",
        name: get_authentication_method_configuration,
        path: "/authenticationMethodConfigurations/{{RID}}"
    );
    patch!(
        doc: "Update entity in authenticationMethodConfigurations",
        name: update_authentication_method_configuration,
        path: "/authenticationMethodConfigurations/{{RID}}",
        body: true
    );
}
