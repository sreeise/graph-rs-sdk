use crate::calendar::{CalendarRequest, CalendarsRequest};
use crate::calendar_groups::{CalendarGroupRequest, CalendarGroupsRequest};
use crate::calendar_view::{CalendarViewRequest, CalendarViewsRequest};
use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::events::{EventRequest, EventsRequest};
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::types::DeltaPhantom;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use reqwest::Method;

register_client!(ActivitiesRequest,);
register_client!(ChildFoldersRequest,);
register_client!(ContactFoldersRequest,);
register_client!(ContactsRequest,);
register_client!(HistoryItemsRequest,);
register_client!(InferenceClassificationRequest,);
register_client!(InsightsRequest,);
register_client!(ManagedAppRegistrationsRequest,);
register_client!(ManagedDevicesRequest,);
register_client!(MeRequest,);
register_client!(OnlineMeetingsRequest,);
register_client!(OutlookRequest,);
register_client!(SettingsRequest,);
register_client!(SharedRequest,);
register_client!(TrendingRequest,);
register_client!(UsedRequest,);

impl<'a, Client> ActivitiesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn history_items(&self) -> HistoryItemsRequest<'a, Client> {
        HistoryItemsRequest::new(self.client)
    }
    get!({
        doc: "# Invoke function recent",
        name: recent,
        response: Collection<serde_json::Value>,
        path: "/me/activities/recent()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get historyItems from me",
        name: list_history_items,
        response: Collection<serde_json::Value>,
        path: "/me/activities/{{id}}/historyItems",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to historyItems for me",
        name: create_history_items,
        response: serde_json::Value,
        path: "/me/activities/{{id}}/historyItems",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get historyItems from me",
        name: get_history_items,
        response: serde_json::Value,
        path: "/me/activities/{{id}}/historyItems/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property historyItems in me",
        name: update_history_items,
        response: GraphResponse<Content>,
        path: "/me/activities/{{id}}/historyItems/{{id2}}",
        params: 2,
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
        path: "/me/contactFolders/{{id}}/childFolders/delta()",
        params: 1,
        has_body: false
    });
}

impl<'a, Client> ContactFoldersRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn child_folders(&self) -> ChildFoldersRequest<'a, Client> {
        ChildFoldersRequest::new(self.client)
    }
    pub fn contacts(&self) -> ContactsRequest<'a, Client> {
        ContactsRequest::new(self.client)
    }
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/me/contactFolders/delta()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get childFolders from me",
        name: list_child_folders,
        response: Collection<serde_json::Value>,
        path: "/me/contactFolders/{{id}}/childFolders",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to childFolders for me",
        name: create_child_folders,
        response: serde_json::Value,
        path: "/me/contactFolders/{{id}}/childFolders",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get childFolders from me",
        name: get_child_folders,
        response: serde_json::Value,
        path: "/me/contactFolders/{{id}}/childFolders/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property childFolders in me",
        name: update_child_folders,
        response: GraphResponse<Content>,
        path: "/me/contactFolders/{{id}}/childFolders/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get contacts from me",
        name: list_contacts,
        response: Collection<serde_json::Value>,
        path: "/me/contactFolders/{{id}}/contacts",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to contacts for me",
        name: create_contacts,
        response: serde_json::Value,
        path: "/me/contactFolders/{{id}}/contacts",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get contacts from me",
        name: get_contacts,
        response: serde_json::Value,
        path: "/me/contactFolders/{{id}}/contacts/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property contacts in me",
        name: update_contacts,
        response: GraphResponse<Content>,
        path: "/me/contactFolders/{{id}}/contacts/{{id2}}",
        params: 2,
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
        path: "/me/contacts/delta()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get extensions from me",
        name: list_extensions,
        response: Collection<serde_json::Value>,
        path: "/me/contacts/{{id}}/extensions",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to extensions for me",
        name: create_extensions,
        response: serde_json::Value,
        path: "/me/contacts/{{id}}/extensions",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get extensions from me",
        name: get_extensions,
        response: serde_json::Value,
        path: "/me/contacts/{{id}}/extensions/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property extensions in me",
        name: update_extensions,
        response: GraphResponse<Content>,
        path: "/me/contacts/{{id}}/extensions/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get photo from me",
        name: get_photo,
        response: serde_json::Value,
        path: "/me/contacts/{{id}}/photo",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property photo in me",
        name: update_photo,
        response: GraphResponse<Content>,
        path: "/me/contacts/{{id}}/photo",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> HistoryItemsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get activity from me",
        name: get_activity,
        response: serde_json::Value,
        path: "/me/activities/{{id}}/historyItems/{{id2}}/activity",
        params: 2,
        has_body: false
    });
}

