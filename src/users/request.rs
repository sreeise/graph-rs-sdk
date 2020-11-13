use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::types::DeltaPhantom;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(UsersRequest, ());
register_client!(ActivitiesRequest,);
register_client!(ChildFoldersRequest,);
register_client!(ContactFolderContactRequest,);
register_client!(ContactFoldersRequest,);
register_client!(ContactsRequest,);
register_client!(EventsRequest,);
register_client!(HistoryItemsRequest,);
register_client!(InferenceClassificationRequest,);
register_client!(InsightsRequest,);
register_client!(InstancesRequest,);
register_client!(ManagedAppRegistrationsRequest,);
register_client!(ManagedDevicesRequest,);
register_client!(OnlineMeetingsRequest,);
register_client!(OutlookRequest,);
register_client!(SettingsRequest,);
register_client!(SharedRequest,);
register_client!(TrendingRequest,);
register_client!(UsedRequest,);
register_client!(UserRequest,);

impl<'a, Client> UserRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> UsersRequest<'a, Client> {
        UsersRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/users/delta()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get entities from users",
        name: list_user,
        response: Collection<serde_json::Value>,
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
}

impl<'a, Client> UsersRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn activities(&self) -> ActivitiesRequest<'a, Client> {
        ActivitiesRequest::new(&self.client)
    }
    pub fn contact_folders(&self) -> ContactFoldersRequest<'a, Client> {
        ContactFoldersRequest::new(&self.client)
    }
    pub fn contacts(&self) -> ContactsRequest<'a, Client> {
        ContactsRequest::new(&self.client)
    }
    pub fn events(&self) -> EventsRequest<'a, Client> {
        EventsRequest::new(&self.client)
    }
    pub fn inference_classification(&self) -> InferenceClassificationRequest<'a, Client> {
        InferenceClassificationRequest::new(&self.client)
    }
    pub fn insights(&self) -> InsightsRequest<'a, Client> {
        InsightsRequest::new(&self.client)
    }
    pub fn managed_app_registrations(&self) -> ManagedAppRegistrationsRequest<'a, Client> {
        ManagedAppRegistrationsRequest::new(&self.client)
    }
    pub fn managed_devices(&self) -> ManagedDevicesRequest<'a, Client> {
        ManagedDevicesRequest::new(&self.client)
    }
    pub fn online_meetings(&self) -> OnlineMeetingsRequest<'a, Client> {
        OnlineMeetingsRequest::new(&self.client)
    }
    pub fn outlook(&self) -> OutlookRequest<'a, Client> {
        OutlookRequest::new(&self.client)
    }
    pub fn settings(&self) -> SettingsRequest<'a, Client> {
        SettingsRequest::new(&self.client)
    }
    get!({
        doc: "# Get activities from users",
        name: get_activities,
        response: serde_json::Value,
        path: "/users/{{RID}}/activities/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property activities in users",
        name: update_activities,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/activities/{{id}}",
        params: 1,
        has_body: true
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
        doc: "# Get contacts from users",
        name: list_contacts,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/contacts",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to contacts for users",
        name: create_contacts,
        response: serde_json::Value,
        path: "/users/{{RID}}/contacts",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get ownedDevices from users",
        name: get_owned_devices,
        response: serde_json::Value,
        path: "/users/{{RID}}/ownedDevices/{{id}}",
        params: 1,
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
    get!({
        doc: "# Get directReports from users",
        name: get_direct_reports,
        response: serde_json::Value,
        path: "/users/{{RID}}/directReports/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get contactFolders from users",
        name: get_contact_folders,
        response: serde_json::Value,
        path: "/users/{{RID}}/contactFolders/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property contactFolders in users",
        name: update_contact_folders,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/contactFolders/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get people from users",
        name: list_people,
        response: Collection<serde_json::Value>,
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
        doc: "# Get registeredDevices from users",
        name: get_registered_devices,
        response: serde_json::Value,
        path: "/users/{{RID}}/registeredDevices/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get outlook from users",
        name: get_outlook,
        response: serde_json::Value,
        path: "/users/{{RID}}/outlook",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property outlook in users",
        name: update_outlook,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/outlook",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get inferenceClassification from users",
        name: get_inference_classification,
        response: serde_json::Value,
        path: "/users/{{RID}}/inferenceClassification",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property inferenceClassification in users",
        name: update_inference_classification,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/inferenceClassification",
        params: 0,
        has_body: true
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
        doc: "# Get manager from users",
        name: get_manager,
        response: serde_json::Value,
        path: "/users/{{RID}}/manager",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action wipeManagedAppRegistrationsByDeviceTag",
        name: wipe_managed_app_registrations_by_device_tag,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/wipeManagedAppRegistrationsByDeviceTag",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get joinedTeams from users",
        name: list_joined_teams,
        response: Collection<serde_json::Value>,
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
        doc: "# Get events from users",
        name: get_events,
        response: serde_json::Value,
        path: "/users/{{RID}}/events/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property events in users",
        name: update_events,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/events/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get ownedObjects from users",
        name: list_owned_objects,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/ownedObjects",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get followedSites from users",
        name: list_followed_sites,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/followedSites",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action getMailTips",
        name: get_mail_tips,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/getMailTips",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get managedAppRegistrations from users",
        name: list_managed_app_registrations,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/managedAppRegistrations",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get memberOf from users",
        name: list_member_of,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/memberOf",
        params: 0,
        has_body: false
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
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/photo",
        params: 0,
        has_body: true
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
        doc: "# Get events from users",
        name: list_events,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/events",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to events for users",
        name: create_events,
        response: serde_json::Value,
        path: "/users/{{RID}}/events",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function getManagedAppPolicies",
        name: get_managed_app_policies,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/getManagedAppPolicies()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get extensions from users",
        name: list_extensions,
        response: Collection<serde_json::Value>,
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
        doc: "# Get managedAppRegistrations from users",
        name: get_managed_app_registrations,
        response: serde_json::Value,
        path: "/users/{{RID}}/managedAppRegistrations/{{id}}",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action translateExchangeIds",
        name: translate_exchange_ids,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/translateExchangeIds",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get managedDevices from users",
        name: list_managed_devices,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/managedDevices",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to managedDevices for users",
        name: create_managed_devices,
        response: serde_json::Value,
        path: "/users/{{RID}}/managedDevices",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get drives from users",
        name: list_drives,
        response: Collection<serde_json::Value>,
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
        doc: "# Get createdObjects from users",
        name: list_created_objects,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/createdObjects",
        params: 0,
        has_body: false
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
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/scopedRoleMemberOf/{{id}}",
        params: 1,
        has_body: true
    });
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
        response: GraphResponse<Content>,
        path: "/users/{{RID}}",
        params: 0,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from users",
        name: delete_user,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}",
        params: 0,
        has_body: false
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
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/extensions/{{id}}",
        params: 1,
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
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/appRoleAssignments/{{id}}",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action exportPersonalData",
        name: export_personal_data,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/exportPersonalData",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action removeAllDevicesFromManagement",
        name: remove_all_devices_from_management,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/removeAllDevicesFromManagement",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get directReports from users",
        name: list_direct_reports,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/directReports",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get managedDevices from users",
        name: get_managed_devices,
        response: serde_json::Value,
        path: "/users/{{RID}}/managedDevices/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property managedDevices in users",
        name: update_managed_devices,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get onlineMeetings from users",
        name: get_online_meetings,
        response: serde_json::Value,
        path: "/users/{{RID}}/onlineMeetings/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property onlineMeetings in users",
        name: update_online_meetings,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/onlineMeetings/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get contactFolders from users",
        name: list_contact_folders,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/contactFolders",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to contactFolders for users",
        name: create_contact_folders,
        response: serde_json::Value,
        path: "/users/{{RID}}/contactFolders",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get activities from users",
        name: list_activities,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/activities",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to activities for users",
        name: create_activities,
        response: serde_json::Value,
        path: "/users/{{RID}}/activities",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get transitiveMemberOf from users",
        name: list_transitive_member_of,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/transitiveMemberOf",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get ownedDevices from users",
        name: list_owned_devices,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/ownedDevices",
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
    get!({
        doc: "# Get registeredDevices from users",
        name: list_registered_devices,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/registeredDevices",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Invoke function getManagedAppDiagnosticStatuses",
        name: get_managed_app_diagnostic_statuses,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/getManagedAppDiagnosticStatuses()",
        params: 0,
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
        doc: "# Get licenseDetails from users",
        name: list_license_details,
        response: Collection<serde_json::Value>,
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
    post!({
        doc: "# Invoke action changePassword",
        name: change_password,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/changePassword",
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
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/licenseDetails/{{id}}",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action sendMail",
        name: send_mail,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/sendMail",
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
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/drives/{{id}}",
        params: 1,
        has_body: true
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
        doc: "# Get oauth2PermissionGrants from users",
        name: users_get_oauth_2_permission_grants,
        response: serde_json::Value,
        path: "/users/{{RID}}/oauth2PermissionGrants/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get scopedRoleMemberOf from users",
        name: list_scoped_role_member_of,
        response: Collection<serde_json::Value>,
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
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/joinedTeams/{{id}}",
        params: 1,
        has_body: true
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
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/drive",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get photos from users",
        name: list_photos,
        response: Collection<serde_json::Value>,
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
        doc: "# Get contacts from users",
        name: get_contacts,
        response: serde_json::Value,
        path: "/users/{{RID}}/contacts/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property contacts in users",
        name: update_contacts,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/contacts/{{id}}",
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
    get!({
        doc: "# Get appRoleAssignments from users",
        name: list_app_role_assignments,
        response: Collection<serde_json::Value>,
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
        doc: "# Get deviceManagementTroubleshootingEvents from users",
        name: list_device_management_troubleshooting_events,
        response: Collection<serde_json::Value>,
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
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/deviceManagementTroubleshootingEvents/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get onlineMeetings from users",
        name: list_online_meetings,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/onlineMeetings",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to onlineMeetings for users",
        name: create_online_meetings,
        response: serde_json::Value,
        path: "/users/{{RID}}/onlineMeetings",
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
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/photos/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get followedSites from users",
        name: get_followed_sites,
        response: serde_json::Value,
        path: "/users/{{RID}}/followedSites/{{id}}",
        params: 1,
        has_body: false
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
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/people/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get insights from users",
        name: get_insights,
        response: serde_json::Value,
        path: "/users/{{RID}}/insights",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property insights in users",
        name: update_insights,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/insights",
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
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/settings",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> ActivitiesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn history_items(&self) -> HistoryItemsRequest<'a, Client> {
        HistoryItemsRequest::new(&self.client)
    }
    get!({
        doc: "# Get historyItems from users",
        name: list_history_items,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/activities/{{id}}/historyItems",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to historyItems for users",
        name: create_history_items,
        response: serde_json::Value,
        path: "/users/{{RID}}/activities/{{id}}/historyItems",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Invoke function recent",
        name: recent,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/activities/recent()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get historyItems from users",
        name: get_history_items,
        response: serde_json::Value,
        path: "/users/{{RID}}/activities/{{id}}/historyItems/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property historyItems in users",
        name: update_history_items,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/activities/{{id}}/historyItems/{{id2}}",
        params: 2,
        has_body: true
    });
}

impl<'a, Client> HistoryItemsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get activity from users",
        name: get_activity,
        response: serde_json::Value,
        path: "/users/{{RID}}/activities/{{id}}/historyItems/{{id2}}/activity",
        params: 2,
        has_body: false
    });
}

impl<'a, Client> ContactFoldersRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn child_folders(&self) -> ChildFoldersRequest<'a, Client> {
        ChildFoldersRequest::new(&self.client)
    }
    pub fn contact_folder_contact(&self) -> ContactFolderContactRequest<'a, Client> {
        ContactFolderContactRequest::new(&self.client)
    }
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/users/{{RID}}/contactFolders/delta()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get childFolders from users",
        name: list_child_folders,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/contactFolders/{{id}}/childFolders",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to childFolders for users",
        name: create_child_folders,
        response: serde_json::Value,
        path: "/users/{{RID}}/contactFolders/{{id}}/childFolders",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get contacts from users",
        name: get_contacts,
        response: serde_json::Value,
        path: "/users/{{RID}}/contactFolders/{{id}}/contacts/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property contacts in users",
        name: update_contacts,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/contactFolders/{{id}}/contacts/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get childFolders from users",
        name: get_child_folders,
        response: serde_json::Value,
        path: "/users/{{RID}}/contactFolders/{{id}}/childFolders/{{id}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property childFolders in users",
        name: update_child_folders,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/contactFolders/{{id}}/childFolders/{{id}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get contacts from users",
        name: list_contacts,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/contactFolders/{{id}}/contacts",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to contacts for users",
        name: create_contacts,
        response: serde_json::Value,
        path: "/users/{{RID}}/contactFolders/{{id}}/contacts",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> ChildFoldersRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/users/{{RID}}/contactFolders/{{id}}/childFolders/delta()",
        params: 1,
        has_body: false
    });
}

