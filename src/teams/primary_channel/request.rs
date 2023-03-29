// GENERATED CODE

use crate::api_default_imports::*;
use crate::teams::*;
use crate::chats::*;

resource_api_client!(PrimaryChannelApiClient, ResourceIdentity::PrimaryChannel);

impl PrimaryChannelApiClient {api_client_link!(members, TeamsMembersApiClient);
api_client_link_id!(message, ChatsMessagesIdApiClient);
api_client_link!(member, TeamsMembersIdApiClient);
api_client_link!(shared_with_teams, SharedWithTeamsApiClient);
api_client_link!(messages, ChatsMessagesApiClient);
api_client_link_id!(shared_with_team, SharedWithTeamsIdApiClient);

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
	post!(
		doc: "Create new navigation property to tabs for teams",
		name: create_tabs,
		path: "/primaryChannel/tabs",
		body: true
	);
	get!(
		doc: "List tabs in channel",
		name: list_tabs,
		path: "/primaryChannel/tabs"
	);
	get!(
		doc: "Get the number of the resource",
		name: get_tabs_count,
		path: "/primaryChannel/tabs/$count"
	);
	delete!(
		doc: "Delete navigation property tabs for teams",
		name: delete_tabs,
		path: "/primaryChannel/tabs/{{id}}",
		params: teams_tab_id
	);
	get!(
		doc: "Get tabs from teams",
		name: get_tabs,
		path: "/primaryChannel/tabs/{{id}}",
		params: teams_tab_id
	);
	patch!(
		doc: "Update the navigation property tabs in teams",
		name: update_tabs,
		path: "/primaryChannel/tabs/{{id}}",
		body: true,
		params: teams_tab_id
	);
	get!(
		doc: "Get teamsApp from teams",
		name: get_tabs_teams_app,
		path: "/primaryChannel/tabs/{{id}}/teamsApp",
		params: teams_tab_id
	);
}
