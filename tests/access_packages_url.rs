use graph_rs_sdk::client::Graph;

#[test]
fn list_access_packages() {
    let client = Graph::new("");

    let url = client
        .identity_governance()
        .entitlement_management()
        .access_packages()
        .list_access_packages()
        .url();
    assert_eq!(
        url.path(),
        "/v1.0/identityGovernance/entitlementManagement/accessPackages"
    );
}

#[test]
fn get_access_package() {
    let client = Graph::new("");

    let url = client
        .identity_governance()
        .entitlement_management()
        .access_package("accessPackageId")
        .get_access_packages()
        .url();
    assert_eq!(
        url.path(),
        "/v1.0/identityGovernance/entitlementManagement/accessPackages/accessPackageId"
    );
}
