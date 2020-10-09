use graph_codegen::traits::RequestParser;

#[test]
fn operation_map_parse() {
    let operation_id = "deviceManagement.detectedApps.managedDevices.disableLostMode";
    let operation_mapping = operation_id.operation_mapping();
    assert_eq!(
        String::from("deviceManagement.detectedApps.managedDevices"),
        operation_mapping
    );

    let operation_id = "deviceManagement.managedDevices.updateWindowsDeviceAccount";
    let operation_mapping = operation_id.operation_mapping();
    assert_eq!(
        String::from("deviceManagement.managedDevices"),
        operation_mapping
    );

    let operation_id = "deviceManagement.UpdateApplePushNotificationCertificate";
    let operation_mapping = operation_id.operation_mapping();
    assert_eq!(String::from("deviceManagement"), operation_mapping);
}
