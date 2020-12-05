use crate::activities::ActivitiesRequest;
use crate::calendar::{CalendarRequest, CalendarsRequest};
use crate::calendar_groups::{CalendarGroupRequest, CalendarGroupsRequest};
use crate::calendar_view::{CalendarViewRequest, CalendarViewsRequest};
use crate::client::Graph;
use crate::contact_folders::{ContactFolderRequest, ContactFoldersRequest};
use crate::contacts::{ContactRequest, ContactsRequest};
use crate::core::ResourceIdentity;
use crate::drive::DrivesRequest;
use crate::education::MeRequest as EducationMeRequest;
use crate::events::{EventRequest, EventsRequest};
use crate::inference_classification::InferenceClassificationRequest;
use crate::insights::InsightsRequest;
use crate::managed_devices::{ManagedDeviceRequest, ManagedDevicesRequest};
use crate::onenote::OnenoteRequest;
use crate::outlook::OutlookRequest;
use crate::settings::SettingsRequest;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(ManagedAppRegistrationsRequest,);
register_client!(MeRequest,);

impl<'a, Client> ManagedAppRegistrationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Invoke function getUserIdsWithFlaggedAppRegistration",
        name: get_user_ids_with_flagged_app_registration,
        response: Collection<serde_json::Value>,
        path: "/me/managedAppRegistrations/getUserIdsWithFlaggedAppRegistration()",
        params: 0,
        has_body: false
    });
}