impl<'a, Client> ContactFolderContactRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/users/{{RID}}/contactFolders/{{id}}/contacts/delta()",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get photo from users",
        name: get_photo,
        response: serde_json::Value,
        path: "/users/{{RID}}/contactFolders/{{id}}/contacts/{{id2}}/photo",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property photo in users",
        name: update_photo,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/contactFolders/{{id}}/contacts/{{id2}}/photo",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get extensions from users",
        name: list_extensions,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/contactFolders/{{id}}/contacts/{{id2}}/extensions",
        params: 2,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to extensions for users",
        name: create_extensions,
        response: serde_json::Value,
        path: "/users/{{RID}}/contactFolders/{{id}}/contacts/{{id2}}/extensions",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get extensions from users",
        name: get_extensions,
        response: serde_json::Value,
        path: "/users/{{RID}}/contactFolders/{{id}}/contacts/{{id2}}/extensions/{{id3}}",
        params: 3,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property extensions in users",
        name: update_extensions,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/contactFolders/{{id}}/contacts/{{id2}}/extensions/{{id3}}",
        params: 3,
        has_body: true
    });
}

impl<'a, Client> ContactsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/users/{{RID}}/contacts/delta()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get extensions from users",
        name: get_extensions,
        response: serde_json::Value,
        path: "/users/{{RID}}/contacts/{{id}}/extensions/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property extensions in users",
        name: update_extensions,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/contacts/{{id}}/extensions/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get extensions from users",
        name: list_extensions,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/contacts/{{id}}/extensions",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to extensions for users",
        name: create_extensions,
        response: serde_json::Value,
        path: "/users/{{RID}}/contacts/{{id}}/extensions",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get photo from users",
        name: get_photo,
        response: serde_json::Value,
        path: "/users/{{RID}}/contacts/{{id}}/photo",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property photo in users",
        name: update_photo,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/contacts/{{id}}/photo",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> EventsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn instances(&self) -> InstancesRequest<'a, Client> {
        InstancesRequest::new(&self.client)
    }
    get!({
        doc: "# Get instances from users",
        name: list_instances,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/events/{{id}}/instances",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to instances for users",
        name: create_instances,
        response: serde_json::Value,
        path: "/users/{{RID}}/events/{{id}}/instances",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action decline",
        name: decline,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/events/{{id}}/decline",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action snoozeReminder",
        name: snooze_reminder,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/events/{{id}}/snoozeReminder",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action dismissReminder",
        name: dismiss_reminder,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/events/{{id}}/dismissReminder",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get instances from users",
        name: get_instances,
        response: serde_json::Value,
        path: "/users/{{RID}}/events/{{id}}/instances/{{id}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property instances in users",
        name: update_instances,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/events/{{id}}/instances/{{id}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/users/{{RID}}/events/delta()",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action accept",
        name: accept,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/events/{{id}}/accept",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get extensions from users",
        name: get_extensions,
        response: serde_json::Value,
        path: "/users/{{RID}}/events/{{id}}/extensions/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property extensions in users",
        name: update_extensions,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/events/{{id}}/extensions/{{id2}}",
        params: 2,
        has_body: true
    });
    post!({
        doc: "# Invoke action tentativelyAccept",
        name: tentatively_accept,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/events/{{id}}/tentativelyAccept",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get extensions from users",
        name: list_extensions,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/events/{{id}}/extensions",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to extensions for users",
        name: create_extensions,
        response: serde_json::Value,
        path: "/users/{{RID}}/events/{{id}}/extensions",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> InstancesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action accept",
        name: accept,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/events/{{id}}/instances/{{id}}/accept",
        params: 2,
        has_body: true
    });
    post!({
        doc: "# Invoke action decline",
        name: decline,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/events/{{id}}/instances/{{id}}/decline",
        params: 2,
        has_body: true
    });
    post!({
        doc: "# Invoke action snoozeReminder",
        name: snooze_reminder,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/events/{{id}}/instances/{{id}}/snoozeReminder",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/users/{{RID}}/events/{{id}}/instances/delta()",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action dismissReminder",
        name: dismiss_reminder,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/events/{{id}}/instances/{{id}}/dismissReminder",
        params: 2,
        has_body: false
    });
    post!({
        doc: "# Invoke action tentativelyAccept",
        name: tentatively_accept,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/events/{{id}}/instances/{{id}}/tentativelyAccept",
        params: 2,
        has_body: true
    });
}

