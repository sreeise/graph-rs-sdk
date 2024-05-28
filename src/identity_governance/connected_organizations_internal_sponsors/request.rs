// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    ConnectedOrganizationsInternalSponsorsApiClient,
    ResourceIdentity::ConnectedOrganizationsInternalSponsors
);

impl ConnectedOrganizationsInternalSponsorsApiClient {
    post!(
        doc: "Create new navigation property to internalSponsors for identityGovernance",
        name: create_internal_sponsors,
        path: "/internalSponsors",
        body: true
    );
    get!(
        doc: "List internalSponsors",
        name: list_internal_sponsors,
        path: "/internalSponsors"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_internal_sponsors_count,
        path: "/internalSponsors/$count"
    );
    post!(
        doc: "Create new navigation property ref to internalSponsors for identityGovernance",
        name: create_ref_internal_sponsors,
        path: "/internalSponsors/$ref",
        body: true
    );
    get!(
        doc: "List internalSponsors",
        name: list_ref_internal_sponsors,
        path: "/internalSponsors/$ref"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/internalSponsors/delta()"
    );
    post!(
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        path: "/internalSponsors/getAvailableExtensionProperties",
        body: true
    );
    post!(
        doc: "Invoke action getByIds",
        name: get_by_ids,
        path: "/internalSponsors/getByIds",
        body: true
    );
    post!(
        doc: "Invoke action validateProperties",
        name: validate_properties,
        path: "/internalSponsors/validateProperties",
        body: true
    );
    delete!(
        doc: "Delete ref of navigation property internalSponsors for identityGovernance",
        name: delete_ref_internal_sponsors,
        path: "/internalSponsors/{{id}}/$ref",
        params: internal_sponsors_id
    );
}