impl<'a, Client> InferenceClassificationRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get overrides from me",
        name: list_overrides,
        response: Collection<serde_json::Value>,
        path: "/me/inferenceClassification/overrides",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to overrides for me",
        name: create_overrides,
        response: serde_json::Value,
        path: "/me/inferenceClassification/overrides",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get overrides from me",
        name: get_overrides,
        response: serde_json::Value,
        path: "/me/inferenceClassification/overrides/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property overrides in me",
        name: update_overrides,
        response: GraphResponse<Content>,
        path: "/me/inferenceClassification/overrides/{{id}}",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> InsightsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn shared(&self) -> SharedRequest<'a, Client> {
        SharedRequest::new(self.client)
    }
    pub fn trending(&self) -> TrendingRequest<'a, Client> {
        TrendingRequest::new(self.client)
    }
    pub fn used(&self) -> UsedRequest<'a, Client> {
        UsedRequest::new(self.client)
    }
    get!({
        doc: "# Get shared from me",
        name: list_shared,
        response: Collection<serde_json::Value>,
        path: "/me/insights/shared",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to shared for me",
        name: create_shared,
        response: serde_json::Value,
        path: "/me/insights/shared",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get shared from me",
        name: get_shared,
        response: serde_json::Value,
        path: "/me/insights/shared/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property shared in me",
        name: update_shared,
        response: GraphResponse<Content>,
        path: "/me/insights/shared/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get trending from me",
        name: list_trending,
        response: Collection<serde_json::Value>,
        path: "/me/insights/trending",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to trending for me",
        name: create_trending,
        response: serde_json::Value,
        path: "/me/insights/trending",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get trending from me",
        name: get_trending,
        response: serde_json::Value,
        path: "/me/insights/trending/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property trending in me",
        name: update_trending,
        response: GraphResponse<Content>,
        path: "/me/insights/trending/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get used from me",
        name: list_used,
        response: Collection<serde_json::Value>,
        path: "/me/insights/used",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to used for me",
        name: create_used,
        response: serde_json::Value,
        path: "/me/insights/used",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get used from me",
        name: get_used,
        response: serde_json::Value,
        path: "/me/insights/used/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property used in me",
        name: update_used,
        response: GraphResponse<Content>,
        path: "/me/insights/used/{{id}}",
        params: 1,
        has_body: true
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
        path: "/me/managedAppRegistrations/getUserIdsWithFlaggedAppRegistration()",
        params: 0,
        has_body: false
    });
}

