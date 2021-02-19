use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(SchemaExtensionsRequest,);

impl<'a, Client> SchemaExtensionsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entities from schemaExtensions",
        name: list_schema_extension,
        response: Collection<serde_json::Value>,
        path: "/schemaExtensions",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Add new entity to schemaExtensions",
        name: create_schema_extension,
        response: serde_json::Value,
        path: "/schemaExtensions",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get entity from schemaExtensions by key",
        name: get_schema_extension,
        response: serde_json::Value,
        path: "/schemaExtensions/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update entity in schemaExtensions",
        name: update_schema_extension,
        response: NoContent,
        path: "/schemaExtensions/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from schemaExtensions",
        name: delete_schema_extension,
        response: NoContent,
        path: "/schemaExtensions/{{id}}",
        params: 1,
        has_body: false
    });
}
