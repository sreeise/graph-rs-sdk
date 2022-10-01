// GENERATED CODE

use crate::api_default_imports::*;
use crate::authentication_method_configurations::{
    AuthenticationMethodConfigurationsIdRequest, AuthenticationMethodConfigurationsRequest,
};
use graph_http::types::NoContent;

register_client!(AuthenticationMethodsPolicyRequest,);

impl<'a, Client> AuthenticationMethodsPolicyRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn authentication_method_configurations(
        &self,
    ) -> AuthenticationMethodConfigurationsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client
            .set_ident(ResourceIdentity::AuthenticationMethodConfigurations);
        AuthenticationMethodConfigurationsRequest::new(self.client)
    }

    pub fn authentication_method_configuration<ID: AsRef<str>>(
        &self,
        id: ID,
    ) -> AuthenticationMethodConfigurationsIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client
            .set_ident(ResourceIdentity::AuthenticationMethodConfigurations);
        AuthenticationMethodConfigurationsIdRequest::new(id.as_ref(), self.client)
    }

    get!({
        doc: "Get authenticationMethodsPolicy",
        name: get_authentication_methods_policy,
        response: serde_json::Value,
        path: "/authenticationMethodsPolicy",
        has_body: false
    });
    patch!({
        doc: "Update authenticationMethodsPolicy",
        name: update_authentication_methods_policy,
        response: NoContent,
        path: "/authenticationMethodsPolicy",
        has_body: true
    });
}
