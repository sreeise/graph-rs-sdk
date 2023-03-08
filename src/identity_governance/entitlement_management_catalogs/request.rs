// GENERATED CODE

use crate::api_default_imports::*;
use crate::identity_governance::{AccessPackagesApiClient, AccessPackagesIdApiClient};

resource_api_client!(
    EntitlementManagementCatalogsApiClient,
    EntitlementManagementCatalogsIdApiClient,
    ResourceIdentity::EntitlementManagementCatalogs
);

impl EntitlementManagementCatalogsApiClient {
    post!(
        doc: "Create accessPackageCatalog",
        name: create_catalogs,
        path: "/catalogs",
        body: true
    );
    get!(
        doc: "List catalogs",
        name: list_catalogs,
        path: "/catalogs"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_catalogs_count,
        path: "/catalogs/$count"
    );
}

impl EntitlementManagementCatalogsIdApiClient {
    api_client_link_id!(
        access_package,
        ResourceIdentity::AccessPackages,
        AccessPackagesIdApiClient
    );
    api_client_link!(
        access_packages,
        ResourceIdentity::AccessPackages,
        AccessPackagesApiClient
    );

    delete!(
        doc: "Delete navigation property catalogs for identityGovernance",
        name: delete_catalogs,
        path: "/catalogs/{{RID}}"
    );
    get!(
        doc: "Get catalogs from identityGovernance",
        name: get_catalogs,
        path: "/catalogs/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property catalogs in identityGovernance",
        name: update_catalogs,
        path: "/catalogs/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to accessPackages for identityGovernance",
        name: create_access_packages,
        path: "/catalogs/{{RID}}/accessPackages",
        body: true
    );
    get!(
        doc: "Get accessPackages from identityGovernance",
        name: list_access_packages,
        path: "/catalogs/{{RID}}/accessPackages"
    );
}
