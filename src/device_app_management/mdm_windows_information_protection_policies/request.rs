// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    MdmWindowsInformationProtectionPoliciesApiClient,
    MdmWindowsInformationProtectionPoliciesIdApiClient,
    ResourceIdentity::MdmWindowsInformationProtectionPolicies
);

impl MdmWindowsInformationProtectionPoliciesApiClient {
    post!(
        doc: "Create new navigation property to mdmWindowsInformationProtectionPolicies for deviceAppManagement",
        name: create_mdm_windows_information_protection_policies,
        path: "/mdmWindowsInformationProtectionPolicies",
        body: true
    );
    get!(
        doc: "Get mdmWindowsInformationProtectionPolicies from deviceAppManagement",
        name: list_mdm_windows_information_protection_policies,
        path: "/mdmWindowsInformationProtectionPolicies"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_mdm_windows_information_protection_policies_count,
        path: "/mdmWindowsInformationProtectionPolicies/$count"
    );
}

impl MdmWindowsInformationProtectionPoliciesIdApiClient {
    delete!(
        doc: "Delete navigation property mdmWindowsInformationProtectionPolicies for deviceAppManagement",
        name: delete_mdm_windows_information_protection_policies,
        path: "/mdmWindowsInformationProtectionPolicies/{{RID}}"
    );
    get!(
        doc: "Get mdmWindowsInformationProtectionPolicies from deviceAppManagement",
        name: get_mdm_windows_information_protection_policies,
        path: "/mdmWindowsInformationProtectionPolicies/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property mdmWindowsInformationProtectionPolicies in deviceAppManagement",
        name: update_mdm_windows_information_protection_policies,
        path: "/mdmWindowsInformationProtectionPolicies/{{RID}}",
        body: true
    );
}
