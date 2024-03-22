// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    WindowsInformationProtectionPoliciesApiClient,
    WindowsInformationProtectionPoliciesIdApiClient,
    ResourceIdentity::WindowsInformationProtectionPolicies
);

impl WindowsInformationProtectionPoliciesApiClient {
    post!(
        doc: "Create new navigation property to windowsInformationProtectionPolicies for deviceAppManagement",
        name: create_windows_information_protection_policies,
        path: "/windowsInformationProtectionPolicies",
        body: true
    );
    get!(
        doc: "Get windowsInformationProtectionPolicies from deviceAppManagement",
        name: list_windows_information_protection_policies,
        path: "/windowsInformationProtectionPolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_windows_information_protection_policies_count,
        path: "/windowsInformationProtectionPolicies/$count"
    );
}

impl WindowsInformationProtectionPoliciesIdApiClient {
    delete!(
        doc: "Delete navigation property windowsInformationProtectionPolicies for deviceAppManagement",
        name: delete_windows_information_protection_policies,
        path: "/windowsInformationProtectionPolicies/{{RID}}"
    );
    get!(
        doc: "Get windowsInformationProtectionPolicies from deviceAppManagement",
        name: get_windows_information_protection_policies,
        path: "/windowsInformationProtectionPolicies/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property windowsInformationProtectionPolicies in deviceAppManagement",
        name: update_windows_information_protection_policies,
        path: "/windowsInformationProtectionPolicies/{{RID}}",
        body: true
    );
}
