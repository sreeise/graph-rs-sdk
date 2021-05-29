use crate::client::Graph;
use graph_http::{types::NoContent, IntoResponse};
use reqwest::Method;

register_client!(OutlookRequest,);

impl<'a, Client> OutlookRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get outlook from me",
        name: get_outlook,
        response: serde_json::Value,
        path: "/outlook",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property outlook in me",
        name: update_outlook,
        response: NoContent,
        path: "/outlook",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get masterCategories from me",
        name: list_master_categories,
        response: serde_json::Value,
        path: "/outlook/masterCategories",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to masterCategories for me",
        name: create_master_categories,
        response: serde_json::Value,
        path: "/outlook/masterCategories",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get masterCategories from me",
        name: get_master_categories,
        response: serde_json::Value,
        path: "/outlook/masterCategories/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property masterCategories in me",
        name: update_master_categories,
        response: NoContent,
        path: "/outlook/masterCategories/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Invoke function supportedLanguages",
        name: supported_languages,
        response: serde_json::Value,
        path: "/outlook/supportedLanguages()",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Invoke function supportedTimeZones",
        name: supported_time_zones,
        response: serde_json::Value,
        path: "/outlook/supportedTimeZones()",
        params: 0,
        has_body: false
    });
}
