use crate::api_default_imports::*;
use crate::calendar::{CalendarApiClient, CalendarIdApiClient};
use crate::calendar_groups::{CalendarGroupsApiClient, CalendarGroupsIdApiClient};
use crate::calendar_view::{CalendarViewApiClient, CalendarViewIdApiClient};
use crate::conversations::{ConversationsApiClient, ConversationsIdApiClient};
use crate::drive::DriveApiClient;
use crate::events::{EventsApiClient, EventsIdApiClient};
use crate::onenote::OnenoteApiClient;
use crate::planner::PlannerApiClient;
use crate::threads::{ThreadsApiClient, ThreadsIdApiClient};

resource_api_client!(GroupsApiClient, GroupsIdApiClient, ResourceIdentity::Groups);

impl GroupsApiClient {
    get!({
        doc: "# Get entities from groups",
        name: list_group,
        response: serde_json::Value,
        path: "/groups",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Add new entity to groups",
        name: create_group,
        response: serde_json::Value,
        path: "/groups",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/groups/delta()",
        params: 0,
        has_body: false
    });
}

impl GroupsIdApiClient {
    get!({
        doc: "# Get entity from groups by key",
        name: get_group,
        response: serde_json::Value,
        path: "/groups/{{RID}}",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update entity in groups",
        name: update_group,
        response: NoContent,
        path: "/groups/{{RID}}",
        params: 0,
        has_body: true
    });

