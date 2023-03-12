// GENERATED CODE

use crate::api_default_imports::*;
use crate::users::*;

resource_api_client!(
    DeletedTeamsApiClient,
    DeletedTeamsIdApiClient,
    ResourceIdentity::DeletedTeams
);

impl DeletedTeamsApiClient {
    post!(
        doc: "Create new navigation property to deletedTeams for teamwork",
        name: create_deleted_teams,
        path: "/deletedTeams",
        body: true
    );
    get!(
        doc: "Get deletedTeams from teamwork",
        name: list_deleted_teams,
        path: "/deletedTeams"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_deleted_teams_count,
        path: "/deletedTeams/$count"
    );
    get!(
        doc: "Invoke function getAllMessages",
        name: get_all_messages,
        path: "/deletedTeams/getAllMessages()"
    );
}

impl DeletedTeamsIdApiClient {
    api_client_link!(channels, ChannelsApiClient);
    api_client_link_id!(channel, ChannelsIdApiClient);

    delete!(
        doc: "Delete navigation property deletedTeams for teamwork",
        name: delete_deleted_teams,
        path: "/deletedTeams/{{RID}}"
    );
    get!(
        doc: "Get deletedTeams from teamwork",
        name: get_deleted_teams,
        path: "/deletedTeams/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property deletedTeams in teamwork",
        name: update_deleted_teams,
        path: "/deletedTeams/{{RID}}",
        body: true
    );
}