impl<'a, Client> InferenceClassificationRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get overrides from users",
        name: list_overrides,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/inferenceClassification/overrides",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to overrides for users",
        name: create_overrides,
        response: serde_json::Value,
        path: "/users/{{RID}}/inferenceClassification/overrides",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get overrides from users",
        name: get_overrides,
        response: serde_json::Value,
        path: "/users/{{RID}}/inferenceClassification/overrides/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property overrides in users",
        name: update_overrides,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/inferenceClassification/overrides/{{id}}",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> InsightsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn shared(&self) -> SharedRequest<'a, Client> {
        SharedRequest::new(&self.client)
    }
    pub fn trending(&self) -> TrendingRequest<'a, Client> {
        TrendingRequest::new(&self.client)
    }
    pub fn used(&self) -> UsedRequest<'a, Client> {
        UsedRequest::new(&self.client)
    }
    get!({
        doc: "# Get trending from users",
        name: get_trending,
        response: serde_json::Value,
        path: "/users/{{RID}}/insights/trending/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property trending in users",
        name: update_trending,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/insights/trending/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get trending from users",
        name: list_trending,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/insights/trending",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to trending for users",
        name: create_trending,
        response: serde_json::Value,
        path: "/users/{{RID}}/insights/trending",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get shared from users",
        name: get_shared,
        response: serde_json::Value,
        path: "/users/{{RID}}/insights/shared/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property shared in users",
        name: update_shared,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/insights/shared/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get used from users",
        name: get_used,
        response: serde_json::Value,
        path: "/users/{{RID}}/insights/used/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property used in users",
        name: update_used,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/insights/used/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get shared from users",
        name: list_shared,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/insights/shared",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to shared for users",
        name: create_shared,
        response: serde_json::Value,
        path: "/users/{{RID}}/insights/shared",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get used from users",
        name: list_used,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/insights/used",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to used for users",
        name: create_used,
        response: serde_json::Value,
        path: "/users/{{RID}}/insights/used",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> SharedRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get resource from users",
        name: get_resource,
        response: serde_json::Value,
        path: "/users/{{RID}}/insights/shared/{{id}}/resource",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get lastSharedMethod from users",
        name: get_last_shared_method,
        response: serde_json::Value,
        path: "/users/{{RID}}/insights/shared/{{id}}/lastSharedMethod",
        params: 1,
        has_body: false
    });
}

