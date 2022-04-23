// GENERATED CODE

use crate::activities::{ActivitiesIdRequest, ActivitiesRequest};
use crate::agreement_acceptances::{AgreementAcceptancesIdRequest, AgreementAcceptancesRequest};
use crate::calendar::{CalendarRequest, CalendarsRequest};
use crate::calendar_groups::{CalendarGroupRequest, CalendarGroupsRequest};
use crate::calendar_view::{CalendarViewRequest, CalendarViewsRequest};
use crate::client::Graph;
use crate::contact_folders::{ContactFolderRequest, ContactFoldersRequest};
use crate::contacts::{ContactRequest, ContactsRequest};
use crate::core::ResourceIdentity;
use crate::drive::DrivesRequest;
use crate::education::UsersRequest as EducationUsersRequest;
use crate::events::{EventRequest, EventsRequest};
use crate::inference_classification::InferenceClassificationRequest;
use crate::insights::InsightsRequest;
use crate::mail_folders::{MailFolderRequest, MailFoldersRequest};
use crate::managed_devices::{ManagedDeviceRequest, ManagedDevicesRequest};
use crate::messages::{MessageRequest, MessagesRequest};
use crate::onenote::OnenoteRequest;
use crate::outlook::OutlookRequest;
use crate::planner::PlannerRequest;
use crate::settings::SettingsRequest;
use graph_http::types::DeltaPhantom;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(ManagedAppRegistrationsRequest,);
register_client!(UserRequest,);
register_client!(UsersRequest, ());

impl<'a, Client> ManagedAppRegistrationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Invoke function getUserIdsWithFlaggedAppRegistration",
        name: get_user_ids_with_flagged_app_registration,
        response: serde_json::Value,
        path: "/users/{{RID}}/managedAppRegistrations/getUserIdsWithFlaggedAppRegistration()",
        params: 0,
        has_body: false
    });
}

impl<'a, Client> UserRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entities from users",
        name: list_user,
        response: serde_json::Value,
        path: "/users",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Add new entity to users",
        name: create_user,
        response: serde_json::Value,
        path: "/users",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/users/delta()",
        has_body: false
    });

    get!({
        doc: "# Invoke function delta with a previous delta token",
        name: delta_token,
        response: DeltaPhantom<serde_json::Value>,
        path: "/users/delta()",
        query: [ key: "$deltaToken", value: delta_token ]
    });

    pub fn education(&self) -> EducationUsersRequest<'a, Client> {
        EducationUsersRequest::new(self.client)
    }

    pub fn id<ID: AsRef<str>>(&self, id: ID) -> UsersRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Users);
        UsersRequest::new(id.as_ref(), self.client)
    }
}

