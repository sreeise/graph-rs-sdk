// GENERATED CODE

use crate::agreement_acceptances::*;
use crate::api_default_imports::*;
use crate::chats::*;
use crate::default_drive::*;
use crate::oauth2_permission_grants::*;
use crate::planner::*;
use crate::teams::*;
use crate::users::*;

api_client!(MeApiClient, ResourceIdentity::Me);

impl MeApiClient {
    api_client_link_id!(message, UsersMessagesIdApiClient);
    api_client_link!(
        device_management_troubleshooting_events,
        DeviceManagementTroubleshootingEventsApiClient
    );
    api_client_link_id!(created_object, CreatedObjectsIdApiClient);
    api_client_link_id!(oauth2_permission_grant, Oauth2PermissionGrantsIdApiClient);
    api_client_link_id!(managed_device, ManagedDevicesIdApiClient);
    api_client_link!(outlook, OutlookApiClient);
    api_client_link!(planner, PlannerApiClient);
    api_client_link!(settings, SettingsApiClient);
    api_client_link_id!(
        device_management_troubleshooting_event,
        DeviceManagementTroubleshootingEventsIdApiClient
    );
    api_client_link_id!(contact, ContactsIdApiClient);
    api_client_link_id!(managed_app_registration, ManagedAppRegistrationsIdApiClient);
    api_client_link_id!(scoped_role_member_of_id, ScopedRoleMemberOfIdApiClient);
    api_client_link!(schedule, ScheduleApiClient);
    api_client_link_id!(activity, ActivitiesIdApiClient);
    api_client_link!(chats, ChatsApiClient);
    api_client_link_id!(joined_team, JoinedTeamsIdApiClient);
    api_client_link_id!(agreement_acceptance, AgreementAcceptancesIdApiClient);
    api_client_link!(direct_reports, DirectReportsApiClient);
    api_client_link!(registered_devices, RegisteredDevicesApiClient);
    api_client_link_id!(registered_device, RegisteredDevicesIdApiClient);
    api_client_link!(onenote, OnenoteApiClient);
    api_client_link!(online_meetings, OnlineMeetingsApiClient);
    api_client_link!(joined_teams, JoinedTeamsApiClient);
    api_client_link!(transitive_member_of, TransitiveMemberOfApiClient);
    api_client_link_id!(owned_object, OwnedObjectsIdApiClient);
    api_client_link!(contacts, ContactsApiClient);
    api_client_link!(scoped_role_member_of, ScopedRoleMemberOfApiClient);
    api_client_link!(inference_classification, InferenceClassificationApiClient);
    api_client_link!(presence, PresenceApiClient);
    api_client_link!(calendars, CalendarsApiClient);
    api_client_link!(authentication, AuthenticationApiClient);
    api_client_link_id!(calendar_view, CalendarViewIdApiClient);
    api_client_link!(todo, TodoApiClient);
    api_client_link_id!(direct_report, DirectReportsIdApiClient);
    api_client_link_id!(photo, PhotosIdApiClient);
    api_client_link_id!(member_of_id, MemberOfIdApiClient);
    api_client_link_id!(calendar, CalendarsIdApiClient);
    api_client_link!(owned_objects, OwnedObjectsApiClient);
    api_client_link_id!(transitive_member_of_id, TransitiveMemberOfIdApiClient);
    api_client_link!(calendar_views, CalendarViewApiClient);
    api_client_link_id!(owned_device, OwnedDevicesIdApiClient);
    api_client_link_id!(chat, ChatsIdApiClient);
    api_client_link_id!(mail_folder, MailFoldersIdApiClient);
    api_client_link!(teamwork, TeamworkApiClient);
    api_client_link_id!(app_role_assignment, AppRoleAssignmentsIdApiClient);
    api_client_link!(license_details, LicenseDetailsApiClient);
    api_client_link!(created_objects, CreatedObjectsApiClient);
    api_client_link!(photos, PhotosApiClient);
    api_client_link!(member_of, MemberOfApiClient);
    api_client_link_id!(extension, ExtensionsIdApiClient);
    api_client_link_id!(contact_folder, ContactFoldersIdApiClient);
    api_client_link!(app_role_assignments, AppRoleAssignmentsApiClient);
    api_client_link!(agreement_acceptances, AgreementAcceptancesApiClient);
    api_client_link_id!(calendar_group, CalendarGroupsIdApiClient);
    api_client_link!(managed_app_registrations, ManagedAppRegistrationsApiClient);
    api_client_link!(mail_folders, MailFoldersApiClient);
    api_client_link!(events, EventsApiClient);
    api_client_link!(messages, UsersMessagesApiClient);
    api_client_link_id!(online_meeting, OnlineMeetingsIdApiClient);
    api_client_link!(activities, ActivitiesApiClient);
    api_client_link!(contact_folders, ContactFoldersApiClient);
    api_client_link!(owned_devices, OwnedDevicesApiClient);
    api_client_link!(calendar_groups, CalendarGroupsApiClient);
    api_client_link_id!(event, EventsIdApiClient);
    api_client_link!(followed_sites, FollowedSitesApiClient);
    api_client_link_id!(channel, ChannelsIdApiClient);
    api_client_link!(extensions, ExtensionsApiClient);
    api_client_link!(default_calendar, DefaultCalendarApiClient);
    api_client_link!(managed_devices, ManagedDevicesApiClient);
    api_client_link_id!(license_detail, LicenseDetailsIdApiClient);
    api_client_link!(oauth2_permission_grants, Oauth2PermissionGrantsApiClient);
    api_client_link!(insights, InsightsApiClient);
    api_client_link!(channels, ChannelsApiClient);
    api_client_link!(drive, DefaultDriveApiClient);
    api_client_link!(mailbox_settings, MailboxSettingsApiClient);

