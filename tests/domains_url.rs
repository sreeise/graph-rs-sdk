#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::prelude::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(3, 20);
}

#[test]
fn domains() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/domains".to_string(),
        client.domains().list_domain().url().path()
    );

    assert_eq!(
        format!("/v1.0/domains/{}", ID_VEC[0]),
        client.domain(ID_VEC[0].as_str()).get_domain().url().path()
    );

    assert_eq!(
        format!("/v1.0/domains/{}/verify", ID_VEC[0]),
        client.domain(ID_VEC[0].as_str()).verify().url().path()
    );

    assert_eq!(
        format!(
            "/v1.0/domains/{}/serviceConfigurationRecords/{}",
            ID_VEC[0], ID_VEC[1]
        ),
        client
            .domain(ID_VEC[0].as_str())
            .get_service_configuration_records(ID_VEC[1].as_str())
            .url()
            .path()
    );
}

#[test]
fn domains_to_id_domain() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/domains/{}", ID_VEC[0]),
        client
            .domains()
            .id(ID_VEC[0].as_str())
            .get_domain()
            .url()
            .path()
    );
}
