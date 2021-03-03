use crate::client::Graph;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(InferenceClassificationRequest,);

impl<'a, Client> InferenceClassificationRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get inferenceClassification from me",
        name: get_inference_classification,
        response: serde_json::Value,
        path: "/inferenceClassification",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property inferenceClassification in me",
        name: update_inference_classification,
        response: NoContent,
        path: "/inferenceClassification",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get overrides from me",
        name: list_overrides,
        response: serde_json::Value,
        path: "/inferenceClassification/overrides",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to overrides for me",
        name: create_overrides,
        response: serde_json::Value,
        path: "/inferenceClassification/overrides",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get overrides from me",
        name: get_overrides,
        response: serde_json::Value,
        path: "/inferenceClassification/overrides/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property overrides in me",
        name: update_overrides,
        response: NoContent,
        path: "/inferenceClassification/overrides/{{id}}",
        params: 1,
        has_body: true
    });
}
