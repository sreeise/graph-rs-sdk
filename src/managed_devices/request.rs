use crate::client::Graph;
use crate::core::ResourceIdentity;
use graph_http::types::Collection;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(ManagedDeviceRequest,);
register_client!(ManagedDevicesRequest, ());

impl<'a, Client> ManagedDeviceRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> ManagedDevicesRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::ManagedDevices);
        ManagedDevicesRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get managedDevices from me",
        name: list_managed_devices,
        response: Collection<serde_json::Value>,
        path: "/managedDevices",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to managedDevices for me",
        name: create_managed_devices,
        response: serde_json::Value,
        path: "/managedDevices",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> ManagedDevicesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action bypassActivationLock",
        name: bypass_activation_lock,
        response: NoContent,
        path: "/managedDevices/{{RID}}/bypassActivationLock",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action cleanWindowsDevice",
        name: clean_windows_device,
        response: NoContent,
        path: "/managedDevices/{{RID}}/cleanWindowsDevice",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action deleteUserFromSharedAppleDevice",
        name: delete_user_from_shared_apple_device,
        response: NoContent,
        path: "/managedDevices/{{RID}}/deleteUserFromSharedAppleDevice",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get deviceCategory from me",
        name: get_device_category,
        response: serde_json::Value,
        path: "/managedDevices/{{RID}}/deviceCategory",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property deviceCategory in me",
        name: update_device_category,
        response: NoContent,
        path: "/managedDevices/{{RID}}/deviceCategory",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get deviceCompliancePolicyStates from me",
        name: list_device_compliance_policy_states,
        response: Collection<serde_json::Value>,
        path: "/managedDevices/{{RID}}/deviceCompliancePolicyStates",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to deviceCompliancePolicyStates for me",
        name: create_device_compliance_policy_states,
        response: serde_json::Value,
        path: "/managedDevices/{{RID}}/deviceCompliancePolicyStates",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get deviceCompliancePolicyStates from me",
        name: get_device_compliance_policy_states,
        response: serde_json::Value,
        path: "/managedDevices/{{RID}}/deviceCompliancePolicyStates/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property deviceCompliancePolicyStates in me",
        name: update_device_compliance_policy_states,
        response: NoContent,
        path: "/managedDevices/{{RID}}/deviceCompliancePolicyStates/{{id}}",
        params: 1,
        has_body: true
    });
    get!({
        doc: "# Get deviceConfigurationStates from me",
        name: list_device_configuration_states,
        response: Collection<serde_json::Value>,
        path: "/managedDevices/{{RID}}/deviceConfigurationStates",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to deviceConfigurationStates for me",
        name: create_device_configuration_states,
        response: serde_json::Value,
        path: "/managedDevices/{{RID}}/deviceConfigurationStates",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get deviceConfigurationStates from me",
        name: get_device_configuration_states,
        response: serde_json::Value,
        path: "/managedDevices/{{RID}}/deviceConfigurationStates/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property deviceConfigurationStates in me",
        name: update_device_configuration_states,
        response: NoContent,
        path: "/managedDevices/{{RID}}/deviceConfigurationStates/{{id}}",
        params: 1,
        has_body: true
    });
    post!({
        doc: "# Invoke action disableLostMode",
        name: disable_lost_mode,
        response: NoContent,
        path: "/managedDevices/{{RID}}/disableLostMode",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action locateDevice",
        name: locate_device,
        response: NoContent,
        path: "/managedDevices/{{RID}}/locateDevice",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action logoutSharedAppleDeviceActiveUser",
        name: logout_shared_apple_device_active_user,
        response: NoContent,
        path: "/managedDevices/{{RID}}/logoutSharedAppleDeviceActiveUser",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action rebootNow",
        name: reboot_now,
        response: NoContent,
        path: "/managedDevices/{{RID}}/rebootNow",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action recoverPasscode",
        name: recover_passcode,
        response: NoContent,
        path: "/managedDevices/{{RID}}/recoverPasscode",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action remoteLock",
        name: remote_lock,
        response: NoContent,
        path: "/managedDevices/{{RID}}/remoteLock",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action requestRemoteAssistance",
        name: request_remote_assistance,
        response: NoContent,
        path: "/managedDevices/{{RID}}/requestRemoteAssistance",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action resetPasscode",
        name: reset_passcode,
        response: NoContent,
        path: "/managedDevices/{{RID}}/resetPasscode",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action retire",
        name: retire,
        response: NoContent,
        path: "/managedDevices/{{RID}}/retire",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action shutDown",
        name: shut_down,
        response: NoContent,
        path: "/managedDevices/{{RID}}/shutDown",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action syncDevice",
        name: sync_device,
        response: NoContent,
        path: "/managedDevices/{{RID}}/syncDevice",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action updateWindowsDeviceAccount",
        name: update_windows_device_account,
        response: NoContent,
        path: "/managedDevices/{{RID}}/updateWindowsDeviceAccount",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action windowsDefenderScan",
        name: windows_defender_scan,
        response: NoContent,
        path: "/managedDevices/{{RID}}/windowsDefenderScan",
        params: 0,
        has_body: true
    });
    post!({
        doc: "# Invoke action windowsDefenderUpdateSignatures",
        name: windows_defender_update_signatures,
        response: NoContent,
        path: "/managedDevices/{{RID}}/windowsDefenderUpdateSignatures",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Invoke action wipe",
        name: wipe,
        response: NoContent,
        path: "/managedDevices/{{RID}}/wipe",
        params: 0,
        has_body: true
    });
}
