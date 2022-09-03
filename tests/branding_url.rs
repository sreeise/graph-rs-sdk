#[macro_use]
extern crate lazy_static;

use graph_http::BlockingHttpClient;
use graph_rs_sdk::prelude::*;
use test_tools::assert_url_eq;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(2, 20);
}

pub fn graph() -> Graph<BlockingHttpClient> {
    Graph::new("")
}

#[test]
fn audit_logs_url() {
    let client = graph();

    client.v1().branding().get_localizations_count();

    assert_url_eq(&client, "/branding/localizations/$count");
}
