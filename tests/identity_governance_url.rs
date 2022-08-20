#[macro_use]
extern crate lazy_static;

use graph_http::BlockingHttpClient;
use graph_rs_sdk::prelude::*;
use test_tools::assert_url_eq;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(2, 20);
}

fn get_graph() -> Graph<BlockingHttpClient> {
    Graph::new("")
}

#[test]
fn terms_of_use_url() {
    let client = get_graph();
    client
        .v1()
        .identity_governance()
        .terms_of_use()
        .get_terms_of_use();

    assert_url_eq(&client, "/identityGovernance/termsOfUse");
}

#[test]
fn app_consent_url() {
    let client = get_graph();
    client
        .v1()
        .identity_governance()
        .app_consent()
        .get_app_consent();

    assert_url_eq(&client, "/identityGovernance/appConsent");
}

#[test]
fn entitlement_management_url() {
    let client = get_graph();
    client
        .v1()
        .identity_governance()
        .entitlement_management()
        .get_entitlement_management();

    assert_url_eq(&client, "/identityGovernance/entitlementManagement");
}

#[test]
fn access_reviews_definitions_url() {
    let client = get_graph();
    client
        .v1()
        .identity_governance()
        .access_reviews()
        .definition(ID_VEC[0].as_str())
        .get_definitions();

    assert_url_eq(
        &client,
        format!(
            "/identityGovernance/accessReviews/definitions/{}",
            ID_VEC[0]
        ),
    );
}

#[test]
fn access_reviews_definitions_instances_url() {
    let client = get_graph();
    client
        .v1()
        .identity_governance()
        .access_reviews()
        .definition(ID_VEC[0].as_str())
        .instance(ID_VEC[1].as_str())
        .get_instances();

    assert_url_eq(
        &client,
        format!(
            "/identityGovernance/accessReviews/definitions/{}/instances/{}",
            ID_VEC[0], ID_VEC[1]
        ),
    );
}

#[test]
fn access_reviews_definitions_instances_stages_url() {
    let client = get_graph();
    client
        .v1()
        .identity_governance()
        .access_reviews()
        .definition(ID_VEC[0].as_str())
        .instance(ID_VEC[1].as_str())
        .stage(ID_VEC[0].as_str())
        .get_stages();

    assert_url_eq(
        &client,
        format!(
            "/identityGovernance/accessReviews/definitions/{}/instances/{}/stages/{}",
            ID_VEC[0], ID_VEC[1], ID_VEC[0]
        ),
    );

    client
        .v1()
        .identity_governance()
        .access_reviews()
        .definition(ID_VEC[0].as_str())
        .instance(ID_VEC[1].as_str())
        .stage(ID_VEC[0].as_str())
        .list_decisions();

    assert_url_eq(
        &client,
        format!(
            "/identityGovernance/accessReviews/definitions/{}/instances/{}/stages/{}/decisions",
            ID_VEC[0], ID_VEC[1], ID_VEC[0]
        ),
    );
}
