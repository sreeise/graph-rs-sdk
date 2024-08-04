// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    ScopedRoleMemberOfApiClient,
    ScopedRoleMemberOfIdApiClient,
    ResourceIdentity::ScopedRoleMemberOf
);

impl ScopedRoleMemberOfApiClient {
    post!(
        doc: "Create new navigation property to scopedRoleMemberOf for users",
        name: create_scoped_role_member_of,
        path: "/scopedRoleMemberOf",
        body: true
    );
    get!(
        doc: "Get scopedRoleMemberOf from users",
        name: list_scoped_role_member_of,
        path: "/scopedRoleMemberOf"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_scoped_role_member_of_count,
        path: "/scopedRoleMemberOf/$count"
    );
}

impl ScopedRoleMemberOfIdApiClient {
    delete!(
        doc: "Delete navigation property scopedRoleMemberOf for users",
        name: delete_scoped_role_member_of,
        path: "/scopedRoleMemberOf/{{RID}}"
    );
    get!(
        doc: "Get scopedRoleMemberOf from users",
        name: get_scoped_role_member_of,
        path: "/scopedRoleMemberOf/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property scopedRoleMemberOf in users",
        name: update_scoped_role_member_of,
        path: "/scopedRoleMemberOf/{{RID}}",
        body: true
    );
}
