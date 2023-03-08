// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::DeltaPhantom;
use crate::identity_governance::{ConnectedOrganizationsExternalSponsorsApiClient, ConnectedOrganizationsInternalSponsorsApiClient};

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
        internal_sponsors,
        ResourceIdentity::ConnectedOrganizationsInternalSponsors,
        ConnectedOrganizationsInternalSponsorsApiClient
    );
    api_client_link!(
        external_sponsors,
        ResourceIdentity::ConnectedOrganizationsExternalSponsors,
        ConnectedOrganizationsExternalSponsorsApiClient
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
    post!(
        doc: "Create new navigation property to externalSponsors for identityGovernance",
        name: create_external_sponsors,
        path: "/connectedOrganizations/{{RID}}/externalSponsors",
        body: true
    );
    get!(
        doc: "List externalSponsors",
        name: list_external_sponsors,
        path: "/connectedOrganizations/{{RID}}/externalSponsors"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_external_sponsors_count,
        path: "/connectedOrganizations/{{RID}}/externalSponsors/$count"
    );
    post!(
        doc: "Create new navigation property ref to externalSponsors for identityGovernance",
        name: create_ref_external_sponsors,
        path: "/connectedOrganizations/{{RID}}/externalSponsors/$ref",
        body: true
    );
    get!(
        doc: "List externalSponsors",
        name: list_ref_external_sponsors,
        path: "/connectedOrganizations/{{RID}}/externalSponsors/$ref"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/connectedOrganizations/{{RID}}/externalSponsors/delta()"
    );
    post!(
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        path: "/connectedOrganizations/{{RID}}/externalSponsors/getAvailableExtensionProperties",
        body: true
    );
    post!(
        doc: "Invoke action getByIds",
        name: get_by_ids,
        path: "/connectedOrganizations/{{RID}}/externalSponsors/getByIds",
        body: true
    );
    post!(
        doc: "Invoke action validateProperties",
        name: validate_properties,
        path: "/connectedOrganizations/{{RID}}/externalSponsors/validateProperties",
        body: true
    );
    delete!(
        doc: "Delete ref of navigation property externalSponsors for identityGovernance",
        name: delete_ref_external_sponsors,
        path: "/connectedOrganizations/{{RID}}/externalSponsors/{{id}}/$ref",
        params: directory_object_id
    );
    post!(
        doc: "Create new navigation property to internalSponsors for identityGovernance",
        name: create_internal_sponsors,
        path: "/connectedOrganizations/{{RID}}/internalSponsors",
        body: true
    );
    get!(
        doc: "List internalSponsors",
        name: list_internal_sponsors,
        path: "/connectedOrganizations/{{RID}}/internalSponsors"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_internal_sponsors_count,
        path: "/connectedOrganizations/{{RID}}/internalSponsors/$count"
    );
    post!(
        doc: "Create new navigation property ref to internalSponsors for identityGovernance",
        name: create_ref_internal_sponsors,
        path: "/connectedOrganizations/{{RID}}/internalSponsors/$ref",
        body: true
    );
    get!(
        doc: "List internalSponsors",
        name: list_ref_internal_sponsors,
        path: "/connectedOrganizations/{{RID}}/internalSponsors/$ref"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/connectedOrganizations/{{RID}}/internalSponsors/delta()"
    );
    post!(
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        path: "/connectedOrganizations/{{RID}}/internalSponsors/getAvailableExtensionProperties",
        body: true
    );
    post!(
        doc: "Invoke action getByIds",
        name: get_by_ids,
        path: "/connectedOrganizations/{{RID}}/internalSponsors/getByIds",
        body: true
    );
    post!(
        doc: "Invoke action validateProperties",
        name: validate_properties,
        path: "/connectedOrganizations/{{RID}}/internalSponsors/validateProperties",
        body: true
    );
    delete!(
        doc: "Delete ref of navigation property internalSponsors for identityGovernance",
        name: delete_ref_internal_sponsors,
        path: "/connectedOrganizations/{{RID}}/internalSponsors/{{id}}/$ref",
        params: directory_object_id
    );
}
