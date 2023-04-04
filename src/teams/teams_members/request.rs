// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    TeamsMembersApiClient,
    TeamsMembersIdApiClient,
    ResourceIdentity::TeamsMembers
);

impl TeamsMembersApiClient {
    post!(
        doc: "Add member to team",
        name: create_members,
        path: "/members",
        body: true
    );
    get!(
        doc: "List members of team",
        name: list_members,
        path: "/members"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_members_count,
        path: "/members/$count"
    );
    post!(
        doc: "Invoke action add",
        name: add,
        path: "/members/add",
        body: true
    );
}

impl TeamsMembersIdApiClient {
    delete!(
        doc: "Delete navigation property members for teams",
        name: delete_members,
        path: "/members/{{RID}}"
    );
    get!(
        doc: "Get members from teams",
        name: get_members,
        path: "/members/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property members in teams",
        name: update_members,
        path: "/members/{{RID}}",
        body: true
    );
}
