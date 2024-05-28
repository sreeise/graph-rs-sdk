// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    ConnectedOrganizationsExternalSponsorsApiClient,
    ResourceIdentity::ConnectedOrganizationsExternalSponsors
);

impl ConnectedOrganizationsExternalSponsorsApiClient {
    post!(
        doc: "Create new navigation property to externalSponsors for identityGovernance",
        name: create_external_sponsors,
        path: "/externalSponsors",
        body: true
    );
    get!(
        doc: "List externalSponsors",
        name: list_external_sponsors,
        path: "/externalSponsors"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_external_sponsors_count,
        path: "/externalSponsors/$count"
    );
    post!(
        doc: "Create new navigation property ref to externalSponsors for identityGovernance",
        name: create_ref_external_sponsors,
        path: "/externalSponsors/$ref",
        body: true
    );
    get!(
        doc: "List externalSponsors",
        name: list_ref_external_sponsors,
        path: "/externalSponsors/$ref"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/externalSponsors/delta()"
    );
    post!(
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        path: "/externalSponsors/getAvailableExtensionProperties",
        body: true
    );
    post!(
        doc: "Invoke action getByIds",
        name: get_by_ids,
        path: "/externalSponsors/getByIds",
        body: true
    );
    post!(
        doc: "Invoke action validateProperties",
        name: validate_properties,
        path: "/externalSponsors/validateProperties",
        body: true
    );
    delete!(
        doc: "Delete ref of navigation property externalSponsors for identityGovernance",
        name: delete_ref_external_sponsors,
        path: "/externalSponsors/{{id}}/$ref",
        params: external_sponsors_id
    );
}