    get!(
        doc: "List manager",
        name: get_user,
        path: "/me"
    );
    patch!(
        doc: "Update user",
        name: update_user,
        path: "/me",
        body: true
    );
    get!(
        doc: "List agreementAcceptances",
        name: list_agreement_acceptances,
        path: "/me/agreementAcceptances"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_agreement_acceptances_count,
        path: "/me/agreementAcceptances/$count"
    );
    get!(
        doc: "Get agreementAcceptances from me",
        name: get_agreement_acceptances,
        path: "/me/agreementAcceptances/{{id}}",
        params: agreement_acceptance_id
    );
    post!(
        doc: "Invoke action assignLicense",
        name: assign_license,
        path: "/me/assignLicense",
        body: true
    );
    post!(
        doc: "Invoke action changePassword",
        name: change_password,
        path: "/me/changePassword",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberGroups",
        name: check_member_groups,
        path: "/me/checkMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action checkMemberObjects",
        name: check_member_objects,
        path: "/me/checkMemberObjects",
        body: true
    );
    get!(
        doc: "Get Drive",
        name: get_drive,
        path: "/me/drive"
    );
    get!(
        doc: "List available drives",
        name: list_drives,
        path: "/me/drives"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_drives_count,
        path: "/me/drives/$count"
    );
    get!(
        doc: "Get drives from me",
        name: get_drives,
        path: "/me/drives/{{id}}",
        params: drive_id
    );
    get!(
        doc: "Invoke function exportDeviceAndAppManagementData",
        name: me_export_device_and_app_management_data_1a_02,
        path: "/me/exportDeviceAndAppManagementData()"
    );
    get!(
        doc: "Invoke function exportDeviceAndAppManagementData",
        name: me_export_device_and_app_management_data_fd_7c,
        path: "/me/exportDeviceAndAppManagementData(skip={{id}},top={{id2}})",
        params: skip, top
    );
    post!(
        doc: "Invoke action exportPersonalData",
        name: export_personal_data,
        path: "/me/exportPersonalData",
        body: true
    );
    post!(
        doc: "Invoke action findMeetingTimes",
        name: find_meeting_times,
        path: "/me/findMeetingTimes",
        body: true
    );
    post!(
        doc: "Invoke action getMailTips",
        name: get_mail_tips,
        path: "/me/getMailTips",
        body: true
    );
    get!(
        doc: "Invoke function getManagedAppDiagnosticStatuses",
        name: get_managed_app_diagnostic_statuses,
        path: "/me/getManagedAppDiagnosticStatuses()"
    );
    get!(
        doc: "Invoke function getManagedAppPolicies",
        name: get_managed_app_policies,
        path: "/me/getManagedAppPolicies()"
    );
    get!(
        doc: "Invoke function getManagedDevicesWithAppFailures",
        name: get_managed_devices_with_app_failures,
        path: "/me/getManagedDevicesWithAppFailures()"
    );
    post!(
        doc: "Invoke action getMemberGroups",
        name: get_member_groups,
        path: "/me/getMemberGroups",
        body: true
    );
    post!(
        doc: "Invoke action getMemberObjects",
        name: get_member_objects,
        path: "/me/getMemberObjects",
        body: true
    );
    get!(
        doc: "List manager",
        name: get_manager,
        path: "/me/manager"
    );
    delete!(
        doc: "Delete ref of navigation property manager for me",
        name: delete_ref_manager,
        path: "/me/manager/$ref"
    );
    get!(
        doc: "List manager",
        name: get_ref_manager,
        path: "/me/manager/$ref"
    );
    put!(
        doc: "Update the ref of navigation property manager in me",
        name: update_ref_manager,
        path: "/me/manager/$ref",
        body: true
    );
    get!(
        doc: "List people",
        name: list_people,
        path: "/me/people"
    );
    get!(
        doc: "Get the number of the resource",
        name: people_eaef,
        path: "/me/people/$count"
    );
    get!(
        doc: "Get people from me",
        name: get_people,
        path: "/me/people/{{id}}",
        params: person_id
    );
    get!(
        doc: "Get photo from me",
        name: get_photo,
        path: "/me/photo"
    );
    patch!(
        doc: "Update the navigation property photo in me",
        name: update_photo,
        path: "/me/photo",
        body: true
    );
    get!(
        doc: "Get media content for the navigation property photo from me",
        name: get_photo_content,
        path: "/me/photo/$value"
    );
    put!(
        doc: "Update media content for the navigation property photo in me",
        name: update_photo_content,
        path: "/me/photo/$value",
        body: true
    );
    delete!(
        doc: "Delete navigation property planner for me",
        name: delete_planner,
        path: "/me/planner"
    );
    get!(
        doc: "Get planner from me",
        name: get_planner,
        path: "/me/planner"
    );
    patch!(
        doc: "Update the navigation property planner in me",
        name: update_planner,
        path: "/me/planner",
        body: true
    );
    get!(
        doc: "Invoke function reminderView",
        name: reminder_view,
        path: "/me/reminderView(StartDateTime='{{id}}',EndDateTime='{{id2}}')",
        params: start_date_time, end_date_time
    );
    post!(
        doc: "Invoke action removeAllDevicesFromManagement",
        name: remove_all_devices_from_management,
        path: "/me/removeAllDevicesFromManagement"
    );
    post!(
        doc: "Invoke action reprocessLicenseAssignment",
        name: reprocess_license_assignment,
        path: "/me/reprocessLicenseAssignment"
    );
    post!(
        doc: "Invoke action restore",
        name: restore,
        path: "/me/restore"
    );
    post!(
        doc: "Invoke action revokeSignInSessions",
        name: revoke_sign_in_sessions,
        path: "/me/revokeSignInSessions"
    );
    post!(
        doc: "Invoke action sendMail",
        name: send_mail,
        path: "/me/sendMail",
        body: true
    );
    delete!(
        doc: "Delete navigation property settings for me",
        name: delete_settings,
        path: "/me/settings"
    );
    get!(
        doc: "Get settings from me",
        name: get_settings,
        path: "/me/settings"
    );
    patch!(
        doc: "Update the navigation property settings in me",
        name: update_settings,
        path: "/me/settings",
        body: true
    );
    delete!(
        doc: "Delete navigation property shiftPreferences for me",
        name: delete_shift_preferences,
        path: "/me/settings/shiftPreferences"
    );
    get!(
        doc: "Get shiftPreferences",
        name: get_shift_preferences,
        path: "/me/settings/shiftPreferences"
    );
    patch!(
        doc: "Update shiftPreferences",
        name: update_shift_preferences,
        path: "/me/settings/shiftPreferences",
        body: true
    );
    post!(
        doc: "Invoke action translateExchangeIds",
        name: translate_exchange_ids,
        path: "/me/translateExchangeIds",
        body: true
    );
    post!(
        doc: "Invoke action wipeManagedAppRegistrationsByDeviceTag",
        name: wipe_managed_app_registrations_by_device_tag,
        path: "/me/wipeManagedAppRegistrationsByDeviceTag",
        body: true
    );
}
