// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    ManagedDevicesApiClient,
    ManagedDevicesIdApiClient,
    ResourceIdentity::ManagedDevices
);

impl ManagedDevicesApiClient {
    post!(
        doc: "Create new navigation property to managedDevices for users",
        name: create_managed_devices,
        path: "/managedDevices",
        body: true
    );
    get!(
        doc: "Get managedDevices from users",
        name: list_managed_devices,
        path: "/managedDevices"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_managed_devices_count,
        path: "/managedDevices/$count"
    );
}

impl ManagedDevicesIdApiClient {
    delete!(
        doc: "Delete navigation property managedDevices for users",
        name: delete_managed_devices,
        path: "/managedDevices/{{RID}}"
    );
    get!(
        doc: "Get managedDevices from users",
        name: get_managed_devices,
        path: "/managedDevices/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property managedDevices in users",
        name: update_managed_devices,
        path: "/managedDevices/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action bypassActivationLock",
        name: bypass_activation_lock,
        path: "/managedDevices/{{RID}}/bypassActivationLock"
    );
    post!(
        doc: "Invoke action cleanWindowsDevice",
        name: clean_windows_device,
        path: "/managedDevices/{{RID}}/cleanWindowsDevice",
        body: true
    );
    post!(
        doc: "Invoke action deleteUserFromSharedAppleDevice",
        name: delete_user_from_shared_apple_device,
        path: "/managedDevices/{{RID}}/deleteUserFromSharedAppleDevice",
        body: true
    );
    delete!(
        doc: "Delete navigation property deviceCategory for users",
        name: delete_device_category,
        path: "/managedDevices/{{RID}}/deviceCategory"
    );
    get!(
        doc: "Get deviceCategory from users",
        name: get_device_category,
        path: "/managedDevices/{{RID}}/deviceCategory"
    );
    patch!(
        doc: "Update the navigation property deviceCategory in users",
        name: update_device_category,
        path: "/managedDevices/{{RID}}/deviceCategory",
        body: true
    );
    post!(
        doc: "Create new navigation property to deviceCompliancePolicyStates for users",
        name: create_device_compliance_policy_states,
        path: "/managedDevices/{{RID}}/deviceCompliancePolicyStates",
        body: true
    );
    get!(
        doc: "Get deviceCompliancePolicyStates from users",
        name: list_device_compliance_policy_states,
        path: "/managedDevices/{{RID}}/deviceCompliancePolicyStates"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_compliance_policy_states_count,
        path: "/managedDevices/{{RID}}/deviceCompliancePolicyStates/$count"
    );
    delete!(
        doc: "Delete navigation property deviceCompliancePolicyStates for users",
        name: delete_device_compliance_policy_states,
        path: "/managedDevices/{{RID}}/deviceCompliancePolicyStates/{{id}}",
        params: device_compliance_policy_state_id
    );
    get!(
        doc: "Get deviceCompliancePolicyStates from users",
        name: get_device_compliance_policy_states,
        path: "/managedDevices/{{RID}}/deviceCompliancePolicyStates/{{id}}",
        params: device_compliance_policy_state_id
    );
    patch!(
        doc: "Update the navigation property deviceCompliancePolicyStates in users",
        name: update_device_compliance_policy_states,
        path: "/managedDevices/{{RID}}/deviceCompliancePolicyStates/{{id}}",
        body: true,
        params: device_compliance_policy_state_id
    );
    post!(
        doc: "Create new navigation property to deviceConfigurationStates for users",
        name: create_device_configuration_states,
        path: "/managedDevices/{{RID}}/deviceConfigurationStates",
        body: true
    );
    get!(
        doc: "Get deviceConfigurationStates from users",
        name: list_device_configuration_states,
        path: "/managedDevices/{{RID}}/deviceConfigurationStates"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_device_configuration_states_count,
        path: "/managedDevices/{{RID}}/deviceConfigurationStates/$count"
    );
    delete!(
        doc: "Delete navigation property deviceConfigurationStates for users",
        name: delete_device_configuration_states,
        path: "/managedDevices/{{RID}}/deviceConfigurationStates/{{id}}",
        params: device_configuration_state_id
    );
    get!(
        doc: "Get deviceConfigurationStates from users",
        name: get_device_configuration_states,
        path: "/managedDevices/{{RID}}/deviceConfigurationStates/{{id}}",
        params: device_configuration_state_id
    );
    patch!(
        doc: "Update the navigation property deviceConfigurationStates in users",
        name: update_device_configuration_states,
        path: "/managedDevices/{{RID}}/deviceConfigurationStates/{{id}}",
        body: true,
        params: device_configuration_state_id
    );
    post!(
        doc: "Invoke action disableLostMode",
        name: disable_lost_mode,
        path: "/managedDevices/{{RID}}/disableLostMode"
    );
    post!(
        doc: "Invoke action locateDevice",
        name: locate_device,
        path: "/managedDevices/{{RID}}/locateDevice"
    );
    post!(
        doc: "Invoke action logoutSharedAppleDeviceActiveUser",
        name: logout_shared_apple_device_active_user,
        path: "/managedDevices/{{RID}}/logoutSharedAppleDeviceActiveUser"
    );
    post!(
        doc: "Invoke action rebootNow",
        name: reboot_now,
        path: "/managedDevices/{{RID}}/rebootNow"
    );
    post!(
        doc: "Invoke action recoverPasscode",
        name: recover_passcode,
        path: "/managedDevices/{{RID}}/recoverPasscode"
    );
    post!(
        doc: "Invoke action remoteLock",
        name: remote_lock,
        path: "/managedDevices/{{RID}}/remoteLock"
    );
    post!(
        doc: "Invoke action requestRemoteAssistance",
        name: request_remote_assistance,
        path: "/managedDevices/{{RID}}/requestRemoteAssistance"
    );
    post!(
        doc: "Invoke action resetPasscode",
        name: reset_passcode,
        path: "/managedDevices/{{RID}}/resetPasscode"
    );
    post!(
        doc: "Invoke action retire",
        name: retire,
        path: "/managedDevices/{{RID}}/retire"
    );
    post!(
        doc: "Invoke action shutDown",
        name: shut_down,
        path: "/managedDevices/{{RID}}/shutDown"
    );
    post!(
        doc: "Invoke action syncDevice",
        name: sync_device,
        path: "/managedDevices/{{RID}}/syncDevice"
    );
    post!(
        doc: "Invoke action updateWindowsDeviceAccount",
        name: update_windows_device_account,
        path: "/managedDevices/{{RID}}/updateWindowsDeviceAccount",
        body: true
    );
    get!(
        doc: "Get users from users",
        name: list_users,
        path: "/managedDevices/{{RID}}/users"
    );
    post!(
        doc: "Invoke action windowsDefenderScan",
        name: windows_defender_scan,
        path: "/managedDevices/{{RID}}/windowsDefenderScan",
        body: true
    );
    post!(
        doc: "Invoke action windowsDefenderUpdateSignatures",
        name: windows_defender_update_signatures,
        path: "/managedDevices/{{RID}}/windowsDefenderUpdateSignatures"
    );
    post!(
        doc: "Invoke action wipe",
        name: wipe,
        path: "/managedDevices/{{RID}}/wipe",
        body: true
    );
}
