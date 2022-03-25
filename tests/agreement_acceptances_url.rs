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
fn agreement_acceptances_url() {
    let client = get_graph();
    client
        .v1()
        .me()
        .agreement_acceptances()
        .list_agreement_acceptance();

    assert_url_eq(&client, "/me/agreementAcceptances");

    client
        .v1()
        .user(ID_VEC[0].as_str())
        .agreement_acceptances()
        .list_agreement_acceptance();

    assert_url_eq(
        &client,
        &format!("/users/{}/agreementAcceptances", ID_VEC[0]),
    );

    client
        .v1()
        .user(ID_VEC[0].as_str())
        .agreement_acceptance(ID_VEC[1].as_str())
        .get_agreement_acceptance();

    assert_url_eq(
        &client,
        &format!("/users/{}/agreementAcceptances/{}", ID_VEC[0], ID_VEC[1]),
    );

    client
        .v1()
        .agreement_acceptance(ID_VEC[0].as_str())
        .update_agreement_acceptance(&serde_json::json!({}));

    assert_url_eq(&client, &format!("/agreementAcceptances/{}", ID_VEC[0]));
}
