// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(MeApiClient, ResourceIdentity::Me);

use crate::activities::{ActivitiesApiClient, ActivitiesIdApiClient};
use crate::agreement_acceptances::{
    AgreementAcceptancesApiClient, AgreementAcceptancesIdApiClient,
};
use crate::calendar::{CalendarApiClient, CalendarIdApiClient};
use crate::calendar_groups::{CalendarGroupsApiClient, CalendarGroupsIdApiClient};
use crate::calendar_view::{CalendarViewApiClient, CalendarViewIdApiClient};
use crate::contact_folders::{ContactFoldersApiClient, ContactFoldersIdApiClient};
use crate::contacts::{ContactsApiClient, ContactsIdApiClient};
use crate::drive::DriveApiClient;
use crate::education::EducationMeApiClient;
use crate::events::{EventsApiClient, EventsIdApiClient};
use crate::inference_classification::InferenceClassificationApiClient;
use crate::insights::InsightsApiClient;
use crate::mail_folders::{MailFoldersApiClient, MailFoldersIdApiClient};
use crate::managed_devices::{ManagedDevicesApiClient, ManagedDevicesIdApiClient};
use crate::messages::{MessagesApiClient, MessagesIdApiClient};
use crate::onenote::OnenoteApiClient;
use crate::outlook::OutlookApiClient;
use crate::planner::PlannerApiClient;
use crate::settings::SettingsApiClient;

impl MeApiClient {
    get!(
        doc: "# Get me",
        name: get_user,
        path: "/me"
    );

    patch!(
        doc: "# Update me",
        name: update_user,
        path: "/me",
        body: true
    );

    get!(
        doc: "# Get appRoleAssignments from me",
        name: list_app_role_assignments,
        path: "/me/appRoleAssignments"
    );

    post!(
        doc: "# Create new navigation property to appRoleAssignments for me",
        name: create_app_role_assignments,
        path: "/me/appRoleAssignments",
        body: true
    );

    get!(
        doc: "# Get appRoleAssignments from me",
        name: get_app_role_assignments,
        path: "/me/appRoleAssignments/{{id}}",
        params: id
    );

    patch!(
        doc: "# Update the navigation property appRoleAssignments in me",
        name: update_app_role_assignments,
        path: "/me/appRoleAssignments/{{id}}",
        body: true,
        params: id
    );

    post!(
        doc: "# Invoke action assignLicense",
        name: assign_license,
        path: "/me/assignLicense",
        body: true
    );

    post!(
        doc: "# Invoke action changePassword",
        name: change_password,
        path: "/me/changePassword",
        body: true
    );

    get!(
        doc: "# Get createdObjects from me",
        name: list_created_objects,
        path: "/me/createdObjects"
    );

    get!(
        doc: "# Get createdObjects from me",
        name: get_created_objects,
        path: "/me/createdObjects/{{id}}",
        params: id
    );

    get!(
        doc: "# Get deviceManagementTroubleshootingEvents from me",
        name: list_device_management_troubleshooting_events,
        path: "/me/deviceManagementTroubleshootingEvents"
    );

    post!(
        doc: "# Create new navigation property to deviceManagementTroubleshootingEvents for me",
        name: create_device_management_troubleshooting_events,
        path: "/me/deviceManagementTroubleshootingEvents",
        body: true
    );

    get!(
        doc: "# Get deviceManagementTroubleshootingEvents from me",
        name: get_device_management_troubleshooting_events,
        path: "/me/deviceManagementTroubleshootingEvents/{{id}}",
        params: id
    );

    patch!(
        doc: "# Update the navigation property deviceManagementTroubleshootingEvents in me",
        name: update_device_management_troubleshooting_events,
        path: "/me/deviceManagementTroubleshootingEvents/{{id}}",
        body: true,
        params: id
    );

    get!(
        doc: "# Get directReports from me",
        name: list_direct_reports,
        path: "/me/directReports"
    );

    get!(
        doc: "# Get directReports from me",
        name: get_direct_reports,
        path: "/me/directReports/{{id}}",
        params: id
    );

    get!(
        doc: "# Get drive from me",
        name: get_drive,
        path: "/me/drive"
    );

