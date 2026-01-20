// GENERATED CODE

use crate::api_default_imports::*;
use crate::service_principals::*;

api_client!(
    ServicePrincipalsSynchronizationApiClient,
    ResourceIdentity::ServicePrincipalsSynchronization
);

impl ServicePrincipalsSynchronizationApiClient {
    api_client_link!(jobs, ServicePrincipalsSynchronizationJobsApiClient);
    api_client_link_id!(job, ServicePrincipalsSynchronizationJobsIdApiClient);
    api_client_link!(
        templates,
        ServicePrincipalsSynchronizationTemplatesApiClient
    );
    api_client_link_id!(
        template,
        ServicePrincipalsSynchronizationTemplatesIdApiClient
    );

    delete!(
        doc: "Delete navigation property synchronization for servicePrincipals",
        name: delete_synchronization,
        path: "/synchronization"
    );
    get!(
        doc: "Get synchronization from servicePrincipals",
        name: get_synchronization,
        path: "/synchronization"
    );
    put!(
        doc: "Update the navigation property synchronization in servicePrincipals",
        name: set_synchronization,
        path: "/synchronization",
        body: true
    );
    post!(
        doc: "Invoke action acquireAccessToken",
        name: acquire_access_token,
        path: "/synchronization/acquireAccessToken",
        body: true
    );
    put!(
        doc: "Add synchronization secrets",
        name: set_secrets,
        path: "/synchronization/secrets",
        body: true
    );
    get!(
        doc: "Get the number of the resource",
        name: secrets,
        path: "/synchronization/secrets/$count"
    );
}
