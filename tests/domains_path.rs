#[macro_use]
extern crate lazy_static;

use graph_http::BlockingHttpClient;
use graph_rs_sdk::prelude::*;
use test_tools::{assert_url_eq, common::TestTools};

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(3, 20);
}

pub fn graph() -> Graph<BlockingHttpClient> {
    Graph::new("")
}

#[test]
fn domains() {
    let client = graph();

    client.v1().domains().list_domain();

    assert_url_eq(&client, "/domains");

    client.v1().domain(ID_VEC[0].as_str()).get_domain();

    assert_url_eq(&client, &format!("/domains/{}", ID_VEC[0]));

    client.v1().domain(ID_VEC[0].as_str()).verify();

    assert_url_eq(&client, &format!("/domains/{}/verify", ID_VEC[0]));

    client
        .v1()
        .domain(ID_VEC[0].as_str())
        .get_service_configuration_records(ID_VEC[1].as_str());

    assert_url_eq(
        &client,
        &format!(
            "/domains/{}/serviceConfigurationRecords/{}",
            ID_VEC[0], ID_VEC[1]
        ),
    );
}

#[test]
fn domains_to_id_domain() {
    let client = graph();

    client.v1().domains().id(ID_VEC[0].as_str()).get_domain();

    assert_url_eq(&client, &format!("/domains/{}", ID_VEC[0]));
}
