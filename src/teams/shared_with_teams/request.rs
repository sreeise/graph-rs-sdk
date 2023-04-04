// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    SharedWithTeamsApiClient,
    SharedWithTeamsIdApiClient,
    ResourceIdentity::SharedWithTeams
);

impl SharedWithTeamsApiClient {
    post!(
        doc: "Create new navigation property to sharedWithTeams for teams",
        name: create_shared_with_teams,
        path: "/sharedWithTeams",
        body: true
    );
    get!(
        doc: "List sharedWithChannelTeamInfo",
        name: list_shared_with_teams,
        path: "/sharedWithTeams"
    );
    get!(
        doc: "Get the number of the resource",
        name: count,
        path: "/sharedWithTeams/$count"
    );
}

impl SharedWithTeamsIdApiClient {
    delete!(
        doc: "Delete navigation property sharedWithTeams for teams",
        name: delete_shared_with_teams,
        path: "/sharedWithTeams/{{RID}}"
    );
    get!(
        doc: "Get sharedWithTeams from teams",
        name: get_shared_with_teams,
        path: "/sharedWithTeams/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property sharedWithTeams in teams",
        name: update_shared_with_teams,
        path: "/sharedWithTeams/{{RID}}",
        body: true
    );
    get!(
        doc: "List allowedMembers",
        name: list_allowed_members,
        path: "/sharedWithTeams/{{RID}}/allowedMembers"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_allowed_members_count,
        path: "/sharedWithTeams/{{RID}}/allowedMembers/$count"
    );
    get!(
        doc: "Get allowedMembers from teams",
        name: get_allowed_members,
        path: "/sharedWithTeams/{{RID}}/allowedMembers/{{id}}",
        params: conversation_member_id
    );
}
