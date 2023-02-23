// GENERATED CODE

use crate::api_default_imports::*;
use crate::assignment_policies::{AssignmentPoliciesApiClient, AssignmentPoliciesIdApiClient};

resource_api_client!(
    AccessPackagesApiClient,
    AccessPackagesIdApiClient,
    ResourceIdentity::AccessPackages
);

impl AccessPackagesApiClient {
    post!(
        doc: "Create accessPackage",
        name: create_access_packages,
        path: "/accessPackages",
        body: true
    );
    get!(
        doc: "List accessPackages",
        name: list_access_packages,
        path: "/accessPackages"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_access_packages_count,
        path: "/accessPackages/$count"
    );
    get!(
        doc: "Invoke function filterByCurrentUser",
        name: filter_by_current_user,
        path: "/accessPackages/microsoft.graph.filterByCurrentUser(on='{{id}}')",
        params: on
    );
}

impl AccessPackagesIdApiClient {
    api_client_link_id!(
        assignment_policy,
        ResourceIdentity::AssignmentPolicies,
        AssignmentPoliciesIdApiClient
    );
    api_client_link!(
        assignment_policies,
        ResourceIdentity::AssignmentPolicies,
        AssignmentPoliciesApiClient
    );

    delete!(
        doc: "Delete navigation property accessPackages for identityGovernance",
        name: delete_access_packages,
        path: "/accessPackages/{{RID}}"
    );
    get!(
        doc: "Get accessPackages from identityGovernance",
        name: get_access_packages,
        path: "/accessPackages/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property accessPackages in identityGovernance",
        name: update_access_packages,
        path: "/accessPackages/{{RID}}",
        body: true
    );
    get!(
        doc: "List accessPackagesIncompatibleWith",
        name: list_access_packages_incompatible_with,
        path: "/accessPackages/{{RID}}/accessPackagesIncompatibleWith"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_access_packages_incompatible_with_count,
        path: "/accessPackages/{{RID}}/accessPackagesIncompatibleWith/$count"
    );
    get!(
        doc: "Get accessPackagesIncompatibleWith from identityGovernance",
        name: get_access_packages_incompatible_with,
        path: "/accessPackages/{{RID}}/accessPackagesIncompatibleWith/{{id}}",
        params: access_package_id_1
    );
    get!(
        doc: "Get catalog from identityGovernance",
        name: get_catalog,
        path: "/accessPackages/{{RID}}/catalog"
    );
    get!(
        doc: "List incompatibleAccessPackages",
        name: list_incompatible_access_packages,
        path: "/accessPackages/{{RID}}/incompatibleAccessPackages"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_incompatible_access_packages_count,
        path: "/accessPackages/{{RID}}/incompatibleAccessPackages/$count"
    );
    post!(
        doc: "Create new navigation property ref to incompatibleAccessPackages for identityGovernance",
        name: create_ref_incompatible_access_packages,
        path: "/accessPackages/{{RID}}/incompatibleAccessPackages/$ref",
        body: true
    );
    get!(
        doc: "List incompatibleAccessPackages",
        name: list_ref_incompatible_access_packages,
        path: "/accessPackages/{{RID}}/incompatibleAccessPackages/$ref"
    );
    delete!(
        doc: "Delete ref of navigation property incompatibleAccessPackages for identityGovernance",
        name: delete_ref_incompatible_access_packages,
        path: "/accessPackages/{{RID}}/incompatibleAccessPackages/{{id}}/$ref",
        params: access_package_id_1
    );
    get!(
        doc: "List incompatibleGroups",
        name: list_incompatible_groups,
        path: "/accessPackages/{{RID}}/incompatibleGroups"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_incompatible_groups_count,
        path: "/accessPackages/{{RID}}/incompatibleGroups/$count"
    );
    post!(
        doc: "Create new navigation property ref to incompatibleGroups for identityGovernance",
        name: create_ref_incompatible_groups,
        path: "/accessPackages/{{RID}}/incompatibleGroups/$ref",
        body: true
    );
    get!(
        doc: "List incompatibleGroups",
        name: list_ref_incompatible_groups,
        path: "/accessPackages/{{RID}}/incompatibleGroups/$ref"
    );
    delete!(
        doc: "Delete ref of navigation property incompatibleGroups for identityGovernance",
        name: delete_ref_incompatible_groups,
        path: "/accessPackages/{{RID}}/incompatibleGroups/{{id}}/$ref",
        params: group_id
    );
    post!(
        doc: "Invoke action getApplicablePolicyRequirements",
        name: get_applicable_policy_requirements,
        path: "/accessPackages/{{RID}}/microsoft.graph.getApplicablePolicyRequirements"
    );
}
