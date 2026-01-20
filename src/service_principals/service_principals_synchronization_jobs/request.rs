// GENERATED CODE

use crate::api_default_imports::*;
use crate::service_principals::*;

api_client!(
    ServicePrincipalsSynchronizationJobsApiClient,
    ServicePrincipalsSynchronizationJobsIdApiClient,
    ResourceIdentity::ServicePrincipalsSynchronizationJobs
);

impl ServicePrincipalsSynchronizationJobsApiClient {
    post!(
        doc: "Create synchronizationJob",
        name: create_jobs,
        path: "/jobs",
        body: true
    );
    get!(
        doc: "List synchronization jobs",
        name: list_jobs,
        path: "/jobs"
    );
    get!(
        doc: "Get the number of the resource",
        name: jobs,
        path: "/jobs/$count"
    );
    post!(
        doc: "Invoke action validateCredentials",
        name: validate_credentials,
        path: "/jobs/validateCredentials",
        body: true
    );
}

impl ServicePrincipalsSynchronizationJobsIdApiClient {
    api_client_link_id!(job, ServicePrincipalsSynchronizationJobsIdApiClient);

    delete!(
        doc: "Delete synchronizationJob",
        name: delete_jobs,
        path: "/jobs/{{RID}}"
    );
    get!(
        doc: "Get synchronizationJob",
        name: get_jobs,
        path: "/jobs/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property jobs in servicePrincipals",
        name: update_jobs,
        path: "/jobs/{{RID}}",
        body: true
    );
    delete!(
        doc: "Delete navigation property bulkUpload for servicePrincipals",
        name: delete_bulk_upload,
        path: "/jobs/{{RID}}/bulkUpload"
    );
    get!(
        doc: "Get bulkUpload from servicePrincipals",
        name: get_bulk_upload,
        path: "/jobs/{{RID}}/bulkUpload"
    );
    patch!(
        doc: "Update the navigation property bulkUpload in servicePrincipals",
        name: update_bulk_upload,
        path: "/jobs/{{RID}}/bulkUpload",
        body: true
    );
    delete!(
        doc: "Delete media content for the navigation property bulkUpload in servicePrincipals",
        name: delete_bulk_upload_content,
        path: "/jobs/{{RID}}/bulkUpload/$value"
    );
    get!(
        doc: "Get media content for the navigation property bulkUpload from servicePrincipals",
        name: get_bulk_upload_content,
        path: "/jobs/{{RID}}/bulkUpload/$value"
    );
    put!(
        doc: "Update media content for the navigation property bulkUpload in servicePrincipals",
        name: update_bulk_upload_content,
        path: "/jobs/{{RID}}/bulkUpload/$value",
        body: true
    );
    post!(
        doc: "Invoke action pause",
        name: pause,
        path: "/jobs/{{RID}}/pause"
    );
    post!(
        doc: "Invoke action provisionOnDemand",
        name: provision_on_demand,
        path: "/jobs/{{RID}}/provisionOnDemand",
        body: true
    );
    post!(
        doc: "Invoke action restart",
        name: restart,
        path: "/jobs/{{RID}}/restart",
        body: true
    );
    delete!(
        doc: "Delete navigation property schema for servicePrincipals",
        name: delete_schema,
        path: "/jobs/{{RID}}/schema"
    );
    get!(
        doc: "Get synchronizationSchema",
        name: get_schema,
        path: "/jobs/{{RID}}/schema"
    );
    patch!(
        doc: "Update synchronizationSchema",
        name: update_schema,
        path: "/jobs/{{RID}}/schema",
        body: true
    );
    post!(
        doc: "Create new navigation property to directories for servicePrincipals",
        name: create_directories,
        path: "/jobs/{{RID}}/schema/directories",
        body: true
    );
    get!(
        doc: "Get directories from servicePrincipals",
        name: list_directories,
        path: "/jobs/{{RID}}/schema/directories"
    );
    get!(
        doc: "Get the number of the resource",
        name: directories,
        path: "/jobs/{{RID}}/schema/directories/$count"
    );
    delete!(
        doc: "Delete navigation property directories for servicePrincipals",
        name: delete_directories,
        path: "/jobs/{{RID}}/schema/directories/{{id}}",
        params: directory_definition_id
    );
    get!(
        doc: "Get directories from servicePrincipals",
        name: get_directories,
        path: "/jobs/{{RID}}/schema/directories/{{id}}",
        params: directory_definition_id
    );
    patch!(
        doc: "Update the navigation property directories in servicePrincipals",
        name: update_directories,
        path: "/jobs/{{RID}}/schema/directories/{{id}}",
        body: true,
        params: directory_definition_id
    );
    post!(
        doc: "Invoke action discover",
        name: discover,
        path: "/jobs/{{RID}}/schema/directories/{{id}}/discover",
        params: directory_definition_id
    );
    get!(
        doc: "Invoke function filterOperators",
        name: filter_operators,
        path: "/jobs/{{RID}}/schema/filterOperators()"
    );
    get!(
        doc: "Invoke function functions",
        name: functions,
        path: "/jobs/{{RID}}/schema/functions()"
    );
    post!(
        doc: "Invoke action parseExpression",
        name: parse_expression,
        path: "/jobs/{{RID}}/schema/parseExpression",
        body: true
    );
    post!(
        doc: "Invoke action start",
        name: start,
        path: "/jobs/{{RID}}/start"
    );
    post!(
        doc: "Invoke action validateCredentials",
        name: validate_credentials,
        path: "/jobs/{{RID}}/validateCredentials",
        body: true
    );
}
