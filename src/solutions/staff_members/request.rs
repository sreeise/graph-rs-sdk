// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    StaffMembersApiClient,
    StaffMembersIdApiClient,
    ResourceIdentity::StaffMembers
);

impl StaffMembersApiClient {
    post!(
        doc: "Create bookingStaffMember",
        name: create_staff_members,
        path: "/staffMembers",
        body: true
    );
    get!(
        doc: "List staffMembers",
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
        doc: "Delete bookingStaffMember",
        name: delete_staff_members,
        path: "/staffMembers/{{RID}}"
    );
    get!(
        doc: "Get bookingStaffMember",
        name: get_staff_members,
        path: "/staffMembers/{{RID}}"
    );
    patch!(
        doc: "Update bookingstaffmember",
        name: update_staff_members,
        path: "/staffMembers/{{RID}}",
        body: true
    );
}
