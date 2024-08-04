// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    InferenceClassificationApiClient,
    ResourceIdentity::InferenceClassification
);

impl InferenceClassificationApiClient {
    get!(
        doc: "Get inferenceClassification from users",
        name: get_inference_classification,
        path: "/inferenceClassification"
    );
    patch!(
        doc: "Update the navigation property inferenceClassification in users",
        name: update_inference_classification,
        path: "/inferenceClassification",
        body: true
    );
    post!(
        doc: "Create inferenceClassificationOverride",
        name: create_overrides,
        path: "/inferenceClassification/overrides",
        body: true
    );
    get!(
        doc: "List overrides",
        name: list_overrides,
        path: "/inferenceClassification/overrides"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_overrides_count,
        path: "/inferenceClassification/overrides/$count"
    );
    delete!(
        doc: "Delete navigation property overrides for users",
        name: delete_overrides,
        path: "/inferenceClassification/overrides/{{id}}",
        params: inference_classification_override_id
    );
    get!(
        doc: "Get overrides from users",
        name: get_overrides,
        path: "/inferenceClassification/overrides/{{id}}",
        params: inference_classification_override_id
    );
    patch!(
        doc: "Update the navigation property overrides in users",
        name: update_overrides,
        path: "/inferenceClassification/overrides/{{id}}",
        body: true,
        params: inference_classification_override_id
    );
}