    delete!({
        doc: "# Delete entity from groups",
        name: delete_group,
        response: NoContent,
        path: "/groups/{{RID}}",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get acceptedSenders from groups",
        name: list_accepted_senders,
        response: serde_json::Value,
        path: "/groups/{{RID}}/acceptedSenders",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to acceptedSenders for groups",
        name: create_accepted_senders,
        response: serde_json::Value,
        path: "/groups/{{RID}}/acceptedSenders",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get acceptedSenders from groups",
        name: get_accepted_senders,
        response: serde_json::Value,
        path: "/groups/{{RID}}/acceptedSenders/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property acceptedSenders in groups",
        name: update_accepted_senders,
        response: NoContent,
        path: "/groups/{{RID}}/acceptedSenders/{{id}}",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action addFavorite",
        name: add_favorite,
        response: NoContent,
        path: "/groups/{{RID}}/addFavorite",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get appRoleAssignments from groups",
        name: list_app_role_assignments,
        response: serde_json::Value,
        path: "/groups/{{RID}}/appRoleAssignments",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to appRoleAssignments for groups",
        name: create_app_role_assignments,
        response: serde_json::Value,
        path: "/groups/{{RID}}/appRoleAssignments",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get appRoleAssignments from groups",
        name: get_app_role_assignments,
        response: serde_json::Value,
        path: "/groups/{{RID}}/appRoleAssignments/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property appRoleAssignments in groups",
        name: update_app_role_assignments,
        response: NoContent,
        path: "/groups/{{RID}}/appRoleAssignments/{{id}}",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action assignLicense",
        name: assign_license,
        response: serde_json::Value,
        path: "/groups/{{RID}}/assignLicense",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get calendar from groups",
        name: get_calendar,
        response: serde_json::Value,
        path: "/groups/{{RID}}/calendar",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property calendar in groups",
        name: update_calendar,
        response: NoContent,
        path: "/groups/{{RID}}/calendar",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get calendarView from groups",
        name: list_calendar_view,
        response: serde_json::Value,
        path: "/groups/{{RID}}/calendarView",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to calendarView for groups",
        name: create_calendar_view,
        response: serde_json::Value,
        path: "/groups/{{RID}}/calendarView",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get conversations from groups",
        name: list_conversations,
        response: serde_json::Value,
        path: "/groups/{{RID}}/conversations",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to conversations for groups",
        name: create_conversations,
        response: serde_json::Value,
        path: "/groups/{{RID}}/conversations",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get createdOnBehalfOf from groups",
        name: get_created_on_behalf_of,
        response: serde_json::Value,
        path: "/groups/{{RID}}/createdOnBehalfOf",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get drive from groups",
        name: get_drive,
        response: serde_json::Value,
        path: "/groups/{{RID}}/drive",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property drive in groups",
        name: update_drive,
        response: NoContent,
        path: "/groups/{{RID}}/drive",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get drives from groups",
        name: list_drives,
        response: serde_json::Value,
        path: "/groups/{{RID}}/drives",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to drives for groups",
        name: create_drives,
        response: serde_json::Value,
        path: "/groups/{{RID}}/drives",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get drives from groups",
        name: get_drives,
        response: serde_json::Value,
        path: "/groups/{{RID}}/drives/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property drives in groups",
        name: update_drives,
        response: NoContent,
        path: "/groups/{{RID}}/drives/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get events from groups",
        name: list_events,
        response: serde_json::Value,
        path: "/groups/{{RID}}/events",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to events for groups",
        name: create_events,
        response: serde_json::Value,
        path: "/groups/{{RID}}/events",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get extensions from groups",
        name: list_extensions,
        response: serde_json::Value,
        path: "/groups/{{RID}}/extensions",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to extensions for groups",
        name: create_extensions,
        response: serde_json::Value,
        path: "/groups/{{RID}}/extensions",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get extensions from groups",
        name: get_extensions,
        response: serde_json::Value,
        path: "/groups/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property extensions in groups",
        name: update_extensions,
        response: NoContent,
        path: "/groups/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get groupLifecyclePolicies from groups",
        name: list_group_lifecycle_policies,
        response: serde_json::Value,
        path: "/groups/{{RID}}/groupLifecyclePolicies",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to groupLifecyclePolicies for groups",
        name: create_group_lifecycle_policies,
        response: serde_json::Value,
        path: "/groups/{{RID}}/groupLifecyclePolicies",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get groupLifecyclePolicies from groups",
        name: get_group_lifecycle_policies,
        response: serde_json::Value,
        path: "/groups/{{RID}}/groupLifecyclePolicies/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property groupLifecyclePolicies in groups",
        name: update_group_lifecycle_policies,
        response: NoContent,
        path: "/groups/{{RID}}/groupLifecyclePolicies/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get memberOf from groups",
        name: list_member_of,
        response: serde_json::Value,
        path: "/groups/{{RID}}/memberOf",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get memberOf from groups",
        name: get_member_of,
        response: serde_json::Value,
        path: "/groups/{{RID}}/memberOf/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get members from groups",
        name: list_members,
        response: serde_json::Value,
        path: "/groups/{{RID}}/members",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get members from groups",
        name: get_members,
        response: serde_json::Value,
        path: "/groups/{{RID}}/members/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get membersWithLicenseErrors from groups",
        name: list_members_with_license_errors,
        response: serde_json::Value,
        path: "/groups/{{RID}}/membersWithLicenseErrors",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get membersWithLicenseErrors from groups",
        name: get_members_with_license_errors,
        response: serde_json::Value,
        path: "/groups/{{RID}}/membersWithLicenseErrors/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get onenote from groups",
        name: get_onenote,
        response: serde_json::Value,
        path: "/groups/{{RID}}/onenote",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property onenote in groups",
        name: update_onenote,
        response: NoContent,
        path: "/groups/{{RID}}/onenote",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get owners from groups",
        name: list_owners,
        response: serde_json::Value,
        path: "/groups/{{RID}}/owners",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get owners from groups",
        name: get_owners,
        response: serde_json::Value,
        path: "/groups/{{RID}}/owners/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get photo from groups",
        name: get_photo,
        response: serde_json::Value,
        path: "/groups/{{RID}}/photo",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property photo in groups",
        name: update_photo,
        response: NoContent,
        path: "/groups/{{RID}}/photo",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get photos from groups",
        name: list_photos,
        response: serde_json::Value,
        path: "/groups/{{RID}}/photos",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to photos for groups",
        name: create_photos,
        response: serde_json::Value,
        path: "/groups/{{RID}}/photos",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get photos from groups",
        name: get_photos,
        response: serde_json::Value,
        path: "/groups/{{RID}}/photos/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property photos in groups",
        name: update_photos,
        response: NoContent,
        path: "/groups/{{RID}}/photos/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get planner from groups",
        name: get_planner,
        response: serde_json::Value,
        path: "/groups/{{RID}}/planner",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property planner in groups",
        name: update_planner,
        response: NoContent,
        path: "/groups/{{RID}}/planner",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get rejectedSenders from groups",
        name: list_rejected_senders,
        response: serde_json::Value,
        path: "/groups/{{RID}}/rejectedSenders",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to rejectedSenders for groups",
        name: create_rejected_senders,
        response: serde_json::Value,
        path: "/groups/{{RID}}/rejectedSenders",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get rejectedSenders from groups",
        name: get_rejected_senders,
        response: serde_json::Value,
        path: "/groups/{{RID}}/rejectedSenders/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property rejectedSenders in groups",
        name: update_rejected_senders,
        response: NoContent,
        path: "/groups/{{RID}}/rejectedSenders/{{id}}",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action removeFavorite",
        name: remove_favorite,
        response: NoContent,
        path: "/groups/{{RID}}/removeFavorite",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Invoke action renew",
        name: renew,
        response: NoContent,
        path: "/groups/{{RID}}/renew",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Invoke action resetUnseenCount",
        name: reset_unseen_count,
        response: NoContent,
        path: "/groups/{{RID}}/resetUnseenCount",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get settings from groups",
        name: list_settings,
        response: serde_json::Value,
        path: "/groups/{{RID}}/settings",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to settings for groups",
        name: create_settings,
        response: serde_json::Value,
        path: "/groups/{{RID}}/settings",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get settings from groups",
        name: get_settings,
        response: serde_json::Value,
        path: "/groups/{{RID}}/settings/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property settings in groups",
        name: update_settings,
        response: NoContent,
        path: "/groups/{{RID}}/settings/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get sites from groups",
        name: list_sites,
        response: serde_json::Value,
        path: "/groups/{{RID}}/sites",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to sites for groups",
        name: create_sites,
        response: serde_json::Value,
        path: "/groups/{{RID}}/sites",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get sites from groups",
        name: get_sites,
        response: serde_json::Value,
        path: "/groups/{{RID}}/sites/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property sites in groups",
        name: update_sites,
        response: NoContent,
        path: "/groups/{{RID}}/sites/{{id}}",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action subscribeByMail",
        name: subscribe_by_mail,
        response: NoContent,
        path: "/groups/{{RID}}/subscribeByMail",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get team from groups",
        name: get_team,
        response: serde_json::Value,
        path: "/groups/{{RID}}/team",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property team in groups",
        name: update_team,
        response: NoContent,
        path: "/groups/{{RID}}/team",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get threads from groups",
        name: list_threads,
        response: serde_json::Value,
        path: "/groups/{{RID}}/threads",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to threads for groups",
        name: create_threads,
        response: serde_json::Value,
        path: "/groups/{{RID}}/threads",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get transitiveMemberOf from groups",
        name: list_transitive_member_of,
        response: serde_json::Value,
        path: "/groups/{{RID}}/transitiveMemberOf",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get transitiveMemberOf from groups",
        name: get_transitive_member_of,
        response: serde_json::Value,
        path: "/groups/{{RID}}/transitiveMemberOf/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get transitiveMembers from groups",
        name: list_transitive_members,
        response: serde_json::Value,
        path: "/groups/{{RID}}/transitiveMembers",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get transitiveMembers from groups",
        name: get_transitive_members,
        response: serde_json::Value,
        path: "/groups/{{RID}}/transitiveMembers/{{id}}",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action unsubscribeByMail",
        name: unsubscribe_by_mail,
        response: NoContent,
        path: "/groups/{{RID}}/unsubscribeByMail",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Invoke action validateProperties",
        name: validate_properties,
        response: NoContent,
        path: "/groups/{{RID}}/validateProperties",
        params: 0,
        has_body: true
    });

