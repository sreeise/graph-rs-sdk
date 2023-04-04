// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    CertificateBasedAuthConfigurationApiClient,
    CertificateBasedAuthConfigurationIdApiClient,
    ResourceIdentity::CertificateBasedAuthConfiguration
);

impl CertificateBasedAuthConfigurationApiClient {
    post!(
        doc: "Add new entity to certificateBasedAuthConfiguration",
        name: create_certificate_based_auth_configuration,
        path: "/certificateBasedAuthConfiguration",
        body: true
    );
    get!(
        doc: "Get entities from certificateBasedAuthConfiguration",
        name: list_certificate_based_auth_configuration,
        path: "/certificateBasedAuthConfiguration"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_certificate_based_auth_configuration_count,
        path: "/certificateBasedAuthConfiguration/$count"
    );
}

impl CertificateBasedAuthConfigurationIdApiClient {
    delete!(
        doc: "Delete entity from certificateBasedAuthConfiguration",
        name: delete_certificate_based_auth_configuration,
        path: "/certificateBasedAuthConfiguration/{{RID}}"
    );
    get!(
        doc: "Get entity from certificateBasedAuthConfiguration by key",
        name: get_certificate_based_auth_configuration,
        path: "/certificateBasedAuthConfiguration/{{RID}}"
    );
    patch!(
        doc: "Update entity in certificateBasedAuthConfiguration",
        name: update_certificate_based_auth_configuration,
        path: "/certificateBasedAuthConfiguration/{{RID}}",
        body: true
    );
}