    patch!(
        doc: "# Update the navigation property drive in me",
        name: update_drive,
        path: "/me/drive",
        body: true
    );

    get!(
        doc: "# Get drives from me",
        name: list_drives,
        path: "/me/drives"
    );

    post!(
        doc: "# Create new navigation property to drives for me",
        name: create_drives,
        path: "/me/drives",
        body: true
    );

    get!(
        doc: "# Get drives from me",
        name: get_drives,
        path: "/me/drives/{{id}}",
        params: id
    );

    patch!(
        doc: "# Update the navigation property drives in me",
        name: update_drives,
        path: "/me/drives/{{id}}",
        body: true,
        params: id
    );

    post!(
        doc: "# Invoke action exportPersonalData",
        name: export_personal_data,
        path: "/me/exportPersonalData",
        body: true
    );

    get!(
        doc: "# Get extensions from me",
        name: list_extensions,
        path: "/me/extensions"
    );

    post!(
        doc: "# Create new navigation property to extensions for me",
        name: create_extensions,
        path: "/me/extensions",
        body: true
    );

    get!(
        doc: "# Get extensions from me",
        name: get_extensions,
        path: "/me/extensions/{{id}}",
        params: id
    );

    patch!(
        doc: "# Update the navigation property extensions in me",
        name: update_extensions,
        path: "/me/extensions/{{id}}",
        body: true,
        params: id
    );

    post!(
        doc: "# Invoke action findMeetingTimes",
        name: find_meeting_times,
        path: "/me/findMeetingTimes",
        body: true
    );

    get!(
        doc: "# Get followedSites from me",
        name: list_followed_sites,
        path: "/me/followedSites"
    );

    get!(
        doc: "# Get followedSites from me",
        name: get_followed_sites,
        path: "/me/followedSites/{{id}}",
        params: id
    );

    post!(
        doc: "# Invoke action getMailTips",
        name: get_mail_tips,
        path: "/me/getMailTips",
        body: true
    );

    get!(
        doc: "# Invoke function getManagedAppDiagnosticStatuses",
        name: get_managed_app_diagnostic_statuses,
        path: "/me/getManagedAppDiagnosticStatuses()"
    );

    get!(
        doc: "# Invoke function getManagedAppPolicies",
        name: get_managed_app_policies,
        path: "/me/getManagedAppPolicies()"
    );

    get!(
        doc: "# Get joinedTeams from me",
        name: list_joined_teams,
        path: "/me/joinedTeams"
    );

    post!(
        doc: "# Create new navigation property to joinedTeams for me",
        name: create_joined_teams,
        path: "/me/joinedTeams",
        body: true
    );

    get!(
        doc: "# Get joinedTeams from me",
        name: get_joined_teams,
        path: "/me/joinedTeams/{{id}}",
        params: id
    );

    patch!(
        doc: "# Update the navigation property joinedTeams in me",
        name: update_joined_teams,
        path: "/me/joinedTeams/{{id}}",
        body: true,
        params: id
    );

    get!(
        doc: "# Get licenseDetails from me",
        name: list_license_details,
        path: "/me/licenseDetails"
    );

    post!(
        doc: "# Create new navigation property to licenseDetails for me",
        name: create_license_details,
        path: "/me/licenseDetails",
        body: true
    );

    get!(
        doc: "# Get licenseDetails from me",
        name: get_license_details,
        path: "/me/licenseDetails/{{id}}",
        params: id
    );

    patch!(
        doc: "# Update the navigation property licenseDetails in me",
        name: update_license_details,
        path: "/me/licenseDetails/{{id}}",
        body: true,
        params: id
    );

    get!(
        doc: "# Get managedAppRegistrations from me",
        name: list_managed_app_registrations,
        path: "/me/managedAppRegistrations"
    );

    get!(
        doc: "# Get managedAppRegistrations from me",
        name: get_managed_app_registrations,
        path: "/me/managedAppRegistrations/{{id}}",
        params: id
    );

    get!(
        doc: "# Get manager from me",
        name: get_manager,
        path: "/me/manager"
    );

