// NOT GENERATED CODE.

use crate::client::Graph;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(ExtendedPropertiesRequest,);

impl<'a, Client> ExtendedPropertiesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        name: get_multi_value_extended_properties,
        response: serde_json::Value,
        path: "/multiValueExtendedProperties/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        name: create_multi_value_extended_properties,
        response: serde_json::Value,
        path: "/multiValueExtendedProperties",
        params: 0,
        has_body: true
    });
    get!({
        name: get_single_value_extended_properties,
        response: serde_json::Value,
        path: "/singleValueExtendedProperties/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        name: create_single_value_extended_properties,
        response: serde_json::Value,
        path: "/singleValueExtendedProperties",
        params: 0,
        has_body: true
    });
}
