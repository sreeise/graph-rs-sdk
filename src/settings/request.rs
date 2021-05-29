use crate::client::Graph;
use graph_http::{types::NoContent, IntoResponse};
use reqwest::Method;

register_client!(SettingsRequest,);

impl<'a, Client> SettingsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get settings from me",
        name: get_settings,
        response: serde_json::Value,
        path: "/settings",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property settings in me",
        name: update_settings,
        response: NoContent,
        path: "/settings",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get shiftPreferences from me",
        name: get_shift_preferences,
        response: serde_json::Value,
        path: "/settings/shiftPreferences",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property shiftPreferences in me",
        name: update_shift_preferences,
        response: NoContent,
        path: "/settings/shiftPreferences",
        params: 0,
        has_body: true
    });
}