impl<'a, Client> ManagedDevicesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action bypassActivationLock",
        name: bypass_activation_lock,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/bypassActivationLock",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action cleanWindowsDevice",
        name: clean_windows_device,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/cleanWindowsDevice",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action deleteUserFromSharedAppleDevice",
        name: delete_user_from_shared_apple_device,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/deleteUserFromSharedAppleDevice",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get deviceCategory from me",
        name: get_device_category,
        response: serde_json::Value,
        path: "/me/managedDevices/{{id}}/deviceCategory",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property deviceCategory in me",
        name: update_device_category,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/deviceCategory",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get deviceCompliancePolicyStates from me",
        name: list_device_compliance_policy_states,
        response: Collection<serde_json::Value>,
        path: "/me/managedDevices/{{id}}/deviceCompliancePolicyStates",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to deviceCompliancePolicyStates for me",
        name: create_device_compliance_policy_states,
        response: serde_json::Value,
        path: "/me/managedDevices/{{id}}/deviceCompliancePolicyStates",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get deviceCompliancePolicyStates from me",
        name: get_device_compliance_policy_states,
        response: serde_json::Value,
        path: "/me/managedDevices/{{id}}/deviceCompliancePolicyStates/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property deviceCompliancePolicyStates in me",
        name: update_device_compliance_policy_states,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/deviceCompliancePolicyStates/{{id2}}",
        params: 2,
        has_body: true
    });
    get!({
        doc: "# Get deviceConfigurationStates from me",
        name: list_device_configuration_states,
        response: Collection<serde_json::Value>,
        path: "/me/managedDevices/{{id}}/deviceConfigurationStates",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to deviceConfigurationStates for me",
        name: create_device_configuration_states,
        response: serde_json::Value,
        path: "/me/managedDevices/{{id}}/deviceConfigurationStates",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get deviceConfigurationStates from me",
        name: get_device_configuration_states,
        response: serde_json::Value,
        path: "/me/managedDevices/{{id}}/deviceConfigurationStates/{{id2}}",
        params: 2,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property deviceConfigurationStates in me",
        name: update_device_configuration_states,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/deviceConfigurationStates/{{id2}}",
        params: 2,
        has_body: true
    });
    post!({
        doc: "# Invoke action disableLostMode",
        name: disable_lost_mode,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/disableLostMode",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action locateDevice",
        name: locate_device,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/locateDevice",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action logoutSharedAppleDeviceActiveUser",
        name: logout_shared_apple_device_active_user,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/logoutSharedAppleDeviceActiveUser",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action rebootNow",
        name: reboot_now,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/rebootNow",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action recoverPasscode",
        name: recover_passcode,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/recoverPasscode",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action remoteLock",
        name: remote_lock,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/remoteLock",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action requestRemoteAssistance",
        name: request_remote_assistance,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/requestRemoteAssistance",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action resetPasscode",
        name: reset_passcode,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/resetPasscode",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action retire",
        name: retire,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/retire",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action shutDown",
        name: shut_down,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/shutDown",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action syncDevice",
        name: sync_device,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/syncDevice",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action updateWindowsDeviceAccount",
        name: update_windows_device_account,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/updateWindowsDeviceAccount",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action windowsDefenderScan",
        name: windows_defender_scan,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/windowsDefenderScan",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action windowsDefenderUpdateSignatures",
        name: windows_defender_update_signatures,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/windowsDefenderUpdateSignatures",
        params: 1,
        has_body: false
    });
    post!({
        doc: "# Invoke action wipe",
        name: wipe,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}/wipe",
        params: 1,
        has_body: true
    });
}

impl<'a, Client> MeRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn activities(&self) -> ActivitiesRequest<'a, Client> {
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
    pub fn contact_folders(&self) -> ContactFoldersRequest<'a, Client> {
        ContactFoldersRequest::new(self.client)
    }
    pub fn contacts(&self) -> ContactsRequest<'a, Client> {
        ContactsRequest::new(self.client)
    }
    pub fn events(&self) -> EventRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
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
        InferenceClassificationRequest::new(self.client)
    }
    pub fn insights(&self) -> InsightsRequest<'a, Client> {
        InsightsRequest::new(self.client)
    }
    pub fn managed_app_registrations(&self) -> ManagedAppRegistrationsRequest<'a, Client> {
        ManagedAppRegistrationsRequest::new(self.client)
    }
    pub fn managed_devices(&self) -> ManagedDevicesRequest<'a, Client> {
        ManagedDevicesRequest::new(self.client)
    }
    pub fn online_meetings(&self) -> OnlineMeetingsRequest<'a, Client> {
        OnlineMeetingsRequest::new(self.client)
    }
    pub fn outlook(&self) -> OutlookRequest<'a, Client> {
        OutlookRequest::new(self.client)
    }
    pub fn settings(&self) -> SettingsRequest<'a, Client> {
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
        doc: "# Get activities from me",
        name: list_activities,
        response: Collection<serde_json::Value>,
        path: "/me/activities",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to activities for me",
        name: create_activities,
        response: serde_json::Value,
        path: "/me/activities",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get activities from me",
        name: get_activities,
        response: serde_json::Value,
        path: "/me/activities/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property activities in me",
        name: update_activities,
        response: GraphResponse<Content>,
        path: "/me/activities/{{id}}",
        params: 1,
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
        doc: "# Get contactFolders from me",
        name: list_contact_folders,
        response: Collection<serde_json::Value>,
        path: "/me/contactFolders",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to contactFolders for me",
        name: create_contact_folders,
        response: serde_json::Value,
        path: "/me/contactFolders",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get contactFolders from me",
        name: get_contact_folders,
        response: serde_json::Value,
        path: "/me/contactFolders/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property contactFolders in me",
        name: update_contact_folders,
        response: GraphResponse<Content>,
        path: "/me/contactFolders/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get contacts from me",
        name: list_contacts,
        response: Collection<serde_json::Value>,
        path: "/me/contacts",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to contacts for me",
        name: create_contacts,
        response: serde_json::Value,
        path: "/me/contacts",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get contacts from me",
        name: get_contacts,
        response: serde_json::Value,
        path: "/me/contacts/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property contacts in me",
        name: update_contacts,
        response: GraphResponse<Content>,
        path: "/me/contacts/{{id}}",
        params: 1,
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
        doc: "# Get inferenceClassification from me",
        name: get_inference_classification,
        response: serde_json::Value,
        path: "/me/inferenceClassification",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property inferenceClassification in me",
        name: update_inference_classification,
        response: GraphResponse<Content>,
        path: "/me/inferenceClassification",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get insights from me",
        name: get_insights,
        response: serde_json::Value,
        path: "/me/insights",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property insights in me",
        name: update_insights,
        response: GraphResponse<Content>,
        path: "/me/insights",
        params: 0,
        has_body: true
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
        doc: "# Get managedDevices from me",
        name: list_managed_devices,
        response: Collection<serde_json::Value>,
        path: "/me/managedDevices",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to managedDevices for me",
        name: create_managed_devices,
        response: serde_json::Value,
        path: "/me/managedDevices",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get managedDevices from me",
        name: get_managed_devices,
        response: serde_json::Value,
        path: "/me/managedDevices/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property managedDevices in me",
        name: update_managed_devices,
        response: GraphResponse<Content>,
        path: "/me/managedDevices/{{id}}",
        params: 1,
        has_body: true
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
        doc: "# Get onlineMeetings from me",
        name: list_online_meetings,
        response: Collection<serde_json::Value>,
        path: "/me/onlineMeetings",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to onlineMeetings for me",
        name: create_online_meetings,
        response: serde_json::Value,
        path: "/me/onlineMeetings",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get onlineMeetings from me",
        name: get_online_meetings,
        response: serde_json::Value,
        path: "/me/onlineMeetings/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property onlineMeetings in me",
        name: update_online_meetings,
        response: GraphResponse<Content>,
        path: "/me/onlineMeetings/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get outlook from me",
        name: get_outlook,
        response: serde_json::Value,
        path: "/me/outlook",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property outlook in me",
        name: update_outlook,
        response: GraphResponse<Content>,
        path: "/me/outlook",
        params: 0,
        has_body: true
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

