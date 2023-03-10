#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::prelude::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(2, 20);
}

fn get_graph() -> GraphV2 {
    GraphV2::new("")
}

#[test]
fn agreement_acceptances_url() {
    let client = get_graph();
    assert_eq!(
        "/v1.0/me/agreementAcceptances".to_string(),
        client.me().list_agreement_acceptances().url().path()
    );

    assert_eq!(
        format!("/v1.0/users/{}/agreementAcceptances", ID_VEC[0]),
        client
            .user(ID_VEC[0].as_str())
            .list_agreement_acceptances()
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/users/{}/agreementAcceptances/{}",
            ID_VEC[0], ID_VEC[1]
        ),
        client
            .user(ID_VEC[0].as_str())
            .get_agreement_acceptances(ID_VEC[1].as_str())
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/agreementAcceptances/{}", ID_VEC[0]),
        client
            .agreement_acceptance(ID_VEC[0].as_str())
            .get_agreement_acceptance()
            .url()
            .path()
    );
}