    get!(
        doc: "# Get memberOf from me",
        name: list_member_of,
        path: "/me/memberOf"
    );

    get!(
        doc: "# Get memberOf from me",
        name: get_member_of,
        path: "/me/memberOf/{{id}}",
        params: id
    );

    get!(
        doc: "# Get oauth2PermissionGrants from me",
        name: me_list_oauth_2_permission_grants,
        path: "/me/oauth2PermissionGrants"
    );

    get!(
        doc: "# Get oauth2PermissionGrants from me",
        name: me_get_oauth_2_permission_grants,
        path: "/me/oauth2PermissionGrants/{{id}}",
        params: id
    );

    get!(
        doc: "# Get ownedDevices from me",
        name: list_owned_devices,
        path: "/me/ownedDevices"
    );

    get!(
        doc: "# Get ownedDevices from me",
        name: get_owned_devices,
        path: "/me/ownedDevices/{{id}}",
        params: id
    );

    get!(
        doc: "# Get ownedObjects from me",
        name: list_owned_objects,
        path: "/me/ownedObjects"
    );

    get!(
        doc: "# Get ownedObjects from me",
        name: get_owned_objects,
        path: "/me/ownedObjects/{{id}}",
        params: id
    );

    get!(
        doc: "# Get people from me",
        name: list_people,
        path: "/me/people"
    );

    post!(
        doc: "# Create new navigation property to people for me",
        name: create_people,
        path: "/me/people",
        body: true
    );

    get!(
        doc: "# Get people from me",
        name: get_people,
        path: "/me/people/{{id}}",
        params: id
    );

    patch!(
        doc: "# Update the navigation property people in me",
        name: update_people,
        path: "/me/people/{{id}}",
        body: true,
        params: id
    );

    get!(
        doc: "# Get photo from me",
        name: get_photo,
        path: "/me/photo"
    );

    patch!(
        doc: "# Update the navigation property photo in me",
        name: update_photo,
        path: "/me/photo",
        body: true
    );

    get!(
        doc: "# Get photos from me",
        name: list_photos,
        path: "/me/photos"
    );

    post!(
        doc: "# Create new navigation property to photos for me",
        name: create_photos,
        path: "/me/photos",
        body: true
    );

    get!(
        doc: "# Get photos from me",
        name: get_photos,
        path: "/me/photos/{{id}}",
        params: id
    );

    patch!(
        doc: "# Update the navigation property photos in me",
        name: update_photos,
        path: "/me/photos/{{id}}",
        body: true,
        params: id
    );

    get!(
        doc: "# Get registeredDevices from me",
        name: list_registered_devices,
        path: "/me/registeredDevices"


    );

    get!(
        doc: "# Get registeredDevices from me",
        name: get_registered_devices,

        path: "/me/registeredDevices/{{id}}",
        params: id
    );

    post!(
        doc: "# Invoke action removeAllDevicesFromManagement",
        name: remove_all_devices_from_management,
        path: "/me/removeAllDevicesFromManagement"
    );

    post!(
        doc: "# Invoke action reprocessLicenseAssignment",
        name: reprocess_license_assignment,
        path: "/me/reprocessLicenseAssignment"
    );

    post!(
        doc: "# Invoke action revokeSignInSessions",
        name: revoke_sign_in_sessions,
        path: "/me/revokeSignInSessions"
    );

    get!(
        doc: "# Get scopedRoleMemberOf from me",
        name: list_scoped_role_member_of,
        path: "/me/scopedRoleMemberOf"
    );

    post!(
        doc: "# Create new navigation property to scopedRoleMemberOf for me",
        name: create_scoped_role_member_of,
        path: "/me/scopedRoleMemberOf",
        body: true
    );

    get!(
        doc: "# Get scopedRoleMemberOf from me",
        name: get_scoped_role_member_of,
        path: "/me/scopedRoleMemberOf/{{id}}",
        params: id
    );

    patch!(
        doc: "# Update the navigation property scopedRoleMemberOf in me",
        name: update_scoped_role_member_of,
        path: "/me/scopedRoleMemberOf/{{id}}",
        body: true,
        params: id
    );

