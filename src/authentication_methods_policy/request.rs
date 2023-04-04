// GENERATED CODE

use crate::api_default_imports::*;
use crate::authentication_method_configurations::*;

resource_api_client!(
    AuthenticationMethodsPolicyApiClient,
    ResourceIdentity::AuthenticationMethodsPolicy
);

impl AuthenticationMethodsPolicyApiClient {
    api_client_link!(
        authentication_method_configurations,
        AuthenticationMethodConfigurationsApiClient
    );
    api_client_link_id!(
        authentication_method_configuration,
        AuthenticationMethodConfigurationsIdApiClient
    );

    get!(
        doc: "Get authenticationMethodsPolicy",
        name: get_authentication_methods_policy,
        path: "/authenticationMethodsPolicy"
    );
    patch!(
        doc: "Update authenticationMethodsPolicy",
        name: update_authentication_methods_policy,
        path: "/authenticationMethodsPolicy",
        body: true
    );
    post!(
        doc: "Create new navigation property to authenticationMethodConfigurations for authenticationMethodsPolicy",
        name: create_authentication_method_configurations,
        path: "/authenticationMethodsPolicy/authenticationMethodConfigurations",
        body: true
    );
    get!(
        doc: "Get authenticationMethodConfigurations from authenticationMethodsPolicy",
        name: list_authentication_method_configurations,
        path: "/authenticationMethodsPolicy/authenticationMethodConfigurations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_authentication_method_configurations_count,
        path: "/authenticationMethodsPolicy/authenticationMethodConfigurations/$count"
    );
    delete!(
        doc: "Delete navigation property authenticationMethodConfigurations for authenticationMethodsPolicy",
        name: delete_authentication_method_configurations,
        path: "/authenticationMethodsPolicy/authenticationMethodConfigurations/{{id}}",
        params: authentication_method_configuration_id
    );
    get!(
        doc: "Get authenticationMethodConfigurations from authenticationMethodsPolicy",
        name: get_authentication_method_configurations,
        path: "/authenticationMethodsPolicy/authenticationMethodConfigurations/{{id}}",
        params: authentication_method_configuration_id
    );
    patch!(
        doc: "Update the navigation property authenticationMethodConfigurations in authenticationMethodsPolicy",
        name: update_authentication_method_configurations,
        path: "/authenticationMethodsPolicy/authenticationMethodConfigurations/{{id}}",
        body: true,
        params: authentication_method_configuration_id
    );
}
