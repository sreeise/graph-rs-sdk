use crate::client::Graph;
use graph_http::{types::NoContent, IntoResponse};
use reqwest::Method;

register_client!(TeamworkRequest,);

impl<'a, Client> TeamworkRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get workforceIntegrations from teamwork",
        name: get_workforce_integrations,
        response: serde_json::Value,
        path: "/teamwork/workforceIntegrations/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property workforceIntegrations in teamwork",
        name: update_workforce_integrations,
        response: NoContent,
        path: "/teamwork/workforceIntegrations/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get workforceIntegrations from teamwork",
        name: list_workforce_integrations,
        response: serde_json::Value,
        path: "/teamwork/workforceIntegrations",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to workforceIntegrations for teamwork",
        name: create_workforce_integrations,
        response: serde_json::Value,
        path: "/teamwork/workforceIntegrations",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get teamwork",
        name: get_teamwork,
        response: serde_json::Value,
        path: "/teamwork",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update teamwork",
        name: update_teamwork,
        response: NoContent,
        path: "/teamwork",
        params: 0,
        has_body: true
    });
}