    post!({
        name: add_member,
        response: NoContent,
        path: "groups/{{RID}}/members/$ref",
        params: 0,
        has_body: true
    });

    delete!({
        name: remove_member,
        response: NoContent,
        path: "groups/{{RID}}/members/{{id}}/$ref",
        params: 1,
        has_body: false
    });

    post!({
        name: add_owner,
        response: NoContent,
        path: "groups/{{RID}}/owners/$ref",
        params: 0,
        has_body: true
    });

    delete!({
        name: remove_owner,
        response: NoContent,
        path: "groups/{{RID}}/owners/{{id}}/$ref",
        params: 1,
        has_body: false
    });

    api_client_link!(calendars, ResourceIdentity::Calendar, CalendarApiClient);

    api_client_link!(calendar, ResourceIdentity::Calendar, CalendarIdApiClient);

    api_client_link!(
        calendar_groups,
        ResourceIdentity::CalendarGroups,
        CalendarGroupsApiClient
    );

    api_client_link!(
        calendar_group,
        ResourceIdentity::CalendarGroups,
        CalendarGroupsIdApiClient
    );

    api_client_link!(
        calendar_views,
        ResourceIdentity::CalendarView,
        CalendarViewApiClient
    );

    api_client_link_id!(
        calendar_view,
        ResourceIdentity::CalendarView,
        CalendarViewIdApiClient
    );

    api_client_link!(
        conversations,
        ResourceIdentity::Conversations,
        ConversationsApiClient
    );

    api_client_link_id!(
        conversation,
        ResourceIdentity::Conversations,
        ConversationsIdApiClient
    );

    api_client_link!(drive, ResourceIdentity::Drive, DriveApiClient);

    api_client_link!(events, ResourceIdentity::Events, EventsApiClient);

    api_client_link_id!(event, ResourceIdentity::Events, EventsIdApiClient);

    api_client_link!(onenote, ResourceIdentity::Onenote, OnenoteApiClient);

    api_client_link!(planner, ResourceIdentity::Planner, PlannerApiClient);

    api_client_link!(threads, ResourceIdentity::Threads, ThreadsApiClient);

    api_client_link_id!(thread, ResourceIdentity::Threads, ThreadsIdApiClient);
}
