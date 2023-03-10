// GENERATED CODE

use crate::api_default_imports::*;
use crate::chats::*;
use crate::teams::*;

resource_api_client!(PrimaryChannelApiClient, ResourceIdentity::PrimaryChannel);

impl PrimaryChannelApiClient {
    api_client_link!(shared_with_teams, SharedWithTeamsApiClient);
    api_client_link!(member, TeamsMembersIdApiClient);
    api_client_link_id!(message, MessagesIdApiClient);
    api_client_link_id!(shared_with_team, SharedWithTeamsIdApiClient);
    api_client_link!(messages, MessagesApiClient);
    api_client_link!(members, TeamsMembersApiClient);

    delete!(
        doc: "Delete navigation property primaryChannel for teams",
        name: delete_primary_channel,
        path: "/primaryChannel"
    );
    get!(
        doc: "Get primaryChannel",
        name: get_primary_channel,
        path: "/primaryChannel"
    );
    patch!(
        doc: "Update the navigation property primaryChannel in teams",
        name: update_primary_channel,
        path: "/primaryChannel",
        body: true
    );
    post!(
        doc: "Invoke action completeMigration",
        name: complete_migration,
        path: "/primaryChannel/completeMigration"
    );
    get!(
        doc: "Invoke function doesUserHaveAccess",
        name: does_user_have_access,
        path: "/primaryChannel/doesUserHaveAccess(userId='@userId',tenantId='@tenantId',userPrincipalName='@userPrincipalName')"
    );
    get!(
        doc: "Get filesFolder",
        name: get_files_folder,
        path: "/primaryChannel/filesFolder"
    );
    get!(
        doc: "Get content for the navigation property filesFolder from teams",
        name: get_files_folder_content,
        path: "/primaryChannel/filesFolder/content"
    );
    put!(
        doc: "Update content for the navigation property filesFolder in teams",
        name: update_files_folder_content,
        path: "/primaryChannel/filesFolder/content",
        body: true
    );
    post!(
        doc: "Invoke action provisionEmail",
        name: provision_email,
        path: "/primaryChannel/provisionEmail"
    );
    post!(
        doc: "Invoke action removeEmail",
        name: remove_email,
        path: "/primaryChannel/removeEmail"
    );
}
