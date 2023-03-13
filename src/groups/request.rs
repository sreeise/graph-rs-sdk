// GENERATED CODE

use crate::api_default_imports::*;
use crate::group_lifecycle_policies::*;
use crate::groups::*;
use crate::planner::*;
use crate::sites::*;
use crate::users::*;

resource_api_client!(GroupsApiClient, GroupsIdApiClient, ResourceIdentity::Groups);

impl GroupsApiClient {
    post!(
        doc: "Create group",
        name: create_group,
        path: "/groups",
        body: true
    );
    get!(
        doc: "List groups",
        name: list_group,
        path: "/groups"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_groups_count,
        path: "/groups/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/groups/delta()"
    );
    post!(
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        path: "/groups/getAvailableExtensionProperties",
        body: true
    );
    post!(
        doc: "Invoke action getByIds",
        name: get_by_ids,
        path: "/groups/getByIds",
        body: true
    );
    post!(
        doc: "Invoke action validateProperties",
        name: validate_properties,
        path: "/groups/validateProperties",
        body: true
    );
}

impl GroupsIdApiClient {
    api_client_link!(threads, ThreadsApiClient);
    api_client_link!(group_lifecycle_policies, GroupLifecyclePoliciesApiClient);
    api_client_link!(sites, SitesApiClient);
    api_client_link!(default_calendar, DefaultCalendarApiClient);
    api_client_link_id!(event, EventsIdApiClient);
    api_client_link!(member_of, MemberOfApiClient);
    api_client_link_id!(calendar_view, CalendarViewIdApiClient);
    api_client_link_id!(thread, ThreadsIdApiClient);
    api_client_link!(planner, PlannerApiClient);
    api_client_link!(groups_team, GroupsTeamApiClient);
    api_client_link_id!(site, SitesIdApiClient);
    api_client_link!(conversations, ConversationsApiClient);
    api_client_link_id!(conversation, ConversationsIdApiClient);
    api_client_link!(events, EventsApiClient);
    api_client_link!(onenote, OnenoteApiClient);
    api_client_link!(calendar_views, CalendarViewApiClient);
    api_client_link_id!(member_of_id, MemberOfIdApiClient);
    api_client_link_id!(owners, GroupsOwnersApiClient);
    api_client_link_id!(owner, GroupsOwnersIdApiClient);

    delete!(
        doc: "Delete group",
        name: delete_group,
        path: "/groups/{{RID}}"
    );
    get!(
        doc: "Get group",
        name: get_group,
        path: "/groups/{{RID}}"
    );
    patch!(
        doc: "Update group",
        name: update_group,
        path: "/groups/{{RID}}",
        body: true
    );
    get!(
        doc: "List acceptedSenders",
        name: list_accepted_senders,
        path: "/groups/{{RID}}/acceptedSenders"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_accepted_senders_count,
        path: "/groups/{{RID}}/acceptedSenders/$count"
    );
    post!(
        doc: "Create new navigation property ref to acceptedSenders for groups",
        name: create_ref_accepted_senders,
        path: "/groups/{{RID}}/acceptedSenders/$ref",
        body: true
    );
    get!(
        doc: "List acceptedSenders",
        name: list_ref_accepted_senders,
        path: "/groups/{{RID}}/acceptedSenders/$ref"
    );
    delete!(
        doc: "Delete ref of navigation property acceptedSenders for groups",
        name: delete_ref_accepted_senders,
        path: "/groups/{{RID}}/acceptedSenders/{{id}}/$ref",
        params: directory_object_id
    );
    post!(
        doc: "Invoke action addFavorite",
        name: add_favorite,
        path: "/groups/{{RID}}/addFavorite"
    );
    post!(
        doc: "Invoke action assignLicense",
        name: assign_license,
        path: "/groups/{{RID}}/assignLicense",
        body: true
    );
    post!(
        doc: "Invoke action checkGrantedPermissionsForApp",
        name: check_granted_permissions_for_app,
        path: "/groups/{{RID}}/checkGrantedPermissionsForApp"
    );
    post!(
        doc: "Invoke action checkMemberGroups",
        name: check_member_groups,
        path: "/groups/{{RID}}/checkMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberObjects",
        name: check_member_objects,
        path: "/groups/{{RID}}/checkMemberObjects",
        body: true
    );
    get!(
        doc: "Get createdOnBehalfOf from groups",
        name: get_created_on_behalf_of,
        path: "/groups/{{RID}}/createdOnBehalfOf"
    );
    get!(
        doc: "Get Drive",
        name: get_drive,
        path: "/groups/{{RID}}/drive"
    );
    get!(
        doc: "List available drives",
        name: list_drives,
        path: "/groups/{{RID}}/drives"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_drives_count,
        path: "/groups/{{RID}}/drives/$count"
    );
    get!(
        doc: "Get drives from groups",
        name: get_drives,
        path: "/groups/{{RID}}/drives/{{id}}",
        params: drive_id
    );
    post!(
        doc: "Invoke action getMemberGroups",
        name: get_member_groups,
        path: "/groups/{{RID}}/getMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action getMemberObjects",
        name: get_member_objects,
        path: "/groups/{{RID}}/getMemberObjects",
        body: true
    );
    get!(
        doc: "Get members from groups",
        name: list_members,
        path: "/groups/{{RID}}/members"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_members_count,
        path: "/groups/{{RID}}/members/$count"
    );
    post!(
        doc: "Create new navigation property ref to members for groups",
        name: create_ref_members,
        path: "/groups/{{RID}}/members/$ref",
        body: true
    );
    get!(
        doc: "Get ref of members from groups",
        name: list_ref_members,
        path: "/groups/{{RID}}/members/$ref"
    );

