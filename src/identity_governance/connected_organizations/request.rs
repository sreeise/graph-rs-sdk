// GENERATED CODE

use crate::api_default_imports::*;
use crate::identity_governance::*;

resource_api_client!(
    ConnectedOrganizationsApiClient,
    ConnectedOrganizationsIdApiClient,
    ResourceIdentity::ConnectedOrganizations
);

impl ConnectedOrganizationsApiClient {
    post!(
        doc: "Create new navigation property to connectedOrganizations for identityGovernance",
        name: create_connected_organizations,
        path: "/connectedOrganizations",
        body: true
    );
    get!(
        doc: "List connectedOrganizations",
        name: list_connected_organizations,
        path: "/connectedOrganizations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_connected_organizations_count,
        path: "/connectedOrganizations/$count"
    );
}

impl ConnectedOrganizationsIdApiClient {
    api_client_link!(
        external_sponsors,
        ConnectedOrganizationsExternalSponsorsApiClient
    );
    api_client_link!(
        internal_sponsors,
        ConnectedOrganizationsInternalSponsorsApiClient
    );

    delete!(
        doc: "Delete navigation property connectedOrganizations for identityGovernance",
        name: delete_connected_organizations,
        path: "/connectedOrganizations/{{RID}}"
    );
    get!(
        doc: "Get connectedOrganizations from identityGovernance",
        name: get_connected_organizations,
        path: "/connectedOrganizations/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property connectedOrganizations in identityGovernance",
        name: update_connected_organizations,
        path: "/connectedOrganizations/{{RID}}",
        body: true
    );
}