impl<'a, Client> OnlineMeetingsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action createOrGet",
        name: create_or_get,
        response: serde_json::Value,
        path: "/me/onlineMeetings/createOrGet",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> OutlookRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get masterCategories from me",
        name: list_master_categories,
        response: Collection<serde_json::Value>,
        path: "/me/outlook/masterCategories",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to masterCategories for me",
        name: create_master_categories,
        response: serde_json::Value,
        path: "/me/outlook/masterCategories",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get masterCategories from me",
        name: get_master_categories,
        response: serde_json::Value,
        path: "/me/outlook/masterCategories/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property masterCategories in me",
        name: update_master_categories,
        response: GraphResponse<Content>,
        path: "/me/outlook/masterCategories/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Invoke function supportedLanguages",
        name: supported_languages,
        response: Collection<serde_json::Value>,
        path: "/me/outlook/supportedLanguages()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Invoke function supportedTimeZones",
        name: supported_time_zones,
        response: Collection<serde_json::Value>,
        path: "/me/outlook/supportedTimeZones()",
        params: 0,
        has_body: false
    });
}

impl<'a, Client> SettingsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get shiftPreferences from me",
        name: get_shift_preferences,
        response: serde_json::Value,
        path: "/me/settings/shiftPreferences",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property shiftPreferences in me",
        name: update_shift_preferences,
        response: GraphResponse<Content>,
        path: "/me/settings/shiftPreferences",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> SharedRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get lastSharedMethod from me",
        name: get_last_shared_method,
        response: serde_json::Value,
        path: "/me/insights/shared/{{id}}/lastSharedMethod",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get resource from me",
        name: get_resource,
        response: serde_json::Value,
        path: "/me/insights/shared/{{id}}/resource",
        params: 1,
        has_body: false
    });
}

impl<'a, Client> TrendingRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get resource from me",
        name: get_resource,
        response: serde_json::Value,
        path: "/me/insights/trending/{{id}}/resource",
        params: 1,
        has_body: false
    });
}

impl<'a, Client> UsedRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get resource from me",
        name: get_resource,
        response: serde_json::Value,
        path: "/me/insights/used/{{id}}/resource",
        params: 1,
        has_body: false
    });
}
