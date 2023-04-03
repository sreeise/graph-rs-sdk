#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(2, 20);
}

#[test]
fn audit_logs_url() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/branding/localizations/$count".to_string(),
        client.branding().get_localizations_count().url().path()
    );
}
