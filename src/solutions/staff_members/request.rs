// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    StaffMembersApiClient,
    StaffMembersIdApiClient,
    ResourceIdentity::StaffMembers
);

impl StaffMembersApiClient {
    post!(
        doc: "Create new navigation property to staffMembers for solutions",
        name: create_staff_members,
        path: "/staffMembers",
        body: true
    );
    get!(
        doc: "Get staffMembers from solutions",
        name: list_staff_members,
        path: "/staffMembers"
    );
    get!(
        doc: "Get the number of the resource",
        name: staff_members,
        path: "/staffMembers/$count"
    );
}

impl StaffMembersIdApiClient {
    delete!(
        doc: "Delete navigation property staffMembers for solutions",
        name: delete_staff_members,
        path: "/staffMembers/{{RID}}"
    );
    get!(
        doc: "Get staffMembers from solutions",
        name: get_staff_members,
        path: "/staffMembers/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property staffMembers in solutions",
        name: update_staff_members,
        path: "/staffMembers/{{RID}}",
        body: true
    );
}
