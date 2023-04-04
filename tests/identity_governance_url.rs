#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::identity_governance::IdentityGovernanceApiClient;
use graph_rs_sdk::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(2, 20);
}

fn client() -> IdentityGovernanceApiClient {
    Graph::new("").identity_governance()
}

#[test]
fn terms_of_use_url() {
    assert_eq!(
        client().get_terms_of_use().url().path(),
        "/v1.0/identityGovernance/termsOfUse"
    );
}

#[test]
fn app_consent_url() {
    assert_eq!(
        client().app_consent().get_app_consent().url().path(),
        "/v1.0/identityGovernance/appConsent"
    );
}

#[test]
fn entitlement_management_url() {
    assert_eq!(
        client()
            .entitlement_management()
            .get_entitlement_management()
            .url()
            .path(),
        "/v1.0/identityGovernance/entitlementManagement"
    );
}

#[test]
fn access_reviews_definitions_url() {
    assert_eq!(
        client()
            .access_reviews()
            .definition(ID_VEC[0].as_str())
            .get_definitions()
            .url()
            .path(),
        format!(
            "/v1.0/identityGovernance/accessReviews/definitions/{}",
            ID_VEC[0]
        )
    );
}

#[test]
fn access_reviews_definitions_instances_url() {
    assert_eq!(
        client()
            .access_reviews()
            .definition(ID_VEC[0].as_str())
            .instance(ID_VEC[1].as_str())
            .get_instances()
            .url()
            .path(),
        format!(
            "/v1.0/identityGovernance/accessReviews/definitions/{}/instances/{}",
            ID_VEC[0], ID_VEC[1]
        )
    );
}

#[test]
fn access_reviews_definitions_instances_stages() {
    assert_eq!(
        client()
            .access_reviews()
            .definition(ID_VEC[0].as_str())
            .instance(ID_VEC[1].as_str())
            .stages()
            .list_stages()
            .url()
            .path(),
        format!(
            "/v1.0/identityGovernance/accessReviews/definitions/{}/instances/{}/stages",
            ID_VEC[0], ID_VEC[1]
        )
    );

    assert_eq!(
        client()
            .access_reviews()
            .definitions()
            .id(ID_VEC[0].as_str())
            .instance(ID_VEC[1].as_str())
            .stages()
            .list_stages()
            .url()
            .path(),
        format!(
            "/v1.0/identityGovernance/accessReviews/definitions/{}/instances/{}/stages",
            ID_VEC[0], ID_VEC[1]
        )
    );

    assert_eq!(
        client()
            .access_reviews()
            .definitions()
            .id(ID_VEC[0].as_str())
            .instances()
            .id(ID_VEC[1].as_str())
            .stages()
            .list_stages()
            .url()
            .path(),
        format!(
            "/v1.0/identityGovernance/accessReviews/definitions/{}/instances/{}/stages",
            ID_VEC[0], ID_VEC[1]
        )
    );
}

#[test]
fn access_reviews_definitions_instances_stages_url() {
    assert_eq!(
        client()
            .access_reviews()
            .definition(ID_VEC[0].as_str())
            .instance(ID_VEC[1].as_str())
            .stage(ID_VEC[0].as_str())
            .get_stages()
            .url()
            .path(),
        format!(
            "/v1.0/identityGovernance/accessReviews/definitions/{}/instances/{}/stages/{}",
            ID_VEC[0], ID_VEC[1], ID_VEC[0]
        )
    );

    assert_eq!(
        client()
            .access_reviews()
            .definition(ID_VEC[0].as_str())
            .instance(ID_VEC[1].as_str())
            .stages()
            .id(ID_VEC[0].as_str())
            .get_stages()
            .url()
            .path(),
        format!(
            "/v1.0/identityGovernance/accessReviews/definitions/{}/instances/{}/stages/{}",
            ID_VEC[0], ID_VEC[1], ID_VEC[0]
        )
    );

    assert_eq!(
        client()
            .access_reviews()
            .definitions()
            .id(ID_VEC[0].as_str())
            .instance(ID_VEC[1].as_str())
            .stages()
            .id(ID_VEC[0].as_str())
            .get_stages()
            .url()
            .path(),
        format!(
            "/v1.0/identityGovernance/accessReviews/definitions/{}/instances/{}/stages/{}",
            ID_VEC[0], ID_VEC[1], ID_VEC[0]
        )
    );

    assert_eq!(
        client()
            .access_reviews()
            .definitions()
            .id(ID_VEC[0].as_str())
            .instances()
            .id(ID_VEC[1].as_str())
            .stages()
            .id(ID_VEC[0].as_str())
            .get_stages()
            .url()
            .path(),
        format!(
            "/v1.0/identityGovernance/accessReviews/definitions/{}/instances/{}/stages/{}",
            ID_VEC[0], ID_VEC[1], ID_VEC[0]
        )
    );
}

