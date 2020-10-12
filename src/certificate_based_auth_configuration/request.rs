use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::{GraphResponse, IntoResponse};
use reqwest::Method;

register_client!(CertificateBasedAuthConfigurationRequest,);

#[allow(dead_code)]
impl<'a, Client> CertificateBasedAuthConfigurationRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entity from certificateBasedAuthConfiguration by key",
        name: get_certificate_based_auth_configuration,
        response: serde_json::Value,
        path: "/certificateBasedAuthConfiguration/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update entity in certificateBasedAuthConfiguration",
        name: update_certificate_based_auth_configuration,
        response: GraphResponse<Content>,
        path: "/certificateBasedAuthConfiguration/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from certificateBasedAuthConfiguration",
        name: delete_certificate_based_auth_configuration,
        response: GraphResponse<Content>,
        path: "/certificateBasedAuthConfiguration/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get entities from certificateBasedAuthConfiguration",
        name: list_certificate_based_auth_configuration,
        response: Collection<serde_json::Value>,
        path: "/certificateBasedAuthConfiguration",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Add new entity to certificateBasedAuthConfiguration",
        name: create_certificate_based_auth_configuration,
        response: serde_json::Value,
        path: "/certificateBasedAuthConfiguration",
        params: 0,
        has_body: true
    });
}