    get!(
        doc: "Get the number of the resource",
        name: get_application_count,
        path: "/groups/{{RID}}/members/graph.application/$count"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_count,
        path: "/groups/{{RID}}/members/graph.device/$count"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_group_count,
        path: "/groups/{{RID}}/members/graph.group/$count"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_org_contact_count,
        path: "/groups/{{RID}}/members/graph.orgContact/$count"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_service_principal_count,
        path: "/groups/{{RID}}/members/graph.servicePrincipal/$count"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_user_count,
        path: "/groups/{{RID}}/members/graph.user/$count"
    );
    delete!(
        doc: "Delete ref of navigation property members for groups",
        name: delete_ref_members,
        path: "/groups/{{RID}}/members/{{id}}/$ref",
        params: directory_object_id
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.application",
        name: get_directory_object_item_as_application_type,
        path: "/groups/{{RID}}/members/{{id}}/graph.application",
        params: directory_object_id
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.device",
        name: get_directory_object_item_as_device_type,
        path: "/groups/{{RID}}/members/{{id}}/graph.device",
        params: directory_object_id
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.group",
        name: get_directory_object_item_as_group_type,
        path: "/groups/{{RID}}/members/{{id}}/graph.group",
        params: directory_object_id
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.orgContact",
        name: get_directory_object_item_as_org_contact_type,
        path: "/groups/{{RID}}/members/{{id}}/graph.orgContact",
        params: directory_object_id
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.servicePrincipal",
        name: get_directory_object_item_as_service_principal_type,
        path: "/groups/{{RID}}/members/{{id}}/graph.servicePrincipal",
        params: directory_object_id
    );
    get!(
        doc: "Get the item of type microsoft.graph.directoryObject as microsoft.graph.user",
        name: get_directory_object_item_as_user_type,
        path: "/groups/{{RID}}/members/{{id}}/graph.user",
        params: directory_object_id
    );
    get!(
        doc: "Get photo from groups",
        name: get_photo,
        path: "/groups/{{RID}}/photo"
    );
    patch!(
        doc: "Update the navigation property photo in groups",
        name: update_photo,
        path: "/groups/{{RID}}/photo",
        body: true
    );
    get!(
        doc: "Get media content for the navigation property photo from groups",
        name: get_photo_content,
        path: "/groups/{{RID}}/photo/$value"
    );
    put!(
        doc: "Update media content for the navigation property photo in groups",
        name: update_photo_content,
        path: "/groups/{{RID}}/photo/$value",
        body: true
    );
    get!(
        doc: "List rejectedSenders",
        name: list_rejected_senders,
        path: "/groups/{{RID}}/rejectedSenders"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_rejected_senders_count,
        path: "/groups/{{RID}}/rejectedSenders/$count"
    );
    post!(
        doc: "Create new navigation property ref to rejectedSenders for groups",
        name: create_ref_rejected_senders,
        path: "/groups/{{RID}}/rejectedSenders/$ref",
        body: true
    );
    get!(
        doc: "List rejectedSenders",
        name: list_ref_rejected_senders,
        path: "/groups/{{RID}}/rejectedSenders/$ref"
    );
    delete!(
        doc: "Delete ref of navigation property rejectedSenders for groups",
        name: delete_ref_rejected_senders,
        path: "/groups/{{RID}}/rejectedSenders/{{id}}/$ref",
        params: directory_object_id
    );
    post!(
        doc: "Invoke action removeFavorite",
        name: remove_favorite,
        path: "/groups/{{RID}}/removeFavorite"
    );
    post!(
        doc: "Invoke action renew",
        name: renew,
        path: "/groups/{{RID}}/renew"
    );
    post!(
        doc: "Invoke action resetUnseenCount",
        name: reset_unseen_count,
        path: "/groups/{{RID}}/resetUnseenCount"
    );
    post!(
        doc: "Invoke action restore",
        name: restore,
        path: "/groups/{{RID}}/restore"
    );
    post!(
        doc: "Create settings",
        name: create_settings,
        path: "/groups/{{RID}}/settings",
        body: true
    );
    get!(
        doc: "List settings",
        name: list_settings,
        path: "/groups/{{RID}}/settings"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_settings_count,
        path: "/groups/{{RID}}/settings/$count"
    );
    delete!(
        doc: "Delete navigation property settings for groups",
        name: delete_settings,
        path: "/groups/{{RID}}/settings/{{id}}",
        params: group_setting_id
    );
    get!(
        doc: "Get settings from groups",
        name: get_settings,
        path: "/groups/{{RID}}/settings/{{id}}",
        params: group_setting_id
    );
    patch!(
        doc: "Update the navigation property settings in groups",
        name: update_settings,
        path: "/groups/{{RID}}/settings/{{id}}",
        body: true,
        params: group_setting_id
    );
    post!(
        doc: "Invoke action subscribeByMail",
        name: subscribe_by_mail,
        path: "/groups/{{RID}}/subscribeByMail"
    );
    post!(
        doc: "Invoke action unsubscribeByMail",
        name: unsubscribe_by_mail,
        path: "/groups/{{RID}}/unsubscribeByMail"
    );
    post!(
        doc: "Invoke action validateProperties",
        name: validate_properties,
        path: "/groups/{{RID}}/validateProperties",
        body: true
    );
}
