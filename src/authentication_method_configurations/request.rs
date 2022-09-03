// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(AuthenticationMethodConfigurationsRequest,);
register_client!(AuthenticationMethodConfigurationsIdRequest, ());

impl<'a, Client> AuthenticationMethodConfigurationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Add new entity to authenticationMethodConfigurations",
        name: create_authentication_method_configuration,
        response: serde_json::Value,
        path: "/authenticationMethodConfigurations",
        has_body: true
    });
    get!({
        doc: "Get entities from authenticationMethodConfigurations",
        name: list_authentication_method_configuration,
        response: serde_json::Value,
        path: "/authenticationMethodConfigurations",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_authentication_methods_configurations_count,
        response: serde_json::Value,
        path: "/authenticationMethodConfigurations/$count",
        has_body: false
    });
}

impl<'a, Client> AuthenticationMethodConfigurationsIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    delete!({
        doc: "Delete entity from authenticationMethodConfigurations",
        name: delete_authentication_method_configuration,
        response: NoContent,
        path: "/authenticationMethodConfigurations/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get entity from authenticationMethodConfigurations by key",
        name: get_authentication_method_configuration,
        response: serde_json::Value,
        path: "/authenticationMethodConfigurations/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update entity in authenticationMethodConfigurations",
        name: update_authentication_method_configuration,
        response: NoContent,
        path: "/authenticationMethodConfigurations/{{RID}}",
        has_body: true
    });
}
