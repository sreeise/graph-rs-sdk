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
        .directory()
        .administrative_unit(ID_VEC[0].as_str())
        .members()
        .create_ref_members(&String::new());

    assert_url_eq(
        &client,
        format!("/directory/administrativeUnits/{}/members/$ref", ID_VEC[0]),
    );
}