#[test]
fn entitlement_management_catalogs_url() {
    assert_eq!(
        client()
            .entitlement_management()
            .catalog(ID_VEC[0].as_str())
            .list_access_packages()
            .url()
            .path(),
        format!(
            "/v1.0/identityGovernance/entitlementManagement/catalogs/{}/accessPackages",
            ID_VEC[0]
        )
    );

    assert_eq!(
        client()
            .entitlement_management()
            .catalog(ID_VEC[0].as_str())
            .access_package(ID_VEC[1].as_str())
            .get_access_packages()
            .url()
            .path(),
        format!(
            "/v1.0/identityGovernance/entitlementManagement/catalogs/{}/accessPackages/{}",
            ID_VEC[0], ID_VEC[1]
        )
    );
}

#[test]
fn connected_organization_url() {
    assert_eq!(
        client()
            .entitlement_management()
            .connected_organizations()
            .list_connected_organizations()
            .url()
            .path(),
        "/v1.0/identityGovernance/entitlementManagement/connectedOrganizations".to_string()
    );

    assert_eq!(
        client()
            .entitlement_management()
            .connected_organization(ID_VEC[0].as_str())
            .get_connected_organizations()
            .url()
            .path(),
        format!(
            "/v1.0/identityGovernance/entitlementManagement/connectedOrganizations/{}",
            ID_VEC[0]
        )
    );

    assert_eq!(
        client()
            .entitlement_management()
            .connected_organization(ID_VEC[0].as_str())
            .internal_sponsors()
            .get_internal_sponsors_count()
            .url()
            .path(),
        format!(
            "/v1.0/identityGovernance/entitlementManagement/connectedOrganizations/{}/internalSponsors/$count",
            ID_VEC[0]
        )
    );

    assert_eq!(
        client()
            .entitlement_management()
            .connected_organization(ID_VEC[0].as_str())
            .external_sponsors()
            .get_external_sponsors_count()
            .url()
            .path(),
        format!(
            "/v1.0/identityGovernance/entitlementManagement/connectedOrganizations/{}/externalSponsors/$count",
            ID_VEC[0]
        )
    );

    assert_eq!(
        client()
            .entitlement_management()
            .connected_organization(ID_VEC[0].as_str())
            .external_sponsors()
            .delete_ref_external_sponsors(ID_VEC[1].as_str())
            .url()
            .path(),
        format!(
            "/v1.0/identityGovernance/entitlementManagement/connectedOrganizations/{}/externalSponsors/{}/$ref",
            ID_VEC[0], ID_VEC[1]
        )
    );

    assert_eq!(
        client()
            .entitlement_management()
            .connected_organization(ID_VEC[0].as_str())
            .internal_sponsors()
            .delete_ref_internal_sponsors(ID_VEC[1].as_str())
            .url()
            .path(),
        format!(
            "/v1.0/identityGovernance/entitlementManagement/connectedOrganizations/{}/internalSponsors/{}/$ref",
            ID_VEC[0], ID_VEC[1]
        )
    );
}