impl<'a, Client> UsersRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entity from users by key",
        name: get_user,
        response: serde_json::Value,
        path: "/users/{{RID}}",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update entity in users",
        name: update_user,
        response: NoContent,
        path: "/users/{{RID}}",
        params: 0,
        has_body: true
    });

    delete!({
        doc: "# Delete entity from users",
        name: delete_user,
        response: NoContent,
        path: "/users/{{RID}}",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get appRoleAssignments from users",
        name: list_app_role_assignments,
        response: serde_json::Value,
        path: "/users/{{RID}}/appRoleAssignments",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to appRoleAssignments for users",
        name: create_app_role_assignments,
        response: serde_json::Value,
        path: "/users/{{RID}}/appRoleAssignments",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get appRoleAssignments from users",
        name: get_app_role_assignments,
        response: serde_json::Value,
        path: "/users/{{RID}}/appRoleAssignments/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property appRoleAssignments in users",
        name: update_app_role_assignments,
        response: NoContent,
        path: "/users/{{RID}}/appRoleAssignments/{{id}}",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action assignLicense",
        name: assign_license,
        response: serde_json::Value,
        path: "/users/{{RID}}/assignLicense",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action changePassword",
        name: change_password,
        response: NoContent,
        path: "/users/{{RID}}/changePassword",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get createdObjects from users",
        name: list_created_objects,
        response: serde_json::Value,
        path: "/users/{{RID}}/createdObjects",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get createdObjects from users",
        name: get_created_objects,
        response: serde_json::Value,
        path: "/users/{{RID}}/createdObjects/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get deviceManagementTroubleshootingEvents from users",
        name: list_device_management_troubleshooting_events,
        response: serde_json::Value,
        path: "/users/{{RID}}/deviceManagementTroubleshootingEvents",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to deviceManagementTroubleshootingEvents for users",
        name: create_device_management_troubleshooting_events,
        response: serde_json::Value,
        path: "/users/{{RID}}/deviceManagementTroubleshootingEvents",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get deviceManagementTroubleshootingEvents from users",
        name: get_device_management_troubleshooting_events,
        response: serde_json::Value,
        path: "/users/{{RID}}/deviceManagementTroubleshootingEvents/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property deviceManagementTroubleshootingEvents in users",
        name: update_device_management_troubleshooting_events,
        response: NoContent,
        path: "/users/{{RID}}/deviceManagementTroubleshootingEvents/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get directReports from users",
        name: list_direct_reports,
        response: serde_json::Value,
        path: "/users/{{RID}}/directReports",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get directReports from users",
        name: get_direct_reports,
        response: serde_json::Value,
        path: "/users/{{RID}}/directReports/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get drive from users",
        name: get_drive,
        response: serde_json::Value,
        path: "/users/{{RID}}/drive",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property drive in users",
        name: update_drive,
        response: NoContent,
        path: "/users/{{RID}}/drive",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get drives from users",
        name: list_drives,
        response: serde_json::Value,
        path: "/users/{{RID}}/drives",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to drives for users",
        name: create_drives,
        response: serde_json::Value,
        path: "/users/{{RID}}/drives",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get drives from users",
        name: get_drives,
        response: serde_json::Value,
        path: "/users/{{RID}}/drives/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property drives in users",
        name: update_drives,
        response: NoContent,
        path: "/users/{{RID}}/drives/{{id}}",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action exportPersonalData",
        name: export_personal_data,
        response: NoContent,
        path: "/users/{{RID}}/exportPersonalData",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get extensions from users",
        name: list_extensions,
        response: serde_json::Value,
        path: "/users/{{RID}}/extensions",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to extensions for users",
        name: create_extensions,
        response: serde_json::Value,
        path: "/users/{{RID}}/extensions",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get extensions from users",
        name: get_extensions,
        response: serde_json::Value,
        path: "/users/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property extensions in users",
        name: update_extensions,
        response: NoContent,
        path: "/users/{{RID}}/extensions/{{id}}",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action findMeetingTimes",
        name: find_meeting_times,
        response: serde_json::Value,
        path: "/users/{{RID}}/findMeetingTimes",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get followedSites from users",
        name: list_followed_sites,
        response: serde_json::Value,
        path: "/users/{{RID}}/followedSites",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get followedSites from users",
        name: get_followed_sites,
        response: serde_json::Value,
        path: "/users/{{RID}}/followedSites/{{id}}",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action getMailTips",
        name: get_mail_tips,
        response: serde_json::Value,
        path: "/users/{{RID}}/getMailTips",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Invoke function getManagedAppDiagnosticStatuses",
        name: get_managed_app_diagnostic_statuses,
        response: serde_json::Value,
        path: "/users/{{RID}}/getManagedAppDiagnosticStatuses()",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Invoke function getManagedAppPolicies",
        name: get_managed_app_policies,
        response: serde_json::Value,
        path: "/users/{{RID}}/getManagedAppPolicies()",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get joinedTeams from users",
        name: list_joined_teams,
        response: serde_json::Value,
        path: "/users/{{RID}}/joinedTeams",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to joinedTeams for users",
        name: create_joined_teams,
        response: serde_json::Value,
        path: "/users/{{RID}}/joinedTeams",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get joinedTeams from users",
        name: get_joined_teams,
        response: serde_json::Value,
        path: "/users/{{RID}}/joinedTeams/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property joinedTeams in users",
        name: update_joined_teams,
        response: NoContent,
        path: "/users/{{RID}}/joinedTeams/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get licenseDetails from users",
        name: list_license_details,
        response: serde_json::Value,
        path: "/users/{{RID}}/licenseDetails",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to licenseDetails for users",
        name: create_license_details,
        response: serde_json::Value,
        path: "/users/{{RID}}/licenseDetails",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get licenseDetails from users",
        name: get_license_details,
        response: serde_json::Value,
        path: "/users/{{RID}}/licenseDetails/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property licenseDetails in users",
        name: update_license_details,
        response: NoContent,
        path: "/users/{{RID}}/licenseDetails/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get managedAppRegistrations from users",
        name: list_managed_app_registrations,
        response: serde_json::Value,
        path: "/users/{{RID}}/managedAppRegistrations",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get managedAppRegistrations from users",
        name: get_managed_app_registrations,
        response: serde_json::Value,
        path: "/users/{{RID}}/managedAppRegistrations/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get manager from users",
        name: get_manager,
        response: serde_json::Value,
        path: "/users/{{RID}}/manager",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get memberOf from users",
        name: list_member_of,
        response: serde_json::Value,
        path: "/users/{{RID}}/memberOf",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get memberOf from users",
        name: get_member_of,
        response: serde_json::Value,
        path: "/users/{{RID}}/memberOf/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get oauth2PermissionGrants from users",
        name: users_list_oauth_2_permission_grants,
        response: serde_json::Value,
        path: "/users/{{RID}}/oauth2PermissionGrants",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get oauth2PermissionGrants from users",
        name: users_get_oauth_2_permission_grants,
        response: serde_json::Value,
        path: "/users/{{RID}}/oauth2PermissionGrants/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get ownedDevices from users",
        name: list_owned_devices,
        response: serde_json::Value,
        path: "/users/{{RID}}/ownedDevices",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get ownedDevices from users",
        name: get_owned_devices,
        response: serde_json::Value,
        path: "/users/{{RID}}/ownedDevices/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get ownedObjects from users",
        name: list_owned_objects,
        response: serde_json::Value,
        path: "/users/{{RID}}/ownedObjects",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get ownedObjects from users",
        name: get_owned_objects,
        response: serde_json::Value,
        path: "/users/{{RID}}/ownedObjects/{{id}}",
        params: 1,
        has_body: false
    });

    get!({
        doc: "# Get people from users",
        name: list_people,
        response: serde_json::Value,
        path: "/users/{{RID}}/people",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to people for users",
        name: create_people,
        response: serde_json::Value,
        path: "/users/{{RID}}/people",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get people from users",
        name: get_people,
        response: serde_json::Value,
        path: "/users/{{RID}}/people/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property people in users",
        name: update_people,
        response: NoContent,
        path: "/users/{{RID}}/people/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get photo from users",
        name: get_photo,
        response: serde_json::Value,
        path: "/users/{{RID}}/photo",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property photo in users",
        name: update_photo,
        response: NoContent,
        path: "/users/{{RID}}/photo",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get photos from users",
        name: list_photos,
        response: serde_json::Value,
        path: "/users/{{RID}}/photos",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to photos for users",
        name: create_photos,
        response: serde_json::Value,
        path: "/users/{{RID}}/photos",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get photos from users",
        name: get_photos,
        response: serde_json::Value,
        path: "/users/{{RID}}/photos/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property photos in users",
        name: update_photos,
        response: NoContent,
        path: "/users/{{RID}}/photos/{{id}}",
        params: 1,
        has_body: true
    });

    get!({
        doc: "# Get registeredDevices from users",
        name: list_registered_devices,
        response: serde_json::Value,
        path: "/users/{{RID}}/registeredDevices",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get registeredDevices from users",
        name: get_registered_devices,
        response: serde_json::Value,
        path: "/users/{{RID}}/registeredDevices/{{id}}",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action removeAllDevicesFromManagement",
        name: remove_all_devices_from_management,
        response: NoContent,
        path: "/users/{{RID}}/removeAllDevicesFromManagement",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Invoke action reprocessLicenseAssignment",
        name: reprocess_license_assignment,
        response: serde_json::Value,
        path: "/users/{{RID}}/reprocessLicenseAssignment",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Invoke action revokeSignInSessions",
        name: revoke_sign_in_sessions,
        response: serde_json::Value,
        path: "/users/{{RID}}/revokeSignInSessions",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get scopedRoleMemberOf from users",
        name: list_scoped_role_member_of,
        response: serde_json::Value,
        path: "/users/{{RID}}/scopedRoleMemberOf",
        params: 0,
        has_body: false
    });

    post!({
        doc: "# Create new navigation property to scopedRoleMemberOf for users",
        name: create_scoped_role_member_of,
        response: serde_json::Value,
        path: "/users/{{RID}}/scopedRoleMemberOf",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get scopedRoleMemberOf from users",
        name: get_scoped_role_member_of,
        response: serde_json::Value,
        path: "/users/{{RID}}/scopedRoleMemberOf/{{id}}",
        params: 1,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property scopedRoleMemberOf in users",
        name: update_scoped_role_member_of,
        response: NoContent,
        path: "/users/{{RID}}/scopedRoleMemberOf/{{id}}",
        params: 1,
        has_body: true
    });

    post!({
        doc: "# Invoke action sendMail",
        name: send_mail,
        response: NoContent,
        path: "/users/{{RID}}/sendMail",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get settings from users",
        name: get_settings,
        response: serde_json::Value,
        path: "/users/{{RID}}/settings",
        params: 0,
        has_body: false
    });

    patch!({
        doc: "# Update the navigation property settings in users",
        name: update_settings,
        response: NoContent,
        path: "/users/{{RID}}/settings",
        params: 0,
        has_body: true
    });

    get!({
        doc: "# Get transitiveMemberOf from users",
        name: list_transitive_member_of,
        response: serde_json::Value,
        path: "/users/{{RID}}/transitiveMemberOf",
        params: 0,
        has_body: false
    });

    get!({
        doc: "# Get transitiveMemberOf from users",
        name: get_transitive_member_of,
        response: serde_json::Value,
        path: "/users/{{RID}}/transitiveMemberOf/{{id}}",
        params: 1,
        has_body: false
    });

    post!({
        doc: "# Invoke action translateExchangeIds",
        name: translate_exchange_ids,
        response: serde_json::Value,
        path: "/users/{{RID}}/translateExchangeIds",
        params: 0,
        has_body: true
    });

    post!({
        doc: "# Invoke action wipeManagedAppRegistrationsByDeviceTag",
        name: wipe_managed_app_registrations_by_device_tag,
        response: NoContent,
        path: "/users/{{RID}}/wipeManagedAppRegistrationsByDeviceTag",
        params: 0,
        has_body: true
    });

    pub fn activities(&self) -> ActivitiesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Activities);
        ActivitiesRequest::new(self.client)
    }

    pub fn activity<ID: AsRef<str>>(&self, id: ID) -> ActivitiesIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Activities);
        ActivitiesIdRequest::new(id.as_ref(), self.client)
    }

    pub fn agreement_acceptances(&self) -> AgreementAcceptancesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client
            .set_ident(ResourceIdentity::AgreementAcceptances);
        AgreementAcceptancesRequest::new(self.client)
    }

    pub fn agreement_acceptance<ID: AsRef<str>>(
        &self,
        id: ID,
    ) -> AgreementAcceptancesIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client
            .set_ident(ResourceIdentity::AgreementAcceptances);
        AgreementAcceptancesIdRequest::new(id.as_ref(), self.client)
    }

    pub fn calendars(&self) -> CalendarRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Calendar);
        CalendarRequest::new(self.client)
    }

    pub fn calendar_groups(&self) -> CalendarGroupRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::CalendarGroup);
        CalendarGroupRequest::new(self.client)
    }

    pub fn calendar_group<ID: AsRef<str>>(&self, id: ID) -> CalendarGroupsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::CalendarGroups);
        CalendarGroupsRequest::new(id.as_ref(), self.client)
    }

    pub fn calendar_view<ID: AsRef<str>>(&self, id: ID) -> CalendarViewRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::CalendarView);
        CalendarViewRequest::new(id.as_ref(), self.client)
    }

    pub fn calendar_views(&self) -> CalendarViewsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::CalendarViews);
        CalendarViewsRequest::new(self.client)
    }

    pub fn calendar<ID: AsRef<str>>(&self, id: ID) -> CalendarsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Calendars);
        CalendarsRequest::new(id.as_ref(), self.client)
    }

    pub fn contacts(&self) -> ContactRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        ContactRequest::new(self.client)
    }

    pub fn contact_folders(&self) -> ContactFolderRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        ContactFolderRequest::new(self.client)
    }

    pub fn contact_folder<ID: AsRef<str>>(&self, id: ID) -> ContactFoldersRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::ContactFolders);
        ContactFoldersRequest::new(id.as_ref(), self.client)
    }

    pub fn contact<ID: AsRef<str>>(&self, id: ID) -> ContactsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Contacts);
        ContactsRequest::new(id.as_ref(), self.client)
    }

    pub fn drive(&self) -> DrivesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        DrivesRequest::new("", self.client)
    }

    pub fn events(&self) -> EventRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Event);
        EventRequest::new(self.client)
    }

    pub fn event<ID: AsRef<str>>(&self, id: ID) -> EventsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Events);
        EventsRequest::new(id.as_ref(), self.client)
    }

    pub fn inference_classification(&self) -> InferenceClassificationRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client
            .set_ident(ResourceIdentity::InferenceClassification);
        InferenceClassificationRequest::new(self.client)
    }

    pub fn insights(&self) -> InsightsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Insights);
        InsightsRequest::new(self.client)
    }

    pub fn mail_folders(&self) -> MailFolderRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        MailFolderRequest::new(self.client)
    }

    pub fn mail_folder<ID: AsRef<str>>(&self, id: ID) -> MailFoldersRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::MailFolders);
        MailFoldersRequest::new(id.as_ref(), self.client)
    }

    pub fn managed_app_registrations(&self) -> ManagedAppRegistrationsRequest<'a, Client> {
        ManagedAppRegistrationsRequest::new(self.client)
    }

    pub fn managed_devices(&self) -> ManagedDeviceRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        ManagedDeviceRequest::new(self.client)
    }

    pub fn managed_device<ID: AsRef<str>>(&self, id: ID) -> ManagedDevicesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::ManagedDevices);
        ManagedDevicesRequest::new(id.as_ref(), self.client)
    }

    pub fn messages(&self) -> MessageRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        MessageRequest::new(self.client)
    }

    pub fn message<ID: AsRef<str>>(&self, id: ID) -> MessagesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Messages);
        MessagesRequest::new(id.as_ref(), self.client)
    }

    pub fn onenote(&self) -> OnenoteRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Onenote);
        OnenoteRequest::new(self.client)
    }

    pub fn outlook(&self) -> OutlookRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Outlook);
        OutlookRequest::new(self.client)
    }

    pub fn planner(&self) -> PlannerRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Planner);
        PlannerRequest::new(self.client)
    }

    pub fn settings(&self) -> SettingsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Settings);
        SettingsRequest::new(self.client)
    }
}