impl<'a, Client> MeRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn activities(&self) -> ActivitiesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Activities);
        ActivitiesRequest::new(self.client)
    }
    pub fn calendars(&self) -> CalendarRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Calendar);
        CalendarRequest::new(self.client)
    }
    pub fn calendar_groups(&self) -> CalendarGroupRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::CalendarGroup);
        CalendarGroupRequest::new(self.client)
    }
    pub fn calendar_group<ID: AsRef<str>>(&self, id: ID) -> CalendarGroupsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::CalendarGroups);
        CalendarGroupsRequest::new(id.as_ref(), self.client)
    }
    pub fn calendar_view<ID: AsRef<str>>(&self, id: ID) -> CalendarViewRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::CalendarView);
        CalendarViewRequest::new(id.as_ref(), self.client)
    }
    pub fn calendar_views(&self) -> CalendarViewsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::CalendarViews);
        CalendarViewsRequest::new(self.client)
    }
    pub fn calendar<ID: AsRef<str>>(&self, id: ID) -> CalendarsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Calendars);
        CalendarsRequest::new(id.as_ref(), self.client)
    }
    pub fn contacts(&self) -> ContactRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        ContactRequest::new(self.client)
    }
    pub fn contact_folders(&self) -> ContactFolderRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        ContactFolderRequest::new(self.client)
    }
    pub fn contact_folder<ID: AsRef<str>>(&self, id: ID) -> ContactFoldersRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::ContactFolders);
        ContactFoldersRequest::new(id.as_ref(), self.client)
    }
    pub fn contact<ID: AsRef<str>>(&self, id: ID) -> ContactsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Contacts);
        ContactsRequest::new(id.as_ref(), self.client)
    }
    pub fn drive(&self) -> DrivesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        DrivesRequest::new("", self.client)
    }
    pub fn education(&self) -> EducationMeRequest<'a, Client> {
        EducationMeRequest::new(self.client)
    }
    pub fn events(&self) -> EventRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Event);
        EventRequest::new(self.client)
    }
    pub fn event<ID: AsRef<str>>(&self, id: ID) -> EventsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Events);
        EventsRequest::new(id.as_ref(), self.client)
    }
    pub fn inference_classification(&self) -> InferenceClassificationRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client
            .set_ident(ResourceIdentity::InferenceClassification);
        InferenceClassificationRequest::new(self.client)
    }
    pub fn insights(&self) -> InsightsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Insights);
        InsightsRequest::new(self.client)
    }
    pub fn managed_app_registrations(&self) -> ManagedAppRegistrationsRequest<'a, Client> {
        ManagedAppRegistrationsRequest::new(self.client)
    }
    pub fn managed_devices(&self) -> ManagedDeviceRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        ManagedDeviceRequest::new(self.client)
    }
    pub fn managed_device<ID: AsRef<str>>(&self, id: ID) -> ManagedDevicesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::ManagedDevices);
        ManagedDevicesRequest::new(id.as_ref(), self.client)
    }
    pub fn onenote(&self) -> OnenoteRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Onenote);
        OnenoteRequest::new(self.client)
    }
    pub fn outlook(&self) -> OutlookRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Outlook);
        OutlookRequest::new(self.client)
    }
    pub fn settings(&self) -> SettingsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Settings);
        SettingsRequest::new(self.client)
    }
    get!({
        doc: "# Get me",
        name: get_user,
        response: serde_json::Value,
        path: "/me",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update me",
        name: update_user,
        response: GraphResponse<Content>,
        path: "/me",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get appRoleAssignments from me",
        name: list_app_role_assignments,
        response: Collection<serde_json::Value>,
        path: "/me/appRoleAssignments",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to appRoleAssignments for me",
        name: create_app_role_assignments,
        response: serde_json::Value,
        path: "/me/appRoleAssignments",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get appRoleAssignments from me",
        name: get_app_role_assignments,
        response: serde_json::Value,
        path: "/me/appRoleAssignments/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property appRoleAssignments in me",
        name: update_app_role_assignments,
        response: GraphResponse<Content>,
        path: "/me/appRoleAssignments/{{id}}",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action assignLicense",
        name: assign_license,
        response: serde_json::Value,
        path: "/me/assignLicense",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action changePassword",
        name: change_password,
        response: GraphResponse<Content>,
        path: "/me/changePassword",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get createdObjects from me",
        name: list_created_objects,
        response: Collection<serde_json::Value>,
        path: "/me/createdObjects",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get createdObjects from me",
        name: get_created_objects,
        response: serde_json::Value,
        path: "/me/createdObjects/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get deviceManagementTroubleshootingEvents from me",
        name: list_device_management_troubleshooting_events,
        response: Collection<serde_json::Value>,
        path: "/me/deviceManagementTroubleshootingEvents",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to deviceManagementTroubleshootingEvents for me",
        name: create_device_management_troubleshooting_events,
        response: serde_json::Value,
        path: "/me/deviceManagementTroubleshootingEvents",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get deviceManagementTroubleshootingEvents from me",
        name: get_device_management_troubleshooting_events,
        response: serde_json::Value,
        path: "/me/deviceManagementTroubleshootingEvents/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property deviceManagementTroubleshootingEvents in me",
        name: update_device_management_troubleshooting_events,
        response: GraphResponse<Content>,
        path: "/me/deviceManagementTroubleshootingEvents/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get directReports from me",
        name: list_direct_reports,
        response: Collection<serde_json::Value>,
        path: "/me/directReports",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get directReports from me",
        name: get_direct_reports,
        response: serde_json::Value,
        path: "/me/directReports/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get drive from me",
        name: get_drive,
        response: serde_json::Value,
        path: "/me/drive",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property drive in me",
        name: update_drive,
        response: GraphResponse<Content>,
        path: "/me/drive",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get drives from me",
        name: list_drives,
        response: Collection<serde_json::Value>,
        path: "/me/drives",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to drives for me",
        name: create_drives,
        response: serde_json::Value,
        path: "/me/drives",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get drives from me",
        name: get_drives,
        response: serde_json::Value,
        path: "/me/drives/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property drives in me",
        name: update_drives,
        response: GraphResponse<Content>,
        path: "/me/drives/{{id}}",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action exportPersonalData",
        name: export_personal_data,
        response: GraphResponse<Content>,
        path: "/me/exportPersonalData",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get extensions from me",
        name: list_extensions,
        response: Collection<serde_json::Value>,
        path: "/me/extensions",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to extensions for me",
        name: create_extensions,
        response: serde_json::Value,
        path: "/me/extensions",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get extensions from me",
        name: get_extensions,
        response: serde_json::Value,
        path: "/me/extensions/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property extensions in me",
        name: update_extensions,
        response: GraphResponse<Content>,
        path: "/me/extensions/{{id}}",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action findMeetingTimes",
        name: find_meeting_times,
        response: serde_json::Value,
        path: "/me/findMeetingTimes",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get followedSites from me",
        name: list_followed_sites,
        response: Collection<serde_json::Value>,
        path: "/me/followedSites",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get followedSites from me",
        name: get_followed_sites,
        response: serde_json::Value,
        path: "/me/followedSites/{{id}}",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action getMailTips",
        name: get_mail_tips,
        response: Collection<serde_json::Value>,
        path: "/me/getMailTips",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function getManagedAppDiagnosticStatuses",
        name: get_managed_app_diagnostic_statuses,
        response: Collection<serde_json::Value>,
        path: "/me/getManagedAppDiagnosticStatuses()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Invoke function getManagedAppPolicies",
        name: get_managed_app_policies,
        response: Collection<serde_json::Value>,
        path: "/me/getManagedAppPolicies()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get joinedTeams from me",
        name: list_joined_teams,
        response: Collection<serde_json::Value>,
        path: "/me/joinedTeams",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to joinedTeams for me",
        name: create_joined_teams,
        response: serde_json::Value,
        path: "/me/joinedTeams",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get joinedTeams from me",
        name: get_joined_teams,
        response: serde_json::Value,
        path: "/me/joinedTeams/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property joinedTeams in me",
        name: update_joined_teams,
        response: GraphResponse<Content>,
        path: "/me/joinedTeams/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get licenseDetails from me",
        name: list_license_details,
        response: Collection<serde_json::Value>,
        path: "/me/licenseDetails",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to licenseDetails for me",
        name: create_license_details,
        response: serde_json::Value,
        path: "/me/licenseDetails",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get licenseDetails from me",
        name: get_license_details,
        response: serde_json::Value,
        path: "/me/licenseDetails/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property licenseDetails in me",
        name: update_license_details,
        response: GraphResponse<Content>,
        path: "/me/licenseDetails/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get managedAppRegistrations from me",
        name: list_managed_app_registrations,
        response: Collection<serde_json::Value>,
        path: "/me/managedAppRegistrations",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get managedAppRegistrations from me",
        name: get_managed_app_registrations,
        response: serde_json::Value,
        path: "/me/managedAppRegistrations/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get manager from me",
        name: get_manager,
        response: serde_json::Value,
        path: "/me/manager",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get memberOf from me",
        name: list_member_of,
        response: Collection<serde_json::Value>,
        path: "/me/memberOf",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get memberOf from me",
        name: get_member_of,
        response: serde_json::Value,
        path: "/me/memberOf/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get oauth2PermissionGrants from me",
        name: me_list_oauth_2_permission_grants,
        response: serde_json::Value,
        path: "/me/oauth2PermissionGrants",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get oauth2PermissionGrants from me",
        name: me_get_oauth_2_permission_grants,
        response: serde_json::Value,
        path: "/me/oauth2PermissionGrants/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get ownedDevices from me",
        name: list_owned_devices,
        response: Collection<serde_json::Value>,
        path: "/me/ownedDevices",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get ownedDevices from me",
        name: get_owned_devices,
        response: serde_json::Value,
        path: "/me/ownedDevices/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get ownedObjects from me",
        name: list_owned_objects,
        response: Collection<serde_json::Value>,
        path: "/me/ownedObjects",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get ownedObjects from me",
        name: get_owned_objects,
        response: serde_json::Value,
        path: "/me/ownedObjects/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get people from me",
        name: list_people,
        response: Collection<serde_json::Value>,
        path: "/me/people",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to people for me",
        name: create_people,
        response: serde_json::Value,
        path: "/me/people",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get people from me",
        name: get_people,
        response: serde_json::Value,
        path: "/me/people/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property people in me",
        name: update_people,
        response: GraphResponse<Content>,
        path: "/me/people/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get photo from me",
        name: get_photo,
        response: serde_json::Value,
        path: "/me/photo",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property photo in me",
        name: update_photo,
        response: GraphResponse<Content>,
        path: "/me/photo",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get photos from me",
        name: list_photos,
        response: Collection<serde_json::Value>,
        path: "/me/photos",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to photos for me",
        name: create_photos,
        response: serde_json::Value,
        path: "/me/photos",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get photos from me",
        name: get_photos,
        response: serde_json::Value,
        path: "/me/photos/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property photos in me",
        name: update_photos,
        response: GraphResponse<Content>,
        path: "/me/photos/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get registeredDevices from me",
        name: list_registered_devices,
        response: Collection<serde_json::Value>,
        path: "/me/registeredDevices",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get registeredDevices from me",
        name: get_registered_devices,
        response: serde_json::Value,
        path: "/me/registeredDevices/{{id}}",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action removeAllDevicesFromManagement",
        name: remove_all_devices_from_management,
        response: GraphResponse<Content>,
        path: "/me/removeAllDevicesFromManagement",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action reprocessLicenseAssignment",
        name: reprocess_license_assignment,
        response: serde_json::Value,
        path: "/me/reprocessLicenseAssignment",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action revokeSignInSessions",
        name: revoke_sign_in_sessions,
        response: serde_json::Value,
        path: "/me/revokeSignInSessions",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get scopedRoleMemberOf from me",
        name: list_scoped_role_member_of,
        response: Collection<serde_json::Value>,
        path: "/me/scopedRoleMemberOf",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to scopedRoleMemberOf for me",
        name: create_scoped_role_member_of,
        response: serde_json::Value,
        path: "/me/scopedRoleMemberOf",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get scopedRoleMemberOf from me",
        name: get_scoped_role_member_of,
        response: serde_json::Value,
        path: "/me/scopedRoleMemberOf/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property scopedRoleMemberOf in me",
        name: update_scoped_role_member_of,
        response: GraphResponse<Content>,
        path: "/me/scopedRoleMemberOf/{{id}}",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action sendMail",
        name: send_mail,
        response: GraphResponse<Content>,
        path: "/me/sendMail",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get settings from me",
        name: get_settings,
        response: serde_json::Value,
        path: "/me/settings",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property settings in me",
        name: update_settings,
        response: GraphResponse<Content>,
        path: "/me/settings",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get transitiveMemberOf from me",
        name: list_transitive_member_of,
        response: Collection<serde_json::Value>,
        path: "/me/transitiveMemberOf",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get transitiveMemberOf from me",
        name: get_transitive_member_of,
        response: serde_json::Value,
        path: "/me/transitiveMemberOf/{{id}}",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action translateExchangeIds",
        name: translate_exchange_ids,
        response: Collection<serde_json::Value>,
        path: "/me/translateExchangeIds",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action wipeManagedAppRegistrationsByDeviceTag",
        name: wipe_managed_app_registrations_by_device_tag,
        response: GraphResponse<Content>,
        path: "/me/wipeManagedAppRegistrationsByDeviceTag",
        params: 0,
        has_body: true
    });
}