    post!(
        doc: "# Invoke action sendMail",
        name: send_mail,
        path: "/me/sendMail",
        body: true
    );

    get!(
        doc: "# Get settings from me",
        name: get_settings,
        path: "/me/settings"
    );

    patch!(
        doc: "# Update the navigation property settings in me",
        name: update_settings,
        path: "/me/settings",
        body: true
    );

    get!(
        doc: "# Get transitiveMemberOf from me",
        name: list_transitive_member_of,
        path: "/me/transitiveMemberOf"
    );

    get!(
        doc: "# Get transitiveMemberOf from me",
        name: get_transitive_member_of,
        path: "/me/transitiveMemberOf/{{id}}",
        params: id
    );

    post!(
        doc: "# Invoke action translateExchangeIds",
        name: translate_exchange_ids,
        path: "/me/translateExchangeIds",
        body: true
    );

    post!(
        doc: "# Invoke action wipeManagedAppRegistrationsByDeviceTag",
        name: wipe_managed_app_registrations_by_device_tag,
        path: "/me/wipeManagedAppRegistrationsByDeviceTag",
        body: true
    );

    api_client_link!(
        activities,
        ResourceIdentity::Activities,
        ActivitiesApiClient
    );

    api_client_link_id!(
        activity,
        ResourceIdentity::Activities,
        ActivitiesIdApiClient
    );

    api_client_link!(
        aggreement_acceptances,
        ResourceIdentity::AgreementAcceptances,
        AgreementAcceptancesApiClient
    );

    api_client_link_id!(
        aggreement_acceptance,
        ResourceIdentity::AgreementAcceptances,
        AgreementAcceptancesIdApiClient
    );

    api_client_link!(calendars, ResourceIdentity::Calendar, CalendarApiClient);

    api_client_link_id!(calendar, ResourceIdentity::Calendar, CalendarIdApiClient);

    api_client_link!(
        calendar_groups,
        ResourceIdentity::CalendarGroups,
        CalendarGroupsApiClient
    );

    api_client_link_id!(
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

    api_client_link!(contacts, ResourceIdentity::Contacts, ContactsApiClient);

    api_client_link_id!(contact, ResourceIdentity::Contacts, ContactsIdApiClient);

    api_client_link!(
        contact_folders,
        ResourceIdentity::ContactFolders,
        ContactFoldersApiClient
    );

    api_client_link_id!(
        contact_folder,
        ResourceIdentity::ContactFolders,
        ContactFoldersIdApiClient
    );

    api_client_link!(drive, ResourceIdentity::Drive, DriveApiClient);

    api_client_link!(education, ResourceIdentity::Education, EducationMeApiClient);

    api_client_link!(events, ResourceIdentity::Events, EventsApiClient);

    api_client_link_id!(event, ResourceIdentity::Events, EventsIdApiClient);

    api_client_link!(
        inference_classification,
        ResourceIdentity::InferenceClassification,
        InferenceClassificationApiClient
    );

    api_client_link!(insights, ResourceIdentity::Insights, InsightsApiClient);

    api_client_link!(
        mail_folders,
        ResourceIdentity::MailFolders,
        MailFoldersApiClient
    );

    api_client_link_id!(
        mail_folder,
        ResourceIdentity::MailFolders,
        MailFoldersIdApiClient
    );

    api_client_link!(
        managed_devices,
        ResourceIdentity::ManagedDevices,
        ManagedDevicesApiClient
    );

    api_client_link_id!(
        managed_device,
        ResourceIdentity::ManagedDevices,
        ManagedDevicesIdApiClient
    );

    api_client_link!(messages, ResourceIdentity::Messages, MessagesApiClient);
    api_client_link_id!(message, ResourceIdentity::Messages, MessagesIdApiClient);

    api_client_link!(onenote, ResourceIdentity::Onenote, OnenoteApiClient);

    api_client_link_id!(outlook, ResourceIdentity::Outlook, OutlookApiClient);

    api_client_link!(planner, ResourceIdentity::Planner, PlannerApiClient);

    api_client_link_id!(settings, ResourceIdentity::Settings, SettingsApiClient);
}
