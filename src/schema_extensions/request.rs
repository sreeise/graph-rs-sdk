// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    SchemaExtensionsApiClient,
    SchemaExtensionsIdApiClient,
    ResourceIdentity::SchemaExtensions
);

impl SchemaExtensionsApiClient {
    post!(
        doc: "Create schemaExtension",
        name: create_schema_extension,
        path: "/schemaExtensions",
        body: true
    );
    get!(
        doc: "List schemaExtensions",
        name: list_schema_extension,
        path: "/schemaExtensions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_schema_extensions_count,
        path: "/schemaExtensions/$count"
    );
}

impl SchemaExtensionsIdApiClient {
    delete!(
        doc: "Delete schemaExtension",
        name: delete_schema_extension,
        path: "/schemaExtensions/{{RID}}"
    );
    get!(
        doc: "Get schemaExtension",
        name: get_schema_extension,
        path: "/schemaExtensions/{{RID}}"
    );
    patch!(
        doc: "Update schemaExtension",
        name: update_schema_extension,
        path: "/schemaExtensions/{{RID}}",
        body: true
    );
}
