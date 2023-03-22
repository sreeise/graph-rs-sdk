#[macro_use]
extern crate lazy_static;

use graph_http::BlockingHttpClient;
use graph_rs_sdk::prelude::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(2, 20);
}

#[test]
fn terms_of_use_url() {
    let client = Graph::new("");

    assert_eq!(
        format!(
            "/v1.0/directory/administrativeUnits/{}/members/$ref",
            ID_VEC[0]
        ),
        client
            .directory()
            .administrative_unit(ID_VEC[0].as_str())
            .members()
            .create_ref_members(&String::new())
            .url()
            .path()
    );
}
