// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    ServicePrincipalsSynchronizationTemplatesApiClient,
    ServicePrincipalsSynchronizationTemplatesIdApiClient,
    ResourceIdentity::ServicePrincipalsSynchronizationTemplates
);

impl ServicePrincipalsSynchronizationTemplatesApiClient {
    post!(
        doc: "Create new navigation property to templates for servicePrincipals",
        name: create_templates,
        path: "/templates",
        body: true
    );
    get!(
        doc: "List existing synchronization templates",
        name: list_templates,
        path: "/templates"
    );
    get!(
        doc: "Get the number of the resource",
        name: templates,
        path: "/templates/$count"
    );
}

impl ServicePrincipalsSynchronizationTemplatesIdApiClient {
    delete!(
        doc: "Delete navigation property templates for servicePrincipals",
        name: delete_templates,
        path: "/templates/{{RID}}"
    );
    get!(
        doc: "Get templates from servicePrincipals",
        name: get_templates,
        path: "/templates/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property templates in servicePrincipals",
        name: update_templates,
        path: "/templates/{{RID}}",
        body: true
    );
    delete!(
        doc: "Delete navigation property schema for servicePrincipals",
        name: delete_schema,
        path: "/templates/{{RID}}/schema"
    );
    get!(
        doc: "Get schema from servicePrincipals",
        name: get_schema,
        path: "/templates/{{RID}}/schema"
    );
    patch!(
        doc: "Update the navigation property schema in servicePrincipals",
        name: update_schema,
        path: "/templates/{{RID}}/schema",
        body: true
    );
    post!(
        doc: "Create new navigation property to directories for servicePrincipals",
        name: create_directories,
        path: "/templates/{{RID}}/schema/directories",
        body: true
    );
    get!(
        doc: "Get directories from servicePrincipals",
        name: list_directories,
        path: "/templates/{{RID}}/schema/directories"
    );
    get!(
        doc: "Get the number of the resource",
        name: directories,
        path: "/templates/{{RID}}/schema/directories/$count"
    );
    delete!(
        doc: "Delete navigation property directories for servicePrincipals",
        name: delete_directories,
        path: "/templates/{{RID}}/schema/directories/{{id}}",
        params: directory_definition_id
    );
    get!(
        doc: "Get directories from servicePrincipals",
        name: get_directories,
        path: "/templates/{{RID}}/schema/directories/{{id}}",
        params: directory_definition_id
    );
    patch!(
        doc: "Update the navigation property directories in servicePrincipals",
        name: update_directories,
        path: "/templates/{{RID}}/schema/directories/{{id}}",
        body: true,
        params: directory_definition_id
    );
    post!(
        doc: "Invoke action discover",
        name: discover,
        path: "/templates/{{RID}}/schema/directories/{{id}}/discover",
        params: directory_definition_id
    );
    get!(
        doc: "Invoke function filterOperators",
        name: filter_operators,
        path: "/templates/{{RID}}/schema/filterOperators()"
    );
    get!(
        doc: "Invoke function functions",
        name: functions,
        path: "/templates/{{RID}}/schema/functions()"
    );
    post!(
        doc: "Invoke action parseExpression",
        name: parse_expression,
        path: "/templates/{{RID}}/schema/parseExpression",
        body: true
    );
}