impl<'a, Client> TrendingRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get resource from users",
        name: get_resource,
        response: serde_json::Value,
        path: "/users/{{RID}}/insights/trending/{{id}}/resource",
        params: 1,
        has_body: false
    });
}

impl<'a, Client> UsedRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get resource from users",
        name: get_resource,
        response: serde_json::Value,
        path: "/users/{{RID}}/insights/used/{{id}}/resource",
        params: 1,
        has_body: false
    });
}

impl<'a, Client> ManagedAppRegistrationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Invoke function getUserIdsWithFlaggedAppRegistration",
        name: get_user_ids_with_flagged_app_registration,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/managedAppRegistrations/getUserIdsWithFlaggedAppRegistration()",
        params: 0,
        has_body: false
    });
}

impl<'a, Client> ManagedDevicesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action disableLostMode",
        name: disable_lost_mode,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/disableLostMode",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action syncDevice",
        name: sync_device,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/syncDevice",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get deviceConfigurationStates from users",
        name: get_device_configuration_states,
        response: serde_json::Value,
        path: "/users/{{RID}}/managedDevices/{{id}}/deviceConfigurationStates/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property deviceConfigurationStates in users",
        name: update_device_configuration_states,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/deviceConfigurationStates/{{id2}}",
        params: 2,
        has_body: true
    });
    post!({
        doc: "# Invoke action rebootNow",
        name: reboot_now,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/rebootNow",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action wipe",
        name: wipe,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/wipe",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get deviceConfigurationStates from users",
        name: list_device_configuration_states,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/managedDevices/{{id}}/deviceConfigurationStates",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to deviceConfigurationStates for users",
        name: create_device_configuration_states,
        response: serde_json::Value,
        path: "/users/{{RID}}/managedDevices/{{id}}/deviceConfigurationStates",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action deleteUserFromSharedAppleDevice",
        name: delete_user_from_shared_apple_device,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/deleteUserFromSharedAppleDevice",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action retire",
        name: retire,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/retire",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get deviceCompliancePolicyStates from users",
        name: get_device_compliance_policy_states,
        response: serde_json::Value,
        path: "/users/{{RID}}/managedDevices/{{id}}/deviceCompliancePolicyStates/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property deviceCompliancePolicyStates in users",
        name: update_device_compliance_policy_states,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/deviceCompliancePolicyStates/{{id2}}",
        params: 2,
        has_body: true
    });
    post!({
        doc: "# Invoke action bypassActivationLock",
        name: bypass_activation_lock,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/bypassActivationLock",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action recoverPasscode",
        name: recover_passcode,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/recoverPasscode",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action windowsDefenderScan",
        name: windows_defender_scan,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/windowsDefenderScan",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action resetPasscode",
        name: reset_passcode,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/resetPasscode",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action remoteLock",
        name: remote_lock,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/remoteLock",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action shutDown",
        name: shut_down,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/shutDown",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action logoutSharedAppleDeviceActiveUser",
        name: logout_shared_apple_device_active_user,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/logoutSharedAppleDeviceActiveUser",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get deviceCategory from users",
        name: get_device_category,
        response: serde_json::Value,
        path: "/users/{{RID}}/managedDevices/{{id}}/deviceCategory",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property deviceCategory in users",
        name: update_device_category,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/deviceCategory",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action windowsDefenderUpdateSignatures",
        name: windows_defender_update_signatures,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/windowsDefenderUpdateSignatures",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action cleanWindowsDevice",
        name: clean_windows_device,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/cleanWindowsDevice",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get deviceCompliancePolicyStates from users",
        name: list_device_compliance_policy_states,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/managedDevices/{{id}}/deviceCompliancePolicyStates",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to deviceCompliancePolicyStates for users",
        name: create_device_compliance_policy_states,
        response: serde_json::Value,
        path: "/users/{{RID}}/managedDevices/{{id}}/deviceCompliancePolicyStates",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action locateDevice",
        name: locate_device,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/locateDevice",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action requestRemoteAssistance",
        name: request_remote_assistance,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/requestRemoteAssistance",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action updateWindowsDeviceAccount",
        name: update_windows_device_account,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/managedDevices/{{id}}/updateWindowsDeviceAccount",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> OnlineMeetingsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action createOrGet",
        name: create_or_get,
        response: serde_json::Value,
        path: "/users/{{RID}}/onlineMeetings/createOrGet",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> OutlookRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get masterCategories from users",
        name: get_master_categories,
        response: serde_json::Value,
        path: "/users/{{RID}}/outlook/masterCategories/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property masterCategories in users",
        name: update_master_categories,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/outlook/masterCategories/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Invoke function supportedLanguages",
        name: supported_languages,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/outlook/supportedLanguages()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Invoke function supportedTimeZones",
        name: supported_time_zones,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/outlook/supportedTimeZones()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get masterCategories from users",
        name: list_master_categories,
        response: Collection<serde_json::Value>,
        path: "/users/{{RID}}/outlook/masterCategories",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to masterCategories for users",
        name: create_master_categories,
        response: serde_json::Value,
        path: "/users/{{RID}}/outlook/masterCategories",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> SettingsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get shiftPreferences from users",
        name: get_shift_preferences,
        response: serde_json::Value,
        path: "/users/{{RID}}/settings/shiftPreferences",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property shiftPreferences in users",
        name: update_shift_preferences,
        response: GraphResponse<Content>,
        path: "/users/{{RID}}/settings/shiftPreferences",
        params: 0,
        has_body: true
    });
}
