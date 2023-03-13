// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    IdentityProvidersApiClient,
    IdentityProvidersIdApiClient,
    ResourceIdentity::IdentityProviders
);

impl IdentityProvidersApiClient {
    post!(
        doc: "Add new entity to identityProviders",
        name: create_identity_provider,
        path: "/identityProviders",
        body: true
    );
    get!(
        doc: "List identityProviders (deprecated)",
        name: list_identity_provider,
        path: "/identityProviders"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_identity_providers_count,
        path: "/identityProviders/$count"
    );
    get!(
        doc: "Invoke function availableProviderTypes",
        name: available_provider_types,
        path: "/identityProviders/availableProviderTypes()"
    );
}

impl IdentityProvidersIdApiClient {
    delete!(
        doc: "Delete identityProvider (deprecated)",
        name: delete_identity_provider,
        path: "/identityProviders/{{RID}}"
    );
    get!(
        doc: "Get identityProvider (deprecated)",
        name: get_identity_provider,
        path: "/identityProviders/{{RID}}"
    );
    patch!(
        doc: "Update identityProvider (deprecated)",
        name: update_identity_provider,
        path: "/identityProviders/{{RID}}",
        body: true
    );
}
