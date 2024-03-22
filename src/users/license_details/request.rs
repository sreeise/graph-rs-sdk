// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    LicenseDetailsApiClient,
    LicenseDetailsIdApiClient,
    ResourceIdentity::LicenseDetails
);

impl LicenseDetailsApiClient {
    post!(
        doc: "Create new navigation property to licenseDetails for users",
        name: create_license_details,
        path: "/licenseDetails",
        body: true
    );
    get!(
        doc: "List licenseDetails",
        name: list_license_details,
        path: "/licenseDetails"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_license_details_count,
        path: "/licenseDetails/$count"
    );
}

impl LicenseDetailsIdApiClient {
    delete!(
        doc: "Delete navigation property licenseDetails for users",
        name: delete_license_details,
        path: "/licenseDetails/{{RID}}"
    );
    get!(
        doc: "Get licenseDetails from users",
        name: get_license_details,
        path: "/licenseDetails/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property licenseDetails in users",
        name: update_license_details,
        path: "/licenseDetails/{{RID}}",
        body: true
    );
}
